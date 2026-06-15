// database schemas to store the data at rustql

struct Schema {
    id: i64,
    session_id: i64,
    timestamp: i64,
    command: String, // this is where the command will be stored
    dir: String,     // The directory will be stored
    output: String,
    error: bool,
    exit_code: i32,
    duration_ms: i64, // how long the command took to run , useful for AI context
    shell: String,    // zsh / bash / fish matters when replaying commands
}

// This is usefull for "show me everything I did last Tuesday in the lore project"
struct Session {
    id: i64,
    started_at: i64,     // unix timestamp
    ended_at: i64,       // so you know session duration
    project: String,     // inferred from dir — "lore", "hellorust" etc
    total_commands: i32, // quick stat without counting rows
}

pub fn init_db() {}

pub fn session_init() {}

pub fn save_command() {}

pub fn end_session() {}
