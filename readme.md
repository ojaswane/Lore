<div align="center">

<!-- logo goes here -->

<img src="./assets/lore-owl.png" width="400" alt="Lore — owl with a big brain" />

# Lore

**The terminal that remembers.**

*Every command. Every output. Every session. Stored locally, forever.*

<!-- [![Python 3.11+](https://img.shields.io/badge/Python-3.11+-a78bfa.svg)](https://python.org)
[![Status: Building](https://img.shields.io/badge/Status-Building-fbbf24.svg)]()
[![Contributors Welcome](https://img.shields.io/badge/Contributors-Welcome-4ade80.svg)](CONTRIBUTING.md) -->

</div>
---

## What is Lore?

Every project has a story the commands you ran, the errors you hit, the fixes that worked, the decisions you made at 2am. Most terminals let all of that disappear the moment you close the window.

Lore doesn't.

Lore is a terminal emulator that stores the **lore of your work** session by session, project by project. It looks and feels like a regular terminal. But underneath, it's building a searchable, compressed, local database of everything you've ever done.

No cloud. No account. No creepiness. Just your lore, on your machine.

---

## Why Lore?

| Problem | Every other terminal | Lore |
|---|---|---|
| Session history | Lost on close | Stored forever |
| Command output | Gone | Saved + compressed |
| Searching past work | `grep ~/.zsh_history` (commands only) | Full semantic search |
| Privacy | Cloud sync (Warp) or nothing | 100% local, SQLite |
| Storage | Unlimited bloat | Smart tiered compression |
| AI features | Requires internet + account | Local LLM via Ollama |

---

## Features (building now)

- **Session storage** — every session saved with timestamps, working directory, exit codes, and full output
- **Smart compression** — tiered zlib compression by age. Your entire year of terminal history fits in under 100MB
- **`lore search`** — search across all past sessions by keyword, project, date, or exit code
- **Feels like a real terminal** — no blocks, no sidebars, no startup screens. just a clean terminal with a memory
- **Local AI (coming soon)** — Ollama integration for private, offline error explanation and command suggestions
- **Automation layer (coming soon)** — schedule tasks, post things, book appointments, get notified when done

---

## Architecture

```
You type a command
       ↓
  PTY layer          ←  ptyprocess (spawns the real shell)
       ↓
  Interceptor        ←  captures command + output + metadata
      / \
     /   \
 Shell   Lore DB     ←  SQLite (local, compressed, searchable)
     \   /
      \ /
   Renderer          ←  Textual UI (looks like a normal terminal)
```

---

## The system design

Lore is built as a local terminal emulator with a memory layer that lives alongside the shell. The system design focuses on capturing commands and output in real time, storing them in a compressed SQLite database, and rendering the terminal UI so it feels like a normal shell.

- `PTY layer` spawns the real shell and proxies stdin/stdout.
- `Interceptor` captures commands, outputs, metadata, and sends them to storage.
- `Lore DB` stores sessions, commands, outputs, and exit codes in a local SQLite file.
- `Renderer` displays terminal output with no additional UI chrome.

View the full design diagram here:

https://excalidraw.com/#json=F041fX85vcvojEIwq7Yqc,3KJ5kW-5muuxaq9lSvPfDQ

---

## Folder structure

```
lore/
├── main.rs                  # entry point
├── config.toml              # user settings (theme, storage limit)
├── requirements.txt
│
├── core/
│   ├── state.rs             # Prints the result
│   ├── pty.rs               # spawns a pty 
│   └── io.rs                # read the output from shell
│
├── db/
│   ├── storage.rs           # rusqlite — sessions, commands, output, exit codes
│   └── search.rs           # query by keyword, date, project, exit code
│
└── ui/
    ├── terminal.rs          # textual app, input handling, output display
    └── history.rs          # lore search panel, session viewer
```

---

## Tech stack

- **Rust** — core language
- **portable-pty** — PTY shell spawning
- **ratatui** — terminal UI framework
- **rusqlite** — local session storage (built into Python, zero setup)
- **flate2** — compression
- **reqwest (HTTP calls to Ollama API)** (coming soon) — local AI integration

---

## Getting started

> Lore is actively being built. This is a pre-alpha. Things will break.

```bash
# clone the repo
git clone https://github.com/ojaswane/lore.git
cd lore

# # install dependencies

# run lore
cargo run main.rs
```

---

## Roadmap

```
v0.1  ✦  working terminal (portable-pty shell + ratatui UI)
v0.2  ✦  session storage (rusqlite, commands + output)
v0.3  ✦  smart compression (flate2, tiered by age)
v0.4  ✦  lore search (search past sessions)
v0.5  ✦  local AI via ollama (error explanation, suggestions)
v0.6  ✦  automation layer (scheduling, notifications)
v1.0  ✦  stable release
```

---

## Contributing

Lore is being built in the open and contributors are genuinely welcome — whether you're fixing bugs, building features, improving docs, or just trying it out and filing issues.

**Good first issues to pick up:**

- `good first issue` — tagged issues for new contributors
- improving the compression logic in `core/compressor.py`
- adding keyboard shortcuts to the Textual UI
- writing tests for the SQLite storage layer
- improving ANSI escape code handling

**To contribute:**

```bash
# fork the repo, then
git checkout -b feature/your-feature-name
# make your changes
git commit -m "what you did"
git push origin feature/your-feature-name
# open a pull request
```

Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a PR.

---

## Philosophy

Lore is built on three principles:

**Local first.** Your data never leaves your machine. No account, no server, no cloud sync. Everything is in a SQLite file on your disk that you can inspect, back up, or delete at any time.

**Feels like a terminal.** No sidebars. No blocks. No AI chat bubbles. Lore looks and behaves like a real terminal — because it is one. The memory layer is invisible until you need it.

**Builds over time.** The longer you use Lore, the more useful it gets. Your history becomes searchable context. Your patterns become suggestions. Your lore becomes yours.

---

---

<div align="center">

*Close the terminal. The lore stays.*

**Built by [@ojaswane](https://github.com/ojaswane)**

</div>
