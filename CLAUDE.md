# CLAUDE.md

Read [AGENTS.md](AGENTS.md) first. This file adds rules for Claude Code.

- **Do not commit unless the user asks.** Then use a `feat:` / `fix:` /
  `chore:` message with no co-author trailer.
- **`data/` is gitignored (about 780 MB).** Never `git add` it or regenerate a
  dataset without being asked (see docs/development.md, "Datasets").
- **`HANDOFF.md` is gitignored session state.** If it has open items, reconcile
  it with `git status --short` before you act.
- **Never quote a benchmark number you did not observe.** Check `uptime` first.

Before you finish: build, test, clippy `-D warnings`, `cargo fmt --all`
(AGENTS.md, "Before you finish").

Exactness is above all else: the invariants are in
[docs/correctness.md](docs/correctness.md). If a speed-up needs one relaxed,
stop and ask.
