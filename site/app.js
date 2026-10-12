/* mossflux.com: background video, live demo, benchmark charts.
   API base: API below, or ?api=<url> for local testing. */
(function () {
  'use strict';

  var API = 'https://api.mossflux.com';
  try {
    var o = new URLSearchParams(location.search).get('api');
    if (o && /^https?:\/\/\S+$/.test(o)) { API = o.replace(/\/+$/, ''); }
  } catch (e) { /* default */ }

  var doc = document;
  var $ = function (id) { return doc.getElementById(id); };
  var reduce = matchMedia('(prefers-reduced-motion: reduce)').matches;
  var fmt = function (n) { return Math.round(n).toLocaleString('en-US'); };
  var esc = function (s) { return String(s).replace(/[&<>"']/g, function (c) { return '&#' + c.charCodeAt(0) + ';'; }); };
  var ease = function (t) { return 1 - Math.pow(1 - t, 3); };

  /* ------------------------------------------------------------ video */
  var THEMES = ['mycelium', 'fern', 'roots', 'glow'];
  (function video() {
    var vs = doc.querySelectorAll('.bg-v');
    var conn = navigator.connection || {};
    var portrait = matchMedia('(max-aspect-ratio: 1/1)').matches;
    var o = portrait ? 'p' : 'l';
    var start = Math.floor(Math.random() * THEMES.length);
    var i = start, cur = 0;
    var src = function (k) { return 'bg/' + THEMES[k % THEMES.length] + '-' + o; };
    vs[0].poster = src(i) + '.jpg';
    if (reduce || conn.saveData) { return; }
    vs[0].src = src(i) + '.mp4';
    vs[0].play().catch(function () {});
    function next() {
      var a = vs[cur], b = vs[1 - cur];
      i++;
      b.src = src(i) + '.mp4';
      b.preload = 'auto';
      b.currentTime = 0;
      b.play().then(function () {
        b.classList.add('is-on');
        a.classList.remove('is-on');
        setTimeout(function () { a.pause(); }, 1800);
        cur = 1 - cur;
      }).catch(function () {});
    }
    vs.forEach(function (v) {
      v.addEventListener('timeupdate', function () {
        if (v === vs[cur] && v.duration && v.duration - v.currentTime < 1.7 && !v.dataset.handed) {
          v.dataset.handed = '1';
          next();
        }
      });
      v.addEventListener('playing', function () { delete v.dataset.handed; });
    });
    doc.addEventListener('visibilitychange', function () {
      var v = vs[cur];
      if (doc.hidden) { v.pause(); } else { v.play().catch(function () {}); }
    });
  })();

  /* ----------------------------------------------------------- scroll */
  (function scroll() {
    var shade = 0;
    function on() {
      var h = innerHeight, y = scrollY;
      doc.body.classList.toggle('past-hero', y > h * 0.55);
      var s = Math.min(0.86, 0.55 + y / (h * 4));
      if (Math.abs(s - shade) > 0.01) { shade = s; doc.body.style.setProperty('--shade', s.toFixed(2)); }
    }
    addEventListener('scroll', on, { passive: true });
    on();
  })();

  /* ------------------------------------------------------- reveal */
  function countUp(el) {
    var to = parseFloat(el.dataset.count), dec = +(el.dataset.dec || 0);
    if (reduce) { el.textContent = to.toFixed(dec); return; }
    var t0 = performance.now(), d = 1400;
    (function f(t) {
      var k = Math.min(1, (t - t0) / d);
      el.textContent = (to * ease(k)).toFixed(dec);
      if (k < 1) { requestAnimationFrame(f); }
    })(t0);
  }
  var io = new IntersectionObserver(function (es) {
    es.forEach(function (e) {
      if (!e.isIntersecting) { return; }
      e.target.classList.add('in');
      e.target.querySelectorAll('[data-count]').forEach(countUp);
      if (e.target.dataset.on) { window[e.target.dataset.on](); }
      io.unobserve(e.target);
    });
  }, { threshold: 0.25 });
  doc.querySelectorAll('.sec-head, .tile, .race, .sub-chart, .code, .facts li').forEach(function (el) {
    el.classList.add('rv'); io.observe(el);
  });

  /* ------------------------------------------------------------- API */
  function api(path, body) {
    var ctl = new AbortController();
    var timer = setTimeout(function () { ctl.abort(); }, 20000);
    var opt = body ? { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body), signal: ctl.signal } : { signal: ctl.signal };
    var t0 = performance.now();
    return fetch(API + path, opt).then(function (r) {
      return r.json().catch(function () { return {}; }).then(function (j) {
        clearTimeout(timer);
        j._rtt = performance.now() - t0;
        if (!r.ok) { var e = new Error(j.error || ('HTTP ' + r.status)); e.status = r.status; throw e; }
        return j;
      });
    }, function (e) { clearTimeout(timer); e.offline = true; throw e; });
  }

  var TOTAL = 2896304;
  function online(j) {
    var d = (j.databases || [])[0];
    if (!d) { return; }
    TOTAL = d.molecules;
    $('live-pill').classList.add('on');
    $('live-text').textContent = 'live · ' + fmt(d.molecules) + ' molecules indexed';
    $('db-line').textContent = fmt(d.molecules) + ' molecules · ' + d.fingerprint.kind.toUpperCase() + ' ' + fmt(d.fingerprint.bits) + '-bit · one server';
    $('hero-n').textContent = (d.molecules / 1e6).toFixed(1) + ' million';
  }
  api('/v1/databases').then(online).then(function () {
    return api('/v1/similarity', { smiles: 'CC(=O)Oc1ccccc1C(=O)O', threshold: 0.7, limit: 1 });
  }).then(function (r) {
    $('hero-ms').textContent = (r.took_us / 1000).toFixed(1);
  }).catch(function () {
    $('live-text').textContent = 'live index offline';
  });

  /* ------------------------------------------------------------ demo */
  var PRESETS = {
    sim: [['aspirin', 'CC(=O)Oc1ccccc1C(=O)O'], ['caffeine', 'CN1C=NC2=C1C(=O)N(C(=O)N2C)C'],
      ['ibuprofen', 'CC(C)Cc1ccc(cc1)C(C)C(=O)O'], ['imatinib', 'Cc1ccc(NC(=O)c2ccc(CN3CCN(C)CC3)cc2)cc1Nc1nccc(-c2cccnc2)n1']],
    sub: [['indole', 'c1ccc2[nH]ccc2c1'], ['sulfonamide', 'S(=O)(=O)N'], ['carboxylic acid', 'C(=O)[OH]'],
      ['steroid core', 'C1CCC2C(C1)CCC1C2CCC2CCCC12'], ['basic amine', '[NX3;H2,H1;!$(NC=O)]']]
  };
  var TOUR = [['sim', 'CC(=O)Oc1ccccc1C(=O)O', 0.7], ['sub', 'c1ccc2[nH]ccc2c1'], ['sim', 'CN1C=NC2=C1C(=O)N(C(=O)N2C)C', 0.5],
    ['sub', 'S(=O)(=O)N'], ['sim', 'Cc1ccc(NC(=O)c2ccc(CN3CCN(C)CC3)cc2)cc1Nc1nccc(-c2cccnc2)n1', 0.6], ['sub', 'C1CCC2C(C1)CCC1C2CCC2CCCC12']];

  var demo = $('demo'), q = $('q'), thr = $('thr'), state = { mode: 'sim', busy: false, auto: true, step: 0, hist: [] };

  function setMode(m) {
    state.mode = m;
    demo.dataset.mode = m;
    $('m-sim').setAttribute('aria-selected', m === 'sim');
    $('m-sub').setAttribute('aria-selected', m === 'sub');
    $('q-kind').textContent = $('q-label').textContent = m === 'sim' ? 'SMILES' : 'SMARTS';
    $('f2k').textContent = m === 'sim' ? 'scored' : 'to matcher';
    $('v-skip-k').textContent = m === 'sim' ? 'skipped by the bound' : 'pruned before VF2';
    var c = $('chips'); c.innerHTML = '';
    PRESETS[m].forEach(function (p) {
      var b = doc.createElement('button'); b.type = 'button'; b.textContent = p[0]; b.title = p[1];
      b.onclick = function () { stopAuto(); q.value = p[1]; run(); };
      c.appendChild(b);
    });
  }
  $('m-sim').onclick = function () { stopAuto(); setMode('sim'); q.value = PRESETS.sim[0][1]; run(); };
  $('m-sub').onclick = function () { stopAuto(); setMode('sub'); q.value = PRESETS.sub[0][1]; run(); };
  thr.oninput = function () { $('thr-o').textContent = (+thr.value).toFixed(2); };
  thr.onchange = function () { stopAuto(); run(); };
  $('q-form').onsubmit = function (e) { e.preventDefault(); stopAuto(); run(); };
  q.addEventListener('focus', stopAuto);
  $('auto').onclick = function () { if (state.auto) { stopAuto(); } else { state.auto = true; paintAuto(); tour(); } };
  $('retry').onclick = function () { $('off').hidden = true; run(); };
  function paintAuto() { $('auto').setAttribute('aria-pressed', state.auto); $('auto-t').textContent = state.auto ? 'autoplay' : 'autoplay off'; }
  function stopAuto() { state.auto = false; clearTimeout(state.timer); paintAuto(); }

  /* ---- the field: one square per ~1,000 molecules ---- */
  var field = $('field'), fx = field.getContext('2d'), F = { cells: 0, cols: 0, rows: 0, st: null, t: null, size: 0 };
  var COL = ['#1e3a2a', '#1e3a2a', '#178a4c', '#3dff8e', '#c6ff4d', '#070b08'];
  // st: 0 idle molecule, 1 pruned/rejected, 2 scanned, 3 scanning head, 4 hit
  function layout() {
    var r = field.getBoundingClientRect(), dpr = Math.min(2, devicePixelRatio || 1);
    field.width = Math.round(r.width * dpr); field.height = Math.round(r.height * dpr);
    var cell = r.width < 500 ? 7 : 9, gap = 2;
    F.size = cell; F.gap = gap; F.dpr = dpr;
    F.cols = Math.floor((r.width - 20) / (cell + gap));
    F.rows = Math.floor((r.height - 50) / (cell + gap));
    F.cells = F.cols * F.rows;
    F.ox = (r.width - F.cols * (cell + gap) + gap) / 2; F.oy = 14;
    F.st = new Uint8Array(F.cells); F.glow = new Float32Array(F.cells);
    draw();
  }
  function draw() {
    var s = F.size, g = F.gap, d = F.dpr;
    fx.setTransform(d, 0, 0, d, 0, 0);
    fx.clearRect(0, 0, field.width, field.height);
    for (var k = 0; k < F.cells; k++) {
      var x = F.ox + (k % F.cols) * (s + g), y = F.oy + Math.floor(k / F.cols) * (s + g);
      var st = F.st[k], gl = F.glow[k];
      fx.fillStyle = COL[st === 1 ? 5 : st === 0 ? 0 : st === 2 ? 2 : st === 3 ? 3 : 4];
      fx.fillRect(x, y, s, s);
      if (gl > 0.02) {
        fx.globalAlpha = gl;
        fx.fillStyle = st === 4 ? '#f4ffe9' : '#3dff8e';
        fx.fillRect(x - 1, y - 1, s + 2, s + 2);
        fx.globalAlpha = 1;
      }
    }
    if (F.rings && F.ringP > 0 && F.ringP < 1) {
      var p = F.ringP;
      fx.lineWidth = 1.5;
      F.rings.forEach(function (c, j) {
        var q = Math.max(0, Math.min(1, p * 1.6 - (j % 8) * 0.07));
        if (q <= 0 || q >= 1) { return; }
        var cx = F.ox + (c % F.cols) * (s + g) + s / 2, cy = F.oy + Math.floor(c / F.cols) * (s + g) + s / 2;
        fx.strokeStyle = 'rgba(198,255,77,' + (1 - q) + ')';
        fx.beginPath(); fx.arc(cx, cy, 4 + q * 34, 0, Math.PI * 2); fx.stroke();
      });
    }
  }
  function hash(i) { i = Math.imul(i ^ 0x9e3779b9, 0x85ebca6b); i ^= i >>> 13; i = Math.imul(i, 0xc2b2ae35); return ((i ^ (i >>> 16)) >>> 0) / 4294967296; }
  layout();
  addEventListener('resize', function () { clearTimeout(F.rt); F.rt = setTimeout(layout, 150); });

  function animateField(r) {
    cancelAnimationFrame(F.raf);
    var n = F.cells, total = r.total || TOTAL;
    var mid = r.mode === 'sim' ? r.scanned / total : r.screened / total;
    var hitFrac = r.count / total;
    var hitsN = r.count === 0 ? 0 : Math.max(1, Math.min(n, Math.round(hitFrac * n)));
    var keep = new Uint8Array(n), hit = new Uint8Array(n), order = [];
    var k;
    if (r.mode === 'sim') {
      // Popcount-sorted arena: the band is one contiguous run.
      var w = Math.max(1, Math.round(mid * n)), a = Math.max(0, Math.min(n - w, Math.round(n * 0.47 - w / 2)));
      for (k = a; k < a + w; k++) { keep[k] = 1; order.push(k); }
    } else {
      for (k = 0; k < n; k++) { if (hash(k + r.seed) < mid) { keep[k] = 1; order.push(k); } }
      if (!order.length) { keep[0] = 1; order.push(0); }
    }
    // Hits are a subset of the kept cells.
    var pick = order.slice().sort(function (x, y) { return hash(x * 7 + r.seed) - hash(y * 7 + r.seed); });
    for (k = 0; k < Math.min(hitsN, pick.length); k++) { hit[pick[k]] = 1; }
    for (k = 0; k < n; k++) { F.st[k] = 0; F.glow[k] = 0; }
    F.rings = pick.slice(0, Math.min(hitsN, 40)); F.ringP = 0;

    var T1 = 500, T2 = r.exact ? 0 : 1100, T3 = 700, t0 = performance.now();
    var over = $('field-over');
    over.innerHTML = r.mode === 'sim' ? '<b>BitBound</b>prunes by popcount' : '<b>screen + census</b>drop impossible molecules';
    function frame(t) {
      var e = t - t0;
      for (var j = 0; j < n; j++) {
        if (!keep[j]) {
          // Pruned: fade out in a wave from the edges (similarity) or at random (substructure).
          var at = r.mode === 'sim' ? Math.abs(j / n - 0.47) * T1 : hash(j + 3) * T1;
          F.st[j] = e > T1 - at ? 1 : 0;
        }
      }
      if (e > T1) {
        if (r.mode === 'sub' && T2 && e < T1 + 60) { over.innerHTML = '<b>VF2 matcher</b>proves each survivor'; }
        if (r.mode === 'sub' && !T2 && e < T1 + 60) { over.innerHTML = '<b>census exact</b>no matcher needed'; }
        var p = T2 ? Math.min(1, (e - T1) / T2) : 1, head = Math.floor(p * order.length);
        for (var m = 0; m < order.length; m++) {
          var c = order[m];
          if (m < head) { F.st[c] = 2; F.glow[c] = Math.max(0, F.glow[c] - 0.06); if (m > head - 24) { F.glow[c] = Math.max(F.glow[c], 0.6 * (1 - (head - m) / 24)); } }
          else if (m === head) { F.st[c] = 3; F.glow[c] = 1; }
        }
      }
      if (e > T1 + T2) {
        if (e < T1 + T2 + 60) { over.innerHTML = '<b>' + fmt(r.count) + ' exact hits</b>' + (r.mode === 'sim' ? 'Tanimoto ≥ ' + r.t.toFixed(2) : 'every one proved'); }
        var q2 = Math.min(1, (e - T1 - T2) / T3);
        F.ringP = Math.min(0.999, (e - T1 - T2) / (T3 + 500));
        for (var h = 0; h < n; h++) {
          if (hit[h]) { F.st[h] = 4; F.glow[h] = 1 - q2 * 0.65; }
          else if (keep[h]) { F.glow[h] = Math.max(0, F.glow[h] - 0.08); }
        }
      }
      draw();
      if (e < T1 + T2 + T3 + 500) { F.raf = requestAnimationFrame(frame); } else { F.ringP = 0; draw(); }
    }
    if (reduce) { frame(t0 + T1 + T2 + T3); } else { F.raf = requestAnimationFrame(frame); }
  }
  $('field-legend').innerHTML = '<span style="--c:#1d3a2a">molecules</span><span style="--c:#0a120d;outline:1px solid #1d3a2a">skipped</span><span style="--c:#19c96a">checked</span><span style="--c:#c6ff4d">hits</span>';

  function pending() {
    cancelAnimationFrame(F.raf);
    $('field-over').innerHTML = '<b>searching</b>' + fmt(TOTAL) + ' molecules';
    if (reduce) { return; }
    var t0 = performance.now();
    (function f(t) {
      var e = (t - t0) / 1000;
      for (var k = 0; k < F.cells; k++) {
        F.st[k] = 0;
        F.glow[k] = Math.max(0, Math.sin(e * 6 - (k % F.cols) * 0.18 - Math.floor(k / F.cols) * 0.3)) * 0.35;
      }
      draw();
      if (state.busy) { F.raf = requestAnimationFrame(f); }
    })(t0);
  }

  /* ---- numbers ---- */
  function tween(el, to, fmtf, d) {
    var from = parseFloat(el.dataset.v || 0); el.dataset.v = to;
    if (reduce) { el.textContent = fmtf(to); return; }
    var t0 = performance.now(); d = d || 900;
    (function f(t) { var k = Math.min(1, (t - t0) / d); el.textContent = fmtf(from + (to - from) * ease(k)); if (k < 1) { requestAnimationFrame(f); } })(t0);
  }
  function rateFmt(v) { return v >= 1e9 ? (v / 1e9).toFixed(2) + ' B' : (v / 1e6).toFixed(0) + ' M'; }
  function meters(r) {
    var ms = r.took_us / 1000;
    $('v-time-u').textContent = ms < 1000 ? 'ms' : 's';
    tween($('v-time'), ms < 1000 ? ms : ms / 1000, function (v) { return v.toFixed(ms < 10 ? 2 : 1); });
    var rate = r.total / (r.took_us / 1e6);
    tween($('v-rate'), rate, rateFmt);
    tween($('v-ns'), r.took_us * 1000 / r.total, function (v) { return v.toFixed(v < 1 ? 3 : 2); });
    var mid = r.mode === 'sim' ? r.scanned : r.screened;
    tween($('v-skip'), 100 * (1 - mid / r.total), function (v) { return v.toFixed(1); });
    var L = function (x) { return Math.log10(x + 1) / Math.log10(r.total + 1) * 100; };
    $('f1').style.width = '100%'; $('f2').style.width = L(mid) + '%'; $('f3').style.width = Math.max(r.count ? 1.5 : 0, L(r.count)) + '%';
    tween($('f1n'), r.total, fmt); tween($('f2n'), mid, fmt); tween($('f3n'), r.count, fmt);
  }
  function history(r, label) {
    state.hist.push({ us: r.took_us, label: label });
    if (state.hist.length > 28) { state.hist.shift(); }
    var svg = $('hist'), W = svg.clientWidth || 600, H = 70, n = state.hist.length, bw = W / 28;
    var max = Math.log10(Math.max.apply(null, state.hist.map(function (h) { return h.us; })) + 10);
    var html = '';
    state.hist.forEach(function (h, i) {
      var hh = Math.max(3, (Math.log10(h.us + 10) / max) * (H - 14));
      html += '<rect class="' + (i === n - 1 ? 'last' : '') + '" x="' + (i * bw + 2) + '" y="' + (H - hh) + '" width="' + (bw - 4) + '" height="' + hh + '" rx="2"><title>' + esc(h.label) + ': ' + (h.us / 1000).toFixed(2) + ' ms</title></rect>';
    });
    var last = state.hist[n - 1];
    html += '<text x="' + Math.min(W - 60, (n - 1) * bw) + '" y="9">' + (last.us / 1000).toFixed(1) + ' ms</text>';
    svg.setAttribute('viewBox', '0 0 ' + W + ' ' + H);
    svg.innerHTML = html;
    var avg = state.hist.reduce(function (a, h) { return a + h.us; }, 0) / n / 1000;
    $('hist-avg').textContent = n + ' queries · mean ' + avg.toFixed(1) + ' ms';
  }

  /* ---- hits ---- */
  var drawer = null, sdLoad = null;
  function loadSD() {
    if (sdLoad) { return sdLoad; }
    sdLoad = new Promise(function (res) {
      var s = doc.createElement('script'); s.src = 'vendor/smiles-drawer.min.js'; s.async = true;
      s.onload = function () {
        var SD = window.SmilesDrawer;
        if (SD && SD.SvgDrawer) {
          drawer = new SD.SvgDrawer({ width: 200, height: 120, bondThickness: 1.2, bondLength: 20, shortBondLength: 0.82, fontSizeLarge: 7, fontSizeSmall: 4.5, padding: 8, compactDrawing: true,
            themes: { moss: { C: '#cfe7d6', O: '#ff9d7a', N: '#7fc8ff', F: '#a6f08a', CL: '#7fe89a', BR: '#e0a96b', I: '#c5a0f0', P: '#e0a96b', S: '#ffd36b', B: '#e0a96b', SI: '#e0a96b', H: '#97a593', BACKGROUND: '#06100a' } } });
        }
        res();
      };
      s.onerror = function () { res(); };
      doc.body.appendChild(s);
    });
    return sdLoad;
  }
  function hits(r) {
    var box = $('hits');
    box.innerHTML = (r.hits || []).slice(0, 12).map(function (h, i) {
      var sc = h.score != null ? '<b>' + h.score.toFixed(3) + '</b>' : '';
      return '<a class="hit-card" style="--i:' + i + '" href="https://www.ebi.ac.uk/chembl/compound_report_card/' + encodeURIComponent(h.id) + '/" target="_blank" rel="noopener">' +
        (h.score != null ? '<div class="score" style="--s:' + (h.score * 100).toFixed(1) + '%"></div>' : '') +
        '<svg data-smi="' + esc(h.smiles) + '"></svg><div class="meta"><span>' + esc(h.id) + '</span>' + sc + '</div></a>';
    }).join('');
    loadSD().then(function () {
      var svgs = box.querySelectorAll('svg[data-smi]'), i = 0;
      (function batch() {
        var t0 = performance.now();
        while (i < svgs.length && performance.now() - t0 < 10) {
          var svg = svgs[i++], smi = svg.dataset.smi;
          try {
            if (!drawer) { throw 0; }
            window.SmilesDrawer.parse(smi, function (tree) { drawer.draw(tree, svg, 'moss', null, false); }, function () { svg.outerHTML = '<code class="hit-smi">' + esc(smi) + '</code>'; });
          } catch (e) { svg.outerHTML = '<code class="hit-smi">' + esc(smi) + '</code>'; }
        }
        if (i < svgs.length) { requestAnimationFrame(batch); }
      })();
    });
  }

  /* ---- run ---- */
  var seed = 1;
  function run() {
    var text = q.value.trim();
    if (!text || state.busy) { return Promise.resolve(); }
    state.busy = true; $('go').classList.add('busy'); $('err').hidden = true;
    pending();
    var mode = state.mode, t = +thr.value;
    var p = mode === 'sim' ? api('/v1/similarity', { smiles: text, threshold: t, limit: 12 })
      : api('/v1/substructure', { smarts: text, limit: 48, timeout_ms: 8000 });
    return p.then(function (r) {
      $('off').hidden = true;
      r.mode = mode; r.t = t; r.seed = seed++;
      state.busy = false;
      if (mode === 'sub') { r.hits = r.hits.slice().sort(function (a, b) { return a.smiles.length - b.smiles.length; }).slice(0, 12); }
      r.exact = mode === 'sub' && r.screened === r.count;
      animateField(r); meters(r); hits(r);
      var name = (PRESETS[mode].filter(function (x) { return x[1] === text; })[0] || [text])[0];
      history(r, name);
      doc.querySelectorAll('#chips button').forEach(function (b) { b.classList.toggle('on', b.title === text); });
    }).catch(function (e) {
      if (e.offline) { $('off').hidden = false; stopAuto(); return; }
      var msg = e.status === 408 ? 'That query passed its deadline. Try a more specific pattern.' : e.message;
      var m = /at byte (\d+)/.exec(msg);
      $('err').textContent = m ? text + '\n' + ' '.repeat(+m[1]) + '^ ' + msg : msg;
      $('err').hidden = false;
    }).then(function () { state.busy = false; $('go').classList.remove('busy'); });
  }

  function typeInto(s, done) {
    if (reduce) { q.value = s; done(); return; }
    q.value = ''; var i = 0;
    (function f() {
      if (!state.auto) { return; }
      q.value = s.slice(0, ++i);
      if (i < s.length) { state.timer = setTimeout(f, Math.max(12, 600 / s.length)); } else { done(); }
    })();
  }
  function tour() {
    if (!state.auto || doc.hidden) { return; }
    var s = TOUR[state.step++ % TOUR.length];
    setMode(s[0]);
    if (s[2]) { thr.value = s[2]; thr.oninput(); }
    typeInto(s[1], function () {
      run().then(function () { if (state.auto) { state.timer = setTimeout(tour, 5200); } });
    });
  }
  setMode('sim');
  var seen = false;
  new IntersectionObserver(function (es) {
    if (es[0].isIntersecting) { loadSD(); }
  }, { rootMargin: '400px' }).observe(demo);
  new IntersectionObserver(function (es) {
    if (es[0].isIntersecting && !seen) { seen = true; layout(); tour(); }
  }, { threshold: 0.35 }).observe(doc.querySelector('.console'));
  doc.addEventListener('visibilitychange', function () { if (!doc.hidden && state.auto && seen && !state.busy) { clearTimeout(state.timer); state.timer = setTimeout(tour, 1500); } });

  /* ------------------------------------------------------------ race */
  var LANES = [
    ['mossflux', 'BitBound, 1 thread', 10.6, true],
    ['FPSim2', '1 worker', 36.1],
    ['RDKit', 'BulkTanimotoSimilarity', 530],
    ['RDKit', 'Python loop', 4700]
  ];
  var race = $('race');
  race.insertAdjacentHTML('afterbegin', LANES.map(function (l) {
    return '<div class="lane' + (l[3] ? ' mf' : '') + '"><span class="lane-name">' + l[0] + '<small>' + l[1] + '</small></span>' +
      '<span class="lane-track"><span class="lane-fill"></span></span><span class="lane-t">0 ms</span></div>';
  }).join(''));
  var lanes = race.querySelectorAll('.lane');
  var MAXT = LANES[LANES.length - 1][2];
  function runRace() {
    cancelAnimationFrame(race.raf);
    lanes.forEach(function (l, i) {
      l.classList.remove('done');
      l.querySelector('.lane-fill').style.transform = 'scaleX(0)';
      l.querySelector('.lane-name small').textContent = LANES[i][1];
    });
    var t0 = performance.now();
    (function f(t) {
      var e = t - t0, all = true;
      lanes.forEach(function (l, i) {
        var d = LANES[i][2], k = Math.min(e, d) / MAXT;
        l.querySelector('.lane-fill').style.transform = 'scaleX(' + Math.max(k, 0.004) + ')';
        if (e < d) { all = false; l.querySelector('.lane-t').textContent = fmt(e) + ' ms'; }
        else if (!l.classList.contains('done')) {
          l.classList.add('done');
          l.querySelector('.lane-t').textContent = (d < 100 ? d.toFixed(1) : fmt(d)) + ' ms';
          var x = d / LANES[0][2];
          l.querySelector('.lane-name small').textContent = i ? LANES[i][1] + ' · ' + x.toFixed(x < 10 ? 1 : 0) + '× slower' : LANES[i][1] + ' · done';
        }
      });
      if (!all) { race.raf = requestAnimationFrame(f); }
    })(t0);
  }
  race.dataset.on = 'mfRace'; window.mfRace = runRace;
  $('race-go').onclick = runRace;

  /* ------------------------------------------------------------ bars */
  var SUB = [['sulfonamide', 160], ['fluorine', 80.1], ['any ring bond', 70.5], ['nitrile', 54.5], ['ester', 40.6], ['carbonyl', 38.6],
    ['amide', 36.5], ['chlorine', 35.0], ['aromatic N', 33.9], ['carboxylic acid', 30.1], ['branched C', 25.7], ['quaternary C', 22.4],
    ['any halogen', 21.5], ['phenol', 13.3], ['naphthalene', 9.8], ['benzene', 6.4], ['pyridine', 5.2]];
  var bars = $('bars');
  bars.innerHTML = SUB.map(function (b, i) {
    var ref = b[0] === 'benzene' ? '<span class="ref" style="--x:' + (4.0 / 160 * 100) + '%" data-l="Arthor benzene ≈ 4.0, 1 thread, i7 (2017)"></span>' : '';
    return '<div class="bar"><span class="n">' + b[0] + '</span><span class="tr"><i style="--w:' + (b[1] / 160 * 100) + '%;--i:' + i + '"></i>' + ref + '</span><span class="v">' + b[1] + '</span></div>';
  }).join('');
  doc.querySelector('.sub-chart').dataset.on = 'mfBars';
  window.mfBars = function () { bars.classList.add('on'); };

  /* ------------------------------------------------------------ code */
  var tabs = doc.querySelectorAll('.code-tabs [data-c]');
  tabs.forEach(function (b) {
    b.onclick = function () {
      tabs.forEach(function (x) { x.setAttribute('aria-selected', x === b); $('code-' + x.dataset.c).hidden = x !== b; });
    };
  });
  $('copy').onclick = function () {
    var pre = doc.querySelector('.code pre:not([hidden])');
    navigator.clipboard.writeText(pre.textContent).then(function () { $('copy').textContent = 'Copied'; setTimeout(function () { $('copy').textContent = 'Copy'; }, 1400); });
  };
})();
