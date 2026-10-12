// Generate the landing-page background videos through LLM Gateway.
//
//   LLM_API_KEY=... node scripts/site/gen_videos.mjs [theme-id ...] [--orient l|p]
//
// One landscape (1920x1080) and one portrait (1080x1920) clip per theme in
// themes.json, no audio. Job ids are written to out/jobs.json the moment a job
// is accepted, so a rerun resumes polling and never pays for the same clip twice.
// Finished clips land in out/<theme>-<l|p>.mp4 and are skipped on later runs.

import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const OUT = path.join(here, 'out');
const JOBS = path.join(OUT, 'jobs.json');
const BASE = (process.env.LLM_API_ENDPOINT || 'https://api.llmgateway.io/v1').replace(/\/$/, '');
const KEY = process.env.LLM_API_KEY;
if (!KEY) { console.error('LLM_API_KEY is not set'); process.exit(2); }

const SIZES = { l: '1920x1080', p: '1080x1920' };
const cfg = JSON.parse(await fs.readFile(path.join(here, 'themes.json'), 'utf8'));

const args = process.argv.slice(2);
const oi = args.indexOf('--orient');
const orients = oi >= 0 ? [args.splice(oi, 2)[1]] : Object.keys(cfg.framing);
const only = new Set(args);
const themes = cfg.themes.filter((t) => !only.size || only.has(t.id));

await fs.mkdir(OUT, { recursive: true });
let jobs = {};
try { jobs = JSON.parse(await fs.readFile(JOBS, 'utf8')); } catch { /* first run */ }
const saveJobs = () => fs.writeFile(JOBS, JSON.stringify(jobs, null, 2));

async function api(method, url, body) {
  for (let attempt = 0; ; attempt++) {
    const res = await fetch(BASE + url, {
      method,
      headers: { Authorization: `Bearer ${KEY}`, 'Content-Type': 'application/json' },
      body: body ? JSON.stringify(body) : undefined,
    });
    if (res.ok) return res;
    const text = await res.text();
    if ((res.status === 429 || res.status >= 500) && attempt < 6) {
      await new Promise((r) => setTimeout(r, 5000 * (attempt + 1)));
      continue;
    }
    throw new Error(`${method} ${url} -> ${res.status}: ${text.slice(0, 400)}`);
  }
}

async function run(theme, o) {
  const key = `${theme.id}-${o}`;
  const file = path.join(OUT, `${key}.mp4`);
  try { await fs.access(file); console.log(`${key}: have it`); return; } catch { /* generate */ }

  if (!jobs[key]) {
    const prompt = [theme.prompt, cfg.framing[o], cfg.style].join(' ');
    const res = await api('POST', '/videos', {
      model: cfg.model, prompt, seconds: cfg.seconds, size: SIZES[o], audio: false,
    });
    const job = await res.json();
    jobs[key] = { id: job.id, prompt, size: SIZES[o], submitted: new Date().toISOString() };
    await saveJobs();
    console.log(`${key}: submitted ${job.id}`);
  }

  const id = jobs[key].id;
  for (;;) {
    const job = await (await api('GET', `/videos/${id}`)).json();
    if (job.status === 'completed') {
      jobs[key].cost = job.usage?.cost ?? null;
      await saveJobs();
      break;
    }
    if (['failed', 'canceled', 'expired'].includes(job.status)) {
      delete jobs[key];
      await saveJobs();
      throw new Error(`${key}: ${job.status} ${JSON.stringify(job.error)}`);
    }
    process.stdout.write(`${key}: ${job.status} ${job.progress ?? ''}\n`);
    await new Promise((r) => setTimeout(r, 15000));
  }

  const bytes = Buffer.from(await (await api('GET', `/videos/${id}/content`)).arrayBuffer());
  await fs.writeFile(file, bytes);
  console.log(`${key}: saved ${(bytes.length / 1e6).toFixed(1)} MB, cost ${jobs[key].cost}`);
}

const results = await Promise.allSettled(
  themes.flatMap((t) => orients.map((o) => run(t, o))),
);
let failed = 0;
for (const r of results) if (r.status === 'rejected') { failed++; console.error(String(r.reason)); }
process.exit(failed ? 1 : 0);
