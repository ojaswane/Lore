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
