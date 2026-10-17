// Builds mossflux.com into dist/: the static site from site/, one HTML page and
// one Markdown copy per doc, llms.txt, sitemap, _headers and share images.
// Run: npm install --prefix scripts/site && node scripts/site/build.mjs
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { Marked, Renderer } from 'marked';
import GithubSlugger from 'github-slugger';
import { createHighlighter } from 'shiki';
import * as esbuild from 'esbuild';

const ROOT = path.resolve(import.meta.dirname, '../..');
const SITE = path.join(ROOT, 'site');
const DIST = path.join(ROOT, 'dist');
const OUT = `${DIST}.tmp-${process.pid}`; // swapped into dist/ when complete
const CACHE = path.join(import.meta.dirname, '.cache');
const ORIGIN = 'https://mossflux.com';
const REPO = 'https://github.com/loruorg/mossflux';

// Sidebar order. Each page: [source file, slug, sidebar label].
const NAV = [
  ['Start', [['docs/README.md', '', 'Get started'], ['docs/cli.md', 'cli', 'CLI']]],
  ['Server', [['docs/api.md', 'api', 'API'], ['docs/deploy.md', 'deploy', 'Deploy']]],
  ['Internals', [['docs/design.md', 'design', 'Design'], ['docs/correctness.md', 'correctness', 'Correctness'], ['docs/benchmarks.md', 'benchmarks', 'Benchmarks']]],
  ['Project', [['docs/development.md', 'development', 'Development'], ['SECURITY.md', 'security', 'Security'], ['CHANGELOG.md', 'changelog', 'Changelog']]],
];
const PAGES = NAV.flatMap(([group, items]) => items.map(([src, slug, label]) => ({
  group, src, slug, label,
  url: slug ? `/docs/${slug}` : '/docs/',
  md: `/docs/${slug || 'index'}.md`,
})));
const BY_SRC = new Map(PAGES.map((p) => [p.src, p]));
const anchors = []; // [source, /docs/page#id] for every in-docs link, checked after rendering
const LANGS = { sh: 'shellscript', bash: 'shellscript', shell: 'shellscript', json: 'json', yaml: 'yaml', toml: 'toml', rust: 'rust', proto: 'proto' };

