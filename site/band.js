/* ---------------------------------------------------------------------------
   band.js - the mossflux landing-page instrument.

   One canvas engine, two views, both running on the SAME 3,072 real ChEMBL-37
   fingerprints in site/data/arena.js.

     mountArena(canvas)      the popcount-sorted arena with the BitBound band
     mountTranspose(canvas)  the same rows read as bit columns, rarest first

   Nothing here is a picture of a computation. Every popcount, every Tanimoto,
   every band edge and every early-abandon decision is computed in the browser
   from the packed bits, with the same integer arithmetic the Rust engine uses.

   Two rules this file keeps:
     1. Content is never hidden. Every number this file writes is already in the
        HTML at its true value. If this script never runs, the page still reads.
     2. Nothing is asserted that is not checked. On every threshold change the
        arena re-runs an exactness gate: no row outside the band may clear the
        threshold. A failure is reported to the console, not swallowed.
   ------------------------------------------------------------------------- */

(function (global) {
  'use strict';

  var A = global.MOSSFLUX_ARENA;
  if (!A || !global.document) { return; }

  var doc = global.document;
  var reduceMotion = global.matchMedia
    ? global.matchMedia('(prefers-reduced-motion: reduce)')
    : { matches: false };

  /* =======================================================================
     1. DECODE
     Base64 -> packed 32-bit lanes. 256 bits per row, 8 lanes per row,
     little-endian byte order, exactly as the FPS file stores them.
     ===================================================================== */

  var N = A.n;                      // 3072 rows
  var BITS = A.bits;                // 256
  var LANES = BITS / 32;            // 8
  var BYTES = BITS / 8;             // 32

  var words = new Uint32Array(N * LANES);
  (function decode() {
    var raw = global.atob(A.fps);
    for (var r = 0; r < N; r++) {
      for (var w = 0; w < LANES; w++) {
        var o = r * BYTES + w * 4;
        words[r * LANES + w] =
          (raw.charCodeAt(o)) |
          (raw.charCodeAt(o + 1) << 8) |
          (raw.charCodeAt(o + 2) << 16) |
          (raw.charCodeAt(o + 3) << 24);
      }
    }
  })();

  var ids = A.ids.split(',');

  /* =======================================================================
     2. EXACT KERNELS
     ===================================================================== */

  /* SWAR population count. The Rust engine calls u64::count_ones and lets LLVM
     pick POPCNT / VPOPCNTQ / CNT; JavaScript has no such intrinsic, so this is
     the portable equivalent. The RESULT is identical, which is all that matters
     for what the plot shows. */
  function popcount32(x) {
    x = x - ((x >>> 1) & 0x55555555);
    x = (x & 0x33333333) + ((x >>> 2) & 0x33333333);
    x = (x + (x >>> 4)) & 0x0f0f0f0f;
    return (Math.imul(x, 0x01010101) >>> 24);
  }

  /* Every row's own popcount is known before any query arrives, so a comparison
     needs one popcount (the intersection), not two. Same trick as the engine. */
  var pop = new Uint16Array(N);
  (function popcounts() {
    for (var r = 0; r < N; r++) {
      var s = 0, base = r * LANES;
      for (var w = 0; w < LANES; w++) { s += popcount32(words[base + w]); }
      pop[r] = s;
    }
  })();

  /* The arena is popcount-sorted. Check it rather than trust it. */
  var sortedOK = true;
  for (var si = 1; si < N; si++) { if (pop[si] < pop[si - 1]) { sortedOK = false; } }
  if (!sortedOK) { global.console.error('mossflux: arena is not popcount-sorted'); }

  /* Tanimoto = |A and B| / |A or B| = c / (a + b - c). */
  function tanimoto(i, j) {
    var c = 0, bi = i * LANES, bj = j * LANES;
    for (var w = 0; w < LANES; w++) { c += popcount32(words[bi + w] & words[bj + w]); }
    var u = pop[i] + pop[j] - c;
    return u ? c / u : 0;
  }

  /* Intersection popcount only, for the transposed view. */
  function inter(i, j) {
    var c = 0, bi = i * LANES, bj = j * LANES;
    for (var w = 0; w < LANES; w++) { c += popcount32(words[bi + w] & words[bj + w]); }
    return c;
  }

  /* Swamidass-Baldi BitBound, in integer arithmetic.

     The threshold becomes a rational num/den that always rounds DOWN. Rounding
     down is not a nicety: it guarantees the band is a superset of what the user
     asked for, so it can never drop a valid hit. Rounding to nearest would fail
     on the exact-equality boundary (t = 2/3), and floating-point division would
     fail on the documented a=396, t=0.55 case where a/t lands on 719.9999999. */
  var DEN = 1000000;
  function bandFor(a, t) {
    var num = Math.floor(t * DEN);
    if (num <= 0) { return { lo: 0, hi: BITS, num: 0 }; }
    return {
      lo: Math.ceil(a * num / DEN),
      hi: Math.floor(a * DEN / num),
      num: num
    };
  }

  /* One pass over every row: band membership, hit count, and the exactness gate.
     The gate is the point. If any row OUTSIDE the band clears the threshold, the
     band is unsound and the page says so out loud. */
  var gateFailed = false;
  function scan(q, t) {
    var a = pop[q];
    var b = bandFor(a, t);
    var cand = 0, hits = 0, violations = 0;
    var hitRows = [];
    for (var r = 0; r < N; r++) {
      var inBand = pop[r] >= b.lo && pop[r] <= b.hi;
      if (inBand) { cand++; }
      var s = tanimoto(q, r);
      if (s >= t) {
        hits++;
        if (hitRows.length < 64) { hitRows.push(r); }
        if (!inBand) { violations++; }
      }
    }
    if (violations && !gateFailed) {
      gateFailed = true;
      global.console.error(
        'mossflux exactness gate FAILED: ' + violations +
        ' row(s) outside the BitBound band cleared t=' + t
      );
    }
    return { a: a, lo: b.lo, hi: b.hi, cand: cand, hits: hits, hitRows: hitRows };
  }

  /* First row index whose popcount is >= p. The arena is sorted, so this is a
     binary search, which is why the band is one contiguous slice. */
  function lowerBound(p) {
    var lo = 0, hi = N;
    while (lo < hi) {
      var mid = (lo + hi) >> 1;
      if (pop[mid] < p) { lo = mid + 1; } else { hi = mid; }
    }
    return lo;
  }

  /* =======================================================================
     3. PALETTE  (single source of truth: the CSS custom properties)
     ===================================================================== */

  var css = global.getComputedStyle(doc.documentElement);
  function hex(name) { return css.getPropertyValue(name).trim() || '#DDE4D7'; }
  function rgba(h, alpha) {
    var s = h.replace('#', '');
    if (s.length === 3) { s = s[0] + s[0] + s[1] + s[1] + s[2] + s[2]; }
    var v = parseInt(s, 16);
    return 'rgba(' + ((v >> 16) & 255) + ',' + ((v >> 8) & 255) + ',' + (v & 255) + ',' + alpha + ')';
  }
  var C = {
    bone: hex('--bone'),
    boneDim: hex('--bone-dim'),
    boneFaint: hex('--bone-faint'),
    amber: hex('--amber'),
    amberLit: hex('--amber-lit'),
    amberDim: hex('--amber-dim')
  };

  /* Both canvases sit on the page's own content measure, so the axis inset is
     measured from the canvas's own left edge and every mark it carries lands
     inside the same margins as the type. The width is READ FROM --axis, the one
     value the CSS also uses to draw the tick rail's rule with no script, so the
     scripted and unscripted rails can never drift apart. */
  function axisWidth() {
    var v = parseFloat(css.getPropertyValue('--axis'));
    return (v > 0 && v < 200) ? v : 56;
  }
  function gutterFor() { return axisWidth(); }
  function reachFor() { return axisWidth() < 56 ? 12 : 16; }

  var DATA_FONT = 'Tabular, ui-monospace, SFMono-Regular, Menlo, monospace';

  /* =======================================================================
     4. SHARED CANVAS PLUMBING
     Device-pixel-ratio aware, and paused when off screen or when the tab is
     hidden. IntersectionObserver is used for PAUSING ONLY, never for revealing.
     ===================================================================== */

  function fitCanvas(cv) {
    var dpr = Math.min(global.devicePixelRatio || 1, 2);
    var box = cv.getBoundingClientRect();
    var w = Math.max(1, Math.round(box.width));
    var h = Math.max(1, Math.round(box.height));
    if (cv.width !== Math.round(w * dpr) || cv.height !== Math.round(h * dpr)) {
      cv.width = Math.round(w * dpr);
      cv.height = Math.round(h * dpr);
    }
    var ctx = cv.getContext('2d');
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    return { ctx: ctx, w: w, h: h };
  }

  function whenVisible(el, onEnter, onLeave) {
    if (!global.IntersectionObserver) { onEnter(); return; }
    var io = new global.IntersectionObserver(function (entries) {
      for (var i = 0; i < entries.length; i++) {
        if (entries[i].isIntersecting) { onEnter(); } else { onLeave(); }
      }
    }, { rootMargin: '120px' });
    io.observe(el);
  }

  /* A small tween on requestAnimationFrame, eased by a cubic-bezier the way CSS
     eases. It degrades to an instant set under reduced motion, so no value on
     the page can ever be stranded mid-tween. */
  function cubicBezier(p) {
    var x1 = p[0], y1 = p[1], x2 = p[2], y2 = p[3];
    function bx(t) { var u = 1 - t; return 3 * u * u * t * x1 + 3 * u * t * t * x2 + t * t * t; }
    function by(t) { var u = 1 - t; return 3 * u * u * t * y1 + 3 * u * t * t * y2 + t * t * t; }
    function dx(t) { var u = 1 - t; return 3 * u * u * x1 + 6 * u * t * (x2 - x1) + 3 * t * t * (1 - x2); }
    return function (x) {
      var t = x, i;
      for (i = 0; i < 8; i++) {
        var e = bx(t) - x, d = dx(t);
        if (Math.abs(e) < 1e-5 || !d) { break; }
        t -= e / d;
      }
      if (!(t >= 0 && t <= 1) || Math.abs(bx(t) - x) > 1e-3) {
        var lo = 0, hi = 1;
        for (i = 0; i < 30; i++) { t = (lo + hi) / 2; if (bx(t) < x) { lo = t; } else { hi = t; } }
      }
      return by(t);
    };
  }

  function tween(from, to, ms, ease, onUpdate, onDone) {
    if (reduceMotion.matches || !global.requestAnimationFrame) {
      onUpdate(to);
      if (onDone) { onDone(); }
      return null;
    }
    var curve = cubicBezier(ease), t0 = 0, raf = 0, stopped = false;
    function step(now) {
      if (stopped) { return; }
      if (!t0) { t0 = now; }
      var k = Math.min(1, (now - t0) / ms);
      onUpdate(from + (to - from) * curve(k));
      if (k < 1) { raf = global.requestAnimationFrame(step); } else if (onDone) { onDone(); }
    }
    raf = global.requestAnimationFrame(step);
    return { stop: function () { stopped = true; global.cancelAnimationFrame(raf); } };
  }

  /* =======================================================================
     5. THE ARENA  (hero view)
     x = bit position, 256 columns.  y = record index, popcount-sorted.
     ===================================================================== */

  /* Twelve real ChEMBL rows spanning popcount 38 to 112. The rotation opens on
     the honest weak case: at popcount 47, near this arena's median, the band at
     t=0.700 still admits 85% of the records. The tight bands come second. */
  var QUERIES = [1524, 3011, 962, 3066, 2414, 1586, 3054, 654, 3062, 1936, 2136, 2275];
  var ROW_PITCH = 3;   // 2px mark + 1px gap
  var MARK_H = 2;

  function mountArena(cv, opts) {
    var state = {
      qi: 0,                   // index into QUERIES
      q: QUERIES[0],
      t: opts.t,
      stats: null,
      prev: null,              // previous query's stats, for the crossfade
      mix: 1                   // 0 = previous query, 1 = current
    };
    state.stats = scan(state.q, state.t);

    var live = true, dirty = true, raf = 0;

    function draw() {
      var f = fitCanvas(cv);
      var ctx = f.ctx, W = f.w, H = f.h;
      var g = gutterFor(W);
      var reach = reachFor(W);
      var colW = (W - g) / BITS;
      var markW = Math.max(1, Math.floor(colW));
      var R = Math.max(40, Math.min(260, Math.floor(H / ROW_PITCH)));

      ctx.clearRect(0, 0, W, H);

      var s = state.stats;
      var p = state.prev;
      var mix = state.mix;

      /* The band edges sweep between the old and the new query, so rows leave
         and enter the band as the line moves, instead of popping. */
      var lo = p ? p.lo + (s.lo - p.lo) * mix : s.lo;
      var hi = p ? p.hi + (s.hi - p.hi) * mix : s.hi;

      var hitSet = Object.create(null);
      var i;
      for (i = 0; i < s.hitRows.length; i++) { hitSet[s.hitRows[i]] = mix; }
      if (p) {
        for (i = 0; i < p.hitRows.length; i++) {
          if (hitSet[p.hitRows[i]] === undefined) { hitSet[p.hitRows[i]] = 1 - mix; }
        }
      }

      /* Which arena rows get drawn. The plot samples R rows across all 3,072,
         then forces the query row and every hit row into the nearest free slot
         so the returned set is always visible. The caption says so. */
      var slots = new Int32Array(R);
      var forced = new Uint8Array(R);
      for (i = 0; i < R; i++) { slots[i] = Math.floor(i * N / R); }
      function place(row) {
        var want = Math.min(R - 1, Math.max(0, Math.round(row * R / N)));
        for (var d = 0; d < R; d++) {
          var up = want - d, dn = want + d;
          if (up >= 0 && !forced[up]) { slots[up] = row; forced[up] = 1; return; }
          if (dn < R && !forced[dn]) { slots[dn] = row; forced[dn] = 1; return; }
        }
      }
      place(state.q);
      for (i = 0; i < s.hitRows.length; i++) { if (s.hitRows[i] !== state.q) { place(s.hitRows[i]); } }

      var colOut = rgba(C.bone, 0.09);   /* outside the band: never read at all */
      var colIn = rgba(C.bone, 0.68);    /* inside the band: scanned, rejected */

      for (i = 0; i < R; i++) {
        var row = slots[i];
        var y = i * ROW_PITCH;
        if (y > H) { break; }
        var isQuery = row === state.q;
        var hitAlpha = hitSet[row];
        var inBand = pop[row] >= lo && pop[row] <= hi;

        if (isQuery) {
          ctx.fillStyle = rgba(C.bone, 0.18);
          ctx.fillRect(g, y - 1, W - g, MARK_H + 2);
          ctx.fillStyle = C.bone;
        } else if (hitAlpha !== undefined && hitAlpha > 0.02) {
          /* A hit is the one thing the search returns, so it has to be the one
             thing the eye finds: a warm bar the full width of the row, with the
             molecule's own bits lit on top of it. */
          ctx.fillStyle = rgba(C.amberDim, 0.45 * hitAlpha);
          ctx.fillRect(g, y - 1, W - g, MARK_H + 2);
          ctx.fillStyle = rgba(C.amberLit, 0.55 + 0.45 * hitAlpha);
        } else {
          ctx.fillStyle = inBand ? colIn : colOut;
        }

        var base = row * LANES;
        for (var w = 0; w < LANES; w++) {
          var word = words[base + w];
          if (!word) { continue; }
          for (var b = 0; b < 32; b++) {
            if (word & (1 << b)) {
              ctx.fillRect(g + (w * 32 + b) * colW, y, markW, MARK_H);
            }
          }
        }
      }

      /* ---- the axis in the left gutter ---- */
      var drawnH = Math.min(H, R * ROW_PITCH);
      function yForPop(p0) {
        var idx = lowerBound(p0);
        return Math.max(0, Math.min(drawnH - 1, (idx / N) * drawnH));
      }

      ctx.font = '10.5px ' + DATA_FONT;
      ctx.textBaseline = 'middle';
      ctx.textAlign = 'right';

      /* Clear the cut. The axis inset is fixed, but the labels inside it are
         not: the widest popcount this arena can print, at the narrowest width,
         would run off the canvas's own left edge. So measure that label and
         pull the ticks in until every glyph keeps a real margin, rather than
         trusting the inset. Rules and ticks share the one value, so the axis
         still reads as a single scale. */
      var MAX_TICK = 9;
      var lead = Math.max(0, Math.min(
        reach,
        (g - 2) - ctx.measureText(String(pop[N - 1])).width - 5 - MAX_TICK
      ));

      /* Every label in the gutter is a popcount, so the axis reads as one scale
         and never needs a word to explain itself. */
      var labels = [];
      function label(y, text, colour, tickW) {
        /* the glyph is centred on its tick, so the two end labels are held
           clear of the canvas edge rather than sliced by it */
        labels.push({
          y: y, ty: Math.max(7, Math.min(H - 7, y)),
          text: text, colour: colour, tickW: tickW
        });
      }
      label(0.5, String(pop[0]), rgba(C.boneDim, 0.85), 4);
      label(drawnH - 1.5, String(pop[N - 1]), rgba(C.boneDim, 0.85), 4);

      var yLo = yForPop(Math.round(lo));
      var yHi = yForPop(Math.round(hi) + 1);

      /* Band rules: 1px, full width, running into the gutter as far as the ticks. */
      ctx.fillStyle = rgba(C.amberDim, 0.95);
      ctx.fillRect(g - lead, Math.round(yLo) + 0.5, W - g + lead, 1);
      ctx.fillRect(g - lead, Math.round(yHi) + 0.5, W - g + lead, 1);

      /* A bound only gets a number when it lands inside the arena's own range.
         hi can legitimately exceed the widest molecule here; the query card
         still prints the true value, but the axis does not invent a tick. */
      if (lo >= pop[0]) { label(yLo, String(Math.round(lo)), C.amber, 6); }
      if (hi <= pop[N - 1]) { label(yHi, String(Math.round(hi)), C.amber, 6); }

      /* The query's own row, marked at its true popcount. The tick stops at the
         plot's edge: the row itself is already drawn in full ink, and a rule
         laid over it would hide the very bits the reader came to see. */
      var yq = 0;
      for (i = 0; i < R; i++) { if (slots[i] === state.q) { yq = i * ROW_PITCH; } }
      label(yq, String(s.a), C.bone, 9);

      /* When two ticks land on the same few pixels, the more specific one wins. */
      var kept = [];
      for (i = 0; i < labels.length; i++) {
        var L = labels[i], clash = false;
        for (var k = 0; k < labels.length; k++) {
          if (k !== i && labels[k].tickW > L.tickW && Math.abs(labels[k].y - L.y) < 13) { clash = true; }
        }
        if (!clash) { kept.push(L); }
      }
      ctx.textAlign = 'right';
      for (i = 0; i < kept.length; i++) {
        var lb = kept[i];
        ctx.fillStyle = lb.colour;
        ctx.fillRect(g - lead - lb.tickW, Math.round(lb.y) + 0.5, lb.tickW, 1);
        ctx.fillText(lb.text, g - lead - lb.tickW - 5, lb.ty);
      }
    }

    function frame() {
      raf = 0;
      if (!dirty) { return; }
      dirty = false;
      draw();
    }
    function invalidate() {
      dirty = true;
      if (!raf && live) { raf = global.requestAnimationFrame(frame); }
    }

    whenVisible(cv, function () { live = true; invalidate(); }, function () { live = false; });
    doc.addEventListener('visibilitychange', function () {
      live = !doc.hidden;
      if (live) { invalidate(); }
    });
    global.addEventListener('resize', invalidate);
    if (doc.fonts && doc.fonts.ready) { doc.fonts.ready.then(invalidate); }

    var sweep = null;
    function cancelSweep() {
      if (sweep && typeof sweep.stop === 'function') { try { sweep.stop(); } catch (e) { /* ignore */ } }
      sweep = null;
    }

    return {
      state: state,
      invalidate: invalidate,
      setThreshold: function (t) {
        cancelSweep();
        state.t = t;
        state.prev = null;
        state.mix = 1;
        state.stats = scan(state.q, t);
        invalidate();
        return state.stats;
      },
      setQuery: function (qi) {
        cancelSweep();
        var prev = state.stats;
        state.qi = qi;
        state.q = QUERIES[qi];
        state.stats = scan(state.q, state.t);
        state.prev = prev;
        state.mix = 0;
        sweep = tween(0, 1, 900, [0.16, 0.84, 0.24, 1], function (v) {
          state.mix = v;
          invalidate();
        }, function () {
          sweep = null;
          state.prev = null;
          state.mix = 1;
          invalidate();
        });
        return state.stats;
      }
    };
  }

  /* =======================================================================
     6. THE TICK RAIL
     A scale, not an ornament: it divides the same width, past the same left
     gutter, into the same 256 bit-columns the arena draws below it.
     ===================================================================== */

  function mountTickRail(cv) {
    function draw() {
      var f = fitCanvas(cv);
      var ctx = f.ctx, W = f.w, H = f.h;
      var g = gutterFor(W);
      var span = W - g;
      /* The rule itself is drawn in CSS, so it survives with no script and is
         never painted twice. The canvas owns the ticks and the two labels. */
      ctx.clearRect(0, 0, W, H);
      ctx.lineCap = 'round';
      ctx.strokeStyle = rgba(C.boneFaint, 0.55);
      for (var b = 0; b <= 256; b += 32) {
        var major = (b % 64) === 0;
        var x = g + (b / 256) * span;
        if (b === 256) { x -= 1; }
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(Math.round(x) + 0.5, 1);
        ctx.lineTo(Math.round(x) + 0.5, 1 + (major ? 6 : 3));
        ctx.stroke();
      }
      if (W >= 480) {
        ctx.font = '10px ' + DATA_FONT;
        ctx.textBaseline = 'top';
        ctx.textAlign = 'right';
        ctx.fillStyle = rgba(C.boneDim, 0.8);
        ctx.fillText('256', g + span, 10);
        ctx.textAlign = 'left';
        ctx.fillText('bit', g, 10);
      }
    }
    draw();
    global.addEventListener('resize', draw);
    if (doc.fonts && doc.fonts.ready) { doc.fonts.ready.then(draw); }
    return { draw: draw };
  }

  /* =======================================================================
     7. THE TRANSPOSE  (mode B)
     The same 3,072 rows, read as 48 words of 64 records. Columns are the
     query's own set bits, ordered rarest first. A word goes dark the frame it
     is provably finished: those bytes are never read again.
     ===================================================================== */

  function mountTranspose(cv, opts) {
    var q = opts.query;
    var t = opts.t;
    var a = pop[q];
    var WORD = 64;
    var nWords = Math.ceil(N / WORD);       // 48

    /* Column order: rarest bit first. Document frequency is counted over the
       real arena, not assumed. */
    var freq = new Int32Array(BITS);
    for (var r = 0; r < N; r++) {
      var base = r * LANES;
      for (var w = 0; w < LANES; w++) {
        var word = words[base + w];
        while (word) {
          var b = 31 - Math.clz32(word & -word);
          freq[w * 32 + b]++;
          word &= word - 1;
        }
      }
    }
    var cols = [];
    for (var bi = 0; bi < BITS; bi++) {
      if (words[q * LANES + (bi >> 5)] & (1 << (bi & 31))) { cols.push(bi); }
    }
    cols.sort(function (x, y) { return freq[x] - freq[y] || x - y; });
    var nCols = cols.length;                 // = popcount of the query
    var nPlanes = Math.max(1, Math.ceil(Math.log(a + 1) / Math.LN2));

    /* The minimum intersection a record needs, in integer arithmetic:
       c * (den + num) >= num * (a + b)  ->  c >= ceil(num*(a+b) / (den+num)) */
    var num = Math.floor(t * DEN);
    var need = new Uint16Array(N);
    for (var i = 0; i < N; i++) {
      need[i] = Math.ceil(num * (a + pop[i]) / (DEN + num));
    }

    /* Per-column contribution, precomputed once from the real bits. */
    var colBits = [];
    for (var c = 0; c < nCols; c++) {
      var bit = cols[c];
      var lane = bit >> 5, mask = 1 << (bit & 31);
      var arr = new Uint8Array(N);
      for (var rr = 0; rr < N; rr++) { arr[rr] = (words[rr * LANES + lane] & mask) ? 1 : 0; }
      colBits.push(arr);
    }

    /* How many of a word's 64 records carry each column's bit. This IS the
       transposed datum the engine reads: one 64-bit word per (column, word).
       Drawn as density, it shows the real thing, that the rarest columns are
       nearly empty and the commonest are nearly full. */
    var density = new Uint8Array(nWords * nCols);
    var densityMax = 1;
    (function densities() {
      for (var c = 0; c < nCols; c++) {
        var arr = colBits[c];
        for (var wi = 0; wi < nWords; wi++) {
          var n = 0;
          for (var k = 0; k < WORD; k++) {
            var idx = wi * WORD + k;
            if (idx < N && arr[idx]) { n++; }
          }
          density[wi * nCols + c] = n;
          if (n > densityMax) { densityMax = n; }
        }
      }
    })();

    /* Replay the scan column by column and record, for each step, which records
       are still alive and what the sampled word's accumulator planes hold. */
    var SAMPLE_WORD = 24;
    var steps = [];
    (function replay() {
      var count = new Uint16Array(N);
      var alive = new Uint8Array(N);
      alive.fill(1);
      for (var s = 0; s < nCols; s++) {
        var remaining = nCols - s - 1;
        var aliveCount = 0, wordsAlive = 0;
        for (var rr2 = 0; rr2 < N; rr2++) {
          if (!alive[rr2]) { continue; }
          count[rr2] += colBits[s][rr2];
          if (count[rr2] + remaining < need[rr2]) { alive[rr2] = 0; } else { aliveCount++; }
        }
        var wordAlive = new Uint8Array(nWords);
        for (var wi = 0; wi < nWords; wi++) {
          for (var k = 0; k < WORD; k++) {
            var idx = wi * WORD + k;
            if (idx < N && alive[idx]) { wordAlive[wi] = 1; break; }
          }
          if (wordAlive[wi]) { wordsAlive++; }
        }
        /* Bit-sliced planes of the sampled word: plane p, bit k = bit p of the
           running intersection count of record (SAMPLE_WORD*64 + k). */
        var planes = new Uint32Array(nPlanes * 2);
        for (var k2 = 0; k2 < WORD; k2++) {
          var idx2 = SAMPLE_WORD * WORD + k2;
          if (idx2 >= N) { break; }
          var v = count[idx2];
          for (var p2 = 0; p2 < nPlanes; p2++) {
            if (v & (1 << p2)) { planes[p2 * 2 + (k2 >> 5)] |= (1 << (k2 & 31)); }
          }
        }
        steps.push({
          alive: alive.slice(),
          wordAlive: wordAlive,
          aliveCount: aliveCount,
          wordsAlive: wordsAlive,
          planes: planes,
          bits: colBits[s]
        });
      }
    })();

    var STEP_MS = 110, HOLD_MS = 1400;
    var cur = nCols - 1;          // first paint shows the finished scan
    var timer = 0, running = false;
    var out = opts.readout || {};

    function setText(el, s) { if (el) { el.textContent = s; } }

    function draw() {
      var f = fitCanvas(cv);
      var ctx = f.ctx, W = f.w, H = f.h;
      ctx.clearRect(0, 0, W, H);

      var padL = 2, padR = 2;
      var planeH = 6, planeGap = 4;
      var planeBlock = nPlanes * (planeH + planeGap) + 22;
      var gridH = Math.max(40, H - planeBlock);
      var cellW = (W - padL - padR) / nCols;
      var cellH = gridH / nWords;
      var mw = Math.max(1, cellW - 1.2);
      var mh = Math.max(1, cellH - 1.4);

      var st = steps[cur];
      var i;

      for (var wi = 0; wi < nWords; wi++) {
        var y = wi * cellH;
        var wordIsDead = !st.wordAlive[wi];
        var deadFrom = wordIsDead ? firstDeadCol(wi) : nCols;
        for (var c = 0; c < nCols; c++) {
          var x = padL + c * cellW;
          var a;
          if (c >= deadFrom) {
            a = 0.035;                       /* provably finished: never read again */
          } else if (c > cur) {
            a = 0.11;                        /* not reached yet */
          } else {
            /* read: how full this 64-record word is for this bit */
            var d = density[wi * nCols + c] / densityMax;
            a = 0.13 + 0.75 * Math.pow(d, 0.62);
          }
          ctx.fillStyle = rgba(C.bone, a);
          ctx.fillRect(x, y, mw, mh);
        }
      }

      /* The sampled word's accumulator planes, drawn as bit-sliced bars. */
      var py = gridH + 16;
      var prev = cur > 0 ? steps[cur - 1] : null;
      for (var p = 0; p < nPlanes; p++) {
        var changed = prev && (st.planes[p * 2] !== prev.planes[p * 2] ||
          st.planes[p * 2 + 1] !== prev.planes[p * 2 + 1]);
        var yy = py + p * (planeH + planeGap);
        var barW = (W - padL - padR) / WORD;
        for (var k = 0; k < WORD; k++) {
          var on = (st.planes[p * 2 + (k >> 5)] >>> (k & 31)) & 1;
          ctx.fillStyle = on
            ? (changed ? rgba(C.amberLit, 0.9) : rgba(C.bone, 0.66))
            : rgba(C.bone, 0.05);
          ctx.fillRect(padL + k * barW, yy, Math.max(1, barW - 1), planeH);
        }
      }
    }

    /* Memoised helpers so the draw loop stays cheap. */
    var firstDeadCache = new Int32Array(nWords).fill(-2);
    function firstDeadCol(wi) {
      if (firstDeadCache[wi] !== -2) { return firstDeadCache[wi]; }
      var v = nCols;
      for (var s = 0; s < nCols; s++) { if (!steps[s].wordAlive[wi]) { v = s; break; } }
      firstDeadCache[wi] = v;
      return v;
    }
    function report() {
      var st = steps[cur];
      setText(out.cols, (cur + 1) + ' / ' + nCols);
      setText(out.alive, st.aliveCount.toLocaleString('en-US') + ' / ' + N.toLocaleString('en-US'));
      setText(out.wordsAlive, st.wordsAlive + ' / ' + nWords);
    }

    function step() {
      if (!running) { return; }
      cur++;
      if (cur >= nCols) {
        cur = nCols - 1;
        draw(); report();
        timer = global.setTimeout(function () { cur = -1; step(); }, HOLD_MS);
        return;
      }
      draw(); report();
      timer = global.setTimeout(step, STEP_MS);
    }

    function start() {
      if (running || reduceMotion.matches) { return; }
      running = true;
      timer = global.setTimeout(step, STEP_MS);
    }
    function stop() {
      running = false;
      if (timer) { global.clearTimeout(timer); timer = 0; }
    }

    draw();
    report();
    whenVisible(cv, start, stop);
    doc.addEventListener('visibilitychange', function () { if (doc.hidden) { stop(); } else { start(); } });
    global.addEventListener('resize', draw);
    if (doc.fonts && doc.fonts.ready) { doc.fonts.ready.then(draw); }

    return { nCols: nCols, nPlanes: nPlanes, a: a, id: ids[q], draw: draw };
  }

  /* =======================================================================
     8. WIRING
     ===================================================================== */

  function fmt(n) { return n.toLocaleString('en-US'); }
  function el(id) { return doc.getElementById(id); }

  function boot() {
    /* -- the moments this page actually generated, read back from the array -- */
    var sum = 0;
    for (var i = 0; i < N; i++) { sum += pop[i]; }
    var moments = {
      min: pop[0],
      median: pop[N >> 1],
      mean: (sum / N).toFixed(2),
      max: pop[N - 1]
    };
    var mEl = el('moments');
    if (mEl) {
      mEl.textContent = 'min ' + moments.min + ', median ' + moments.median +
        ', mean ' + moments.mean + ', max ' + moments.max;
    }

    var arenaCv = el('arena');
    var thr = el('thr');
    if (!arenaCv || !thr) { return; }

    var arena = mountArena(arenaCv, { t: parseFloat(thr.value) });
    mountTickRail(el('tickrail'));

    /* --- readouts. Every one already holds its true value in the HTML. --- */
    var R = {
      id: el('q-id'), row: el('q-row'), a: el('q-a'), t: el('q-t'),
      tOut: el('q-t-out'), band: el('q-band'), cand: el('q-cand'), scan: el('q-scan'),
      prune: el('q-prune'), hits: el('q-hits'), navT: el('nav-t'),
      live: el('live')
    };

    var counters = {};
    function count(node, to, format) {
      if (!node) { return; }
      var from = counters[node.id] === undefined ? to : counters[node.id];
      counters[node.id] = to;
      if (from === to) { node.textContent = format(to); return; }
      tween(from, to, 520, [0.33, 1, 0.68, 1], function (v) {
        node.textContent = format(v);
      }, function () { node.textContent = format(to); });
    }

    function paint(s, animateNumbers) {
      var scanned = 100 * s.cand / N;
      var prune = N / s.cand;
      if (R.id) { R.id.textContent = 'CHEMBL' + ids[arena.state.q]; }
      if (R.row) { R.row.textContent = 'row ' + fmt(arena.state.q); }
      if (R.band) { R.band.textContent = '[ ' + s.lo + ' , ' + s.hi + ' ]'; }
      if (R.t) { R.t.textContent = arena.state.t.toFixed(3); }
      if (R.tOut) { R.tOut.textContent = arena.state.t.toFixed(3); }
      if (R.navT) { R.navT.textContent = 't = ' + arena.state.t.toFixed(3); }
      if (animateNumbers) {
        count(R.a, s.a, function (v) { return String(Math.round(v)); });
        count(R.cand, s.cand, function (v) { return fmt(Math.round(v)) + ' / ' + fmt(N); });
        count(R.scan, scanned, function (v) { return v.toFixed(2) + ' %'; });
        count(R.prune, prune, function (v) { return v.toFixed(2) + ' x'; });
        count(R.hits, s.hits, function (v) { return String(Math.round(v)); });
      } else {
        counters['q-a'] = s.a; counters['q-cand'] = s.cand;
        counters['q-scan'] = scanned; counters['q-prune'] = prune; counters['q-hits'] = s.hits;
        if (R.a) { R.a.textContent = String(s.a); }
        if (R.cand) { R.cand.textContent = fmt(s.cand) + ' / ' + fmt(N); }
        if (R.scan) { R.scan.textContent = scanned.toFixed(2) + ' %'; }
        if (R.prune) { R.prune.textContent = prune.toFixed(2) + ' x'; }
        if (R.hits) { R.hits.textContent = String(s.hits); }
      }
      if (R.live) {
        R.live.textContent = 'Threshold ' + arena.state.t.toFixed(3) +
          '. Band ' + s.lo + ' to ' + s.hi + ' set bits. ' +
          fmt(s.cand) + ' of ' + fmt(N) + ' records scanned, ' +
          scanned.toFixed(1) + ' percent. ' + s.hits + ' hits.';
      }
    }

    paint(arena.state.stats, false);

    /* --- M1: the scrubber, 1:1 with the pointer --- */
    var pending = 0;
    thr.addEventListener('input', function () {
      if (pending) { return; }
      pending = global.requestAnimationFrame(function () {
        pending = 0;
        var t = parseFloat(thr.value);
        if (isNaN(t)) { return; }
        paint(arena.setThreshold(t), false);
      });
    });

    /* --- M2: the query rotates on its own --- */
    var qi = 0, rotate = 0;
    function nextQuery() {
      qi = (qi + 1) % QUERIES.length;
      paint(arena.setQuery(qi), true);
    }
    function startRotate() {
      if (rotate || reduceMotion.matches) { return; }
      rotate = global.setInterval(nextQuery, 7000);
    }
    function stopRotate() { if (rotate) { global.clearInterval(rotate); rotate = 0; } }
    whenVisible(arenaCv, startRotate, stopRotate);
    doc.addEventListener('visibilitychange', function () {
      if (doc.hidden) { stopRotate(); } else { startRotate(); }
    });
    /* Touching the scrubber hands control to the reader and keeps it. */
    thr.addEventListener('pointerdown', stopRotate);
    thr.addEventListener('keydown', stopRotate);

    /* --- the transpose --- */
    var tpCv = el('transpose');
    if (tpCv) {
      var tp = mountTranspose(tpCv, {
        query: 3011,
        t: 0.60,
        readout: { cols: el('tp-cols'), alive: el('tp-alive'), wordsAlive: el('tp-words') }
      });
      var tpMeta = el('tp-meta');
      if (tpMeta) {
        tpMeta.textContent = 'CHEMBL' + tp.id + ', ' + tp.a + ' set bits, ' +
          tp.nPlanes + ' planes, t = 0.600';
      }
    }

    /* --- M10: the copy control --- */
    var copy = el('copy');
    if (copy) {
      copy.addEventListener('click', function () {
        var text = (el('build-cmd') || {}).textContent || '';
        text = text.replace(/^\s+|\s+$/g, '');
        var label = el('copy-label');
        function done() {
          if (!label) { return; }
          label.textContent = 'copied';
          global.setTimeout(function () { label.textContent = 'Copy'; }, 1200);
        }
        function fallback() {
          var ta = doc.createElement('textarea');
          ta.value = text;
          ta.setAttribute('readonly', '');
          ta.style.position = 'fixed';
          ta.style.top = '0';
          ta.style.opacity = '0';
          doc.body.appendChild(ta);
          ta.select();
          try { doc.execCommand('copy'); done(); } catch (e) { /* nothing to do */ }
          doc.body.removeChild(ta);
        }
        if (global.navigator && global.navigator.clipboard && global.navigator.clipboard.writeText) {
          global.navigator.clipboard.writeText(text).then(done, fallback);
        } else {
          fallback();
        }
      });
    }
  }

  if (doc.readyState === 'loading') {
    doc.addEventListener('DOMContentLoaded', boot);
  } else {
    boot();
  }

  global.MossfluxBand = { N: N, BITS: BITS, pop: pop, tanimoto: tanimoto, bandFor: bandFor, scan: scan };
})(window);
