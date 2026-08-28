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

// Ingest runs in a background worker thread, so database writes do not block
// the main UI thread. The main thread sends completed command events through
// a channel, and the ingest worker receives them and writes them to SQLite.

// TODO :
// Keep saving the full command in commands like you already do.
// Add a chunking function that splits output into smaller pieces.
// Later, those chunks will go into a separate chunks table for embeddings.

pub fn ingest_worker(rx: std::sync::mpsc::Receiver<IngestEvent>) {
    // adding a connection with the db
    let conn = crate::db::storage::init_db().expect("Failed to initialize database");

    // process events in a loop
    while let Ok(event) = rx.recv() {
        match event {
            IngestEvent::CommandFinished {
                session_id,
                command,
                dir,
                timestamp: _timestamp,
                output,
                exit_code,
                duration_ms,
            } => {
                // calls the chunking function to split the output into smaller chunks
                let chunks = crate::core::chunk::chunk_output(&output);
                println!("Chunks: {}", chunks.len());

                // let _ = crate::core::chunk::store_chunks(&chunks);

                // Insert the command into the database
                if let Err(e) = crate::db::storage::save_command(
                    &conn,
                    session_id,
                    &command,
                    &dir,
                    &output,
                    exit_code,
                    duration_ms,
                ) {
                    eprintln!("Failed to insert command into database: {:?}", e);
                }
            }
            IngestEvent::Shutdown => {
                break; // Exit the loop and terminate the thread
            }
        }
    }
}
