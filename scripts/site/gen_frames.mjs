// Generate keyframes for the background loops through LLM Gateway.
//
//   LLM_API_KEY=... node scripts/site/gen_frames.mjs [theme-id ...] [--n 2]
//
// Writes out/bg/frames/<theme>-<k>.jpg (16:9, 2K). Existing files are skipped.
// Copy the best candidate to out/bg/frames/<theme>.jpg; gen_videos.mjs uses that file.

import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const OUT = path.join(here, 'out', 'bg', 'frames');
const BASE = (process.env.LLM_API_ENDPOINT || 'https://api.llmgateway.io/v1').replace(/\/$/, '');
const KEY = process.env.LLM_API_KEY;
if (!KEY) { console.error('LLM_API_KEY is not set'); process.exit(2); }

const cfg = JSON.parse(await fs.readFile(path.join(here, 'themes.json'), 'utf8'));
const args = process.argv.slice(2);
const ni = args.indexOf('--n');
const n = ni >= 0 ? +args.splice(ni, 2)[1] : 2;
const only = new Set(args);
const themes = cfg.themes.filter((t) => !only.size || only.has(t.id));
await fs.mkdir(OUT, { recursive: true });

async function one(theme, k) {
  const file = path.join(OUT, `${theme.id}-${k}.jpg`);
  try { await fs.access(file); return 0; } catch { /* generate */ }
  for (let attempt = 0; ; attempt++) {
    const res = await fetch(`${BASE}/chat/completions`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${KEY}`, 'Content-Type': 'application/json' },
      body: JSON.stringify({
        model: cfg.frameModel,
        messages: [{ role: 'user', content: `${theme.frame} ${cfg.frameStyle}` }],
        image_config: { aspect_ratio: '16:9', image_size: '2K' },
      }),
    });
    const body = await res.json().catch(() => ({}));
    const url = body.choices?.[0]?.message?.images?.[0]?.image_url?.url;
    if (res.ok && url) {
      await fs.writeFile(file, Buffer.from(url.split(',', 2)[1], 'base64'));
      const cost = body.usage?.cost ?? 0;
      console.log(`${theme.id}-${k}: saved, cost ${cost.toFixed(3)}`);
      return cost;
    }
    if (attempt >= 4) { throw new Error(`${theme.id}-${k}: ${res.status} ${JSON.stringify(body).slice(0, 300)}`); }
    await new Promise((r) => setTimeout(r, 4000 * (attempt + 1)));
  }
}

const results = await Promise.allSettled(themes.flatMap((t) => Array.from({ length: n }, (_, k) => one(t, k + 1))));
let cost = 0, failed = 0;
for (const r of results) {
  if (r.status === 'fulfilled') { cost += r.value; } else { failed++; console.error(String(r.reason)); }
}
console.log(`total cost ${cost.toFixed(2)}`);
process.exit(failed ? 1 : 0);
