// Render the share card (site/og.jpg, 1200x630) and site/apple-touch-icon.png.
//
//   node scripts/site/make_share.mjs [theme] [seconds]
//
// Needs ffmpeg and a global Playwright install (npm i -g playwright). The card
// is a frame of one source clip (default: moss at 3.5 s) with the headline set
// in the site's own fonts, so it matches the page it links to.

import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { createRequire } from 'node:module';
import { execFileSync, execSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const site = path.join(here, '..', '..', 'site');
const [theme = 'moss', at = '3.5'] = process.argv.slice(2);
const require = createRequire(import.meta.url);
const { chromium } = require(path.join(execSync('npm root -g').toString().trim(), 'playwright'));

const tmp = await fs.mkdtemp(path.join(os.tmpdir(), 'mossflux-share-'));
const frame = path.join(tmp, 'frame.png');
execFileSync('ffmpeg', ['-v', 'error', '-y', '-ss', at, '-i',
  path.join(here, 'out', `${theme}-l.mp4`), '-frames:v', '1', frame]);

const font = (f) => pathToFileURL(path.join(site, 'fonts', f)).href;
const mark = `<svg viewBox="0 0 20 20" width="30" height="30"><rect x="0" y="4" width="20" height="2" fill="#DDE4D7"/><rect x="0" y="14" width="20" height="2" fill="#DDE4D7"/><rect x="9.5" y="7" width="1" height="6" fill="#D2BC93"/></svg>`;

const card = `<!doctype html><meta charset="utf-8"><style>
@font-face { font-family: Erode; src: url(${font('Erode-Variable.woff2')}); font-weight: 300 700; }
@font-face { font-family: Erode; src: url(${font('Erode-VariableItalic.woff2')}); font-weight: 300 700; font-style: italic; }
@font-face { font-family: Tabular; src: url(${font('Tabular-Regular.woff2')}); }
html, body { margin: 0; }
body { width: 1200px; height: 630px; position: relative; overflow: hidden; color: #DDE4D7;
  font-family: Erode, Georgia, serif; -webkit-font-smoothing: antialiased;
  background: #0E120F url(${pathToFileURL(frame).href}) 70% 50% / cover; }
.v { position: absolute; inset: 0; background: linear-gradient(90deg,
  rgba(8,10,9,.9) 0, rgba(8,10,9,.78) 34%, rgba(8,10,9,.35) 60%, rgba(8,10,9,0) 80%); }
.t { position: absolute; left: 76px; top: 70px; bottom: 70px; display: flex; flex-direction: column; }
.brand { display: flex; align-items: center; gap: 14px; font-weight: 600; font-size: 30px; letter-spacing: -.01em; }
.brand span { font-family: Tabular; font-size: 16px; font-weight: 400; padding-top: 4px; }
h1 { margin: auto 0 0; font-weight: 600; font-size: 74px; line-height: 1.0; letter-spacing: -.024em; max-width: 12ch; }
p { margin: 30px 0 0; font-family: Tabular; font-size: 19px; letter-spacing: .01em; }
</style>
<div class="v"></div>
<div class="t">
  <div class="brand">${mark}mossflux <span>0.1.0</span></div>
  <h1>Every fast path has a <em>slow</em> twin that checks it.</h1>
  <p>Exact Tanimoto and SMARTS search · Rust</p>
</div>`;

const icon = `<!doctype html><style>html,body{margin:0;background:#0E120F}</style>
<svg viewBox="0 0 20 20" width="180" height="180"><rect width="20" height="20" fill="#0E120F"/>
<rect x="3.5" y="5" width="13" height="1.6" fill="#DDE4D7"/><rect x="3.5" y="13.4" width="13" height="1.6" fill="#DDE4D7"/>
<rect x="9.5" y="7.6" width="1" height="4.8" fill="#AE8F61"/></svg>`;

const browser = await chromium.launch();
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
