// The main goal here is to handle shell output in a background thread,
// so blocking reads from the shell do not freeze the main thread.
// This keeps the main thread free to handle UI rendering, user input,
// and other application logic.

// Architecture:
// main thread
//-> captures finished command
//-> sends IngestEvent

// one ingest worker thread
//-> receives events
//-> writes to SQLite

pub enum IngestEvent {
    CommandFinished {
        session_id: i64,
        command: String,
        dir: String,
        timestamp: i64,
        output: String,
        exit_code: i32,
        duration_ms: i64,
    },
    Shutdown,
}

pub fn ingest_worker() {}
