// Render the share card (site/og.jpg, 1200x630) and site/apple-touch-icon.png.
//
//   node scripts/site/make_share.mjs [theme] [seconds]
//
// Needs ffmpeg and a global Playwright install (npm i -g playwright). The card
// is a frame of one source clip (default: ignite at 7.5 s) with the headline set
// in the site's own fonts, so it matches the page it links to.

import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { createRequire } from 'node:module';
import { execFileSync, execSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const site = path.join(here, '..', '..', 'site');
const [theme = 'ignite', at = '7.5'] = process.argv.slice(2);
const require = createRequire(import.meta.url);
const { chromium } = require(process.env.PLAYWRIGHT || path.join(execSync('npm root -g').toString().trim(), 'playwright'));

const tmp = await fs.mkdtemp(path.join(os.tmpdir(), 'mossflux-share-'));
const frame = path.join(tmp, 'frame.png');
execFileSync('ffmpeg', ['-v', 'error', '-y', '-ss', at, '-i',
  path.join(here, 'out', `${theme}-l.mp4`), '-frames:v', '1', '-vf', 'hue=s=0', frame]);

const font = (f) => pathToFileURL(path.join(site, 'fonts', f)).href;
const mark = `<svg viewBox="0 0 20 20" width="18" height="18"><circle cx="10" cy="10" r="7" fill="none" stroke="#fff" stroke-width="3"/></svg>`;

const card = `<!doctype html><meta charset="utf-8"><style>
@font-face { font-family: Erode; src: url(${font('Erode-Variable.woff2')}); font-weight: 300 700; }
@font-face { font-family: Erode; src: url(${font('Erode-VariableItalic.woff2')}); font-weight: 300 700; font-style: italic; }
@font-face { font-family: Tabular; src: url(${font('Tabular-Regular.woff2')}); }
html, body { margin: 0; }
body { width: 1200px; height: 630px; position: relative; overflow: hidden; color: #fff;
  font-family: Erode, Georgia, serif; -webkit-font-smoothing: antialiased;
  background: #000 url(${pathToFileURL(frame).href}) 70% 50% / cover; }
.v { position: absolute; inset: 0; background: linear-gradient(90deg,
  rgba(0,0,0,.95) 0, rgba(0,0,0,.85) 42%, rgba(0,0,0,.35) 70%, rgba(0,0,0,.1) 90%); }
.t { position: absolute; left: 76px; top: 70px; bottom: 70px; display: flex; flex-direction: column; }
.brand { display: flex; align-items: center; gap: 14px; font-weight: 600; font-size: 30px; letter-spacing: -.01em; }
.brand span { font-family: Tabular; font-size: 16px; font-weight: 400; padding-top: 4px; }
h1 b { color: #fff; font-weight: 600; }
h1 { margin: auto 0 0; font-weight: 600; font-size: 74px; line-height: 1.0; letter-spacing: -.024em; max-width: 12ch; }
p { margin: 30px 0 0; font-family: Tabular; font-size: 19px; letter-spacing: .01em; }
</style>
<div class="v"></div>
<div class="t">
  <div class="brand">${mark}mossflux</div>
  <h1>Exact similarity search at billions of molecules per second.</h1>
  <p>Similarity and SMARTS search · Rust · REST, gRPC, MCP</p>
</div>`;

const icon = `<!doctype html><style>html,body{margin:0;background:#000}</style>
<svg viewBox="0 0 20 20" width="180" height="180"><rect width="20" height="20" fill="#000"/>
<circle cx="10" cy="10" r="5" fill="none" stroke="#fff" stroke-width="2"/></svg>`;

const browser = await chromium.launch({ channel: process.env.PLAYWRIGHT_CHANNEL });
const page = await browser.newPage({ viewport: { width: 1200, height: 630 } });
await fs.writeFile(path.join(tmp, 'card.html'), card);
await page.goto(pathToFileURL(path.join(tmp, 'card.html')).href);
await page.evaluate(() => document.fonts.ready);
await page.screenshot({ path: path.join(site, 'og.jpg'), type: 'jpeg', quality: 86 });

await page.setViewportSize({ width: 180, height: 180 });
await fs.writeFile(path.join(tmp, 'icon.html'), icon);
await page.goto(pathToFileURL(path.join(tmp, 'icon.html')).href);
await page.screenshot({ path: path.join(site, 'apple-touch-icon.png') });
await browser.close();
await fs.rm(tmp, { recursive: true, force: true });

for (const f of ['og.jpg', 'apple-touch-icon.png']) {
  const { size } = await fs.stat(path.join(site, f));
  console.log(`site/${f}: ${(size / 1024).toFixed(0)} KiB`);
}
