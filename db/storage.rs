use rusqlite::{Connection, Result, params};

// to init the db
pub fn init_db() -> Result<Connection> {
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

        CREATE TABLE IF NOT EXISTS chunks (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            command_id  INTEGER NOT NULL,
            command     TEXT NOT NULL,
            cwd         TEXT,
            exit_code   INTEGER DEFAULT 0,
            timestamp   INTEGER NOT NULL,
            output      TEXT,
            embedding   BLOB,
            FOREIGN KEY (command_id) REFERENCES commands(id)
        );

        CREATE INDEX IF NOT EXISTS idx_chunks_timestamp ON chunks(timestamp);
    ",
    )?;

    Ok(conn)
}

// start the session
pub fn session_init(conn: &Connection, project: &str) -> Result<i64> {
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO sessions (started_at, project) VALUES (?1, ?2)",
        params![now, project],
    )?;

    Ok(conn.last_insert_rowid()) // returns the session_id
}

// save the commands into db
pub fn save_command(
    conn: &Connection,
    session_id: i64,
    command: &str,
    dir: &str,
    output: &str,
    exit_code: i32,
    duration_ms: i64,
) -> Result<i64> {
    let now = chrono::Utc::now().timestamp();
    let error = if exit_code != 0 { 1 } else { 0 };

    conn.execute(
        "INSERT INTO commands
            (session_id, timestamp, command, dir, output, error, exit_code, duration_ms)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            session_id,
            now,
            command,
            dir,
            output,
            error,
            exit_code,
            duration_ms
        ],
    )?;

    Ok(conn.last_insert_rowid())
}
// end the session
pub fn end_session(conn: &Connection, session_id: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "UPDATE sessions SET ended_at = ?1,
         total_commands = (SELECT COUNT(*) FROM commands WHERE session_id = ?2)
         WHERE id = ?2",
        params![now, session_id],
    )?;

    Ok(())
}

pub fn save_chunk(
    conn: &Connection,
    command_id: i64,
    command: &str,
    cwd: &str,
    exit_code: i32,
    output: &str,
) -> Result<()> {
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO chunks 
            (command_id, command, cwd, exit_code, timestamp, output)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![command_id, command, cwd, exit_code, now, output],
    )?;

    Ok(())
}
