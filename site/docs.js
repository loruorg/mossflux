/* Docs pages: copy buttons, "Copy page" as Markdown, heading links that copy
   their URL, the current section in "On this page", and keyboard access to
   tables and code blocks that scroll sideways. */
(function () {
  'use strict';

  var doc = document, clip = navigator.clipboard;
  var toast = doc.createElement('p');
  toast.className = 'toast'; toast.setAttribute('role', 'status');
  doc.body.appendChild(toast);
  function say(msg) {
    toast.textContent = msg; toast.classList.add('on');
    clearTimeout(say.t); say.t = setTimeout(function () { toast.classList.remove('on'); }, 1600);
  }
  function done(p, msg) { p.then(function () { say(msg); }, function () { say('Copy failed'); }); }

  if (clip) {
    doc.querySelectorAll('.cb').forEach(function (cb) {
      var b = doc.createElement('button');
      b.type = 'button'; b.className = 'copy'; b.textContent = 'Copy';
      b.setAttribute('aria-label', 'Copy code');
      b.onclick = function () { done(clip.writeText(cb.querySelector('pre').textContent.replace(/\n+$/, '')), 'Copied'); };
      cb.appendChild(b);
    });

    var cp = doc.querySelector('[data-copy-md]'), md = null;
    if (cp) {
      var get = function () {
        md = md || fetch(cp.dataset.copyMd).then(function (r) { if (!r.ok) { throw new Error(r.status); } return r.text(); });
        md.catch(function () { md = null; });
        return md;
      };
      cp.classList.remove('wait');
      cp.addEventListener('pointerenter', get);
      cp.addEventListener('focus', get);
      cp.onclick = function () {
        // Safari keeps the click's permission only if the clipboard write starts now.
        done(window.ClipboardItem && clip.write
          ? clip.write([new ClipboardItem({ 'text/plain': get().then(function (t) { return new Blob([t], { type: 'text/plain' }); }) })])
          : get().then(function (t) { return clip.writeText(t); }), 'Page copied as Markdown');
      };
    }

    doc.addEventListener('click', function (e) {
      var a = e.target.closest && e.target.closest('a.h');
      if (a && !e.metaKey && !e.ctrlKey && !e.shiftKey && !e.altKey) { done(clip.writeText(a.href), 'Link copied'); }
    });
  }

  function scrollables() {
    doc.querySelectorAll('.table, .prose pre').forEach(function (el) {
      if (el.scrollWidth > el.clientWidth + 1) { el.tabIndex = 0; } else { el.removeAttribute('tabindex'); }
    });
  }
  scrollables();
  addEventListener('resize', function () { clearTimeout(scrollables.t); scrollables.t = setTimeout(scrollables, 150); });

  var links = [].slice.call(doc.querySelectorAll('.toc a'));
  var heads = links.map(function (a) { return doc.getElementById(decodeURIComponent(a.hash.slice(1))); });
  var raf = 0;
  function spy() {
    raf = 0;
    var cur = 0, y = parseFloat(getComputedStyle(doc.documentElement).scrollPaddingTop) || 80;
    for (var i = 0; i < heads.length; i++) {
      if (heads[i] && heads[i].getBoundingClientRect().top <= y + 1) { cur = i; } else { break; }
    }
    if (innerHeight + scrollY >= doc.documentElement.scrollHeight - 2) { cur = heads.length - 1; }
    links.forEach(function (a, j) {
      a.classList.toggle('on', j === cur);
      if (j === cur) { a.setAttribute('aria-current', 'location'); } else { a.removeAttribute('aria-current'); }
    });
  }
  if (links.length) {
    addEventListener('scroll', function () { raf = raf || requestAnimationFrame(spy); }, { passive: true });
    spy();
  }
})();
