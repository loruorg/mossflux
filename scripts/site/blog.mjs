// The blog: one page per post in blog/{slug}.md, the index (newest first, 20
// per page), an RSS feed, and the post URLs for the sitemap and llms.txt.
// Files in blog/{slug}/ (images) are served at /blog/{slug}/.
// Front matter: title, description, publishDate (YYYY-MM-DD), and optionally
// updatedDate, tags, faq [{question, answer}], author, image, image_alt and
// schema_json (escaped JSON-LD, printed as is). Other keys are ignored.
import fs from 'node:fs';
import path from 'node:path';
import { Marked, Renderer } from 'marked';
import markedFootnote from 'marked-footnote';
import GithubSlugger from 'github-slugger';
import { bundledLanguages } from 'shiki';
import YAML from 'yaml';

const PER_PAGE = 20;
const WPM = 230;

export async function buildBlog({ ROOT, ORIGIN, write, esc, plain, head, topbar, footer, css, assets, shiki, shareImage }) {
  const dir = path.join(ROOT, 'blog');
  const files = fs.existsSync(dir) ? fs.readdirSync(dir).filter((f) => /^[^._].*\.mdx?$/.test(f)) : [];
  const abs = (u) => (/^https?:\/\//i.test(u) ? u : ORIGIN + u);
  const local = (href) => {
    if (/^([a-z][a-z0-9+.-]*:|#|\/\/|\/)/i.test(href)) { return href; }
    return path.posix.join('/blog/', href).replace(/\.mdx?(?=$|#)/, '');
  };
  const day = (v, file, key) => {
    if (v instanceof Date) { v = v.toISOString(); }
    const m = /^(\d{4})-(\d{2})-(\d{2})/.exec(String(v ?? '').trim());
    const d = m && new Date(`${m[0]}T00:00:00Z`);
    if (!d || Number.isNaN(d.getTime()) || d.toISOString().slice(0, 10) !== m[0]) { throw new Error(`blog/${file}: ${key} must be YYYY-MM-DD`); }
    return m[0];
  };
  const human = (d) => new Date(`${d}T00:00:00Z`).toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric', timeZone: 'UTC' });
  const time = (d) => `<time datetime="${d}">${human(d)}</time>`;
  const str = (v) => (typeof v === 'string' ? v.trim() : typeof v === 'number' ? String(v) : '');

  // Code languages the posts use, loaded once into the shared highlighter.
  const langs = new Set();

  const posts = files.map((file) => {
    const slug = file.replace(/\.mdx?$/, '');
    if (!/^[a-z0-9][a-z0-9._-]*$/i.test(slug) || slug === 'index') { throw new Error(`blog/${file}: the file name is not a usable slug`); }
    const raw = fs.readFileSync(path.join(dir, file), 'utf8').replace(/^\uFEFF/, '').replace(/\r\n?/g, '\n');
    const fm = /^---\n([\s\S]*?)\n---[ \t]*(?:\n|$)/.exec(raw);
    if (!fm) { throw new Error(`blog/${file}: no front matter`); }
    const data = YAML.parse(fm[1]) ?? {};
    const body = raw.slice(fm[0].length);
    const title = str(data.title);
    if (!title) { throw new Error(`blog/${file}: no title`); }
    for (const m of body.matchAll(/^\s*(?:```|~~~)\s*([\w+#-]+)/gm)) {
      if (bundledLanguages[m[1].toLowerCase()]) { langs.add(m[1].toLowerCase()); }
    }
    const tags = (Array.isArray(data.tags) ? data.tags : typeof data.tags === 'string' ? data.tags.split(',') : []).map(str).filter(Boolean);
    const faq = (Array.isArray(data.faq) ? data.faq : [])
      .map((f) => ({ question: str(f?.question), answer: str(f?.answer) }))
      .filter((f) => f.question && f.answer);
    const date = day(data.publishDate, file, 'publishDate');
    const updatedDate = data.updatedDate ? day(data.updatedDate, file, 'updatedDate') : '';
    return {
      file, slug, title, body, tags, faq, date,
      updated: updatedDate > date ? updatedDate : '',
      description: str(data.description),
      author: str(data.author),
      image: str(data.image) ? local(str(data.image)) : '',
      imageAlt: str(data.image_alt),
      schema: str(data.schema_json),
      url: `/blog/${slug}`,
    };
  });
  for (const l of langs) { await shiki.loadLanguage(l); }

  // ------------------------------------------------------------------ render

  function markdown(md, slugger) {
    let section = '';
    const marked = new Marked({
      gfm: true,
      renderer: {
        heading({ tokens, depth }) {
          const inner = this.parser.parseInline(tokens);
          const level = Math.max(depth, 2); // the title is the only H1
          const id = slugger.slug(plain(inner));
          section = plain(inner);
          return `<h${level} id="${id}"><a class="h" href="#${id}">${inner}</a></h${level}>\n`;
        },
        link({ href, title: t, tokens }) {
          return `<a href="${esc(local(href))}"${t ? ` title="${esc(t)}"` : ''}>${this.parser.parseInline(tokens)}</a>`;
        },
        image({ href, title: t, text }) {
          return `<img src="${esc(local(href))}" alt="${esc(text)}"${t ? ` title="${esc(t)}"` : ''} loading="lazy" decoding="async">`;
        },
        code({ text, lang }) {
          const l = (lang || '').trim().toLowerCase();
          const html = shiki.codeToHtml(text.replace(/\n$/, ''), { lang: langs.has(l) ? l : 'text', theme: 'mossflux' })
            .replace(/^<pre[^>]*>/, '<pre class="code">');
          return `<div class="cb">${html}</div>\n`;
        },
        table(token) {
          return `<div class="table" role="region" aria-label="${esc(section || 'Post')} table">${Renderer.prototype.table.call(this, token)}</div>\n`;
        },
      },
    }).use(markedFootnote({ description: 'Notes' }));
    return marked.parse(md);
  }

  // A footnote reference with no definition would show as "[^c1]": drop it.
  function dropOrphanRefs(md) {
    const defined = new Set([...md.matchAll(/^\[\^([^\]\s]+)\]:/gm)].map((m) => m[1]));
    let fenced = false;
    return md.split('\n').map((line) => {
      if (/^\s*(```|~~~)/.test(line)) { fenced = !fenced; }
      return fenced ? line : line.replace(/\[\^([^\]\s]+)\](?!:)/g, (all, id) => (defined.has(id) ? all : ''));
    }).join('\n');
  }

  for (const p of posts) {
    const slugger = new GithubSlugger();
    p.html = markdown(dropOrphanRefs(p.body), slugger);
    if (!p.description) {
      const first = new Marked().lexer(p.body).find((t) => t.type === 'paragraph');
      p.description = first ? plain(new Marked().parseInline(first.text)).replace(/\s+/g, ' ').trim().slice(0, 160) : p.title;
    }
    const words = plain(p.html).split(/\s+/).filter(Boolean).length + p.faq.reduce((n, f) => n + `${f.question} ${f.answer}`.split(/\s+/).length, 0);
    p.minutes = Math.max(1, Math.round(words / WPM));
    p.words = words;
    if (p.faq.length) {
      const id = slugger.slug('FAQ');
      for (const f of p.faq) { f.html = markdown(f.answer, slugger).trim(); }
      p.faqHtml = `<section class="faq" aria-labelledby="${id}"><h2 id="${id}"><a class="h" href="#${id}">FAQ</a></h2>\n${p.faq.map((f) =>
        `<h3>${esc(f.question)}</h3>\n${f.html}`).join('\n')}</section>\n`;
    }
  }
  posts.sort((a, b) => b.date.localeCompare(a.date) || (b.updated || b.date).localeCompare(a.updated || a.date) || a.title.localeCompare(b.title));

  // ------------------------------------------------------------------ pages

  const blogCss = `<style>${css['blog.css']}</style>`;
  const rss = `<link rel="alternate" type="application/rss+xml" title="mossflux blog" href="${ORIGIN}/blog/rss.xml">`;
  const tagList = (tags) => (tags.length ? `<ul class="tags" aria-label="Tags">${tags.map((t) => `<li>${esc(t)}</li>`).join('')}</ul>` : '');
  const ld = (o) => `<script type="application/ld+json">${JSON.stringify(o).replace(/</g, '\\u003c')}</script>`;
  const cta = `<aside class="cta" aria-labelledby="cta-h">
  <p class="cta-h" id="cta-h">Exact search at billions of molecules per second</p>
  <p>mossflux runs Tanimoto similarity and SMARTS substructure search in Rust. Every result matches a brute-force scan.</p>
  <p class="cta-a"><a class="btn hi" href="/#demo">Try the demo</a><a class="btn" href="/docs/">Read the docs</a></p>
</aside>`;

  function related(p) {
    const others = posts.filter((o) => o !== p);
    const shared = (o) => o.tags.filter((t) => p.tags.includes(t)).length;
    return others.map((o, i) => ({ o, s: shared(o), i })).sort((a, b) => b.s - a.s || a.i - b.i).slice(0, 3).map((x) => x.o);
  }

  const card = (p, h = 'h2') => `<li class="post">
  <p class="meta">${time(p.date)}<span>${p.minutes} min read</span></p>
  <${h} class="pt"><a href="${p.url}">${esc(p.title)}</a></${h}>
  <p class="pd">${esc(p.description)}</p>
  ${tagList(p.tags)}
</li>`;

  for (const p of posts) {
    const share = p.image || shareImage(p, 'blog', `/og${p.url}.jpg`);
    const people = p.author ? { '@type': 'Person', name: p.author } : { '@type': 'Organization', name: 'mossflux', url: `${ORIGIN}/` };
    const schema = p.schema
      ? `<script type="application/ld+json">${p.schema.replace(/</g, '\\u003c')}</script>`
      : ld([
        { '@context': 'https://schema.org', '@type': 'BlogPosting', headline: p.title, description: p.description,
          url: ORIGIN + p.url, mainEntityOfPage: ORIGIN + p.url, datePublished: p.date, dateModified: p.updated || p.date,
          image: abs(share), author: people, publisher: { '@type': 'Organization', name: 'mossflux', url: `${ORIGIN}/` },
          ...(p.tags.length ? { keywords: p.tags.join(', ') } : {}), wordCount: p.words, inLanguage: 'en',
          isPartOf: { '@type': 'Blog', name: 'mossflux blog', url: `${ORIGIN}/blog/` } },
        ...(p.faq.length ? [{ '@context': 'https://schema.org', '@type': 'FAQPage', mainEntity: p.faq.map((f) => ({
          '@type': 'Question', name: f.question, acceptedAnswer: { '@type': 'Answer', text: f.html } })) }] : []),
      ]);
    const crumbs = ld({ '@context': 'https://schema.org', '@type': 'BreadcrumbList', itemListElement: [
      { '@type': 'ListItem', position: 1, name: 'Blog', item: `${ORIGIN}/blog/` },
      { '@type': 'ListItem', position: 2, name: p.title, item: ORIGIN + p.url },
    ] });
    const more = related(p);
    write(`${p.url}.html`, `${head({ title: `${p.title} · mossflux`, description: p.description, url: p.url, image: share,
      imageAlt: p.image ? p.imageAlt || p.title : p.title, imageSize: !p.image })}
${blogCss}
<meta property="article:published_time" content="${p.date}">
${p.updated ? `<meta property="article:modified_time" content="${p.updated}">\n` : ''}${p.tags.map((t) => `<meta property="article:tag" content="${esc(t)}">\n`).join('')}${rss}
${schema}
${crumbs}
<script src="${assets.get('docs.js')}" defer></script>
</head>
<body>
${topbar(false, true)}
<main id="main" class="blog" tabindex="-1">
  <article class="prose">
    <nav class="crumb" aria-label="Breadcrumb"><a href="/blog/">Blog</a></nav>
    <h1>${esc(p.title)}</h1>
    <p class="meta">${time(p.date)}${p.updated ? `<span>Updated ${time(p.updated)}</span>` : ''}<span>${p.minutes} min read</span>${p.author ? `<span>By ${esc(p.author)}</span>` : ''}</p>
    ${tagList(p.tags)}
${p.image ? `<figure class="hero"><img src="${esc(p.image)}" alt="${esc(p.imageAlt)}" fetchpriority="high" decoding="async"></figure>\n` : ''}${p.html}${p.faqHtml || ''}
  </article>
  ${cta}
${more.length ? `  <section class="more" aria-labelledby="more-h"><h2 id="more-h">More posts</h2><ul class="posts">${more.map((o) => card(o, 'h3')).join('')}</ul></section>\n` : ''}</main>
${footer}
</body>
</html>
`);
  }

  // Index pages: /blog/, then /blog/page/2 and on.
  const pages = Math.max(1, Math.ceil(posts.length / PER_PAGE));
  const pageUrl = (n) => (n === 1 ? '/blog/' : `/blog/page/${n}`);
  for (let n = 1; n <= pages; n++) {
    const list = posts.slice((n - 1) * PER_PAGE, n * PER_PAGE);
    const title = n === 1 ? 'Blog · mossflux' : `Blog, page ${n} · mossflux`;
    const description = 'Articles on exact molecular similarity and substructure search, fingerprints and mossflux.';
    const pn = pages > 1 ? `<nav class="pn" aria-label="Pages">${n > 1 ? `<a class="prev" href="${pageUrl(n - 1)}"><span>Previous</span>Newer posts</a>` : '<span></span>'}${n < pages ? `<a class="next" href="${pageUrl(n + 1)}"><span>Next</span>Older posts</a>` : '<span></span>'}</nav>` : '';
    write(n === 1 ? '/blog/index.html' : `${pageUrl(n)}.html`, `${head({ title, description, url: pageUrl(n), image: '/og.jpg', type: 'website' })}
${blogCss}
${posts.length ? '' : '<meta name="robots" content="noindex">\n'}${n > 1 ? `<link rel="prev" href="${ORIGIN}${pageUrl(n - 1)}">\n` : ''}${n < pages ? `<link rel="next" href="${ORIGIN}${pageUrl(n + 1)}">\n` : ''}${rss}
${posts.length ? ld({ '@context': 'https://schema.org', '@type': 'Blog', name: 'mossflux blog', url: `${ORIGIN}/blog/`, description }) : ''}
</head>
<body>
${topbar(false, true)}
<main id="main" class="blog" tabindex="-1">
  <header class="bh"><h1>Blog${n > 1 ? `<span>Page ${n}</span>` : ''}</h1><a class="act" href="/blog/rss.xml">RSS</a></header>
  ${list.length ? `<ul class="posts">${list.map((p) => card(p)).join('')}</ul>` : '<p class="none">No posts yet.</p>'}
  ${pn}
</main>
${footer}
</body>
</html>
`);
  }

  // RSS 2.0, newest 50 posts.
  const rfc822 = (d) => new Date(`${d}T00:00:00Z`).toUTCString();
  const x = (s) => String(s).replace(/[&<>]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;' })[c]);
  write('/blog/rss.xml', `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
<channel>
  <title>mossflux blog</title>
  <link>${ORIGIN}/blog/</link>
  <description>Articles on exact molecular similarity and substructure search.</description>
  <language>en</language>
  <atom:link href="${ORIGIN}/blog/rss.xml" rel="self" type="application/rss+xml"/>
${posts.length ? `  <lastBuildDate>${rfc822(posts.map((p) => p.updated || p.date).sort().at(-1))}</lastBuildDate>\n` : ''}${posts.slice(0, 50).map((p) => `  <item>
    <title>${x(p.title)}</title>
    <link>${ORIGIN}${p.url}</link>
    <guid isPermaLink="true">${ORIGIN}${p.url}</guid>
    <pubDate>${rfc822(p.date)}</pubDate>
    <description>${x(p.description)}</description>
${p.tags.map((t) => `    <category>${x(t)}</category>\n`).join('')}  </item>
`).join('')}</channel>
</rss>
`);

  // Images and other files committed next to the posts: blog/{slug}/...
  function copy(rel) {
    for (const e of fs.readdirSync(path.join(dir, rel), { withFileTypes: true })) {
      const r = path.posix.join(rel, e.name);
      if (e.name.startsWith('.')) { continue; }
      if (e.isDirectory()) { copy(r); } else if (rel && !/\.mdx?$/.test(e.name)) { write(`/blog/${r}`, fs.readFileSync(path.join(dir, r))); }
    }
  }
  if (fs.existsSync(dir)) { copy(''); }

  return {
    count: posts.length,
    // The index joins the sitemap only once a post exists.
    urls: posts.length ? [{ url: '/blog/', date: posts.map((p) => p.updated || p.date).sort().at(-1) }, ...posts.map((p) => ({ url: p.url, date: p.updated || p.date }))] : [],
    llms: posts.length ? `## Blog\n\n${posts.map((p) => `- [${p.title}](${ORIGIN}${p.url}): ${p.description}`).join('\n')}\n\n` : '',
  };
}
