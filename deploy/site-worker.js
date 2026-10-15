// mossflux.com runs this Worker for /bg/* only (wrangler.jsonc, run_worker_first).
// Workers static assets answer a Range request with the whole file and 200;
// Safari plays video only from 206 responses, so the Worker cuts the range.

export default {
  async fetch(req, env) {
    const res = await env.ASSETS.fetch(req);
    const m = /^bytes=(\d*)-(\d*)$/.exec(req.headers.get('range') || '');
    if (req.method !== 'GET' || res.status !== 200 || !m || (m[1] === '' && m[2] === '')) { return res; }

    const buf = await res.arrayBuffer();
    const size = buf.byteLength;
    let start = +m[1], end = m[2] === '' ? size - 1 : Math.min(+m[2], size - 1);
    if (m[1] === '') { start = Math.max(0, size - +m[2]); end = size - 1; }
    if (start >= size || start > end) {
      return new Response(null, { status: 416, headers: { 'content-range': `bytes */${size}` } });
    }
    const h = new Headers(res.headers);
    h.set('accept-ranges', 'bytes');
    h.set('content-range', `bytes ${start}-${end}/${size}`);
    h.set('content-length', String(end - start + 1));
    return new Response(buf.slice(start, end + 1), { status: 206, headers: h });
  },
};
