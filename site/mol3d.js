/* ---------------------------------------------------------------------------
   mol3d.js - the query, turning in 3D.

   A small SMILES (and simple SMARTS) graph reader, a force relaxation that
   gives the graph a plausible 3D shape (bond lengths, bond angles from
   hybridisation, flat aromatic rings, no clashes), and a canvas renderer
   with one orthographic camera. It is a sketch from the graph, not a
   computed conformer: hydrogens are left out and stereo is ignored.

   window.MFMOL.mount(canvas) -> { set(smiles) -> boolean }
   ------------------------------------------------------------------------- */

(function (global) {
  'use strict';

  var ORGANIC = { B: 1, C: 1, N: 1, O: 1, P: 1, S: 1, F: 1, Cl: 1, Br: 1, I: 1 };
  var AROM = { b: 'B', c: 'C', n: 'N', o: 'O', p: 'P', s: 'S' };

  /* Graph reader: atoms {el, ar}, bonds [a, b, order]. Throws on what it
     does not understand; the caller then shows no sketch. */
  function parse(str) {
    var atoms = [], bonds = [], stack = [], rings = {};
    var prev = -1, order = 0, i = 0, n = str.length;
    function addAtom(el, ar) {
      atoms.push({ el: el, ar: ar });
      var k = atoms.length - 1;
      if (prev >= 0) { bonds.push([prev, k, order || (ar && atoms[prev].ar ? 1.5 : 1)]); }
      prev = k; order = 0;
      if (atoms.length > 120) { throw new Error('too large'); }
    }
    while (i < n) {
      var ch = str[i];
      if (ch === '(') { stack.push(prev); i++; continue; }
      if (ch === ')') { if (!stack.length) { throw new Error('paren'); } prev = stack.pop(); i++; continue; }
      if (ch === '-' ) { order = 1; i++; continue; }
      if (ch === '=') { order = 2; i++; continue; }
      if (ch === '#') { order = 3; i++; continue; }
      if (ch === ':') { order = 1.5; i++; continue; }
      if (ch === '~' || ch === '/' || ch === '\\' || ch === '@') { i++; continue; }
      if (ch === '.') { prev = -1; i++; continue; }
      if (ch === '[') {
        var j = str.indexOf(']', i);
        if (j < 0) { throw new Error('bracket'); }
        var body = str.slice(i + 1, j);
        var m = /^\d*(#\d+|[A-Z][a-z]?|[a-z][a-z]?|\*)/.exec(body);
        if (!m) { throw new Error('atom'); }
        var sym = m[1], el, ar = false;
        if (sym[0] === '#') { el = ({ 6: 'C', 7: 'N', 8: 'O', 9: 'F', 15: 'P', 16: 'S', 17: 'Cl', 35: 'Br', 53: 'I' })[sym.slice(1)] || 'C'; }
        else if (sym === '*') { el = 'C'; }
        else if (/^[a-z]/.test(sym)) { el = AROM[sym] || (sym === 'se' ? 'Se' : AROM[sym[0]]); ar = true; if (!el) { throw new Error('atom'); } }
        else { el = sym; }
        addAtom(el, ar);
        i = j + 1; continue;
      }
      if (/[0-9%]/.test(ch)) {
        var num;
        if (ch === '%') { num = str.slice(i + 1, i + 3); i += 3; } else { num = ch; i++; }
        if (prev < 0) { throw new Error('ring'); }
        if (rings[num]) {
          var r = rings[num];
          bonds.push([r.a, prev, order || r.order || (atoms[r.a].ar && atoms[prev].ar ? 1.5 : 1)]);
          delete rings[num];
        } else { rings[num] = { a: prev, order: order }; }
        order = 0; continue;
      }
      var two = str.slice(i, i + 2);
      if (two === 'Cl' || two === 'Br') { addAtom(two, false); i += 2; continue; }
      if (ORGANIC[ch]) { addAtom(ch, false); i++; continue; }
      if (AROM[ch]) { addAtom(AROM[ch], true); i++; continue; }
      throw new Error('char ' + ch);
    }
    if (!atoms.length || stack.length || Object.keys(rings).length) { throw new Error('incomplete'); }
    return { atoms: atoms, bonds: bonds };
  }

  /* Seeded random, so one SMILES always gets the same shape. */
  function rng(seed) {
    var x = seed >>> 0 || 1;
    return function () { x ^= x << 13; x ^= x >>> 17; x ^= x << 5; return ((x >>> 0) % 100000) / 100000; };
  }
  function hash(s) { var h = 2166136261; for (var i = 0; i < s.length; i++) { h = Math.imul(h ^ s.charCodeAt(i), 16777619); } return h; }

  /* Distance constraints: bonds, 1-3 pairs by hybridisation, the para pair
     of a six-ring, then a soft clash term for all other pairs. */
  function embed(g, seed) {
    var N = g.atoms.length, adj = [], i, j, k;
    for (i = 0; i < N; i++) { adj.push([]); }
    g.bonds.forEach(function (b) { adj[b[0]].push(b[1]); adj[b[1]].push(b[0]); });
    var sp = [];
    for (i = 0; i < N; i++) {
      var tri = false, lin = false;
      g.bonds.forEach(function (b) { if (b[0] === i || b[1] === i) { if (b[2] === 3) { lin = true; } if (b[2] === 2 || b[2] === 1.5) { tri = true; } } });
      sp.push(lin ? 1 : tri || g.atoms[i].ar ? 2 : 3);
    }
    var C = {}, list = [];
    function con(a, b, d, w) {
      var key = a < b ? a + ',' + b : b + ',' + a;
      if (C[key]) { return; }
      C[key] = 1; list.push([a, b, d, w]);
    }
    g.bonds.forEach(function (b) { con(b[0], b[1], b[2] === 3 ? 1.2 : b[2] === 2 ? 1.33 : b[2] === 1.5 ? 1.4 : 1.52, 1); });
    for (i = 0; i < N; i++) {
      var ang = sp[i] === 1 ? 180 : sp[i] === 2 ? 120 : 109.5;
      for (j = 0; j < adj[i].length; j++) {
        for (k = j + 1; k < adj[i].length; k++) {
          var a = adj[i][j], b = adj[i][k];
          con(a, b, 1.46 * Math.sqrt(2 - 2 * Math.cos(ang * Math.PI / 180)), 0.6);
        }
      }
    }
    // six-membered aromatic rings: the para pair keeps the hexagon flat
    for (i = 0; i < N; i++) {
      if (!g.atoms[i].ar) { continue; }
      // walk paths of length 3 inside aromatic atoms; if two such paths meet, the ends are para
      var seen = {};
      adj[i].forEach(function (a) {
        if (!g.atoms[a].ar) { return; }
        adj[a].forEach(function (b) {
          if (b === i || !g.atoms[b].ar) { return; }
          adj[b].forEach(function (c) {
            if (c === a || c === i || !g.atoms[c].ar) { return; }
            seen[c] = (seen[c] || 0) + 1;
          });
        });
      });
      for (var key in seen) { if (seen[key] >= 2) { con(i, +key, 2.8, 0.5); } }
    }
    var R = rng(hash(seed)), X = new Float64Array(N * 3);
    for (i = 0; i < N * 3; i++) { X[i] = (R() - 0.5) * Math.cbrt(N) * 2.4; }
    var it, step;
    for (it = 0; it < 900; it++) {
      step = it < 600 ? 0.3 : 0.12;
      for (var q = 0; q < list.length; q++) {
        var L = list[q], a3 = L[0] * 3, b3 = L[1] * 3;
        var dx = X[b3] - X[a3], dy = X[b3 + 1] - X[a3 + 1], dz = X[b3 + 2] - X[a3 + 2];
        var d = Math.sqrt(dx * dx + dy * dy + dz * dz) || 1e-6, e = (d - L[2]) / d * 0.5 * L[3] * step;
        X[a3] += dx * e; X[a3 + 1] += dy * e; X[a3 + 2] += dz * e;
        X[b3] -= dx * e; X[b3 + 1] -= dy * e; X[b3 + 2] -= dz * e;
      }
      if (it % 2 === 0) {
        for (i = 0; i < N; i++) {
          for (j = i + 1; j < N; j++) {
            var key2 = i + ',' + j;
            if (C[key2]) { continue; }
            var i3 = i * 3, j3 = j * 3;
            var ex = X[j3] - X[i3], ey = X[j3 + 1] - X[i3 + 1], ez = X[j3 + 2] - X[i3 + 2];
            var dd = Math.sqrt(ex * ex + ey * ey + ez * ez) || 1e-6;
            var min = it < 300 ? 3.4 : 2.9;
            if (dd < min) {
              var f = (dd - min) / dd * 0.25 * step;
              X[i3] += ex * f; X[i3 + 1] += ey * f; X[i3 + 2] += ez * f;
              X[j3] -= ex * f; X[j3 + 1] -= ey * f; X[j3 + 2] -= ez * f;
            }
          }
        }
      }
    }
    // centre and put the longest axis horizontal-ish
    var cx = 0, cy = 0, cz = 0;
    for (i = 0; i < N; i++) { cx += X[i * 3]; cy += X[i * 3 + 1]; cz += X[i * 3 + 2]; }
    cx /= N; cy /= N; cz /= N;
    var rmax = 0;
    for (i = 0; i < N; i++) {
      X[i * 3] -= cx; X[i * 3 + 1] -= cy; X[i * 3 + 2] -= cz;
      rmax = Math.max(rmax, Math.hypot(X[i * 3], X[i * 3 + 1], X[i * 3 + 2]));
    }
    return { X: X, r: rmax || 1 };
  }

  var COL = {
    C: [150, 165, 147], N: [126, 164, 196], O: [214, 128, 104], S: [210, 188, 147], P: [210, 150, 100],
    F: [150, 196, 128], Cl: [130, 190, 140], Br: [190, 140, 100], I: [170, 140, 200], B: [200, 160, 120], Se: [200, 170, 110]
  };
  var RAD = { C: 0.36, N: 0.36, O: 0.36, S: 0.46, P: 0.44, F: 0.32, Cl: 0.44, Br: 0.5, I: 0.56, B: 0.36, Se: 0.48 };

  function mount(cv) {
    var ctx = cv.getContext('2d');
    var still = global.matchMedia ? global.matchMedia('(prefers-reduced-motion: reduce)') : { matches: false };
    var mol = null, geo = null, yaw = 0.6, pitch = -0.35, vyaw = 0, raf = 0, last = 0, visible = false;
    var dragging = false, px = 0, py = 0, idle = 0, W = 0, H = 0, dpr = 1;

    function size() {
      var r = cv.getBoundingClientRect();
      dpr = Math.min(global.devicePixelRatio || 1, 2);
      W = Math.max(1, Math.round(r.width * dpr)); H = Math.max(1, Math.round(r.height * dpr));
      if (cv.width !== W || cv.height !== H) { cv.width = W; cv.height = H; }
    }

    function draw() {
      ctx.clearRect(0, 0, W, H);
      if (!mol) { return; }
      var N = mol.atoms.length, X = geo.X;
      var cy = Math.cos(yaw), sy = Math.sin(yaw), cp = Math.cos(pitch), spp = Math.sin(pitch);
      var sc = Math.min(W, H) * 0.42 / (geo.r + 0.6);
      var P = new Float64Array(N * 3);
      for (var i = 0; i < N; i++) {
        var x = X[i * 3], y = X[i * 3 + 1], z = X[i * 3 + 2];
        var x1 = x * cy + z * sy, z1 = -x * sy + z * cy;
        var y1 = y * cp - z1 * spp, z2 = y * spp + z1 * cp;
        P[i * 3] = W / 2 + x1 * sc; P[i * 3 + 1] = H / 2 - y1 * sc; P[i * 3 + 2] = z2;
      }
      // items: bonds as half-bonds at their midpoint depth, atoms at theirs
      var items = [];
      mol.bonds.forEach(function (b) { items.push({ k: (P[b[0] * 3 + 2] + P[b[1] * 3 + 2]) / 2 - 0.01, b: b }); });
      for (i = 0; i < N; i++) { items.push({ k: P[i * 3 + 2], a: i }); }
      items.sort(function (u, v) { return u.k - v.k; });
      var zr = geo.r + 0.6;
      for (var t = 0; t < items.length; t++) {
        var it = items[t];
        var fog = 0.5 + 0.5 * (it.k + zr) / (2 * zr);
        if (it.b) {
          var a = it.b[0], b = it.b[1];
          var ax = P[a * 3], ay = P[a * 3 + 1], bx = P[b * 3], by = P[b * 3 + 1];
          var dx = bx - ax, dy = by - ay, len = Math.hypot(dx, dy) || 1, nx = -dy / len, ny = dx / len;
          var lines = it.b[2] === 3 ? [-1, 0, 1] : it.b[2] === 2 ? [-0.5, 0.5] : it.b[2] === 1.5 ? [-0.5, 0.5] : [0];
          var gap = sc * 0.09;
          ctx.lineCap = 'round';
          for (var q = 0; q < lines.length; q++) {
            var o = lines[q] * gap;
            ctx.strokeStyle = 'rgba(151,165,147,' + (0.25 + 0.55 * fog).toFixed(3) + ')';
            ctx.lineWidth = Math.max(1, sc * (lines.length > 1 ? 0.05 : 0.08));
            if (it.b[2] === 1.5 && q === 1) { ctx.setLineDash([sc * 0.12, sc * 0.1]); } else { ctx.setLineDash([]); }
            ctx.beginPath(); ctx.moveTo(ax + nx * o, ay + ny * o); ctx.lineTo(bx + nx * o, by + ny * o); ctx.stroke();
          }
          ctx.setLineDash([]);
        } else {
          var el = mol.atoms[it.a].el, col = COL[el] || COL.C, rr = (RAD[el] || 0.36) * sc;
          var x0 = P[it.a * 3], y0 = P[it.a * 3 + 1];
          var gr = ctx.createRadialGradient(x0 - rr * 0.35, y0 - rr * 0.4, rr * 0.1, x0, y0, rr);
          var k1 = 0.55 + 0.45 * fog;
          gr.addColorStop(0, 'rgb(' + Math.round(Math.min(255, col[0] * 1.35 * k1)) + ',' + Math.round(Math.min(255, col[1] * 1.35 * k1)) + ',' + Math.round(Math.min(255, col[2] * 1.35 * k1)) + ')');
          gr.addColorStop(1, 'rgb(' + Math.round(col[0] * 0.38 * k1) + ',' + Math.round(col[1] * 0.38 * k1) + ',' + Math.round(col[2] * 0.38 * k1) + ')');
          ctx.fillStyle = gr;
          ctx.beginPath(); ctx.arc(x0, y0, rr, 0, Math.PI * 2); ctx.fill();
          ctx.lineWidth = Math.max(1, dpr * 0.75);
          ctx.strokeStyle = 'rgba(8,10,9,.8)';
          ctx.stroke();
        }
      }
    }

    function tick(now) {
      raf = 0;
      var dt = last ? Math.min((now - last) / 1000, 1 / 30) : 1 / 60;
      last = now;
      if (!dragging) {
        idle += dt;
        var cruise = still.matches ? 0 : 0.32;
        vyaw += (cruise - vyaw) * (1 - Math.exp(-dt / 0.9));
        yaw += vyaw * dt;
      }
      draw();
      var moving = dragging || Math.abs(vyaw) > 1e-3;
      if (visible && moving) { raf = global.requestAnimationFrame(tick); } else { last = 0; }
    }
    function kick() { if (!raf && visible) { raf = global.requestAnimationFrame(tick); } }

    cv.addEventListener('pointerdown', function (e) {
      dragging = true; px = e.clientX; py = e.clientY; vyaw = 0;
      try { cv.setPointerCapture(e.pointerId); } catch (err) { /* ignore */ }
      kick();
    });
    cv.addEventListener('pointermove', function (e) {
      if (!dragging) { return; }
      var dx = e.clientX - px, dy = e.clientY - py;
      px = e.clientX; py = e.clientY;
      yaw += dx * 0.01; pitch = Math.max(-1.3, Math.min(1.3, pitch + dy * 0.01));
      vyaw = still.matches ? 0 : dx * 0.6;
      draw();
    });
    function up() { dragging = false; kick(); }
    cv.addEventListener('pointerup', up);
    cv.addEventListener('pointercancel', up);
    cv.addEventListener('keydown', function (e) {
      var k = e.key;
      if (k === 'ArrowLeft' || k === 'ArrowRight') { yaw += k === 'ArrowLeft' ? -0.2 : 0.2; e.preventDefault(); draw(); }
      if (k === 'ArrowUp' || k === 'ArrowDown') { pitch = Math.max(-1.3, Math.min(1.3, pitch + (k === 'ArrowUp' ? -0.2 : 0.2))); e.preventDefault(); draw(); }
    });

    if (global.IntersectionObserver) {
      new global.IntersectionObserver(function (es) {
        visible = es[es.length - 1].isIntersecting;
        if (visible) { kick(); }
      }, { rootMargin: '60px 0px' }).observe(cv);
    } else { visible = true; }
    global.addEventListener('resize', function () { size(); draw(); });
    size();

    return {
      set: function (smiles) {
        var g;
        try { g = parse(String(smiles || '').trim()); } catch (e) { mol = null; geo = null; draw(); return false; }
        mol = g; geo = embed(g, smiles);
        if (!still.matches) { vyaw = 0.32; }
        size(); draw(); kick();
        return true;
      }
    };
  }

  global.MFMOL = { mount: mount, parse: parse };
})(window);
