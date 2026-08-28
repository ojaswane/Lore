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
