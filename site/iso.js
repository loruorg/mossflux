/* ---------------------------------------------------------------------------
   iso.js - a small isometric drawing core for the figures in figs.js.

   World units, z up. One orthographic camera per figure: turn the floor by
   the azimuth, then tilt by the elevation. Solids are convex: an extrusion of
   a planar polygon along a vector. Only faces that look at the viewer are
   drawn, so a convex solid needs no sorting of its own faces; whole solids
   are painted back to front by depth.

   Every fill and stroke is a class, so the tones live in styles.css.
   ------------------------------------------------------------------------- */

(function (global) {
  'use strict';

  var D2R = Math.PI / 180;

  function camera(azimuth, elevation) {
    var a = azimuth * D2R, e = elevation * D2R;
    var c = { ca: Math.cos(a), sa: Math.sin(a), ce: Math.cos(e), se: Math.sin(e) };
    // toward the viewer, in world space
    c.v = [c.ce * c.sa, c.ce * c.ca, c.se];
    // the light: from the upper left, a little in front
    c.l = norm([-0.55, 0.35, 0.76]);
    return c;
  }

  function norm(v) {
    var m = Math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) || 1;
    return [v[0] / m, v[1] / m, v[2] / m];
  }
  function sub(a, b) { return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]; }
  function add(a, b) { return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]; }
  function dot(a, b) { return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]; }
  function cross(a, b) { return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]; }

  /* World point -> screen point. */
  function P(c, p) {
    var xr = p[0] * c.ca - p[1] * c.sa;
    var yr = p[0] * c.sa + p[1] * c.ca;
    return [xr, yr * c.se - p[2] * c.ce];
  }
  /* Larger is nearer the viewer. */
  function depth(c, p) { return dot(p, c.v); }

  /* Screen offset of a world vector: moving parts are translated by this. */
  function offset(c, v) { var s = P(c, v); return s; }

  function f(n) { return Math.round(n * 100) / 100; }
  function pathOf(c, pts, closed) {
    var s = '';
    for (var i = 0; i < pts.length; i++) {
      var q = P(c, pts[i]);
      s += (i ? 'L' : 'M') + f(q[0]) + ' ' + f(q[1]);
    }
    return closed === false ? s : s + 'Z';
  }

  /* Convex hull of screen points (monotone chain), for a smooth solid's outline. */
  function hull(pts) {
    pts = pts.slice().sort(function (a, b) { return a[0] - b[0] || a[1] - b[1]; });
    function cr(o, a, b) { return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]); }
    var lo = [], up = [], i;
    for (i = 0; i < pts.length; i++) {
      while (lo.length >= 2 && cr(lo[lo.length - 2], lo[lo.length - 1], pts[i]) <= 0) { lo.pop(); }
      lo.push(pts[i]);
    }
    for (i = pts.length - 1; i >= 0; i--) {
      while (up.length >= 2 && cr(up[up.length - 2], up[up.length - 1], pts[i]) <= 0) { up.pop(); }
      up.push(pts[i]);
    }
    up.pop(); lo.pop();
    return lo.concat(up);
  }

  /* Facing -> one of four side shades, or the top. */
  function toneOf(c, n) {
    if (n[2] > 0.8) { return 'tp'; }
    var k = (dot(n, c.l) + 1) / 2;
    return 's' + Math.max(0, Math.min(3, Math.floor(k * 4)));
  }

  /* An extrusion of the planar polygon `base` (3D points, any winding) along
     `v`. opts: cls (extra class), line (stroke tone: hi mid lo faint lit),
     smooth (no edges between side facets: draw a hull outline instead),
     attrs (raw attribute string on the group). Returns an SVG string. */
  function extrude(c, base, v, opts) {
    opts = opts || {};
    var n = base.length, top = [], i;
    for (i = 0; i < n; i++) { top.push(add(base[i], v)); }
    var cen = [0, 0, 0];
    for (i = 0; i < n; i++) { cen = add(cen, base[i]); }
    cen = [cen[0] / n + v[0] / 2, cen[1] / n + v[1] / 2, cen[2] / n + v[2] / 2];

    var faces = [];
    function face(pts, isCap) {
      var nn = norm(cross(sub(pts[1], pts[0]), sub(pts[2], pts[0])));
      var mid = [0, 0, 0];
      for (var j = 0; j < pts.length; j++) { mid = add(mid, pts[j]); }
      mid = [mid[0] / pts.length, mid[1] / pts.length, mid[2] / pts.length];
      if (dot(nn, sub(mid, cen)) < 0) { nn = [-nn[0], -nn[1], -nn[2]]; }
      if (dot(nn, c.v) > 1e-6) { faces.push({ pts: pts, n: nn, cap: isCap }); }
    }
    for (i = 0; i < n; i++) {
      var j2 = (i + 1) % n;
      face([base[i], base[j2], top[j2], top[i]], false);
    }
    face(base, true);
    face(top, true);

    var line = opts.line || 'mid';
    var s = '<g class="iso-s' + (opts.cls ? ' ' + opts.cls : '') + '"' + (opts.attrs ? ' ' + opts.attrs : '') + '>';
    var outline = [];
    for (i = 0; i < faces.length; i++) {
      var F = faces[i], tone = F.cap && F.n[2] > 0.8 ? 'tp' : toneOf(c, F.n);
      var d = pathOf(c, F.pts);
      if (opts.smooth && !F.cap) {
        s += '<path class="' + tone + ' sm" d="' + d + '"/>';
      } else if (opts.smooth) {
        s += '<path class="' + tone + '" d="' + d + '"/>';
      } else {
        s += '<path class="' + tone + ' ln-' + line + '" d="' + d + '"/>';
      }
      for (var k = 0; k < F.pts.length; k++) { outline.push(P(c, F.pts[k])); }
    }
    if (opts.smooth) {
      var h = hull(outline), hs = '';
      for (i = 0; i < h.length; i++) { hs += (i ? 'L' : 'M') + f(h[i][0]) + ' ' + f(h[i][1]); }
      s += '<path class="ln-' + line + ' nofill" d="' + hs + 'Z"/>';
      for (i = 0; i < faces.length; i++) {
        if (faces[i].cap) { s += '<path class="ln-' + line + ' nofill" d="' + pathOf(c, faces[i].pts) + '"/>'; }
      }
    }
    return s + '</g>';
  }

  /* Axis-aligned box. */
  function box(c, x, y, z, w, d, h, opts) {
    return extrude(c, [[x, y, z], [x + w, y, z], [x + w, y + d, z], [x, y + d, z]], [0, 0, h], opts);
  }

  /* Circle of radius r around ctr, in the plane normal to axis 'x' | 'y' | 'z'. */
  function circle(ctr, r, axis, seg) {
    var out = [];
    seg = seg || 28;
    for (var i = 0; i < seg; i++) {
      var t = i / seg * Math.PI * 2, a = Math.cos(t) * r, b = Math.sin(t) * r;
      if (axis === 'z') { out.push([ctr[0] + a, ctr[1] + b, ctr[2]]); }
      else if (axis === 'y') { out.push([ctr[0] + a, ctr[1], ctr[2] + b]); }
      else { out.push([ctr[0], ctr[1] + a, ctr[2] + b]); }
    }
    return out;
  }

  /* Vertical cylinder standing on (x, y, z). */
  function cyl(c, x, y, z, r, h, opts) {
    opts = opts || {};
    opts.smooth = true;
    return extrude(c, circle([x, y, z], r, 'z', opts.seg || 28), [0, 0, h], opts);
  }

  /* Rounded rectangle plan, as a polygon at height z. */
  function rrect(x, y, w, d, r, z, seg) {
    var out = [], k = seg || 4;
    var cs = [[x + w - r, y + r, -90], [x + w - r, y + d - r, 0], [x + r, y + d - r, 90], [x + r, y + r, 180]];
    for (var i = 0; i < 4; i++) {
      for (var j = 0; j <= k; j++) {
        var t = (cs[i][2] + j / k * 90) * D2R;
        out.push([cs[i][0] + Math.cos(t) * r, cs[i][1] + Math.sin(t) * r, z]);
      }
    }
    return out;
  }

  function line(c, pts, tone, extra) {
    return '<path class="ln-' + (tone || 'lo') + ' nofill"' + (extra ? ' ' + extra : '') + ' d="' + pathOf(c, pts, false) + '"/>';
  }
  function loop(c, pts, tone) {
    return '<path class="ln-' + (tone || 'lo') + ' nofill" d="' + pathOf(c, pts) + '"/>';
  }
  function dots(c, pts, r, tone) {
    var s = '';
    for (var i = 0; i < pts.length; i++) {
      var q = P(c, pts[i]);
      s += '<circle class="dot-' + (tone || 'mid') + '" cx="' + f(q[0]) + '" cy="' + f(q[1]) + '" r="' + r + '"/>';
    }
    return s;
  }
  /* A screw head on a top face: a ring and a dot. */
  function screw(c, x, y, z) {
    return loop(c, circle([x, y, z], 1.6, 'z', 14), 'lo') + dots(c, [[x, y, z]], 0.55, 'mid');
  }
  /* A soft shadow under a plan. */
  function halo(c, x, y, w, d, z) {
    return '<path class="iso-halo" d="' + pathOf(c, rrect(x, y, w, d, Math.min(w, d) * 0.18, z, 4)) + '"/>';
  }

  /* Text engraved on the floor plane at height z: x along world x. */
  function floorText(c, x, y, z, size, text, anchor) {
    var o = P(c, [x, y, z]);
    // images of the world x and y unit vectors
    var ax = [c.ca, c.sa * c.se], ay = [-c.sa, c.ca * c.se];
    return '<text class="iso-tx" text-anchor="' + (anchor || 'middle') + '" font-size="' + size +
      '" transform="matrix(' + f(ax[0]) + ' ' + f(ax[1]) + ' ' + f(ay[0]) + ' ' + f(ay[1]) + ' ' + f(o[0]) + ' ' + f(o[1]) + ')">' +
      text + '</text>';
  }

  /* Upright text at a projected point, for ruler numerals. */
  function label(c, p, size, text, anchor) {
    var o = P(c, p);
    return '<text class="iso-tx" text-anchor="' + (anchor || 'middle') + '" font-size="' + size + '" x="' + f(o[0]) + '" y="' + f(o[1]) + '">' + text + '</text>';
  }

  /* The fitted viewBox of a list of world points. */
  function fit(c, pts, pad) {
    var x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
    for (var i = 0; i < pts.length; i++) {
      var q = P(c, pts[i]);
      if (q[0] < x0) { x0 = q[0]; } if (q[0] > x1) { x1 = q[0]; }
      if (q[1] < y0) { y0 = q[1]; } if (q[1] > y1) { y1 = q[1]; }
    }
    pad = pad || 10;
    return { x: x0 - pad, y: y0 - pad, w: x1 - x0 + pad * 2, h: y1 - y0 + pad * 2 };
  }

  /* Exponential follow: x moves toward target with time constant tau (s). */
  function follow(x, target, dt, tau) {
    var n = x + (target - x) * (1 - Math.exp(-dt / tau));
    return Math.abs(target - n) < 1e-3 ? target : n;
  }

  global.MFISO = {
    camera: camera, P: P, depth: depth, offset: offset, pathOf: pathOf,
    extrude: extrude, box: box, cyl: cyl, circle: circle, rrect: rrect,
    line: line, loop: loop, dots: dots, screw: screw, halo: halo,
    floorText: floorText, label: label, fit: fit, follow: follow, f: f
  };
})(window);
