/* mossflux.com. Scroll-scrubbed scenes, a WebGL particle explainer, a small
   live demo. API base: API below, or ?api=<url> for local testing. */
(function () {
  'use strict';

  var API = 'https://api.mossflux.com';
  try {
    var o = new URLSearchParams(location.search).get('api');
    if (o && /^https?:\/\/\S+$/.test(o)) { API = o.replace(/\/+$/, ''); }
  } catch (e) { /* default */ }

  var doc = document, $ = function (id) { return doc.getElementById(id); };
  var fmt = function (n) { return Math.round(n).toLocaleString('en-US'); };
  var esc = function (s) { return String(s).replace(/[&<>"']/g, function (c) { return '&#' + c.charCodeAt(0) + ';'; }); };
  var clamp = function (x, a, b) { return x < a ? a : x > b ? b : x; };
  var sstep = function (a, b, x) { var t = clamp((x - a) / (b - a), 0, 1); return t * t * (3 - 2 * t); };
  var ease = function (t) { return 1 - Math.pow(1 - t, 3); };
  var DPR = Math.min(1.5, devicePixelRatio || 1);
  var VH = innerHeight;

  /* ------------------------------------------------------------ scenes */
  var scenes = [].map.call(doc.querySelectorAll('.scene'), function (el) {
    var s = {
      el: el, p: 0, sp: 0, top: 0, h: 0, vis: false,
      canvas: el.querySelector('canvas.seq'),
      beats: [].map.call(el.querySelectorAll('.beat'), function (b) {
        return { el: b, a: +b.dataset.in, b: +b.dataset.out, op: -1 };
      }),
      id: el.dataset.seq, n: +(el.dataset.frames || 0), imgs: [], loading: false, drawn: -1
    };
    if (s.canvas) { s.ctx = s.canvas.getContext('2d'); }
    return s;
  });

  function measure() {
    VH = innerHeight;
    scenes.forEach(function (s) {
      var r = s.el.getBoundingClientRect();
      s.top = r.top + scrollY; s.h = s.el.offsetHeight;
      if (s.canvas) {
        s.canvas.width = Math.round(s.canvas.clientWidth * DPR);
        s.canvas.height = Math.round(s.canvas.clientHeight * DPR);
        s.drawn = -1;
      }
    });
    gl && gl.resize();
  }

  // Load frames coarse to fine, so scrubbing works before all have arrived.
  function load(s) {
    if (s.loading || !s.id) { return; }
    s.loading = true;
    var order = [], seen = {};
    [16, 8, 4, 2, 1].forEach(function (st) {
      for (var i = 0; i < s.n; i += st) { if (!seen[i]) { seen[i] = 1; order.push(i); } }
    });
    if (!seen[s.n - 1]) { order.push(s.n - 1); }
    var k = 0, live = 0;
    function next() {
      while (live < 6 && k < order.length) {
        (function (i) {
          var im = new Image();
          live++;
          im.decoding = 'async';
          im.onload = function () { s.imgs[i] = im; live--; s.drawn = -1; next(); };
          im.onerror = function () { live--; next(); };
          im.src = 'seq/' + s.id + '/' + String(i + 1).padStart(3, '0') + '.webp';
        })(order[k++]);
      }
    }
    next();
  }
  function near(s, i, dir) {
    for (var d = 0; d < s.n; d++) {
      var j = i + d * dir;
      if (j >= 0 && j < s.n && s.imgs[j]) { return j; }
      j = i - d * dir;
      if (j >= 0 && j < s.n && s.imgs[j]) { return j; }
    }
    return -1;
  }
  function cover(ctx, im, cw, ch) {
    var sc = Math.max(cw / im.width, ch / im.height), w = im.width * sc, h = im.height * sc;
    ctx.drawImage(im, (cw - w) / 2, (ch - h) / 2, w, h);
  }
  function drawSeq(s) {
    var f = s.sp * (s.n - 1), i0 = Math.floor(f), fr = f - i0;
    var key = Math.round(f * 8);
    if (key === s.drawn) { return; }
    var a = near(s, i0, -1), b = near(s, Math.min(s.n - 1, i0 + 1), 1);
    if (a < 0) { return; }
    s.drawn = key;
    var c = s.ctx, cw = s.canvas.width, ch = s.canvas.height;
    c.globalAlpha = 1;
    c.fillStyle = '#000'; c.fillRect(0, 0, cw, ch);
    cover(c, s.imgs[a], cw, ch);
    if (b >= 0 && b !== a && fr > 0.02) { c.globalAlpha = fr; cover(c, s.imgs[b], cw, ch); c.globalAlpha = 1; }
  }
  function beats(s) {
    s.beats.forEach(function (bt) {
      var fin = bt.a <= 0 ? 1 : sstep(bt.a, bt.a + 0.07, s.sp);
      var fout = bt.b >= 1 ? 1 : 1 - sstep(bt.b - 0.07, bt.b, s.sp);
      var op = Math.min(fin, fout);
      if (Math.abs(op - bt.op) < 0.002) { return; }
      bt.op = op;
      bt.el.style.opacity = op.toFixed(3);
      bt.el.style.visibility = op < 0.01 ? 'hidden' : 'visible';
      var y = (1 - op) * (fin < 1 ? 28 : -28);
      bt.el.style.translate = '0 ' + y.toFixed(1) + 'px';
      if (bt.onShow && op > 0.6 && !bt.shown) { bt.shown = true; bt.onShow(); }
      if (op < 0.05) { bt.shown = false; }
    });
  }

  var last = performance.now();
  function tick(now) {
    var dt = Math.min(64, now - last); last = now;
    var y = scrollY, k = 1 - Math.pow(0.86, dt / 16.7);
    scenes.forEach(function (s) {
      var span = Math.max(1, s.h - VH);
      s.p = clamp((y - s.top) / span, 0, 1);
      s.sp += (s.p - s.sp) * k;
      if (Math.abs(s.p - s.sp) < 0.0004) { s.sp = s.p; }
      var vis = y + VH > s.top - VH * 1.5 && y < s.top + s.h + VH * 0.5;
      if (vis) { load(s); }
      s.vis = y + VH > s.top && y < s.top + s.h;
      if (!s.vis) { return; }
      if (s.canvas) { drawSeq(s); }
      beats(s);
      if (s.onFrame) { s.onFrame(s, now); }
    });
    ticker(now);
    requestAnimationFrame(tick);
  }

  /* ------------------------------------------------------------ ticker */
  var t0 = performance.now(), tk = $('ticker'), RATE = 1.45e9;
  function ticker(now) {
    if (!tk) { return; }
    tk.textContent = fmt((now - t0) / 1000 * RATE);
  }

  /* ------------------------------------------------------------ WebGL */
  // ChEMBL 37, 2048-bit Morgan r=2: molecules per popcount 0..160.
  var HIST = [0,49,52,33,50,75,127,99,232,210,281,410,373,739,782,1017,1544,1711,2096,2971,3440,4529,5346,6597,7471,9483,11070,13295,15876,18100,21655,25251,28815,33588,38279,42354,47408,52490,57306,62469,67458,71395,75542,78787,81783,84848,86500,88335,89507,89674,88486,87305,84916,83053,80975,78385,75299,72009,68967,65594,62006,58907,55657,52208,48367,45043,41681,37707,34800,31152,28696,25664,23411,20581,18658,16953,14942,13220,11806,10555,9229,8105,7132,6430,5659,5447,4758,4220,3960,3744,3476,3264,3271,2910,3075,2872,3259,2474,2376,2296,2092,2004,1783,1744,1679,1572,1532,1620,1355,1409,1296,1193,1059,1128,1002,867,842,813,774,750,699,664,678,633,565,588,550,448,459,452,401,390,345,347,345,313,313,305,265,356,247,300,301,229,220,243,265,236,176,188,178,138,157,116,147,128,126,121,108,109,107];
  var gl = null;
  (function initGL() {
    var scene = scenes.filter(function (s) { return s.el.dataset.gl; })[0];
    var cv = $('gl');
    if (!scene || !cv) { return; }
    var g = cv.getContext('webgl', { antialias: false, alpha: false, premultipliedAlpha: false });
    if (!g) { return; }
    var small = innerWidth < 720;
    var N = small ? 26000 : 70000;
    var LO = 68, HI = 94, Q = 80, MAXP = 120;

    // Sample popcounts from the real histogram.
    var hist = HIST.slice(); var tail = 0;
    for (var i = MAXP + 1; i < hist.length; i++) { tail += hist[i]; }
    hist = hist.slice(0, MAXP + 1); hist[MAXP] += tail;
    var cdf = [], acc = 0, tot = hist.reduce(function (a, b) { return a + b; }, 0);
    hist.forEach(function (v) { acc += v / tot; cdf.push(acc); });
    var seed = 7; function rnd() { seed = (seed * 16807) % 2147483647; return seed / 2147483647; }
    function pop() { var u = rnd(), lo = 0, hi = cdf.length - 1; while (lo < hi) { var m = (lo + hi) >> 1; if (cdf[m] < u) { lo = m + 1; } else { hi = m; } } return lo; }

    var pops = new Float32Array(N), cnt = new Int32Array(MAXP + 1);
    for (i = 0; i < N; i++) { pops[i] = pop(); cnt[pops[i]]++; }
    var maxc = Math.max.apply(null, Array.prototype.slice.call(cnt));
    var rank = new Int32Array(MAXP + 1);
    var cloud = new Float32Array(N * 3), histp = new Float32Array(N * 2), sd = new Float32Array(N), hit = new Float32Array(N), pp = new Float32Array(N);
    var X0 = small ? -0.92 : -0.86, X1 = -X0, Y0 = small ? -0.05 : -0.62, Y1 = small ? 0.6 : 0.62;
    for (i = 0; i < N; i++) {
      var p = pops[i];
      // Cloud: a slowly turning disc of clusters.
      var r = Math.pow(rnd(), 0.6) * 1.05, a = rnd() * Math.PI * 2, arm = Math.floor(rnd() * 5);
      a += arm * 1.2566 + r * 2.2;
      cloud[i * 3] = Math.cos(a) * r; cloud[i * 3 + 1] = (rnd() - 0.5) * 0.18 * (1.2 - r); cloud[i * 3 + 2] = Math.sin(a) * r;
      var cw = (X1 - X0) / (MAXP + 1);
      histp[i * 2] = X0 + (p + 0.15 + rnd() * 0.7) * cw;
      histp[i * 2 + 1] = Y0 + (rank[p]++ / maxc) * (Y1 - Y0);
      sd[i] = rnd(); pp[i] = p;
      hit[i] = (p >= LO && p <= HI && rnd() < 0.012 * Math.exp(-Math.pow((p - Q) / 9, 2))) ? 1 : 0;
    }

    var vs = [
      'attribute vec3 a_cloud; attribute vec2 a_hist; attribute float a_seed; attribute float a_pop; attribute float a_hit;',
      'uniform float u_morph, u_band, u_hits, u_time, u_aspect, u_px, u_rot, u_lo, u_hi;',
      'varying float v_a; varying float v_h;',
      'void main() {',
      '  float an = u_time * 0.05 + u_rot;',
      '  vec3 c = a_cloud; float cs = cos(an), sn = sin(an);',
      '  c.xz = vec2(c.x * cs - c.z * sn, c.x * sn + c.z * cs);',
      '  float ti = 0.42; float cy = c.y * cos(ti) - c.z * sin(ti); float cz = c.y * sin(ti) + c.z * cos(ti);',
      '  float z = cz + 2.6; vec2 cp = vec2(c.x, cy) * (2.1 / z); cp.x /= u_aspect;',
      '  float m = clamp(u_morph * 1.7 - a_seed * 0.7, 0.0, 1.0); m = m * m * (3.0 - 2.0 * m);',
      '  vec2 p = mix(cp, a_hist, m);',
      '  float inb = step(u_lo, a_pop) * step(a_pop, u_hi);',
      '  float o = (1.0 - inb) * u_band;',
      '  p.y -= o * o * (0.25 + a_seed * 0.9);',
      '  p.x += o * (a_seed - 0.5) * 0.08;',
      '  float al = mix(0.42, 0.36, m) * (1.0 - o * 0.94) + inb * u_band * 0.22;',
      '  float h = a_hit * u_hits * (0.8 + 0.2 * sin(u_time * 5.0 + a_seed * 40.0));',
      '  v_a = mix(al, 1.0, h); v_h = h;',
      '  gl_Position = vec4(p, 0.0, 1.0);',
      '  gl_PointSize = u_px * (1.0 + (1.0 - m) * (2.1 / z - 0.7) * 0.9 + h * 7.0);',
      '}'
    ].join('\n');
    var fs = [
      'precision mediump float; varying float v_a; varying float v_h;',
      'void main() {',
      '  float d = length(gl_PointCoord - 0.5);',
      '  float core = smoothstep(0.5, 0.15, d);',
      '  float glow = exp(-d * d * 18.0) * v_h;',
      '  gl_FragColor = vec4(vec3(1.0), (core * (1.0 - v_h) + glow + core * v_h) * v_a);',
      '}'
    ].join('\n');
    function sh(t, src) { var o = g.createShader(t); g.shaderSource(o, src); g.compileShader(o); return o; }
    var pr = g.createProgram();
    g.attachShader(pr, sh(g.VERTEX_SHADER, vs)); g.attachShader(pr, sh(g.FRAGMENT_SHADER, fs));
    g.linkProgram(pr);
    if (!g.getProgramParameter(pr, g.LINK_STATUS)) { return; }
    g.useProgram(pr);
    function attr(name, data, size) {
      var b = g.createBuffer(); g.bindBuffer(g.ARRAY_BUFFER, b); g.bufferData(g.ARRAY_BUFFER, data, g.STATIC_DRAW);
      var l = g.getAttribLocation(pr, name); g.enableVertexAttribArray(l); g.vertexAttribPointer(l, size, g.FLOAT, false, 0, 0);
    }
    attr('a_cloud', cloud, 3); attr('a_hist', histp, 2); attr('a_seed', sd, 1); attr('a_pop', pp, 1); attr('a_hit', hit, 1);
    var U = {};
    ['u_morph', 'u_band', 'u_hits', 'u_time', 'u_aspect', 'u_px', 'u_rot', 'u_lo', 'u_hi'].forEach(function (n) { U[n] = g.getUniformLocation(pr, n); });
    g.uniform1f(U.u_lo, LO - 0.5); g.uniform1f(U.u_hi, HI + 0.5);
    g.enable(g.BLEND); g.blendFunc(g.SRC_ALPHA, g.ONE);

    // Band edges and the query mark, in the same coordinates as the histogram.
    var stage = scene.el.querySelector('.stage');
    function edge(p, label, cls) {
      var d = doc.createElement('div'); d.className = 'edge ' + (cls || '');
      d.style.cssText = 'position:absolute;z-index:1;top:' + ((1 - Y1) / 2 * 100) + '%;bottom:' + ((1 + Y0) / 2 * 100) + '%;width:1px;background:#fff;opacity:0;pointer-events:none;';
      d.style.left = ((X0 + (p + 0.5) * (X1 - X0) / (MAXP + 1) + 1) / 2 * 100) + '%';
      d.innerHTML = '<span style="position:absolute;top:-22px;left:50%;transform:translateX(-50%);font:11px Tabular,monospace;color:#fff;white-space:nowrap">' + label + '</span>';
      stage.appendChild(d); return d;
    }
    var eLo = edge(LO - 0.5, String(LO)), eHi = edge(HI + 0.5, String(HI)), eQ = edge(Q, 'query', 'q');
    eQ.style.background = 'repeating-linear-gradient(#fff 0 4px, transparent 4px 8px)';
    var read = $('gl-read');

    gl = {
      resize: function () {
        cv.width = Math.round(cv.clientWidth * DPR); cv.height = Math.round(cv.clientHeight * DPR);
        g.viewport(0, 0, cv.width, cv.height);
      }
    };
    gl.resize();
    scene.onFrame = function (s, now) {
      var sp = s.sp;
      var morph = sstep(0.16, 0.44, sp), band = sstep(0.5, 0.64, sp), hits = sstep(0.76, 0.88, sp);
      g.clearColor(0, 0, 0, 1); g.clear(g.COLOR_BUFFER_BIT);
      g.uniform1f(U.u_morph, morph); g.uniform1f(U.u_band, band); g.uniform1f(U.u_hits, hits);
      g.uniform1f(U.u_time, now / 1000); g.uniform1f(U.u_rot, sp * 3.0);
      g.uniform1f(U.u_aspect, cv.width / cv.height); g.uniform1f(U.u_px, (small ? 1.6 : 1.9) * DPR);
      g.drawArrays(g.POINTS, 0, N);
      var eo = (band * (1 - hits * 0.4)).toFixed(3);
      eLo.style.opacity = eHi.style.opacity = eo;
      eQ.style.opacity = (sstep(0.46, 0.56, sp) * 0.8).toFixed(3);
      var txt = sp < 0.2 ? '2,897,819 fingerprints · 2,048 bits each'
        : sp < 0.48 ? 'sorted by popcount · 1 → 120+'
        : sp < 0.76 ? 'query popcount 80 · Tanimoto ≥ 0.85\nband 68–94 · 88.8% never read'
        : 'band scored exactly · every hit found';
      if (read.textContent !== txt) { read.textContent = txt; }
    };
  })();

  /* ------------------------------------------------------------ demo */
  function api(path, body) {
    var ctl = new AbortController(), timer = setTimeout(function () { ctl.abort(); }, 20000);
    return fetch(API + path, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body), signal: ctl.signal })
      .then(function (r) {
        return r.json().catch(function () { return {}; }).then(function (j) {
          clearTimeout(timer);
          if (!r.ok) { var e = new Error(j.error || 'HTTP ' + r.status); e.status = r.status; throw e; }
          return j;
        });
      }, function (e) { clearTimeout(timer); e.offline = true; throw e; });
  }
  var PRE = {
    sim: [['aspirin', 'CC(=O)Oc1ccccc1C(=O)O'], ['caffeine', 'CN1C=NC2=C1C(=O)N(C(=O)N2C)C'], ['imatinib', 'Cc1ccc(NC(=O)c2ccc(CN3CCN(C)CC3)cc2)cc1Nc1nccc(-c2cccnc2)n1'], ['ibuprofen', 'CC(C)Cc1ccc(cc1)C(C)C(=O)O']],
    sub: [['indole', 'c1ccc2[nH]ccc2c1'], ['sulfonamide', 'S(=O)(=O)N'], ['steroid core', 'C1CCC2C(C1)CCC1C2CCC2CCCC12'], ['carboxylic acid', 'C(=O)[OH]']]
  };
  var mode = 'sim', busy = false, q = $('q');
  function setMode(m) {
    mode = m;
    doc.querySelectorAll('.modes button').forEach(function (b) { b.setAttribute('aria-checked', b.dataset.mode === m); });
    $('chips').innerHTML = '';
    PRE[m].forEach(function (p) {
      var b = doc.createElement('button'); b.type = 'button'; b.textContent = p[0]; b.title = p[1];
      b.onclick = function () { q.value = p[1]; run(); };
      $('chips').appendChild(b);
    });
  }
  doc.querySelectorAll('.modes button').forEach(function (b) {
    b.onclick = function () { setMode(b.dataset.mode); q.value = PRE[b.dataset.mode][0][1]; run(); };
  });
  $('q-form').onsubmit = function (e) { e.preventDefault(); run(); };

  function tween(el, to, f) {
    var from = parseFloat(el.dataset.v || 0), s = performance.now(); el.dataset.v = to;
    (function fr(t) { var k = Math.min(1, (t - s) / 700); el.textContent = f(from + (to - from) * ease(k)); if (k < 1) { requestAnimationFrame(fr); } })(s);
  }

  // The strip: the whole index as thin bars; pruned bars drop, the scan sweeps the rest, hits flare.
  var strip = $('strip'), sx = strip.getContext('2d'), sraf = 0;
  function hash(i) { i = Math.imul(i ^ 0x9e3779b9, 0x85ebca6b); i ^= i >>> 13; i = Math.imul(i, 0xc2b2ae35); return ((i ^ (i >>> 16)) >>> 0) / 4294967296; }
  function animStrip(r) {
    cancelAnimationFrame(sraf);
    var W = strip.clientWidth, H = strip.clientHeight;
    strip.width = W * DPR; strip.height = H * DPR; sx.setTransform(DPR, 0, 0, DPR, 0, 0);
    var n = Math.floor(W / 3), keepF = (r.mode === 'sim' ? r.scanned : r.screened) / r.total, hitF = r.count / r.total;
    var keep = [], hits = [], order = [];
    var w = Math.max(1, Math.round(keepF * n)), a0 = Math.round(n * 0.5 - w / 2);
    for (var i = 0; i < n; i++) {
      keep[i] = r.mode === 'sim' ? (i >= a0 && i < a0 + w) : hash(i + r.seed) < keepF;
      if (keep[i]) { order.push(i); }
    }
    var nh = r.count ? Math.max(1, Math.min(order.length, Math.round(hitF * n))) : 0;
    order.slice().sort(function (x, y) { return hash(x + 99 * r.seed) - hash(y + 99 * r.seed); }).slice(0, nh).forEach(function (i) { hits[i] = 1; });
    var s = performance.now();
    (function fr(t) {
      var e = t - s, P = Math.min(1, e / 450), S = Math.min(1, Math.max(0, (e - 350) / 650)), Hh = Math.max(0, (e - 950) / 500);
      sx.clearRect(0, 0, W, H);
      for (var i = 0; i < n; i++) {
        var x = i * 3, h = H * 0.42, al = 0.35;
        if (!keep[i]) {
          var d = clamp(P * 1.6 - Math.abs(i / n - 0.5) * (r.mode === 'sim' ? 1.6 : 0) - hash(i) * (r.mode === 'sim' ? 0 : 1), 0, 1);
          h *= 1 - d * 0.85; al = 0.35 - d * 0.27;
        } else {
          var k = order.indexOf(i) / order.length;
          if (k < S) { al = 0.75; }
          if (Math.abs(k - S) < 0.02 && S < 1) { al = 1; h = H * 0.7; }
          if (hits[i] && Hh > 0) { al = 1; h = H * Math.min(1, 0.42 + Hh * 0.9); }
        }
        sx.fillStyle = 'rgba(255,255,255,' + al + ')';
        sx.fillRect(x, H - h, 2, h);
      }
      if (e < 1600) { sraf = requestAnimationFrame(fr); }
    })(s);
  }

  var SD = null, sdP = null;
  function loadSD() {
    if (sdP) { return sdP; }
    sdP = new Promise(function (res) {
      var sc = doc.createElement('script'); sc.src = 'vendor/smiles-drawer.min.js'; sc.async = true;
      sc.onload = function () {
        var L = window.SmilesDrawer;
        if (L && L.SvgDrawer) {
          SD = new L.SvgDrawer({ width: 160, height: 90, bondThickness: 1.1, bondLength: 16, shortBondLength: 0.82, fontSizeLarge: 6, fontSizeSmall: 4, padding: 6, compactDrawing: true,
            themes: { bw: { C: '#fff', O: '#fff', N: '#fff', F: '#fff', CL: '#fff', BR: '#fff', I: '#fff', P: '#fff', S: '#fff', B: '#fff', SI: '#fff', H: '#aaa', BACKGROUND: '#000' } } });
        }
        res();
      };
      sc.onerror = function () { res(); };
      doc.body.appendChild(sc);
    });
    return sdP;
  }
  function mols(r) {
    var hs = r.hits.slice();
    if (r.mode === 'sub') { hs.sort(function (a, b) { return a.smiles.length - b.smiles.length; }); }
    hs = hs.slice(0, 6);
    var box = $('r-mols');
    box.innerHTML = hs.map(function (h, i) {
      return '<a style="--i:' + i + '" href="https://www.ebi.ac.uk/chembl/compound_report_card/' + encodeURIComponent(h.id) + '/" target="_blank" rel="noopener"><svg data-s="' + esc(h.smiles) + '"></svg><span><em>' + esc(h.id) + '</em>' + (h.score != null ? '<b>' + h.score.toFixed(2) + '</b>' : '') + '</span></a>';
    }).join('');
    loadSD().then(function () {
      box.querySelectorAll('svg[data-s]').forEach(function (svg) {
        var s = svg.dataset.s;
        try {
          if (!SD) { throw 0; }
          window.SmilesDrawer.parse(s, function (t) { SD.draw(t, svg, 'bw', null, false); }, function () { svg.outerHTML = '<code>' + esc(s) + '</code>'; });
        } catch (e) { svg.outerHTML = '<code>' + esc(s) + '</code>'; }
      });
    });
  }

  var seedN = 1;
  function run() {
    var text = q.value.trim();
    if (!text || busy) { return; }
    busy = true; $('err').hidden = true;
    var m = mode;
    var p = m === 'sim' ? api('/v1/similarity', { smiles: text, threshold: 0.7, limit: 6 })
      : api('/v1/substructure', { smarts: text, limit: 40, timeout_ms: 8000 });
    p.then(function (r) {
      r.mode = m; r.seed = seedN++;
      tween($('r-hits'), r.count, fmt);
      $('r-hits-k').textContent = r.count === 1 ? 'hit' : 'hits';
      var ms = r.took_us / 1000;
      tween($('r-ms'), ms, function (v) { return v.toFixed(ms < 10 ? 2 : ms < 100 ? 1 : 0); });
      $('r-of').textContent = 'across ' + fmt(r.total) + ' molecules';
      var kept = m === 'sim' ? r.scanned : r.screened;
      $('r-skip').textContent = (100 * (1 - kept / r.total)).toFixed(1) + '% ' + (m === 'sim' ? 'skipped by the bound' : 'ruled out before matching');
      $('r-rate').textContent = fmt(r.total / (r.took_us / 1e6) / 1e6) + ' M molecules/s on this demo box';
      animStrip(r); mols(r);
      doc.querySelectorAll('#chips button').forEach(function (b) { b.classList.toggle('on', b.title === text); });
    }).catch(function (e) {
      var msg = e.offline ? 'The demo server is not answering right now.' : e.status === 408 ? 'This query passed its deadline.' : e.message;
      var mm = /at byte (\d+)/.exec(msg);
      $('err').textContent = mm ? text + '\n' + ' '.repeat(+mm[1]) + '^ ' + msg : msg;
      $('err').hidden = false;
    }).then(function () { busy = false; });
  }
  setMode('sim');
  new IntersectionObserver(function (es, ob) {
    if (es[0].isIntersecting) { loadSD(); run(); ob.disconnect(); }
  }, { rootMargin: '0px 0px -30% 0px' }).observe($('demo'));

  /* ------------------------------------------------------------ race */
  var LANES = [['mossflux', 'BitBound', 10.6, 1], ['FPSim2', '1 worker', 36.1], ['RDKit', 'BulkTanimotoSimilarity', 530], ['RDKit', 'Python loop', 4700]];
  var race = $('race');
  race.innerHTML = LANES.map(function (l) {
    return '<div class="lane' + (l[3] ? ' mf' : '') + '"><span class="nm">' + l[0] + '<small>' + l[1] + '</small></span><span class="tr"><span class="fl"></span></span><span class="t">0 ms</span></div>';
  }).join('');
  var lanes = race.querySelectorAll('.lane'), rraf = 0;
  function runRace() {
    cancelAnimationFrame(rraf);
    var s = performance.now(), MAX = 4700;
    (function f(t) {
      var e = t - s, all = true;
      lanes.forEach(function (l, i) {
        var d = LANES[i][2], k = Math.min(e, d) / MAX;
        l.querySelector('.fl').style.transform = 'scaleX(' + k + ')';
        if (e < d) { all = false; l.querySelector('.t').textContent = fmt(e) + ' ms'; l.classList.remove('done'); }
        else if (!l.classList.contains('done')) {
          l.classList.add('done');
          l.querySelector('.t').textContent = (d < 100 ? d.toFixed(1) : fmt(d)) + ' ms';
          if (i) { l.querySelector('small').textContent = LANES[i][1] + ' · ' + Math.round(d / LANES[0][2]) + '× slower'; }
        }
      });
      if (!all) { rraf = requestAnimationFrame(f); }
    })(s);
  }
  scenes.forEach(function (s) {
    s.beats.forEach(function (b) { if (b.el.classList.contains('race-beat')) { b.onShow = runRace; } });
  });

  /* ------------------------------------------------------------ start */
  measure();
  addEventListener('resize', function () { clearTimeout(measure.t); measure.t = setTimeout(measure, 120); });
  addEventListener('load', measure);
  requestAnimationFrame(tick);
})();
