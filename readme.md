<div align="center">

<img src="./assets/lore-owl.png" width="400" alt="Lore owl logo" />

# Lore

**The terminal that remembers.**

*Every command. Every output. Every session. Stored locally, forever.*

<!--
[![Rust](https://img.shields.io/badge/Rust-2024-f97316.svg)](https://www.rust-lang.org/)
[![Status: Pre--alpha](https://img.shields.io/badge/Status-Pre--alpha-fbbf24.svg)]()
[![Contributors Welcome](https://img.shields.io/badge/Contributors-Welcome-4ade80.svg)](CONTRIBUTING.md)
-->

</div>

---

## What is Lore?

Every project has a story: the commands you ran, the errors you hit, the fixes that worked, and the decisions you made late at night. Most terminals let all of that disappear the moment you close the window.

Lore doesn't.

Lore is a local terminal emulator with a memory layer. a local-first terminal that captures terminal sessions, indexes debugging history, and uses AI to retrieve how you previously solved problems.

It looks and feels like a normal terminal, but underneath it stores your sessions, commands, output, working directories, timestamps, and exit metadata in a local SQLite database.

No cloud. No account. No remote sync. Just your lore, on your machine.

---

## Why Lore?

| Problem | Most terminals | Lore |
|---|---|---|
| Session history | Lost when the terminal closes | Stored locally |
| Command output | Gone once it scrolls away | Saved with the command |
| Searching past work | Usually command history only | Search commands and output |
| Privacy | Often cloud sync or accounts | 100% local SQLite |
| AI help | Usually internet/account based | Planned local Ollama support |

---

## Current Features

- **Terminal UI** - a Ratatui-based terminal interface backed by a real shell PTY.
- **PTY shell spawning** - runs an actual shell through `portable-pty`.
- **Session storage** - creates sessions in SQLite with start/end timestamps.
- **Command storage** - saves typed commands with working directory, output snapshot, exit code field, and duration.
- **Search overlay UI** - `Ctrl+L` / `Cmd+L` opens the in-app search panel.
- **AI panel UI** - `F2` or `Ctrl+E` opens the assistant panel shell, ready for Ollama integration.

Lore is still pre-alpha. Some pieces are intentionally rough while the core loop is being shaped.

---

## In Progress

- Implementing local embedding inference for stored output chunks.
- Connecting generated embeddings to the ingest worker and SQLite.
- Building semantic search over embedded command output.
- Adding local AI explanations through Ollama.
- Adding focused tests for storage and search.

---

## AI Search / Ingest Pipeline

Lore's next major feature is a local AI search pipeline. The goal is to keep the terminal responsive while command history is captured, processed, and stored in the background.

The intended ingest flow:

```text
command finishes
       |
       v
main thread builds IngestEvent
       |
       v
single ingest worker receives event
       |
       v
worker saves command data to SQLite
       |
       v
worker chunks output -> local embedding model -> SQLite
       |
       v
semantic search embeds the query and ranks matching chunks
```

Phase 1 focuses only on the ingest boundary:

- [x] Define an `IngestEvent` type for completed commands.
- [x] Include command, directory, output, exit code, duration, timestamp, and session id.
- [x] Wire the ingest module into the project.
- [x] Create the ingest channel in `main.rs`.
- [x] Start exactly one ingest worker.
- [x] Send completed command events from `main.rs` into the worker.
- [x] Make the worker call `db::storage::save_command`.
- [x] Remove old direct-save code from the main loop.
- [x] Keep embedding dependencies out until the ingest worker is stable.
- [x] Confirm `cargo check` passes.

Phase 1 is complete: command persistence now flows through one background ingest worker instead of writing directly from the main UI loop.

Phase 2 focuses on chunking command output so future embeddings can search smaller, meaningful pieces instead of giant terminal logs:

- [x] Create `core/chunk.rs`.
- [x] Add `chunk_output(output: &str) -> Vec<String>`.
- [x] Wire the chunk module into `core/mod.rs`.
- [x] Call `chunk_output` from the ingest worker.
- [x] Replace full chunk debug output with a chunk-count log.
- [x] Remove or implement the placeholder `store_chunks` function.
- [x] Add a `chunks` table in SQLite.
- [x] Add an index on chunk timestamps.
- [x] Add a storage function for saving chunks.
- [x] Make the ingest worker save chunks after saving the full command.
- [x] Confirm `cargo check` passes without warnings.

Phase 2 is complete: completed commands are saved, output is split into chunks, and each chunk is persisted for future embedding.

Phase 3 starts local embedding support using the bundled
`all-MiniLM-L6-v2` model. Model inference and persistence are not complete yet:

- [x] Add the model configuration, tokenizer, and safetensors weights under `core/models/all-MiniLM-L6-v2/`.
- [x] Implement masked mean pooling for token embeddings.
- [x] Implement L2 normalization for sentence vectors.
- [ ] Fix and compile the model-loading and inference flow in `core/embedder.rs`.
- [ ] Return an embedding vector for each input chunk.
- [ ] Call the embedder from the ingest worker and store vectors in `chunks.embedding`.
- [ ] Embed search queries and rank stored chunks by cosine similarity.
- [ ] Verify model inference works offline after model files are present.

The model files are local and inference is intended to run on CPU without an API.
Downloading the model files initially requires internet access; embedding after
download should not.

---

## Architecture

```text
You type a command
       |
       v
  Ratatui UI
       |
       v
  PTY layer              portable-pty spawns the real shell
       |
       v
  Output reader          reads shell output in a background thread
       |
       v
  vt100 parser           converts terminal bytes into screen state
       |
       v
  Lore DB                SQLite sessions + commands + output
```

Lore is built around a normal shell running inside a pseudo-terminal. The app proxies keyboard input into the shell, reads shell output, renders the terminal screen with Ratatui, and stores session history in SQLite.

---

## Folder Structure

```text
Lore/
├── Cargo.toml              # Rust package and dependencies
├── Cargo.lock
├── main.rs                 # app entry point, event loop, mode switching
├── config.toml             # local configuration
├── lore.db                 # local SQLite database
│
├── core/
│   ├── io.rs               # PTY reader/writer setup
│   ├── pty.rs              # shell spawning through portable-pty
│   ├── state.rs            # background output reader + vt100 parser updates
│   ├── embedder.rs         # local text embedding inference (in progress)
│   ├── chunk.rs            # output chunking
│   └── models/
│       └── all-MiniLM-L6-v2/ # local config, tokenizer, and model weights
│
├── db/
│   ├── storage.rs          # SQLite session and command writes
│   ├── search.rs           # search/query layer
│   └── schema.txt          # database schema notes
│
├── ui/
│   ├── terminal.rs         # terminal renderer
│   ├── search.rs           # search panel UI
│   └── ai_panel.rs         # AI assistant panel UI
│
└── assets/
    └── lore-owl.png
```

---

## Tech Stack

- **Rust 2024** - core language.
- **Ratatui** - terminal UI rendering.
- **Crossterm** - terminal input/output backend.
- **portable-pty** - pseudo-terminal shell spawning.
- **vt100** - terminal output parsing.
- **rusqlite** - local SQLite storage.
- **chrono** - timestamps.
- **uuid** - session/id utilities.
- **Ollama** - planned local AI integration.

---

## Getting Started

> Lore is actively being built. Expect rough edges.

```bash
git clone https://github.com/ojaswane/lore.git
cd lore/Lore
cargo run
```

Inside Lore:

```text
Esc             exit Lore
Ctrl+L / Cmd+L  open search
F2 / Ctrl+E     open AI panel
```

To check that the project compiles:

```bash
cargo check
```

---

## Local Data

Lore stores data in:

```text
lore.db
```

The database currently has three main tables:

- `sessions` - one row per Lore session.
- `commands` - command text, directory, output snapshot, exit code field, and duration.
- `chunks` - split command output and optional embedding vector.

Everything stays local.

---

## Roadmap

```text
v0.1  working terminal UI with portable-pty + Ratatui
v0.2  SQLite session and command storage
v0.3  search overlay backed by real database queries
v0.4  cleaner output capture and real exit codes
v0.5  background ingest pipeline
v0.6  output chunking and chunk storage
v0.7  local AI explanations through Ollama
v0.8  local semantic search over command output
v1.0  stable local-first terminal memory
```

---

## Contributing

Lore is being built in the open, and contributions are welcome.

Good first areas:

- Wire `ui/search.rs` to `db/search.rs`.
- Add SQLite tests for `db/storage.rs`.
- Improve command/output capture.
- Record real shell exit codes.
- Build the ingest worker that stores completed command events in the background.
- Add the Ollama client for the AI panel.
- Improve terminal rendering and ANSI handling.

Before opening a PR, please read [CONTRIBUTING.md](CONTRIBUTING.md).

---

## Philosophy

**Local first.** Your data should stay on your machine. No account, no server, no cloud dependency.

**Feels like a terminal.** Lore should behave like a real terminal first. The memory layer should help without getting in the way.

**Builds over time.** The longer you use Lore, the more useful your local history becomes.

---

<div align="center">

*Close the terminal. The lore stays.*

**Built by [@ojaswane](https://github.com/ojaswane)**

</div>
