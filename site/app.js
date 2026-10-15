/* mossflux.com. A background loop (chosen in index.html) behind four sections;
   scroll zooms it and dims it behind the content.
   API base: API below, or ?api=<url> for local testing. */
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

  /* ------------------------------------------------------------ scroll */
  var centers = [], k = 0, last = performance.now(), shown = -1, bgv = $('bgv'), bgd = $('bgd');
  function measure() {
    var max = doc.documentElement.scrollHeight - innerHeight;
    centers = secs.map(function (s) { return clamp(s.offsetTop + s.offsetHeight / 2 - innerHeight / 2, 0, max); });
    centers[0] = 0; centers[centers.length - 1] = max;
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
    if (k !== shown) {
      shown = k;
      if (!reduce) { bgv.style.transform = 'scale(' + (1.02 + k * 0.025).toFixed(4) + ')'; }
      bgd.style.opacity = (Math.min(k, 1) * 0.45).toFixed(3);
    }
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
