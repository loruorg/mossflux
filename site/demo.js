/* ---------------------------------------------------------------------------
   demo.js - the live demo: similarity and substructure search against the
   mossflux REST API.

   API base: API below, or ?api=<http(s) URL> on the page URL for local
   testing (for example ?api=http://localhost:8080).

   Loaded with the page (small). The 2D depiction library (vendor/,
   smiles-drawer, MIT) and the 3D sketch (mol3d.js) load only when the demo
   comes within one viewport.
   ------------------------------------------------------------------------- */

(function (global) {
  'use strict';

  var API = 'https://api.mossflux.com';

  var doc = global.document;
  var root = doc.getElementById('demo');
  if (!root) { return; }

  (function override() {
    try {
      var o = new global.URLSearchParams(global.location.search).get('api');
      if (o && /^https?:\/\/[^\s]+$/i.test(o)) { API = o.replace(/\/+$/, ''); }
    } catch (e) { /* keep the default */ }
  })();

  function $(id) { return doc.getElementById(id); }
  function fmt(n) { return Math.round(n).toLocaleString('en-US'); }
  function esc(s) { return String(s).replace(/[&<>"']/g, function (c) { return '&#' + c.charCodeAt(0) + ';'; }); }

  var PRESETS = {
    sim: [
      ['aspirin', 'CC(=O)Oc1ccccc1C(=O)O'],
      ['caffeine', 'CN1C=NC2=C1C(=O)N(C(=O)N2C)C'],
      ['ibuprofen', 'CC(C)Cc1ccc(cc1)C(C)C(=O)O'],
      ['indole', 'c1ccc2[nH]ccc2c1']
    ],
    sub: [
      ['benzene ring', 'c1ccccc1'],
      ['carboxylic acid', 'C(=O)[OH]'],
      ['sulfonamide', 'S(=O)(=O)N'],
      ['indole', 'c1ccc2[nH]ccc2c1']
    ]
  };

  var el = {
    tabs: [$('tab-sim'), $('tab-sub')],
    form: $('demo-form'), label: $('demo-label'), q: $('demo-q'), go: $('demo-go'),
    chips: $('demo-chips'), simctl: $('demo-simctl'), subctl: $('demo-subctl'),
    t: $('demo-t'), tOut: $('demo-t-out'), modeT: $('demo-mode-t'), modeK: $('demo-mode-k'),
    err: $('demo-err'), db: $('demo-db'), status: $('demo-status'),
    total: $('p-total'), time: $('p-time'), rate: $('p-rate'), prune: $('p-prune'), pruneK: $('p-prune-k'),
    count: $('demo-count'), hits: $('demo-hits'), off: $('demo-off'), offUrl: $('demo-off-url'), retry: $('demo-retry'),
    mol: $('demo-mol'), molNote: $('demo-mol-note'), perf: $('demo-perf')
  };

  var state = { mode: 'sim', db: 'chembl', online: null, busy: null, last: { sim: PRESETS.sim[0][1], sub: PRESETS.sub[0][1] } };
  var mol3d = null, drawer = null, libs = null;

  /* ------------------------------------------------------------ loading */

  function script(src) {
    return new Promise(function (res, rej) {
      var s = doc.createElement('script');
      s.src = src; s.async = true;
      s.onload = function () { res(); };
      s.onerror = function () { rej(new Error('load ' + src)); };
      doc.body.appendChild(s);
    });
  }

  function loadLibs() {
    if (libs) { return libs; }
    libs = Promise.all([
      script('vendor/smiles-drawer.min.js').then(function () {
        var SD = global.SmilesDrawer;
        if (!SD || !SD.SvgDrawer) { return; }
        drawer = new SD.SvgDrawer({
          width: 220, height: 160, bondThickness: 1.1, bondLength: 22, shortBondLength: 0.82,
          fontSizeLarge: 7, fontSizeSmall: 4.5, padding: 8, compactDrawing: true, explicitHydrogens: true,
          themes: {
            moss: {
              C: '#C9D1C3', O: '#E0907A', N: '#8FB8D8', F: '#9CCB8A', CL: '#8FC79A', BR: '#D2A06B',
              I: '#B89AD0', P: '#D2A06B', S: '#D2BC93', B: '#D2A06B', SI: '#D2A06B', H: '#97A593',
              BACKGROUND: '#141a16'
            }
          }
        });
      }).catch(function () { drawer = null; }),
      script('mol3d.js').then(function () {
        if (global.MFMOL && el.mol) { mol3d = global.MFMOL.mount(el.mol); sketch(); }
      }).catch(function () { /* no 3D sketch */ })
    ]);
    return libs;
  }

  /* ------------------------------------------------------------- API */

  function call(path, body, ms) {
    var ctl = global.AbortController ? new global.AbortController() : null;
    var timer = ctl ? global.setTimeout(function () { ctl.abort(); }, ms || 15000) : 0;
    var init = { method: body ? 'POST' : 'GET', headers: { 'Accept': 'application/json' }, signal: ctl ? ctl.signal : undefined };
    if (body) { init.headers['Content-Type'] = 'application/json'; init.body = JSON.stringify(body); }
    var t0 = global.performance ? global.performance.now() : Date.now();
    var p = global.fetch(API + path, init).then(function (r) {
      return r.text().then(function (txt) {
        var j = null;
        try { j = txt ? JSON.parse(txt) : null; } catch (e) { j = null; }
        var rtt = (global.performance ? global.performance.now() : Date.now()) - t0;
        if (!r.ok) {
          var err = new Error((j && j.error) || ('HTTP ' + r.status));
          err.status = r.status; err.position = j && typeof j.position === 'number' ? j.position : null;
          var at = err.position === null && /\bbyte (\d+)/.exec(err.message);
          if (at) { err.position = +at[1]; }
          throw err;
        }
        if (!j) { var e2 = new Error('The API sent a reply that is not JSON.'); e2.status = r.status; throw e2; }
        j._rtt = rtt;
        return j;
      });
    }, function (e) {
      var err = new Error(e && e.name === 'AbortError' ? 'timeout' : 'network');
      err.network = true; err.aborted = e && e.name === 'AbortError';
      throw err;
    });
    p.abort = function () { if (ctl) { ctl.abort(); } };
    p.finally(function () { global.clearTimeout(timer); }).catch(function () {});
    return p;
  }

  function connect() {
    setStatus('Connecting to ' + API.replace(/^https?:\/\//, '') + ' …');
    return call('/v1/databases', null, 8000).then(function (j) {
      var list = (j && j.databases) || [];
      var d = list.filter(function (x) { return x.name === 'chembl'; })[0] || list[0];
      if (!d) { throw Object.assign(new Error('no database'), { network: true }); }
      state.db = d.name;
      state.online = true;
      el.off.hidden = true;
      var fp = d.fingerprint ? (String(d.fingerprint.kind || '').toUpperCase() + ', ' + fmt(d.fingerprint.bits || 0) + ' bits') : '';
      el.db.textContent = d.name + ' · ' + fmt(d.molecules || 0) + ' molecules' + (fp ? ' · ' + fp : '') + (d.ready === false ? ' · loading' : '');
      el.db.setAttribute('data-state', d.ready === false ? 'wait' : 'ok');
      setStatus('');
      return d;
    }, function (e) {
      offline(e);
      throw e;
    });
  }

  function offline(e) {
    state.online = false;
    el.db.textContent = 'API not reachable';
    el.db.setAttribute('data-state', 'off');
    el.offUrl.textContent = API;
    el.off.hidden = false;
    el.count.textContent = '';
    setStatus('The demo API did not answer.');
    void e;
  }

  /* ---------------------------------------------------------- search */

  function setStatus(s) { if (el.status) { el.status.textContent = s; } }

  function showError(msg, pos, query) {
    if (!msg) { el.err.hidden = true; el.err.textContent = ''; el.q.removeAttribute('aria-invalid'); return; }
    var html = '<span class="err-msg">' + esc(msg) + '</span>';
    if (typeof pos === 'number' && query) {
      var p = Math.max(0, Math.min(query.length, pos));
      html += '<code class="err-at" aria-hidden="true">' + esc(query) + '\n' + ' '.repeat(p) + '^</code>' +
        '<span class="vh">at character ' + (p + 1) + '</span>';
    }
    el.err.innerHTML = html;
    el.err.hidden = false;
    el.q.setAttribute('aria-invalid', 'true');
  }

  function search() {
    var query = el.q.value.trim();
    if (!query) { showError('Type a ' + (state.mode === 'sim' ? 'SMILES string' : 'SMARTS pattern') + ', or pick an example.'); return; }
    showError(null);
    state.last[state.mode] = query;
    if (state.busy) { state.busy.abort(); }
    var mode = state.mode, body, path;
    if (mode === 'sim') {
      path = '/v1/similarity';
      body = { db: state.db, smiles: query, limit: 50 };
      if (el.modeK.checked) { body.k = 20; } else { body.threshold = +(+el.t.value).toFixed(2); }
    } else {
      path = '/v1/substructure';
      body = { db: state.db, smarts: query, limit: 50, timeout_ms: 5000 };
    }
    root.setAttribute('data-busy', '');
    el.go.disabled = true;
    setStatus('Searching …');
    var req = call(path, body, mode === 'sub' ? 12000 : 10000);
    state.busy = req;
    req.then(function (j) {
      if (state.busy !== req) { return; }
      if (state.online !== true) { state.online = true; el.off.hidden = true; connect().catch(function () {}); }
      render(mode, j, query);
    }, function (e) {
      if (state.busy !== req) { return; }
      if (e.network && !e.aborted) { offline(e); return; }
      if (e.status === 408 || e.message === 'timeout') {
        showError('Timed out. A short pattern can take exponential time to match; try a more specific one.');
      } else if (e.status === 503 || e.status === 429) {
        showError('The server is busy or still loading. Try again in a moment.');
      } else {
        showError(e.message, e.position, query);
      }
      setStatus('');
    }).then(function () {
      if (state.busy === req) { state.busy = null; root.removeAttribute('data-busy'); el.go.disabled = false; }
    });
  }

  function render(mode, j, query) {
    var total = +j.total || 0, us = +j.took_us || 0;
    var hits = j.hits || [], count = typeof j.count === 'number' ? j.count : hits.length;
    var touched = mode === 'sim' ? (typeof j.scanned === 'number' ? j.scanned : total) : (typeof j.screened === 'number' ? j.screened : total);
    var pruned = total ? 1 - touched / total : 0;
    el.total.textContent = fmt(total);
    el.time.textContent = us >= 100000 ? (us / 1e6).toFixed(2) + ' s' : (us / 1000).toFixed(us < 10000 ? 2 : 1) + ' ms';
    var rate = us ? total / (us / 1e6) : 0;
    el.rate.textContent = rate >= 1e9 ? (rate / 1e9).toFixed(2) + ' B' : rate >= 1e6 ? (rate / 1e6).toFixed(0) + ' M' : fmt(rate);
    el.prune.textContent = (pruned * 100).toFixed(1) + ' %';
    el.pruneK.textContent = mode === 'sim' ? 'skipped by the bound' : 'never reached the matcher';
    el.perf.setAttribute('data-live', '');
    el.count.textContent = fmt(count) + (count === 1 ? ' hit' : ' hits') + (hits.length < count ? ', first ' + hits.length + ' shown' : '') +
      ' · server time ' + el.time.textContent + ', round trip ' + Math.round(j._rtt || 0) + ' ms';
    setStatus(fmt(count) + ' hits in ' + el.time.textContent + '.');
    if (mode === 'sim') {
      hits = hits.slice().sort(function (a, b) { return (b.score || 0) - (a.score || 0); });
    }
    list(hits, mode);
    if (mode === 'sub') {
      var detail = { total: total, screened: touched, count: count, took_us: us, smarts: query };
      var ev;
      try { ev = new global.CustomEvent('mossflux:substructure', { detail: detail }); }
      catch (e) { ev = doc.createEvent('CustomEvent'); ev.initCustomEvent('mossflux:substructure', false, false, detail); }
      doc.dispatchEvent(ev);
    }
  }

  function list(hits, mode) {
    el.hits.textContent = '';
    if (!hits.length) {
      el.hits.innerHTML = '<li class="hit-none">No molecule in the database passes this query.</li>';
      return;
    }
    append(hits, mode);
  }

  function append(hits, mode) {
    var FIRST = 12, rest = hits.length > FIRST + 2 ? hits.slice(FIRST) : [];
    if (rest.length) { hits = hits.slice(0, FIRST); }
    var frag = doc.createDocumentFragment(), todo = [];
    hits.forEach(function (h) {
      var li = doc.createElement('li');
      li.className = 'hit';
      var id = String(h.id || '');
      var url = /^CHEMBL\d+$/.test(id) ? 'https://www.ebi.ac.uk/chembl/compound_report_card/' + id + '/' : null;
      li.innerHTML =
        '<div class="hit-pic"><svg xmlns="http://www.w3.org/2000/svg" role="img" aria-label="' + esc(id) + ' structure"></svg></div>' +
        '<div class="hit-meta">' +
        (url ? '<a class="hit-id" href="' + url + '" target="_blank" rel="noopener">' + esc(id) + '</a>' : '<span class="hit-id">' + esc(id) + '</span>') +
        (mode === 'sim' && typeof h.score === 'number' ? '<span class="hit-score">' + h.score.toFixed(3) + '</span>' : '') +
        '</div>';
      li.title = h.smiles || '';
      frag.appendChild(li);
      todo.push([li.querySelector('svg'), h.smiles || '']);
    });
    el.hits.appendChild(frag);
    if (rest.length) {
      var more = doc.createElement('li');
      more.className = 'hit-more';
      more.innerHTML = '<button class="qgo qgo-ghost" type="button">Show ' + rest.length + ' more</button>';
      more.firstChild.addEventListener('click', function () {
        more.parentNode.removeChild(more);
        append(rest, mode);
      });
      el.hits.appendChild(more);
    }
    loadLibs().then(function () { depict(todo); });
  }

  /* Draw a few per frame, so 50 depictions never block input. */
  function depict(todo) {
    var SD = global.SmilesDrawer;
    if (!drawer || !SD) {
      todo.forEach(function (t) { t[0].parentNode.innerHTML = '<code class="hit-smi">' + esc(t[1]) + '</code>'; });
      return;
    }
    var i = 0;
    function batch() {
      var t0 = Date.now();
      while (i < todo.length && Date.now() - t0 < 12) {
        var t = todo[i++], svg = t[0], smi = t[1];
        if (!svg.isConnected) { continue; }
        try {
          SD.parse(smi, function (tree) { drawer.draw(tree, svg, 'moss', null, false); },
            function () { svg.parentNode.innerHTML = '<code class="hit-smi">' + esc(smi) + '</code>'; });
        } catch (e) { svg.parentNode.innerHTML = '<code class="hit-smi">' + esc(smi) + '</code>'; }
        svg.removeAttribute('style');
      }
      if (i < todo.length) { global.requestAnimationFrame(batch); }
    }
    global.requestAnimationFrame(batch);
  }

  /* ------------------------------------------------------------ UI */

  function sketch() {
    if (!mol3d) { return; }
    var ok = mol3d.set(el.q.value);
    el.molNote.textContent = ok
      ? 'The query, as a 3D sketch from its graph. Hydrogens left out. Drag to turn.'
      : 'No 3D sketch for this input. The search still runs.';
    el.mol.setAttribute('data-empty', ok ? 'false' : 'true');
  }

  function chips() {
    el.chips.textContent = '';
    PRESETS[state.mode].forEach(function (p) {
      var b = doc.createElement('button');
      b.type = 'button'; b.className = 'chip';
      b.innerHTML = '<span class="chip-name">' + esc(p[0]) + '</span><span class="chip-q">' + esc(p[1]) + '</span>';
      b.addEventListener('click', function () {
        el.q.value = p[1]; sketch(); mark();
        if (state.online !== false) { search(); }
      });
      el.chips.appendChild(b);
    });
    mark();
  }
  function mark() {
    var bs = el.chips.children;
    for (var i = 0; i < bs.length; i++) {
      var on = PRESETS[state.mode][i][1] === el.q.value.trim();
      bs[i].setAttribute('aria-pressed', on ? 'true' : 'false');
    }
  }

  function setMode(m, focus) {
    if (m === state.mode && el.q.value) { return; }
    state.last[state.mode] = el.q.value.trim() || state.last[state.mode];
    state.mode = m;
    el.tabs.forEach(function (t) {
      var on = t.getAttribute('data-mode') === m;
      t.setAttribute('aria-selected', on ? 'true' : 'false');
      t.tabIndex = on ? 0 : -1;
      if (on && focus) { t.focus(); }
    });
    root.setAttribute('data-mode', m);
    el.label.textContent = m === 'sim' ? 'SMILES' : 'SMARTS';
    el.q.placeholder = m === 'sim' ? 'a SMILES string, e.g. CC(=O)Oc1ccccc1C(=O)O' : 'a SMARTS pattern, e.g. C(=O)[OH]';
    el.simctl.hidden = m !== 'sim';
    el.subctl.hidden = m !== 'sub';
    el.q.value = state.last[m];
    showError(null);
    chips(); sketch();
  }

  el.tabs.forEach(function (t, i) {
    t.addEventListener('click', function () { setMode(t.getAttribute('data-mode')); });
    t.addEventListener('keydown', function (e) {
      if (e.key === 'ArrowRight' || e.key === 'ArrowLeft') {
        e.preventDefault();
        setMode(el.tabs[(i + 1) % 2].getAttribute('data-mode'), true);
      }
    });
  });
  el.form.addEventListener('submit', function (e) { e.preventDefault(); search(); });
  el.q.addEventListener('input', function () { mark(); showError(null); });
  el.q.addEventListener('change', sketch);
  function tShow() { el.tOut.textContent = (+el.t.value).toFixed(2); }
  el.t.addEventListener('input', tShow);
  el.t.addEventListener('change', function () { if (state.online) { search(); } });
  [el.modeT, el.modeK].forEach(function (r) {
    r.addEventListener('change', function () {
      el.t.disabled = el.modeK.checked;
      if (state.online) { search(); }
    });
  });
  el.retry.addEventListener('click', function () {
    connect().then(function () { search(); }, function () {});
  });

  /* Copy buttons outside the build block (that one is band.js's). */
  [].forEach.call(doc.querySelectorAll('[data-copy]'), function (b) {
    b.addEventListener('click', function () {
      var src = $(b.getAttribute('data-copy')), lab = b.querySelector('.copy-label');
      if (!src) { return; }
      var text = src.textContent;
      function done(ok) {
        if (!lab) { return; }
        lab.textContent = ok ? 'Copied' : 'Select it';
        global.setTimeout(function () { lab.textContent = 'Copy'; }, 1600);
      }
      if (global.navigator.clipboard && global.navigator.clipboard.writeText) {
        global.navigator.clipboard.writeText(text).then(function () { done(true); }, function () { done(false); });
      } else { done(false); }
    });
  });

  setMode('sim');
  el.q.value = state.last.sim;
  tShow();
  chips();

  var started = false;
  function start() {
    if (started) { return; }
    started = true;
    loadLibs();
    connect().then(function (d) { if (d.ready !== false) { search(); } }, function () {});
  }
  if (global.IntersectionObserver) {
    var io = new global.IntersectionObserver(function (es) {
      if (es[es.length - 1].isIntersecting) { io.disconnect(); start(); }
    }, { rootMargin: '100% 0px' });
    io.observe(root);
  } else { start(); }

  global.MOSSFLUX_DEMO = { api: function () { return API; } };
})(window);
