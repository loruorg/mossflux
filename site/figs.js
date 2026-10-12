/* ---------------------------------------------------------------------------
   figs.js - two isometric figures, drawn with iso.js.

   Fig 1  BitBound as a card drawer. One card per popcount, its height the
          number of ChEMBL 37 molecules (2048-bit Morgan, radius 2) with that
          popcount. Two brass dividers stand at the bound
          [ceil(t*q), floor(q/t)]; only the cards between them are read.
          The histogram below is counted from data/chembl_37.fps.gz:
          2,897,819 records, min 1, median 51, mean 52.53, max 249.

   Fig 2  The substructure line. Tokens ride a belt through the screen, the
          census and the matcher. The tokens are symbolic (log scale); the
          readout prints the API's own counts.

   Both figures: no motion under prefers-reduced-motion (they jump to the
   end state), and no animation frame while offscreen or settled.
   ------------------------------------------------------------------------- */

(function (global) {
  'use strict';

  var I = global.MFISO;
  var doc = global.document;
  if (!I || !doc) { return; }

  var still = global.matchMedia ? global.matchMedia('(prefers-reduced-motion: reduce)') : { matches: false };
  var NS = 'http://www.w3.org/2000/svg';
  function fmt(n) { return Math.round(n).toLocaleString('en-US'); }
  function $(id) { return doc.getElementById(id); }

  function onView(el, cb) {
    if (!global.IntersectionObserver) { cb(true); return; }
    new global.IntersectionObserver(function (es) { cb(es[es.length - 1].isIntersecting); },
      { rootMargin: '120px 0px' }).observe(el);
  }

  function mountSvg(stage, vb, label, body) {
    stage.innerHTML = '<svg class="iso-svg" xmlns="' + NS + '" viewBox="' + [I.f(vb.x), I.f(vb.y), I.f(vb.w), I.f(vb.h)].join(' ') +
      '" role="img" aria-label="' + label + '">' + body + '</svg>';
    return stage.firstChild;
  }

  /* =======================================================================
     FIG 1. THE DRAWER
     ===================================================================== */

  // Records per popcount 0..249, ChEMBL 37, 2048-bit Morgan radius 2.
  var HIST = [0,49,52,33,50,75,127,99,232,210,281,410,373,739,782,1017,1544,1711,2096,2971,3440,4529,5346,6597,7471,9483,11070,13295,15876,18100,21655,25251,28815,33588,38279,42354,47408,52490,57306,62469,67458,71395,75542,78787,81783,84848,86500,88335,89507,89674,88486,87305,84916,83053,80975,78385,75299,72009,68967,65594,62006,58907,55657,52208,48367,45043,41681,37707,34800,31152,28696,25664,23411,20581,18658,16953,14942,13220,11806,10555,9229,8105,7132,6430,5659,5447,4758,4220,3960,3744,3476,3264,3271,2910,3075,2872,3259,2474,2376,2296,2092,2004,1783,1744,1679,1572,1532,1620,1355,1409,1296,1193,1059,1128,1002,867,842,813,774,750,699,664,678,633,565,588,550,448,459,452,401,390,345,347,345,313,313,305,265,356,247,300,301,229,220,243,265,236,176,188,178,138,157,116,147,128,126,121,108,109,107,80,81,63,131,79,75,62,62,65,40,49,45,50,70,58,30,34,30,33,49,42,31,22,27,40,30,58,36,23,18,33,26,14,28,17,19,12,12,8,13,10,5,6,4,12,6,8,6,12,11,8,1,9,8,10,9,4,5,3,9,3,5,6,1,2,2,2,3,1,2,3,2,6,0,0,4,0,1,0,0,0,2,0,0,0,0,0,0,1];
  var PMAX = 249;
  var LAST = 160;                    // cards 1..160, then one card for 161..249
  var NC = LAST + 1;
  var NREC = 0, CMAX = 0;
  for (var hi0 = 0; hi0 < HIST.length; hi0++) { NREC += HIST[hi0]; if (HIST[hi0] > CMAX) { CMAX = HIST[hi0]; } }
  var PREFIX = [0];
  for (var p0 = 0; p0 < HIST.length; p0++) { PREFIX.push(PREFIX[p0] + HIST[p0]); }

  /* The bound, in integers: t is a whole number of hundredths, so both
     edges are exact (no float division lands just under an integer). */
  function bandOf(q, tp) {
    var lo = Math.ceil(q * tp / 100), hi = Math.floor(q * 100 / tp);
    return { lo: Math.max(1, Math.min(PMAX, lo)), hi: Math.max(1, Math.min(PMAX, hi)), rawHi: hi };
  }
  function rowsIn(lo, hi) { return lo > hi ? 0 : PREFIX[hi + 1] - PREFIX[lo]; }

  function drawer() {
    var stage = $('fa-stage');
    if (!stage) { return; }
    var qIn = $('fa-q'), tIn = $('fa-t'), qOut = $('fa-q-out'), tOut = $('fa-t-out');
    var readEl = $('fa-read'), rowsEl = $('fa-rows'), liveEl = $('fa-live');

    var c = I.camera(15, 28);
    var PITCH = 2.8, TH = 1.3, DEP = 56, HMAX = 150, HMIN = 4.5;
    var X1 = NC * PITCH + 8;
    var s = '';
    var pts = [];

    function countOf(k) { return k <= LAST ? HIST[k] : rowsIn(LAST + 1, PMAX); }
    function cardH(k) { var n = countOf(k); return n ? HMIN + (HMAX - HMIN) * n / CMAX : 0; }

    // base: halo, feet, floor
    s += I.halo(c, -12, -12, X1 + 24, DEP + 40, -12);
    [[2, -2], [X1 - 10, -2], [2, DEP + 8], [X1 - 10, DEP + 8]].forEach(function (f) {
      s += I.cyl(c, f[0] + 3, f[1] + 3, -12, 4.5, 2, { line: 'lo' }) + I.cyl(c, f[0] + 3, f[1] + 3, -10, 3.4, 6, { line: 'lo' });
    });
    s += I.extrude(c, I.rrect(-8, -8, X1 + 16, DEP + 26, 3, -4), [0, 0, 4], { smooth: true, line: 'mid' });
    // back wall and the far side wall
    s += I.box(c, -8, -8, 0, X1 + 16, 4, 22, { line: 'mid' });
    s += I.box(c, -8, -4, 0, 4, DEP + 8, 22, { line: 'mid' });
    s += I.screw(c, -6, -6, 22) + I.screw(c, X1 + 6, -6, 22);
    // a groove along the back wall's top
    s += I.line(c, [[-8, -6, 22], [X1 + 8, -6, 22]], 'faint');

    // cards, one group each, painted far (low x) to near (high x)
    var cards = '';
    for (var p = 1; p <= NC; p++) {
      var h = cardH(p);
      if (!h) { cards += '<g data-p="' + p + '"></g>'; continue; }
      cards += '<g class="card" data-p="' + p + '">' + I.box(c, p * PITCH - TH / 2, 0, 0, TH, DEP, h, { line: 'lo' }) +
        (p === NC ? I.box(c, p * PITCH - TH / 2 - 0.2, 4, h, TH + 0.4, 14, 5, { line: 'lo' }) : '') + '</g>';
    }
    s += '<g id="fa-cards">' + cards + '</g>';

    // near side wall, front lip, front rail with the ruler
    s += I.box(c, X1 + 4, -4, 0, 4, DEP + 8, 22, { line: 'mid' });
    s += I.screw(c, X1 + 6, DEP + 2, 22);
    s += I.box(c, -4, DEP + 2, 0, X1 + 8, 2, 3, { line: 'mid' });
    s += I.box(c, -8, DEP + 4, -4, X1 + 16, 12, 7, { line: 'mid' });
    var rz = 3, ry = DEP + 4.8;
    for (var t = 0; t <= LAST; t += 5) {
      var len = t % 50 === 0 ? 4.4 : t % 10 === 0 ? 3 : 1.8;
      s += I.line(c, [[t * PITCH, ry, rz], [t * PITCH, ry + len, rz]], t % 50 === 0 ? 'hi' : t % 10 === 0 ? 'mid' : 'lo');
      if (t % 50 === 0) { s += I.label(c, [t * PITCH, ry + 10.6, rz], 6, String(t)); }
    }
    s += I.line(c, [[0, ry, rz], [NC * PITCH, ry, rz]], 'lo');
    s += I.label(c, [NC * PITCH, ry + 10.6, rz], 6, '161+');
    s += I.screw(c, -4, DEP + 10, 3) + I.screw(c, X1 + 4, DEP + 10, 3);
    // the pull: two posts and a bar on the rail's front face
    var hx = X1 / 2;
    s += I.box(c, hx - 22, DEP + 16, -2.5, 4, 3, 3, { line: 'mid' }) + I.box(c, hx + 18, DEP + 16, -2.5, 4, 3, 3, { line: 'mid' });
    s += I.box(c, hx - 23, DEP + 19, -3, 46, 2.4, 4, { line: 'mid' });

    // the query pointer: a slider on the ruler, built at x = 0
    var ptr = '<g id="fa-ptr">' + I.box(c, -6, DEP + 12.5, 3, 12, 3.4, 3.2, { line: 'lit', cls: 'lit' }) +
      I.extrude(c, [[-4, DEP + 12.5, 6.2], [4, DEP + 12.5, 6.2], [0, DEP + 5.2, 6.2]], [0, 0, 1], { line: 'lit', cls: 'lit' }) + '</g>';
    s += ptr;

    // the two dividers, built at x = 0; moved into their slot among the cards
    function divider(id) {
      return '<g id="' + id + '" class="lit">' + I.box(c, -0.25, -1, 0, 0.5, DEP + 2, 112, { line: 'lit', cls: 'lit' }) +
        I.box(c, -0.6, 30, 112, 1.2, 20, 9, { line: 'lit', cls: 'lit' }) +
        I.dots(c, [[0.6, 40, 116.5]], 0.5, 'hi') + '</g>';
    }

    pts.push([-12, -12, -12], [X1 + 12, -12, -12], [-12, DEP + 26, -12], [X1 + 12, DEP + 26, -12], [100, 0, HMAX], [X1, -8, 22], [-8, -8, 130]);
    var vb = I.fit(c, pts, 8);
    var svg = mountSvg(stage, vb,
      'A card drawer. One card per popcount from 1 to 249, its height the number of ChEMBL 37 molecules with that many set bits. Two brass dividers mark the popcount band that BitBound must read; cards outside it are skipped.',
      s);

    var cardsG = svg.querySelector('#fa-cards');
    var cardEls = [].slice.call(cardsG.children);      // cardEls[p - 1] is popcount p
    var tmp = doc.createElementNS(NS, 'g');
    tmp.innerHTML = divider('fa-dlo') + divider('fa-dhi');
    var dLo = tmp.firstChild, dHi = tmp.lastChild;
    cardsG.appendChild(dLo); cardsG.appendChild(dHi);
    var ptrG = svg.querySelector('#fa-ptr');

    /* model */
    var target = { q: +qIn.value, tp: Math.round(+tIn.value * 100) };
    var b0 = bandOf(target.q, target.tp);
    var cur = { q: target.q, lo: b0.lo - 0.5, hi: b0.hi + 0.5 };
    var litLo = -1, litHi = -1, slotLo = -1, slotHi = -1;

    function place(g, x) {
      var o = I.offset(c, [x, 0, 0]);
      g.setAttribute('transform', 'translate(' + I.f(o[0]) + ' ' + I.f(o[1]) + ')');
    }
    /* A divider at boundary `edge` (a half-integer) is painted just before
       card ceil(edge): after every farther card, before every nearer one. */
    function slot(g, edge) {
      var k = Math.max(1, Math.min(NC + 1, Math.ceil(edge)));
      cardsG.insertBefore(g, k <= NC ? cardEls[k - 1] : null);
      return k;
    }

    function draw() {
      place(ptrG, cur.q * PITCH);
      place(dLo, cur.lo * PITCH);
      place(dHi, cur.hi * PITCH);
      if (Math.ceil(cur.lo) !== slotLo) { slotLo = slot(dLo, cur.lo); }
      if (Math.ceil(cur.hi) !== slotHi) { slotHi = slot(dHi, cur.hi); }
      var lo = Math.ceil(cur.lo), hi = Math.floor(cur.hi);
      if (lo !== litLo || hi !== litHi) {
        for (var p = 1; p <= NC; p++) {
          var on = p >= lo && p <= hi, g = cardEls[p - 1];
          if (on !== g.hasAttribute('data-on')) {
            if (on) { g.setAttribute('data-on', ''); } else { g.removeAttribute('data-on'); }
          }
        }
        litLo = lo; litHi = hi;
      }
    }

    function say() {
      var b = bandOf(target.q, target.tp);
      var read = rowsIn(b.lo, b.hi), skip = 1 - read / NREC;
      var t = (target.tp / 100).toFixed(2);
      qOut.textContent = String(target.q);
      tOut.textContent = t;
      readEl.textContent = 'band ' + b.lo + '–' + b.hi + ' · skips ' + (skip * 100).toFixed(1) + ' %';
      rowsEl.textContent = fmt(read) + ' of ' + fmt(NREC) + ' rows read';
      return 'Query popcount ' + target.q + ', threshold ' + t + '. Band ' + b.lo + ' to ' + b.hi + '. ' +
        fmt(read) + ' rows read, ' + (skip * 100).toFixed(1) + ' percent skipped.';
    }

    /* loop */
    var raf = 0, last = 0, visible = false;
    function tick(now) {
      raf = 0;
      var dt = last ? Math.min((now - last) / 1000, 1 / 30) : 1 / 60;
      last = now;
      if (touring) { tour(dt); }
      var b = bandOf(target.q, target.tp);
      var tl = Math.min(b.lo, NC) - 0.5, th = Math.max(Math.min(b.lo, NC), Math.min(NC, b.hi)) + 0.5;
      if (still.matches) { cur.q = target.q; cur.lo = tl; cur.hi = th; }
      else {
        cur.q = I.follow(cur.q, target.q, dt, 0.22);
        cur.lo = I.follow(cur.lo, tl, dt, 0.26);
        cur.hi = I.follow(cur.hi, th, dt, 0.3);
      }
      draw();
      var settled = cur.q === target.q && cur.lo === tl && cur.hi === th;
      if (visible && (!settled || touring)) { raf = global.requestAnimationFrame(tick); } else { last = 0; }
    }
    function kick() { if (!raf && visible) { raf = global.requestAnimationFrame(tick); } }

    /* idle tour: a few true cases, then hands off the moment anyone touches it */
    var STOPS = [[51, 70], [51, 90], [120, 80], [18, 60], [51, 50]];
    var touring = !still.matches, clock = 0, stop = 0, idleT = 0;
    function tour(dt) {
      clock += dt;
      if (clock > 3.2) {
        clock = 0; stop = (stop + 1) % STOPS.length;
        set(STOPS[stop][0], STOPS[stop][1], true);
      }
    }
    function set(q, tp, fromTour) {
      target.q = q; target.tp = tp;
      qIn.value = q; tIn.value = (tp / 100).toFixed(2);
      var text = say();
      if (!fromTour && liveEl) { liveEl.textContent = text; }
      kick();
    }
    function touch() {
      touring = false;
      global.clearTimeout(idleT);
      if (!still.matches) {
        idleT = global.setTimeout(function () { touring = true; clock = 0; kick(); }, 9000);
      }
    }

    qIn.addEventListener('input', function () { touch(); set(+qIn.value, target.tp); });
    tIn.addEventListener('input', function () { touch(); set(target.q, Math.round(+tIn.value * 100)); });

    // drag across the drawer to move the query pointer
    var dragging = false;
    function qAt(e) {
      var m = svg.getScreenCTM();
      if (!m) { return target.q; }
      var pt = svg.createSVGPoint(); pt.x = e.clientX; pt.y = e.clientY;
      var v = pt.matrixTransform(m.inverse());
      // invert the projection on the ruler's line y = DEP + 8, z = 3
      var y = DEP + 8, x = (v.x + y * c.sa) / c.ca;
      return Math.max(1, Math.min(LAST, Math.round(x / PITCH)));
    }
    stage.addEventListener('pointerdown', function (e) {
      dragging = true; touch();
      try { stage.setPointerCapture(e.pointerId); } catch (err) { /* ignore */ }
      set(qAt(e), target.tp);
    });
    stage.addEventListener('pointermove', function (e) { if (dragging) { set(qAt(e), target.tp); } });
    function up() { dragging = false; }
    stage.addEventListener('pointerup', up);
    stage.addEventListener('pointercancel', up);

    onView(stage, function (v) { visible = v; if (v) { kick(); } });
    say();
    draw();
  }

  /* =======================================================================
     FIG 2. THE LINE
     ===================================================================== */

  function line() {
    var stage = $('fb-stage');
    if (!stage) { return; }
    var readEl = $('fb-read'), hintEl = $('fb-hint'), cntEl = $('fb-counts'), liveEl = $('fb-live');
    var c = I.camera(28, 30);
    var L = 460, BW = 36, BZ = 24, TK = 9;
    var S1 = 119, S2 = 232, S3 = 340;           // station centres along x
    var R1 = 150, R2 = 268, R3 = 384;           // where rejects leave the belt
    var BIN_A = { x: 142, w: 146 }, BIN_B = { x: 362, w: 50 }, BIN_Y = 52, BIN_D = 34;
    var TRAY = { x: 474, w: 52 };
    var s = '';

    function gantryBack(x) {
      return I.box(c, x - 4, -14, 0, 8, 6, 66, { line: 'mid' });
    }
    function gantryFront(x) {
      return I.box(c, x - 4, BW + 8, 0, 8, 6, 66, { line: 'mid' }) +
        I.box(c, x - 7, -14, 66, 14, BW + 28, 7, { line: 'mid' }) +
        I.screw(c, x, -11, 73) + I.screw(c, x, BW + 11, 73);
    }

    // base
    s += I.halo(c, -70, -30, 610, 136, -10);
    [[-62, -22], [520, -22], [-62, 92], [520, 92]].forEach(function (f) {
      s += I.cyl(c, f[0] + 4, f[1] + 4, -10, 5, 2, { line: 'lo' }) + I.cyl(c, f[0] + 4, f[1] + 4, -8, 3.8, 4, { line: 'lo' });
    });
    s += I.extrude(c, I.rrect(-66, -26, 602, 126, 4, -6), [0, 0, 6], { smooth: true, line: 'mid' });
    s += I.loop(c, I.rrect(-61, -21, 592, 116, 2, 0), 'faint');
    s += I.screw(c, -58, -18, 0) + I.screw(c, 526, -18, 0) + I.screw(c, -58, 92, 0) + I.screw(c, 526, 92, 0);

    // back structure
    s += I.box(c, -48, -10, 0, 6, 6, 34, { line: 'lo' }) + I.box(c, 14, -10, 0, 6, 6, 34, { line: 'lo' });
    s += gantryBack(S1) + gantryBack(S2);
    s += I.box(c, S3 - 6, -18, 0, 12, 10, 92, { line: 'mid' });               // press column
    s += I.box(c, S3 - 9, -21, 0, 18, 16, 4, { line: 'mid' });                // its foot
    [20, 230, 440].forEach(function (x) { s += I.box(c, x - 3, -4, 0, 6, 4, 21, { line: 'lo' }); });
    s += I.box(c, 0, -4, 18, L, 4, 9, { line: 'mid' });                      // back rail
    // rollers and the belt
    s += I.extrude(c, I.circle([0, 0, 21], 4, 'y', 20), [0, BW, 0], { smooth: true, line: 'lo' });
    s += I.extrude(c, I.circle([L, 0, 21], 4, 'y', 20), [0, BW, 0], { smooth: true, line: 'lo' });
    s += I.box(c, 0, 0, 21, L, BW, 3, { line: 'lo', cls: 'belt' });
    s += '<path id="fb-belt" class="ln-faint nofill" d=""/>';
    s += '<g id="fb-tok"></g>';
    // front rail and legs
    s += I.box(c, 0, BW, 18, L, 4, 9, { line: 'mid' });
    [20, 230, 440].forEach(function (x) { s += I.box(c, x - 3, BW + 4, 0, 6, 4, 21, { line: 'lo' }); });
    for (var sx = 30; sx < L; sx += 60) { s += I.dots(c, [[sx, BW + 4, 22.5]], 0.5, 'mid'); }

    // the hopper on four posts
    s += I.box(c, -48, 40, 0, 6, 6, 34, { line: 'lo' }) + I.box(c, 14, 40, 0, 6, 6, 34, { line: 'lo' });
    s += I.box(c, -52, -12, 34, 76, 60, 30, { line: 'mid' });
    s += I.box(c, -48, -8, 64, 68, 52, 3, { line: 'lo' });
    s += I.loop(c, [[-40, 48, 40], [-4, 48, 40], [-4, 48, 58], [-40, 48, 58]], 'faint');
    s += I.line(c, [[-36, 48, 54], [-12, 48, 54]], 'lo') + I.line(c, [[-36, 48, 50], [-18, 48, 50]], 'lo') + I.line(c, [[-36, 48, 46], [-22, 48, 46]], 'lo');
    s += I.dots(c, [[-48, 48, 38], [20, 48, 38], [-48, 48, 60], [20, 48, 60]], 0.55, 'mid');

    // station 1: the screen, a perforated plate under a gantry
    s += gantryFront(S1);
    s += '<g id="fb-sieve">' + I.box(c, S1 - 16, -2, 40, 32, BW + 4, 3, { line: 'mid' });
    var holes = [];
    for (var hx = 0; hx < 6; hx++) { for (var hy = 0; hy < 7; hy++) { holes.push([S1 - 12.5 + hx * 5, 2 + hy * 5, 43]); } }
    s += I.dots(c, holes, 0.7, 'lo');
    s += I.box(c, S1 - 3, 12, 43, 6, 12, 23, { line: 'lo' }) + '</g>';

    // station 2: the census, a reading head and four counter drums
    s += gantryFront(S2);
    s += I.box(c, S2 - 10, 8, 38, 20, 20, 28, { line: 'mid' });
    s += I.box(c, S2 - 12, 6, 36, 24, 24, 3, { line: 'lo' });
    for (var d = 0; d < 4; d++) {
      s += I.extrude(c, I.circle([S2 - 4, -7 + d * 10, 79], 5, 'x', 18), [8, 0, 0], { smooth: true, line: 'lo' });
      s += I.extrude(c, I.circle([S2 - 5, -7 + d * 10, 79], 1.4, 'x', 10), [10, 0, 0], { smooth: true, line: 'faint' });
    }
    s += '<circle id="fb-lamp" class="lamp" r="2.2" cx="' + I.f(I.P(c, [S2 + 8, BW + 14, 70])[0]) + '" cy="' + I.f(I.P(c, [S2 + 8, BW + 14, 70])[1]) + '"/>';

    // station 3: the matcher, a press on a column
    s += I.box(c, S3 - 9, -18, 86, 18, BW + 12, 9, { line: 'mid' });
    s += I.screw(c, S3, -14, 95) + I.screw(c, S3, BW - 10, 95);
    s += I.cyl(c, S3, 18, 80, 6, 6, { line: 'mid' });
    s += '<g id="fb-ram">' + I.cyl(c, S3, 18, 50, 2.6, 30, { line: 'mid' }) + I.cyl(c, S3, 18, 44, 7, 6, { line: 'lit', cls: 'lit' }) + '</g>';

    // reject bins and the hits tray: back walls and floors first
    function binBack(b, lit) {
      var o = { line: lit ? 'lit' : 'mid', cls: lit ? 'lit' : '' };
      return I.box(c, b.x, BIN_Y, 0, b.w, BIN_D, 2, { line: 'lo' }) +
        I.box(c, b.x, BIN_Y, 2, b.w, 2, 14, o) + I.box(c, b.x, BIN_Y + 2, 2, 2, BIN_D - 4, 14, o);
    }
    function binFront(b, lit) {
      var o = { line: lit ? 'lit' : 'mid', cls: lit ? 'lit' : '' };
      return I.box(c, b.x + b.w - 2, BIN_Y + 2, 2, 2, BIN_D - 4, 14, o) + I.box(c, b.x, BIN_Y + BIN_D - 2, 2, b.w, 2, 14, o) +
        I.screw(c, b.x + 1, BIN_Y + BIN_D - 1, 16) + I.screw(c, b.x + b.w - 1, BIN_Y + BIN_D - 1, 16);
    }
    var trayBox = { x: TRAY.x, w: TRAY.w };
    s += binBack(BIN_A) + binBack(BIN_B);
    s += I.box(c, TRAY.x, -6, 0, TRAY.w, 48, 2, { line: 'lo' });
    s += I.box(c, TRAY.x, -6, 2, TRAY.w, 2, 14, { line: 'lit', cls: 'lit' }) + I.box(c, TRAY.x, -4, 2, 2, 44, 14, { line: 'lit', cls: 'lit' });
    s += '<g id="fb-out"></g>';
    s += binFront(BIN_A) + binFront(BIN_B);
    s += I.box(c, TRAY.x + TRAY.w - 2, -4, 2, 2, 44, 14, { line: 'lit', cls: 'lit' }) + I.box(c, TRAY.x, 40, 2, TRAY.w, 2, 14, { line: 'lit', cls: 'lit' });
    void trayBox;

    var vb = I.fit(c, [[-70, -30, -10], [540, -30, -10], [-70, 106, -10], [540, 106, -10], [S3, -18, 95], [-52, -12, 67]], 8);
    var svg = mountSvg(stage, vb,
      'A conveyor with three stations. Molecules ride a belt from a hopper through a perforated screen, a census head and a press that runs the subgraph match. Rejects fall into bins; hits land in a brass tray.',
      s);
    var tokG = svg.querySelector('#fb-tok'), outG = svg.querySelector('#fb-out');
    var beltP = svg.querySelector('#fb-belt'), ram = svg.querySelector('#fb-ram'), lamp = svg.querySelector('#fb-lamp');
    var sieve = svg.querySelector('#fb-sieve');

    var TOKEN = I.box(c, -TK / 2, -TK / 2, 0, TK, TK, TK, { line: 'hi', cls: 'tok' });

    function beltLines(phase) {
      var d = '';
      for (var x = (phase % 14 + 14) % 14; x < L; x += 14) {
        var a = I.P(c, [x, 0, 24]), b = I.P(c, [x, BW, 24]);
        d += 'M' + I.f(a[0]) + ' ' + I.f(a[1]) + 'L' + I.f(b[0]) + ' ' + I.f(b[1]);
      }
      beltP.setAttribute('d', d);
    }
    beltLines(0);

    /* A run: M tokens; fates chosen so the share passing each gate follows
       the true ratios on a log scale. */
    var M = 36, SPEED = 130, GAP = 0.16;
    var run = null, raf = 0, last = 0, visible = false, beltPhase = 0;

    function logShare(a, b) { return b > 0 ? Math.log(1 + a) / Math.log(1 + b) : 0; }

    function plan(r) {
      var f1 = logShare(r.screened, r.total), f2 = logShare(r.matched, r.screened);
      var nC = r.screened > 0 ? Math.max(1, Math.round(M * f1)) : 0;
      var nS = Math.round(M * Math.sqrt(f1));
      if (nS < nC) { nS = nC; }
      var nH = r.matched > 0 ? Math.max(1, Math.round(nC * f2)) : 0;
      if (r.screened === r.total) { nS = nC = M; }
      if (r.matched === r.screened) { nH = nC; }
      var fates = [];
      for (var i = 0; i < M; i++) { fates.push(i < nH ? 3 : i < nC ? 2 : i < nS ? 1 : 0); }
      for (var j = M - 1; j > 0; j--) { var k = Math.floor(Math.random() * (j + 1)), tt = fates[j]; fates[j] = fates[k]; fates[k] = tt; }
      return fates;
    }

    function tokenEl(hit) {
      var g = doc.createElementNS(NS, 'g');
      g.innerHTML = TOKEN;
      return g;
    }
    function put(g, p) {
      var o = I.P(c, p);
      g.setAttribute('transform', 'translate(' + I.f(o[0]) + ' ' + I.f(o[1]) + ')');
    }
    function landing(fate, i) {
      var b = fate === 0 || fate === 1 ? BIN_A : fate === 2 ? BIN_B : { x: TRAY.x, w: TRAY.w };
      var yb = fate === 3 ? -4 : BIN_Y + 2, db = fate === 3 ? 44 : BIN_D - 4;
      var cols = Math.max(1, Math.floor((b.w - 6) / (TK + 1))), rows = Math.max(1, Math.floor((db - 2) / (TK + 1)));
      var k = i, layer = Math.floor(k / (cols * rows)), r = k % (cols * rows);
      return [b.x + 3 + TK / 2 + (r % cols) * (TK + 1) + (layer % 2) * 2, yb + 1 + TK / 2 + Math.floor(r / cols) * (TK + 1), 2 + layer * TK];
    }

    function counts() {
      var r = run.r;
      var done = [0, 0, 0];
      for (var i = 0; i < run.toks.length; i++) {
        var tk = run.toks[i];
        if (tk.stage >= 1) { done[0]++; }
        if (tk.stage >= 2) { done[1]++; }
        if (tk.stage >= 3) { done[2]++; }
      }
      var a = r.total * done[0] / M, b = r.screened * done[1] / M, h = r.matched * done[2] / M;
      readEl.textContent = fmt(a) + ' → ' + fmt(b) + ' → ' + fmt(h);
      return done[2] === M;
    }

    function start(r) {
      outG.textContent = ''; tokG.textContent = '';
      var fates = plan(r);
      run = { r: r, toks: [], t: 0, piles: [0, 0, 0, 0], press: 0, flash: 0 };
      for (var i = 0; i < M; i++) {
        var g = tokenEl();
        run.toks.push({ g: g, fate: fates[i], x: -10, y: BW / 2, z: BZ, t0: i * GAP, stage: 0, fall: -1, from: null, to: null, inBelt: false });
      }
      if (hintEl) { hintEl.textContent = 'Click to replay'; }
      if (still.matches) { finish(); return; }
      kick();
    }

    function finish() {
      for (var i = 0; i < run.toks.length; i++) {
        var tk = run.toks[i];
        tk.stage = 3;
        var to = landing(tk.fate, run.piles[tk.fate === 1 ? 0 : tk.fate]++);
        if (tk.fate === 3) { tk.g.setAttribute('data-hit', ''); }
        tk.g.setAttribute('data-k', String(I.depth(c, to) + to[2] * 0.01));
        outG.appendChild(tk.g);
        put(tk.g, to);
      }
      sortOut();
      counts();
      run.done = true;
    }

    function sortOut() {
      var gs = [].slice.call(outG.children);
      gs.sort(function (a, b) { return (+a.getAttribute('data-k')) - (+b.getAttribute('data-k')); });
      for (var i = 0; i < gs.length; i++) { outG.appendChild(gs[i]); }
    }

    function step(dt) {
      run.t += dt;
      var moving = false;
      for (var i = 0; i < run.toks.length; i++) {
        var tk = run.toks[i];
        if (run.t < tk.t0) { moving = true; continue; }
        if (tk.fall < 0) {
          if (!tk.inBelt) { tk.inBelt = true; tokG.appendChild(tk.g); }
          tk.x += SPEED * dt;
          if (tk.stage < 1 && tk.x >= S1) { tk.stage = 1; }
          if (tk.stage < 2 && tk.x >= S2) { tk.stage = 2; run.flash = 1; }
          if (tk.stage < 3 && tk.x >= S3) { tk.stage = 3; run.press = 1; }
          var off = tk.fate === 0 ? R1 : tk.fate === 1 ? R2 : tk.fate === 2 ? R3 : L + 8;
          if (tk.x >= off) {
            tk.stage = 3;
            tk.fall = 0;
            tk.from = [off, tk.y, BZ];
            var pileIx = tk.fate === 1 ? 0 : tk.fate;
            tk.to = landing(tk.fate, run.piles[pileIx]++);
            tk.g.setAttribute('data-k', String(I.depth(c, tk.to) + tk.to[2] * 0.01));
            if (tk.fate === 3) { tk.g.setAttribute('data-hit', ''); }
            outG.appendChild(tk.g);
            sortOut();
          } else {
            put(tk.g, [tk.x, tk.y, tk.z]);
          }
          moving = true;
        } else if (tk.fall < 1) {
          tk.fall = Math.min(1, tk.fall + dt / 0.55);
          var u = tk.fall, e = u * u * (3 - 2 * u);
          var p = [tk.from[0] + (tk.to[0] - tk.from[0]) * e, tk.from[1] + (tk.to[1] - tk.from[1]) * e,
            tk.from[2] + (tk.to[2] - tk.from[2]) * u * u + Math.sin(Math.PI * u) * 14];
          put(tk.g, p);
          moving = true;
        }
      }
      // the press and the lamp ride envelopes that decay
      run.press = Math.max(0, run.press - dt / 0.28);
      run.flash = Math.max(0, run.flash - dt / 0.35);
      var pd = Math.sin(Math.PI * Math.min(1, 1 - run.press)) * 7;
      var po = I.offset(c, [0, 0, -pd]);
      ram.setAttribute('transform', 'translate(' + I.f(po[0]) + ' ' + I.f(po[1]) + ')');
      lamp.style.opacity = String(0.25 + 0.75 * run.flash);
      var shake = moving ? Math.sin(run.t * 70) * 0.35 : 0;
      sieve.setAttribute('transform', 'translate(0 ' + I.f(shake) + ')');
      if (moving) { beltPhase += SPEED * dt; beltLines(beltPhase); }
      var all = counts();
      return moving || !all;
    }

    function tick(now) {
      raf = 0;
      if (!run || run.done) { return; }
      var dt = last ? Math.min((now - last) / 1000, 1 / 30) : 1 / 60;
      last = now;
      var going = step(dt);
      if (!going) { run.done = true; last = 0; lamp.style.opacity = ''; return; }
      if (visible) { raf = global.requestAnimationFrame(tick); } else { last = 0; }
    }
    function kick() { if (!raf && visible && run && !run.done) { raf = global.requestAnimationFrame(tick); } }

    var lastResult = null;
    function show(r) {
      lastResult = r;
      if (cntEl) {
        cntEl.textContent = fmt(r.total) + ' read, ' + fmt(r.screened) + ' reached the matcher, ' + fmt(r.matched) + ' matched' +
          (r.took_us != null ? ', in ' + (r.took_us / 1000).toFixed(1) + ' ms' : '') + '.';
      }
      if (liveEl) { liveEl.textContent = cntEl ? cntEl.textContent : ''; }
      start(r);
    }
    stage.addEventListener('click', function () { if (lastResult) { start(lastResult); } });

    doc.addEventListener('mossflux:substructure', function (e) {
      var d = e.detail || {};
      if (typeof d.total !== 'number') { return; }
      show({ total: d.total, screened: typeof d.screened === 'number' ? d.screened : d.total, matched: d.count || 0, took_us: d.took_us, smarts: d.smarts });
    });
    onView(stage, function (v) { visible = v; if (v) { kick(); } });
    global.MOSSFLUX_LINE = { show: show };
  }

  /* Build the figures only when they come near the viewport. */
  var how = $('how'), built = false;
  function build() { if (built) { return; } built = true; drawer(); line(); }
  if (how && global.IntersectionObserver) {
    var io = new global.IntersectionObserver(function (es) {
      if (es[es.length - 1].isIntersecting) { io.disconnect(); build(); }
    }, { rootMargin: '100% 0px' });
    io.observe(how);
  } else { build(); }
})(window);
