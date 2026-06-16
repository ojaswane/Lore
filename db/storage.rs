use super::schema;
use rusqlite::{Connection, Result};

// to init the db
pub fn init_db() -> Result<()> {
    // initialising the table
    let conn = Connection::open("lore.db")?;

    // create the table
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS sessions (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            started_at    INTEGER NOT NULL,
            ended_at      INTEGER,
            project       TEXT,
            total_commands INTEGER DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS commands (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id  INTEGER NOT NULL,
            timestamp   INTEGER NOT NULL,
            command     TEXT NOT NULL,
            dir         TEXT NOT NULL,
            output      TEXT,
            error       INTEGER DEFAULT 0,
            exit_code   INTEGER DEFAULT 0,
            duration_ms INTEGER,
            FOREIGN KEY (session_id) REFERENCES sessions(id)
        );
    ",
    )?;

    Ok(())
}

// start the session
pub fn session_init() {}

// save the commands
pub fn save_command() {}

// end the session
pub fn end_session() {}
