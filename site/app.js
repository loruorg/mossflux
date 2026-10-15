/* mossflux.com. One continuous WebGL particle flow behind four sections.
   Scroll moves the flow through four shapes; light trails come from a
   feedback buffer. API base: API below, or ?api=<url> for local testing. */
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
  var ease = function (t) { return 1 - Math.pow(1 - t, 3); };
  var DPR = Math.min(1.5, devicePixelRatio || 1);
  var reduce = matchMedia('(prefers-reduced-motion: reduce)').matches;
  var secs = [].slice.call(doc.querySelectorAll('.sec'));

  /* ------------------------------------------------------------ flow */
  var flow = (function () {
    var cv = $('flux');
    var g = cv.getContext('webgl', { antialias: false, alpha: false, depth: false, preserveDrawingBuffer: false });
    if (!g) { return null; }
    var small = innerWidth < 760;
    var N = small ? 40000 : 110000;

    var VS = [
      'attribute vec4 a_s;',
      'uniform float u_k, u_t, u_aspect, u_px, u_off, u_stir;',
      'varying float v_a;',
      'const float PI = 3.14159265;',
      'vec3 vortex(vec4 s, float t) {',
      '  float r = 0.16 + 0.84 * pow(s.y, 0.65);',
      '  float th = s.x * 6.2832 + t * 0.10 / (r + 0.25) + floor(s.z * 3.0) * 2.0944 + r * 2.8;',
      '  vec3 p = vec3(cos(th) * r, (s.w - 0.5) * 0.10 * (1.15 - r), sin(th) * r);',
      '  float tl = 1.08; p = vec3(p.x, p.y * cos(tl) - p.z * sin(tl), p.y * sin(tl) + p.z * cos(tl));',
      '  float z = p.z + 2.3;',
      '  return vec3(p.x * 1.7 / z / u_aspect * 1.25 + u_off, p.y * 1.7 / z + 0.02, 0.55 * (1.25 - r * 0.6));',
      '}',
      'vec3 lanes(vec4 s, float t) {',
      '  float lane = floor(s.z * 9.0);',
      '  float fast = step(8.0, lane);',
      '  float sp = mix(0.05 + 0.25 * fract(lane * 0.618 + 0.3), 1.6, fast) * (0.8 + 0.4 * s.y);',
      '  float x = fract(s.x + t * sp * 0.3) * 2.8 - 1.4;',
      '  float y = -0.6 + lane * 0.15 + (s.w - 0.5) * 0.05 + 0.02 * sin(x * 3.0 + lane + t);',
      '  float pk = smoothstep(0.55, 1.0, fract(s.x * 5.0 + s.y * 0.15));',
      '  return vec3(x, y, (0.5 + 0.5 * fast) * (0.2 + 0.8 * pk));',
      '}',
      'vec3 band(vec4 s, float t) {',
      '  float v = (s.x + s.y + s.z) / 3.0;',
      '  float col = floor(v * 96.0) / 96.0;',
      '  float h = exp(-pow((col - 0.5) / 0.11, 2.0)) * 1.25;',
      '  float y = -0.72 + fract(s.w + t * 0.03) * h;',
      '  float inb = step(0.565, col) * step(col, 0.635);',
      '  return vec3((col - 0.5) * 2.4 + u_off * 0.4, y, mix(0.05, 0.5, inb));',
      '}',
      'vec3 field(vec4 s, float t) {',
      '  float gx = s.x - 0.5, gz = fract(s.y - t * 0.025);',
      '  float z = 0.25 + gz * 5.0, x = gx * 9.0;',
      '  float y = -0.5 + 0.06 * sin(gx * 18.0 + t * 0.7 + gz * 9.0) + 0.04 * sin(s.z * 40.0 + t);',
      '  return vec3(x / z * 0.9, y / z * 0.9 + 0.12, 0.5 * (1.0 - gz) * (1.0 - gz));',
      '}',
      'vec3 form(float k, vec4 s, float t) {',
      '  if (k < 0.5) return vortex(s, t);',
      '  if (k < 1.5) return lanes(s, t);',
      '  if (k < 2.5) return band(s, t);',
      '  return field(s, t);',
      '}',
      'void main() {',
      '  float i = floor(u_k), f = u_k - i;',
      '  float m = clamp((f - 0.5) * 1.7 + 0.5 + (a_s.w - 0.5) * 0.7, 0.0, 1.0);',
      '  m = m * m * (3.0 - 2.0 * m);',
      '  vec3 A = form(i, a_s, u_t), B = form(min(i + 1.0, 3.0), a_s, u_t);',
      '  vec3 p = mix(A, B, m);',
      '  float sw = sin(m * PI);',
      '  vec2 q = p.xy * 2.3 + vec2(a_s.z * 6.0, a_s.x * 6.0);',
      '  vec2 curl = vec2(sin(q.y + u_t * 0.7), cos(q.x * 1.3 - u_t * 0.5));',
      '  p.xy += curl * (sw * 0.22 + u_stir * 0.02 + 0.006);',
      '  v_a = p.z * (1.0 + sw * 0.6);',
      '  gl_Position = vec4(p.xy, 0.0, 1.0);',
      '  gl_PointSize = u_px * (1.0 + sw * 0.6);',
      '}'
    ].join('\n');
    var FS = 'precision mediump float; varying float v_a; void main() { float d = length(gl_PointCoord - 0.5); gl_FragColor = vec4(vec3(v_a * smoothstep(0.5, 0.0, d) * 0.42), 1.0); }';
    var QV = 'attribute vec2 a_p; varying vec2 v_uv; void main() { v_uv = a_p * 0.5 + 0.5; gl_Position = vec4(a_p, 0.0, 1.0); }';
    var FADE = 'precision mediump float; uniform sampler2D u_tex; uniform float u_decay; varying vec2 v_uv; void main() { vec3 c = texture2D(u_tex, v_uv).rgb; gl_FragColor = vec4(max(c * u_decay - 0.006, 0.0), 1.0); }';
    var SHOW = 'precision mediump float; uniform sampler2D u_tex; varying vec2 v_uv; void main() { vec3 c = texture2D(u_tex, v_uv).rgb; float l = dot(c, vec3(0.333)); l = 1.0 - exp(-l * 1.6); float vg = smoothstep(1.25, 0.35, length(v_uv - 0.5) * 1.6); gl_FragColor = vec4(vec3(l * vg), 1.0); }';

    function prog(vs, fs) {
      var p = g.createProgram();
      [[g.VERTEX_SHADER, vs], [g.FRAGMENT_SHADER, fs]].forEach(function (x) {
        var s = g.createShader(x[0]); g.shaderSource(s, x[1]); g.compileShader(s);
        if (!g.getShaderParameter(s, g.COMPILE_STATUS)) { throw new Error(g.getShaderInfoLog(s)); }
        g.attachShader(p, s);
      });
      g.linkProgram(p);
      return p;
    }
    var P, F, S;
    try { P = prog(VS, FS); F = prog(QV, FADE); S = prog(QV, SHOW); } catch (e) { return null; }

    var seeds = new Float32Array(N * 4), r = 1;
    for (var i = 0; i < N * 4; i++) { r = (r * 16807) % 2147483647; seeds[i] = r / 2147483647; }
    var sb = g.createBuffer(); g.bindBuffer(g.ARRAY_BUFFER, sb); g.bufferData(g.ARRAY_BUFFER, seeds, g.STATIC_DRAW);
    var qb = g.createBuffer(); g.bindBuffer(g.ARRAY_BUFFER, qb); g.bufferData(g.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), g.STATIC_DRAW);
    var U = {};
    ['u_k', 'u_t', 'u_aspect', 'u_px', 'u_off', 'u_stir'].forEach(function (n) { U[n] = g.getUniformLocation(P, n); });

    var fb = [], tx = [], W = 0, H = 0, cur = 0;
    function target(w, h) {
      tx.forEach(function (t) { g.deleteTexture(t); }); fb.forEach(function (f) { g.deleteFramebuffer(f); });
      tx = []; fb = [];
      for (var j = 0; j < 2; j++) {
        var t = g.createTexture(); g.bindTexture(g.TEXTURE_2D, t);
        g.texImage2D(g.TEXTURE_2D, 0, g.RGBA, w, h, 0, g.RGBA, g.UNSIGNED_BYTE, null);
        g.texParameteri(g.TEXTURE_2D, g.TEXTURE_MIN_FILTER, g.LINEAR); g.texParameteri(g.TEXTURE_2D, g.TEXTURE_MAG_FILTER, g.LINEAR);
        g.texParameteri(g.TEXTURE_2D, g.TEXTURE_WRAP_S, g.CLAMP_TO_EDGE); g.texParameteri(g.TEXTURE_2D, g.TEXTURE_WRAP_T, g.CLAMP_TO_EDGE);
        var f = g.createFramebuffer(); g.bindFramebuffer(g.FRAMEBUFFER, f);
        g.framebufferTexture2D(g.FRAMEBUFFER, g.COLOR_ATTACHMENT0, g.TEXTURE_2D, t, 0);
        g.clearColor(0, 0, 0, 1); g.clear(g.COLOR_BUFFER_BIT);
        tx.push(t); fb.push(f);
      }
    }
    function resize() {
      W = Math.round(cv.clientWidth * DPR); H = Math.round(cv.clientHeight * DPR);
      if (cv.width !== W || cv.height !== H) { cv.width = W; cv.height = H; target(W, H); }
    }
    function quad(p) {
      g.bindBuffer(g.ARRAY_BUFFER, qb);
      var l = g.getAttribLocation(p, 'a_p'); g.enableVertexAttribArray(l); g.vertexAttribPointer(l, 2, g.FLOAT, false, 0, 0);
      g.drawArrays(g.TRIANGLE_STRIP, 0, 4);
    }
    function frame(k, t, stir) {
      var nxt = 1 - cur;
      g.bindFramebuffer(g.FRAMEBUFFER, fb[nxt]); g.viewport(0, 0, W, H);
      g.disable(g.BLEND);
      g.useProgram(F); g.activeTexture(g.TEXTURE0); g.bindTexture(g.TEXTURE_2D, tx[cur]);
      g.uniform1i(g.getUniformLocation(F, 'u_tex'), 0); g.uniform1f(g.getUniformLocation(F, 'u_decay'), 0.9);
      quad(F);
      g.enable(g.BLEND); g.blendFunc(g.ONE, g.ONE);
      g.useProgram(P);
      g.bindBuffer(g.ARRAY_BUFFER, sb);
      var l = g.getAttribLocation(P, 'a_s'); g.enableVertexAttribArray(l); g.vertexAttribPointer(l, 4, g.FLOAT, false, 0, 0);
      g.uniform1f(U.u_k, k); g.uniform1f(U.u_t, t); g.uniform1f(U.u_aspect, W / H);
      g.uniform1f(U.u_px, (small ? 1.3 : 1.5) * DPR); g.uniform1f(U.u_off, small ? 0.0 : 0.36); g.uniform1f(U.u_stir, stir);
      g.drawArrays(g.POINTS, 0, N);
      g.disable(g.BLEND);
      g.bindFramebuffer(g.FRAMEBUFFER, null); g.viewport(0, 0, W, H);
      g.useProgram(S); g.bindTexture(g.TEXTURE_2D, tx[nxt]); g.uniform1i(g.getUniformLocation(S, 'u_tex'), 0);
      quad(S);
      cur = nxt;
    }
    resize();
    return { resize: resize, frame: frame };
  })();

  /* ------------------------------------------------------------ scroll */
  var centers = [], k = 0, last = performance.now(), t = 0, lastY = scrollY, stir = 0;
  function measure() {
    var max = doc.documentElement.scrollHeight - innerHeight;
    centers = secs.map(function (s) { return clamp(s.offsetTop + s.offsetHeight / 2 - innerHeight / 2, 0, max); });
    centers[0] = 0; centers[centers.length - 1] = max;
    if (flow) { flow.resize(); }
  }
  function targetK(y) {
    for (var i = 0; i < centers.length - 1; i++) {
      if (y <= centers[i + 1]) { return i + clamp((y - centers[i]) / Math.max(1, centers[i + 1] - centers[i]), 0, 1); }
    }
    return centers.length - 1;
  }
  function sections() {
    var vh = innerHeight;
    secs.forEach(function (s) {
      var r = s.getBoundingClientRect();
      var v = clamp(Math.min(r.bottom - vh * 0.22, vh * 0.78 - r.top) / (vh * 0.42), 0, 1);
      var inn = s.firstElementChild;
      if (Math.abs((+inn.dataset.v || -1) - v) < 0.003) { return; }
      inn.dataset.v = v;
      inn.style.setProperty('--v', v.toFixed(3));
      inn.style.setProperty('--dir', r.top > 0 ? 1 : -1);
      inn.style.visibility = v < 0.01 ? 'hidden' : 'visible';
    });
  }
  function tick(now) {
    var dt = Math.min(50, now - last) / 1000; last = now;
    var y = scrollY, kt = targetK(y);
    k += (kt - k) * (1 - Math.pow(0.04, dt));
    if (Math.abs(kt - k) < 0.0005) { k = kt; }
    stir = stir * Math.pow(0.05, dt) + Math.abs(y - lastY) / innerHeight * 6; lastY = y;
    if (!reduce) { t += dt; }
    if (flow) { flow.frame(k, t, Math.min(stir, 3)); }
    sections();
    requestAnimationFrame(tick);
  }

  /* ------------------------------------------------------------ charts */
  var CH = {
    chembl: { unit: 'M/s', rows: [['mossflux', 273.4, 1], ['FPSim2', 80.3], ['RDKit, bulk', 5.47], ['RDKit, loop', 0.62]] },
    arthor: { unit: 'B/s', rows: [['mossflux', 1.45, 1], ['Arthor', 1.2, 0, 0.9]] }
  };
  [].forEach.call(doc.querySelectorAll('.chart'), function (fig) {
    var c = CH[fig.dataset.chart], max = c.rows[0][1];
    fig.querySelector('.bars').innerHTML = c.rows.map(function (r, i) {
      var w = Math.max(0.6, r[1] / max * 100), lead = c.rows[0][1] / r[1];
      var val = r[3] ? r[3] + '–' + r[1] : (r[1] < 10 ? r[1].toFixed(2) : r[1].toFixed(r[1] < 100 ? 1 : 0));
      var tag = r[2] ? '' : '<small>' + (lead < 10 ? lead.toFixed(1) : Math.round(lead)) + '× slower</small>';
      var range = r[3] ? '<span class="r" style="--l:' + (r[3] / max * 100) + '%;--rw:' + ((r[1] - r[3]) / max * 100) + '%"></span>' : '';
      return '<div class="b' + (r[2] ? ' me' : '') + '" style="--i:' + i + '"><span class="n">' + r[0] + '</span><span class="tr"><span class="f" style="--w:' + w + '%"></span>' + range + '<span class="v" style="--w:' + w + '%">' + val + ' ' + c.unit + tag + '</span></span></div>';
    }).join('');
    if (fig.dataset.chart === 'arthor') { fig.querySelectorAll('.b')[1].querySelector('.f').style.width = (0.9 / max * 100) + '%'; }
  });
  var cio = new IntersectionObserver(function (es) {
    es.forEach(function (e) { if (e.isIntersecting) { e.target.classList.add('on'); cio.unobserve(e.target); } });
  }, { threshold: 0.4 });
  [].forEach.call(doc.querySelectorAll('.chart'), function (f) { cio.observe(f); });

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
    var box = $('mols');
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

  /* ------------------------------------------------------------ start */
  measure();
  addEventListener('resize', function () { clearTimeout(measure.t); measure.t = setTimeout(measure, 120); });
  addEventListener('load', measure);
  requestAnimationFrame(tick);
})();
