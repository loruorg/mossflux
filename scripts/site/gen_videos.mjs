// Generate the background loops through LLM Gateway.
//
//   LLM_API_KEY=... node scripts/site/gen_videos.mjs [theme-id ...] [--model id] [--seconds n] [--size WxH|none] [--audio] [--ref]
//
// Each clip starts and ends on out/bg/frames/<theme>.jpg, so it loops without a seam.
// Job ids go to out/bg/jobs.json the moment a job is accepted: a rerun resumes
// polling and never pays for the same clip twice. Clips land in
// out/bg/<theme>.<model>.mp4 and are skipped on later runs.

import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const OUT = path.join(here, 'out', 'bg');
const JOBS = path.join(OUT, 'jobs.json');
const BASE = (process.env.LLM_API_ENDPOINT || 'https://api.llmgateway.io/v1').replace(/\/$/, '');
const KEY = process.env.LLM_API_KEY;
if (!KEY) { console.error('LLM_API_KEY is not set'); process.exit(2); }

const cfg = JSON.parse(await fs.readFile(path.join(here, 'themes.json'), 'utf8'));
const args = process.argv.slice(2);
const opt = (name, def) => { const i = args.indexOf(name); return i >= 0 ? args.splice(i, 2)[1] : def; };
const model = opt('--model', cfg.videoModel);
const seconds = +opt('--seconds', cfg.seconds);
// Seedance takes the ratio from the first frame and rejects an explicit size: --size none.
const size = opt('--size', cfg.size);
const flag = (name) => { const i = args.indexOf(name); return i >= 0 && !!args.splice(i, 1); };
// --audio: for models that only make clips with sound (the encoder drops it).
const audio = flag('--audio');
// --ref: send the keyframe as a style reference instead of first and last frame.
const ref = flag('--ref');
const only = new Set(args);
const themes = cfg.themes.filter((t) => !only.size || only.has(t.id));

// Several runs (one per model) may share jobs.json, so every write re-reads it.
async function loadJobs() {
  try { return JSON.parse(await fs.readFile(JOBS, 'utf8')); } catch { return {}; }
}
async function setJob(key, val) {
  const all = await loadJobs();
  if (val) { all[key] = val; } else { delete all[key]; }
  await fs.writeFile(JOBS, JSON.stringify(all, null, 2));
}

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

async function run(theme) {
  const key = `${theme.id}.${model}${ref ? '.ref' : ''}`;
  const file = path.join(OUT, `${key}.mp4`);
  try { await fs.access(file); console.log(`${key}: have it`); return; } catch { /* generate */ }

  let job = (await loadJobs())[key];
  if (!job) {
    const frame = `data:image/jpeg;base64,${(await fs.readFile(path.join(OUT, 'frames', `${theme.id}.jpg`))).toString('base64')}`;
    const prompt = ref
      ? `${theme.frame} ${theme.motion} Match the look of the reference image. ${cfg.frameStyle} ${cfg.motionStyle}`
      : `${theme.motion} ${cfg.motionStyle}`;
    const res = await api('POST', '/videos', {
      model, prompt, seconds, ...(size === 'none' ? {} : { size }), audio,
      ...(ref ? { reference_images: [{ image_url: frame }] } : { image: { image_url: frame }, last_frame: { image_url: frame } }),
    });
    job = { id: (await res.json()).id, model, seconds, prompt, submitted: new Date().toISOString() };
    await setJob(key, job);
    console.log(`${key}: submitted ${job.id}`);
  }

  for (;;) {
    const st = await (await api('GET', `/videos/${job.id}`)).json();
    if (st.status === 'completed') {
      job.cost = st.usage?.cost ?? null;
      await setJob(key, job);
      break;
    }
    if (['failed', 'canceled', 'expired'].includes(st.status)) {
      await setJob(key, null);
      throw new Error(`${key}: ${st.status} ${JSON.stringify(st.error)}`);
    }
    console.log(`${key}: ${st.status} ${st.progress ?? ''}`);
    await new Promise((r) => setTimeout(r, 20000));
  }

  const bytes = Buffer.from(await (await api('GET', `/videos/${job.id}/content`)).arrayBuffer());
  await fs.writeFile(file, bytes);
  console.log(`${key}: saved ${(bytes.length / 1e6).toFixed(1)} MB, cost ${job.cost}`);
}

const results = await Promise.allSettled(themes.map(run));
let failed = 0;
for (const r of results) if (r.status === 'rejected') { failed++; console.error(String(r.reason)); }
process.exit(failed ? 1 : 0);
