# Contributing to Lore

Thanks for wanting to help build Lore.

Lore is a local-first terminal memory tool: a terminal that remembers the commands you ran, the output they produced, the projects they belonged to, and the fixes that worked. The goal is not to become another generic terminal emulator. The goal is to make terminal work easier to search, understand, and continue later.

Lore is still pre-alpha, so rough edges are expected. Good contributions should make the core loop more reliable:

```text
run command -> capture what happened -> search it later -> reuse the fix
```

---

## Project Status

The current app is written in Rust and uses:

- `ratatui` for the terminal UI
- `crossterm` for input/output events
- `portable-pty` for spawning a real shell
- `vt100` for terminal output parsing
- `rusqlite` for local SQLite storage

Some parts are already started:

- PTY-backed terminal UI
- SQLite sessions and command storage
- search overlay UI
- AI panel UI shell

Some parts are still incomplete:

- real database-backed search results
- clean per-command output capture
- real exit code capture
- project-aware history
- failure-to-fix memory
- local AI explanations through Ollama

---

## Before You Start

If you are proposing a larger feature, open an issue first so the direction can be discussed.

Keep Lore local-first. Contributions should not require accounts, cloud sync, telemetry, or remote services for the core app to work.

---

## Getting Set Up

Fork and clone the repository:

```bash
git clone https://github.com/yourusername/lore.git
cd lore/Lore
```

Check that the project builds:

```bash
cargo check
```

Run Lore locally:

```bash
cargo run
```

Inside Lore:

```text
Esc             exit Lore
Ctrl+L / Cmd+L  open search
Ctrl+E / Cmd+E  open AI panel
```

---

## How To Contribute

### 1. Choose a focused change

Good pull requests usually do one thing:

- fix one bug
- add one small feature
- improve one part of the UI
- add tests for one module
- update one piece of documentation

Avoid mixing unrelated refactors with feature work.

### 2. Create a branch

```bash
git checkout -b feature/search-results
# or
git checkout -b fix/session-count
```

### 3. Make the change

Follow the current module boundaries:

- `main.rs` handles app startup, event routing, and mode switching.
- `core/` contains PTY, IO, shell output, and AI-related logic.
- `db/` contains SQLite storage and search logic.
- `ui/` contains Ratatui rendering code.

### 4. Test it

At minimum, run:

```bash
cargo check
```

If you touch formatting-sensitive code, also run:

```bash
cargo fmt
```

If tests exist for the area you changed, run:

```bash
cargo test
```

Also run the app manually when changing terminal behavior, keyboard shortcuts, storage, or UI rendering.

### 5. Open a pull request

In the PR description, explain:

- what changed
- why it changed
- how you tested it
- any known limitations

---

## Good First Issues

If you are new to the project, these are useful places to start:

- Implement `db/search.rs` with SQLite queries.
- Wire the search overlay to real database results.
- Replace the hardcoded status bar command count with real session data.
- Add focused tests for `db/storage.rs`.
- Update `db/schema.txt` so it matches the actual SQLite schema.
- Improve README accuracy as features evolve.
- Make the default shell configurable instead of hardcoded.

---

## Higher-Impact Areas

These are larger and may need discussion before implementation:

- Cleanly capture output for each command.
- Record real command exit codes.
- Detect the current project from `.git`, `Cargo.toml`, `package.json`, and similar files.
- Add a project-aware command timeline.
- Detect when a later command likely fixed an earlier failure.
- Add session recaps.
- Add local Ollama-based explanations using stored command context.

---

## Code Style

- Prefer clear, small functions.
- Keep UI rendering code in `ui/`.
- Keep database reads and writes in `db/`.
- Keep shell, PTY, and output processing in `core/`.
- Use `anyhow::Result` for app-level fallible operations.
- Do not add new dependencies without a clear reason.
- Add comments only where they explain non-obvious behavior.

Run formatting before submitting:

```bash
cargo fmt
```

---

## Product Principles

Lore should be:

- **Local first**: user data stays on the machine.
- **Keyboard friendly**: common workflows should not require the mouse.
- **Memory oriented**: features should help users remember, understand, or reuse terminal work.
- **Terminal first**: the app should still feel like a real terminal.
- **Quietly useful**: AI should explain real terminal context, not become generic chat.

When unsure whether a feature belongs, ask:

```text
Does this help someone recover context, understand a failure, or reuse a fix?
```

If yes, it probably fits Lore.

---

## What We Are Not Looking For Right Now

- cloud sync
- telemetry
- account systems
- Electron or web-based rewrites
- generic chatbot features
- terminal-theme-only changes
- large plugin systems before the core memory loop is solid

---

## Questions

Open an issue and tag it `question`.

No question is too small. Lore is early, and clear discussion is part of shaping the product.

---

*Close the terminal. The lore stays.*