const esc = (s) => String(s).replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
const hash = (buf) => crypto.createHash('sha256').update(buf).digest('hex').slice(0, 10);
const read = (p) => fs.readFileSync(p, 'utf8');
const write = (rel, data) => {
  const p = path.join(OUT, rel);
  fs.mkdirSync(path.dirname(p), { recursive: true });
  fs.writeFileSync(p, data);
};
const plain = (html) => html.replace(/<[^>]+>/g, '')
  .replace(/&#(\d+);/g, (_, n) => String.fromCharCode(+n))
  .replace(/&(amp|lt|gt|quot);/g, (_, e) => ({ amp: '&', lt: '<', gt: '>', quot: '"' })[e]);

// Last commit date of a file; today if it has uncommitted changes.
function updated(src) {
  const git = (...a) => execFileSync('git', a, { cwd: ROOT, encoding: 'utf8' }).trim();
  try {
    if (git('status', '--porcelain', '--', src)) { return new Date().toISOString().slice(0, 10); }
    return git('log', '-1', '--format=%cs', '--', src) || new Date().toISOString().slice(0, 10);
  } catch { return new Date().toISOString().slice(0, 10); }
}

// A link in a source file: a doc page, a repo file on GitHub, or unchanged.
function resolveLink(href, src, asMarkdown) {
  if (/^([a-z]+:|#|\/\/)/i.test(href)) { return href; }
  const [file, frag] = href.split('#');
  const target = path.posix.normalize(path.posix.join(path.posix.dirname(src), file));
  const page = BY_SRC.get(target);
  const tail = frag ? `#${frag}` : '';
  if (page) { return asMarkdown ? ORIGIN + page.md + tail : page.url + tail; }
  if (!fs.existsSync(path.join(ROOT, target))) { throw new Error(`${src}: broken link ${href}`); }
  const kind = fs.statSync(path.join(ROOT, target)).isDirectory() ? 'tree' : 'blob';
  return `${REPO}/${kind}/main/${target}${tail}`;
}

// The Markdown copy: the source with every relative link made absolute.
function markdownCopy(md, src) {
  let fenced = false;
  return md.split('\n').map((line) => {
    if (/^\s*(```|~~~)/.test(line)) { fenced = !fenced; }
    if (fenced) { return line; }
    return line.replace(/\]\(([^)\s]+)\)/g, (_, h) => `](${resolveLink(h, src, true)})`);
  }).join('\n');
}

// ------------------------------------------------------------------ assets

fs.mkdirSync(OUT, { recursive: true });
process.on('exit', () => fs.rmSync(OUT, { recursive: true, force: true }));

const assets = new Map(); // site-relative path -> /assets/name.hash.ext
function emit(rel, data) {
  const ext = path.extname(rel);
  const url = `/assets/${path.basename(rel, ext)}.${hash(data)}${ext}`;
  write(url, data);
  assets.set(rel, url);
  return url;
}
const swap = (text) => {
  for (const [rel, url] of assets) { text = text.replaceAll(`"${rel}"`, `"${url}"`).replaceAll(`'${rel}'`, `'${url}'`); }
  return text;
};
for (const f of fs.readdirSync(path.join(SITE, 'fonts')).filter((f) => f.endsWith('.woff2'))) {
  emit(`fonts/${f}`, fs.readFileSync(path.join(SITE, 'fonts', f)));
}
emit('vendor/smiles-drawer.min.js', fs.readFileSync(path.join(SITE, 'vendor/smiles-drawer.min.js')));
// CSS is small, so each page inlines it: no render-blocking request.
const css = {};
for (const f of ['site.css', 'docs.css']) {
  css[f] = (await esbuild.transform(swap(read(path.join(SITE, f))), { loader: 'css', minify: true })).code.trim();
}
for (const f of ['app.js', 'docs.js']) {
  emit(f, (await esbuild.transform(swap(read(path.join(SITE, f))), { loader: 'js', minify: true })).code);
}

// Everything else in site/ is copied as is.
const handled = new Set([...assets.keys(), ...Object.keys(css), 'index.html']);
(function copy(dir) {
  for (const e of fs.readdirSync(path.join(SITE, dir), { withFileTypes: true })) {
    const rel = path.posix.join(dir, e.name);
    if (e.name.startsWith('.') || handled.has(rel)) { continue; }
    if (e.isDirectory()) { copy(rel); } else { write(rel, fs.readFileSync(path.join(SITE, rel))); }
  }
})('');
const home = swap(read(path.join(SITE, 'index.html')));
if (!home.includes('<link rel="stylesheet" href="site.css">')) { throw new Error('site/index.html: no site.css link to inline'); }
write('index.html', home.replace('<link rel="stylesheet" href="site.css">', `<style>${css['site.css']}</style>`));

// ------------------------------------------------------------------ docs

const THEME = {
  name: 'mossflux',
  type: 'dark',
  colors: { 'editor.background': '#0b0b0b', 'editor.foreground': '#d6d6d6' },
  tokenColors: [
    { scope: ['comment', 'punctuation.definition.comment'], settings: { foreground: '#8c8c8c', fontStyle: 'italic' } },
    { scope: ['string', 'punctuation.definition.string', 'string.unquoted'], settings: { foreground: '#b9e0c9' } },
    { scope: ['constant.numeric', 'constant.language'], settings: { foreground: '#f0d5a8' } },
    { scope: ['support.type.property-name', 'entity.name.tag', 'meta.mapping.key'], settings: { foreground: '#ffffff' } },
    { scope: ['entity.name.function', 'support.function.builtin', 'entity.name.command', 'keyword'], settings: { foreground: '#ffffff' } },
    { scope: ['constant.other.option', 'variable', 'variable.other'], settings: { foreground: '#bcd3f0' } },
    { scope: ['punctuation', 'keyword.operator'], settings: { foreground: '#a3a3a3' } },
  ],
};
const shiki = await createHighlighter({ themes: [THEME], langs: [...new Set(Object.values(LANGS))] });

function renderPage(page) {
  const md = read(path.join(ROOT, page.src));
  const slugger = new GithubSlugger();
  const toc = [];
  const ids = new Set();
  let title = '';
  let section = '';
  const marked = new Marked({
    gfm: true,
    renderer: {
      heading({ tokens, depth }) {
        const inner = this.parser.parseInline(tokens);
        if (depth === 1) { title = plain(inner); return `<h1>${inner}</h1>\n`; }
        const id = slugger.slug(plain(inner));
        ids.add(id);
        section = plain(inner);
        if (depth <= 3) { toc.push({ depth, id, text: plain(inner) }); }
        return `<h${depth} id="${id}"><a class="h" href="#${id}">${inner}</a></h${depth}>\n`;
      },
      link({ href, title: t, tokens }) {
        const url = resolveLink(href, page.src, false);
        if (url.startsWith('/docs/') || url.startsWith('#')) { anchors.push([page.src, url.startsWith('#') ? page.url + url : url]); }
        return `<a href="${esc(url)}"${t ? ` title="${esc(t)}"` : ''}>${this.parser.parseInline(tokens)}</a>`;
      },
      code({ text, lang }) {
        const l = LANGS[(lang || '').trim()] || 'text';
        const html = shiki.codeToHtml(text.replace(/\n$/, ''), { lang: l, theme: 'mossflux' })
          .replace(/^<pre[^>]*>/, `<pre class="code${l === 'text' ? ' out' : ''}">`);
        return `<div class="cb">${html}</div>\n`;
      },
      table(token) {
        const label = section || title;
        return `<div class="table" role="region" aria-label="${esc(label)} table">${Renderer.prototype.table.call(this, token)}</div>\n`;
      },
    },
  });
  const html = marked.parse(md);
  const first = marked.lexer(md).find((t) => t.type === 'paragraph');
  const description = first ? plain(marked.parseInline(first.text)).replace(/\s+/g, ' ').trim() : title;
  return { ...page, title, toc, ids, description, body: html, markdown: markdownCopy(md, page.src), date: updated(page.src) };
}

const docs = PAGES.map(renderPage);
for (const [src, url] of anchors) {
  const [p, id] = url.split('#');
  const d = docs.find((x) => x.url === p);
  if (!d || (id && !d.ids.has(id))) { throw new Error(`${src}: link to missing page or heading ${url}`); }
}
for (const d of docs) {
  if (!d.title) { throw new Error(`${d.src}: no H1`); }
  if (d.description.length > 200) { console.warn(`warn: ${d.src}: first paragraph is ${d.description.length} chars (meta description)`); }
}

// ------------------------------------------------------------------ share images

// One 1200x630 image per page, drawn by headless Chrome with the site fonts.
// Cached by content in scripts/site/.cache; without Chrome, pages use /og.jpg.
const CHROME = process.env.CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
function shareImage(d) {
  if (!fs.existsSync(CHROME)) { return '/og.jpg'; }
  const font = (f) => `file://${path.join(SITE, 'fonts', f)}`;
  const html = `<!doctype html><meta charset="utf-8"><style>
@font-face { font-family: Erode; src: url("${font('Erode-Variable.woff2')}"); font-weight: 300 700; }
@font-face { font-family: Tabular; src: url("${font('Tabular-Regular.woff2')}"); }
html, body { margin: 0; width: 1200px; height: 630px; background: #000; color: #fff; overflow: hidden; }
body { position: relative; font-family: -apple-system, "Helvetica Neue", sans-serif; }
.bg { position: absolute; top: 0; right: -60px; width: 600px; height: 630px; opacity: 0.42;
  background: url("file://${path.join(SITE, 'og.jpg')}") -600px 0 / 1200px 630px; -webkit-mask: linear-gradient(90deg, transparent, #000 45%); }
.in { position: absolute; inset: 64px 80px; display: flex; flex-direction: column; }
.logo { display: flex; align-items: center; gap: 12px; font: 600 30px Erode; }
.logo i { width: 16px; height: 16px; border: 3px solid #fff; border-radius: 50%; }
.logo span { font: 22px Tabular; color: rgba(255,255,255,.6); margin-left: 6px; }
h1 { font: 600 ${d.title.length > 14 ? 100 : 128}px/0.95 Erode; letter-spacing: -0.04em; margin: auto 0 26px; max-width: 900px; }
p { font-size: 30px; line-height: 1.35; color: rgba(255,255,255,.72); margin: 0; max-width: 940px;
  display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; }
</style><div class="bg"></div><div class="in"><div class="logo"><i></i>mossflux<span>docs</span></div>
<h1>${esc(d.title)}</h1><p>${esc(d.description)}</p></div>`;
  const key = hash(html + hash(fs.readFileSync(path.join(SITE, 'og.jpg'))));
  const jpg = path.join(CACHE, 'og', `${key}.jpg`);
  if (!fs.existsSync(jpg)) {
    fs.mkdirSync(path.dirname(jpg), { recursive: true });
    const tmp = fs.mkdtempSync(path.join(CACHE, 'tmp-'));
    fs.writeFileSync(path.join(tmp, 'og.html'), html);
    execFileSync(CHROME, ['--headless', '--disable-gpu', '--hide-scrollbars', '--force-device-scale-factor=1',
      '--window-size=1200,630', '--virtual-time-budget=3000', `--user-data-dir=${path.join(tmp, 'profile')}`,
      `--screenshot=${path.join(tmp, 'og.png')}`, `file://${path.join(tmp, 'og.html')}`], { stdio: 'ignore' });
    execFileSync('sips', ['-s', 'format', 'jpeg', '-s', 'formatOptions', '82', path.join(tmp, 'og.png'), '--out', jpg], { stdio: 'ignore' });
    fs.rmSync(tmp, { recursive: true, force: true });
  }
  const url = `/og/${d.slug || 'docs'}.jpg`;
  write(url, fs.readFileSync(jpg));
  return url;
}

// ------------------------------------------------------------------ templates

const head = ({ title, description, url, image, type = 'article', md }) => `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover">
<title>${esc(title)}</title>
<meta name="description" content="${esc(description)}">
<link rel="canonical" href="${ORIGIN}${url}">
${md ? `<link rel="alternate" type="text/markdown" href="${md}" title="Markdown">\n` : ''}<meta name="color-scheme" content="dark">
<meta name="theme-color" content="#000000">
<meta property="og:type" content="${type}">
<meta property="og:site_name" content="mossflux">
<meta property="og:url" content="${ORIGIN}${url}">
<meta property="og:title" content="${esc(title)}">
<meta property="og:description" content="${esc(description)}">
<meta property="og:image" content="${ORIGIN}${image}">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta property="og:image:alt" content="${esc(title)}">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="${esc(title)}">
<meta name="twitter:description" content="${esc(description)}">
<meta name="twitter:image" content="${ORIGIN}${image}">
<link rel="icon" href="/favicon.svg" type="image/svg+xml">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
<link rel="preload" href="${assets.get('fonts/Erode-Variable.woff2')}" as="font" type="font/woff2" crossorigin>
<link rel="preload" href="${assets.get('fonts/Tabular-Regular.woff2')}" as="font" type="font/woff2" crossorigin>
<style>${css['docs.css']}</style>`;

const topbar = (current) => `<a class="skip" href="#main">Skip to content</a>
<header class="top">
  <a class="logo" href="/"><span class="mark" aria-hidden="true"></span>mossflux</a>
  <nav aria-label="Site">
    <a href="/docs/"${current ? ' aria-current="true"' : ''}>Docs</a>
    <a href="/#demo">Demo</a>
    <a class="pill" href="${REPO}">GitHub</a>
  </nav>
</header>`;

const sidebar = (cur) => NAV.map(([group, items]) => `<p class="g">${group}</p><ul>${items.map(([src, slug, label]) => {
  const p = BY_SRC.get(src);
  return `<li><a href="${p.url}"${p === cur ? ' aria-current="page"' : ''}>${label}</a></li>`;
}).join('')}</ul>`).join('');

const footer = `<footer class="foot">
  <p>mossflux · MIT or Apache-2.0</p>
  <p><a href="/llms.txt">llms.txt</a><a href="/sitemap.xml">Sitemap</a><a href="${REPO}">GitHub</a></p>
</footer>`;

function docPage(d, i) {
  const prev = docs[i - 1], next = docs[i + 1];
  const image = shareImage(d);
  const pageTitle = d.slug ? `${d.title} · mossflux docs` : 'mossflux docs';
  const ld = JSON.stringify([
    { '@context': 'https://schema.org', '@type': 'TechArticle', headline: d.title, description: d.description,
      url: ORIGIN + d.url, dateModified: d.date, image: ORIGIN + image, inLanguage: 'en',
      isPartOf: { '@type': 'WebSite', name: 'mossflux', url: `${ORIGIN}/` } },
    { '@context': 'https://schema.org', '@type': 'BreadcrumbList', itemListElement: [
      { '@type': 'ListItem', position: 1, name: 'Docs', item: `${ORIGIN}/docs/` },
      ...(d.slug ? [{ '@type': 'ListItem', position: 2, name: d.label, item: ORIGIN + d.url }] : []),
    ] },
  ]).replace(/</g, '\\u003c');
  const [h1, ...rest] = d.body.split(/(?<=<\/h1>\n)/);
  const toc = d.toc.length > 1 ? `<aside class="toc" aria-labelledby="toc-h"><p class="g" id="toc-h">On this page</p><ul>${d.toc.map((t) =>
    `<li class="d${t.depth}"><a href="#${t.id}">${esc(t.text)}</a></li>`).join('')}</ul></aside>` : '<div class="toc"></div>';
  return `${head({ title: pageTitle, description: d.description, url: d.url, image, md: d.md })}
<script type="application/ld+json">${ld}</script>
<script src="${assets.get('docs.js')}" defer></script>
</head>
<body>
${topbar(true)}
<div class="wrap">
  <nav class="side" aria-label="Docs">${sidebar(d)}</nav>
  <main id="main" tabindex="-1">
    <details class="mnav"><summary>${esc(d.label)}<span>Menu</span></summary><nav aria-label="Docs">${sidebar(d)}</nav></details>
    <article class="prose">
${h1}<div class="acts"><button type="button" class="act wait" data-copy-md="${d.md}">Copy page</button><a class="act" href="${d.md}">Markdown</a><a class="act" href="${REPO}/edit/main/${d.src}">Edit</a></div>
${rest.join('')}
    </article>
    <nav class="pn" aria-label="Previous and next page">${prev ? `<a class="prev" href="${prev.url}"><span>Previous</span>${esc(prev.label)}</a>` : '<span></span>'}${next ? `<a class="next" href="${next.url}"><span>Next</span>${esc(next.label)}</a>` : '<span></span>'}</nav>
    <p class="upd">Updated ${d.date}</p>
  </main>
  ${toc}
</div>
${footer}
</body>
</html>
`;
}

docs.forEach((d, i) => {
  write(`${d.url === '/docs/' ? '/docs/index' : d.url}.html`, docPage(d, i));
  write(d.md, d.markdown);
});

write('404.html', `${head({ title: 'Page not found · mossflux', description: 'This page does not exist.', url: '/404', image: '/og.jpg', type: 'website' })}
<meta name="robots" content="noindex">
</head>
<body>
${topbar(false)}
<main id="main" class="lost" tabindex="-1">
  <h1>Page not found.</h1>
  <p>This page does not exist. Try the <a href="/docs/">docs</a> or the <a href="/">home page</a>.</p>
</main>
${footer}
</body>
</html>
`);

// ------------------------------------------------------------------ indexes

const summary = 'Exact molecular similarity (Tanimoto) and SMARTS substructure search in Rust. Every result matches a brute-force scan.';
write('llms.txt', `# mossflux

> ${summary}

Each page is also plain Markdown at the URL below. All pages in one file: ${ORIGIN}/llms-full.txt

${NAV.map(([group, items]) => `## ${group}\n\n${items.map(([src]) => {
  const d = docs.find((x) => x.src === src);
  return `- [${d.title}](${ORIGIN}${d.md}): ${d.description}`;
}).join('\n')}`).join('\n\n')}

## Links

- [Live demo](${ORIGIN}/#demo): search ChEMBL 37 in the browser
- [Source](${REPO})
`);
write('llms-full.txt', docs.map((d) => `<!-- ${ORIGIN}${d.url} -->\n\n${d.markdown.trim()}\n`).join('\n\n'));

const urls = [{ url: '/', date: updated('site/index.html') }, ...docs];
write('sitemap.xml', `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urls.map((u) => `  <url><loc>${ORIGIN}${u.url}</loc><lastmod>${u.date}</lastmod></url>`).join('\n')}
</urlset>
`);

write('_headers', `/*
  X-Content-Type-Options: nosniff
  Referrer-Policy: strict-origin-when-cross-origin
  Permissions-Policy: camera=(), microphone=(), geolocation=(), interest-cohort=()
  Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'; connect-src 'self' https://api.mossflux.com http://localhost:* http://127.0.0.1:*; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'

/assets/*
  Cache-Control: public, max-age=31536000, immutable

/docs/*.md
  Content-Type: text/markdown; charset=utf-8

/llms.txt
  Content-Type: text/plain; charset=utf-8

/llms-full.txt
  Content-Type: text/plain; charset=utf-8
`);

fs.rmSync(DIST, { recursive: true, force: true });
fs.renameSync(OUT, DIST);
console.log(`built dist/: ${docs.length} doc pages, ${assets.size} hashed assets`);
