/* ---------------------------------------------------------------------------
   bg.js - the scroll-scrubbed background, and the reveal observer.

   The head script in index.html picks this visit's theme (html[data-bg]) and
   orientation (html[data-bgo]). This file draws that theme's frames on the
   fixed canvas, one position per scroll offset, from the top of the page to
   the bottom.

   Legibility is computed, not hoped for. bg/manifest.js carries, per frame,
   the smallest floor-coloured overlay alpha that keeps bone text at 5:1 over
   the brightest 16 px block of that frame (scripts/site/build_frames.py).
   The canvas lays a constant veil; every .scrim gets the opacity that makes
   veil + scrim reach that alpha for whatever frames are on screen.

   Performance: frames are decoded off the main thread (img.decode), loaded
   coarse-to-fine starting at the frame in view (the rest after window load),
   drawn only when the position changes, and the backing store is capped at
   1.5x DPR and 2.6 MP.
   ------------------------------------------------------------------------- */

(function (global) {
  'use strict';

  var doc = global.document;
  var root = doc.documentElement;

  /* =======================================================================
     1. REVEAL
     Content is only hidden once this observer is armed (html.rv), so if this
     file never runs, nothing is stranded at opacity 0. Under reduced motion it
     never arms: everything is simply there.
     ===================================================================== */

  (function reveal() {
    var els = doc.querySelectorAll('.reveal, .rate, .funnel');
    var still = global.matchMedia && global.matchMedia('(prefers-reduced-motion: reduce)').matches;
    if (still || !global.IntersectionObserver || !els.length) { return; }
    var io = new global.IntersectionObserver(function (entries) {
      for (var i = 0; i < entries.length; i++) {
        if (entries[i].isIntersecting) {
          entries[i].target.classList.add('in');
          io.unobserve(entries[i].target);
        }
      }
    }, { rootMargin: '0px 0px -10% 0px' });
    for (var i = 0; i < els.length; i++) { io.observe(els[i]); }
    root.classList.add('rv');
  })();

  /* =======================================================================
     2. INSTRUMENTS
     The live plots (data/arena.js, 156 KB of real fingerprints, and band.js)
     load when the first plot is one viewport away, not with the page. Every
     number they write is already in the HTML at its true value.
     ===================================================================== */

  (function instruments() {
    var me = doc.querySelector('script[data-instruments]');
    var srcs = me ? me.getAttribute('data-instruments').split(/\s+/) : [];
    var band = doc.getElementById('band');
    var done = false;
    function load() {
      if (done) { return; }
      done = true;
      for (var i = 0; i < srcs.length; i++) {
        var s = doc.createElement('script');
        s.src = srcs[i];
        s.async = false;            // run in the listed order: data first
        doc.body.appendChild(s);
      }
    }
    if (!srcs.length) { return; }
    if (!band || !global.IntersectionObserver) { load(); return; }
    var io = new global.IntersectionObserver(function (entries) {
      if (entries[0].isIntersecting) { io.disconnect(); load(); }
    }, { rootMargin: '100% 0px' });
    io.observe(band);
  })();

  /* =======================================================================
     3. BACKGROUND
     ===================================================================== */

  var M = global.MOSSFLUX_BG;
  var cv = doc.getElementById('bg');
  if (!M || !M.order || !M.order.length || !cv || !cv.getContext) { return; }
  var ctx = cv.getContext('2d', { alpha: false });
  if (!ctx) { return; }

  var id = root.getAttribute('data-bg');
  if (!M.themes[id]) { id = M.order[0]; }
  var theme = M.themes[id];

  var VEIL = 0.14;      // constant floor-coloured veil on every frame: mood, not legibility
  var PLATEAU = 0.90;   // the scrim's alpha at opacity 1 (--scrim in styles.css)
  var S_MIN = 0.4;      // free type always reads as set on something
  var TAU = 90;         // ms; how far the picture trails the scroll

  var reduce = !!(global.matchMedia && global.matchMedia('(prefers-reduced-motion: reduce)').matches);
  var conn = global.navigator.connection;
  var lite = !!(conn && (conn.saveData || /(^|-)2g$/.test(conn.effectiveType || '')));

  var scrims = [].slice.call(doc.querySelectorAll('.scrim'));
  var capName = doc.getElementById('bg-name');
  var capFrame = doc.getElementById('bg-frame');
  var capCount = doc.getElementById('bg-count');
  if (capName) { capName.textContent = theme.name; }

  function pad3(k) { return (k < 10 ? '00' : k < 100 ? '0' : '') + k; }

  /* The frame in view first, then both ends, then halving strides: the whole
     shot is roughly there after a handful of requests and sharpens in place. */
  function refineOrder(n, first, minStride) {
    var seen = new Uint8Array(n), out = [];
    function add(k) { if (k >= 0 && k < n && !seen[k]) { seen[k] = 1; out.push(k); } }
    add(first); add(0); add(n - 1);
    var s = 1;
    while (s < n) { s <<= 1; }
    for (; s >= minStride; s >>= 1) {
      for (var k = 0; k < n; k += s) { add(k); }
    }
    return out;
  }

  /* One orientation of the theme: 72 frames, filled in as they decode. */
  function FrameSet(o) {
    this.o = o;
    this.meta = theme[o];
    this.n = this.meta.n;
    this.imgs = new Array(this.n);
    this.count = 0;
  }
  FrameSet.prototype.load = function (first) {
    var self = this;
    var order = reduce ? [first] : refineOrder(this.n, first, lite ? 4 : 1);
    var next = 0, active = 0, MAX = 4;
    function start(k) {
      active++;
      var img = new global.Image();
      img.decoding = 'async';
      img.src = 'bg/' + id + '/' + self.o + '/' + pad3(k) + '.webp';
      var ready = img.decode
        ? img.decode().catch(function () { if (!(img.complete && img.naturalWidth)) { throw 0; } })
        : new Promise(function (res, rej) { img.onload = res; img.onerror = rej; });
      ready.then(function () {
        self.imgs[k] = img;
        self.count++;
        loaded(self);
      }, function () { /* a missing frame is drawn as its nearest neighbour */ })
        .then(function () { active--; pump(); });
    }
    function pump() {
      while (active < MAX && next < order.length && (next === 0 || pageLoaded)) { start(order[next++]); }
    }
    pumps.push(pump);
    pump();
  };
  FrameSet.prototype.nearest = function (k) {
    if (this.imgs[k]) { return k; }
    for (var d = 1; d < this.n; d++) {
      if (k - d >= 0 && this.imgs[k - d]) { return k - d; }
      if (k + d < this.n && this.imgs[k + d]) { return k + d; }
    }
    return -1;
  };

  /* The frame in view loads at once; the rest wait for the window load event,
     so on a slow link they never compete with the fonts the headline needs. */
  var pageLoaded = doc.readyState === 'complete';
  var pumps = [];
  if (!pageLoaded) {
    global.addEventListener('load', function () {
      pageLoaded = true;
      for (var i = 0; i < pumps.length; i++) { pumps[i](); }
    });
  }

  var sets = {};
  var set = null;           // the set being drawn
  var want = null;          // the orientation the viewport asks for
  var W = 0, H = 0;
  var target = 0, cur = -1, raf = 0, lastT = 0;
  var lastS = -1, lastShown = -1, shown = false;

  function orient() { return global.innerWidth < global.innerHeight ? 'p' : 'l'; }

  function progress() {
    var se = doc.scrollingElement || root;
    var max = se.scrollHeight - global.innerHeight;
    var y = global.pageYOffset || se.scrollTop || 0;
    return max > 0 ? Math.min(1, Math.max(0, y / max)) : 0;
  }

  function frameFor(s) { return reduce ? 0 : progress() * (s.n - 1); }

  function loaded(s) {
    /* Switch sets on rotation only once the new one has something to show. */
    if (s !== set && s.o === want) {
      set = s;
      target = frameFor(s);
      cur = -1;
    }
    if (s === set) { invalidate(); }
  }

  function ensure(o) {
    if (!sets[o]) {
      sets[o] = new FrameSet(o);
      sets[o].load(Math.round(frameFor(sets[o])));
    }
    return sets[o];
  }

  function fit() {
    var r = cv.getBoundingClientRect();
    var d = Math.min(global.devicePixelRatio || 1, 1.5);
    var w = Math.max(1, Math.round(r.width * d));
    var h = Math.max(1, Math.round(r.height * d));
    var over = (w * h) / 2.6e6;
    if (over > 1) { w = Math.round(w / Math.sqrt(over)); h = Math.round(h / Math.sqrt(over)); }
    if (cv.width !== w || cv.height !== h) { cv.width = w; cv.height = h; }
    W = w; H = h;
  }

  function cover(img) {
    var iw = img.naturalWidth, ih = img.naturalHeight;
    var k = Math.max(W / iw, H / ih);
    var dw = iw * k, dh = ih * k;
    ctx.drawImage(img, (W - dw) / 2, (H - dh) / 2, dw, dh);
  }

  function draw() {
    var i0 = Math.floor(cur), t = cur - i0, i1 = Math.min(set.n - 1, i0 + 1);
    var a = set.nearest(i0);
    if (a < 0) { return; }
    var b = set.nearest(i1);
    var blend = b !== a && t > 0.02;

    ctx.globalAlpha = 1;
    cover(set.imgs[a]);
    if (blend) {
      ctx.globalAlpha = t;
      cover(set.imgs[b]);
      ctx.globalAlpha = 1;
    }
    ctx.fillStyle = 'rgba(8,10,9,' + VEIL + ')';
    ctx.fillRect(0, 0, W, H);

    /* A crossfade is a convex mix, so it is never brighter than the brighter
       of its two frames: covering both covers the mix. Rounded UP to 1/50 so
       the scrim is written rarely and never lands under what is needed. */
    var need = set.meta.a[a];
    if (blend) { need = Math.max(need, set.meta.a[b]); }
    var s = (1 - (1 - need) / (1 - VEIL)) / PLATEAU;
    s = Math.ceil(Math.min(1, Math.max(S_MIN, s)) * 50) / 50;
    if (s !== lastS) {
      lastS = s;
      for (var i = 0; i < scrims.length; i++) { scrims[i].style.opacity = String(s); }
    }

    var f = Math.round(cur) + 1;
    if (capFrame && f !== lastShown) { lastShown = f; capFrame.textContent = pad3(f); }

    if (!shown) { shown = true; cv.classList.add('on'); }
  }

  function tick(now) {
    raf = 0;
    if (!set || !set.count) { return; }
    if (cur < 0 || reduce) {
      cur = target;
    } else {
      var dt = lastT ? Math.min(64, now - lastT) : 16;
      cur += (target - cur) * (1 - Math.exp(-dt / TAU));
      if (Math.abs(target - cur) < 0.004) { cur = target; }
    }
    lastT = cur === target ? 0 : now;
    draw();
    if (cur !== target) { raf = global.requestAnimationFrame(tick); }
  }

  function invalidate() {
    if (!raf) { raf = global.requestAnimationFrame(tick); }
  }

  function onScroll() {
    if (!set) { return; }
    target = frameFor(set);
    invalidate();
  }

  function onResize() {
    fit();
    var o = orient();
    if (o !== want) {
      want = o;
      var s = ensure(o);
      if (s.count) { loaded(s); }
    }
    onScroll();
    invalidate();
  }

  fit();
  want = root.getAttribute('data-bgo') === 'p' ? 'p' : 'l';
  set = ensure(want);
  target = frameFor(set);
  if (capCount) { capCount.textContent = pad3(set.n); }

  if (!reduce) { global.addEventListener('scroll', onScroll, { passive: true }); }
  global.addEventListener('resize', onResize);
  global.addEventListener('pageshow', onScroll);
})(window);
