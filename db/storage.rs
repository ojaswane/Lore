use super::schema;
use rusqlite::{Connection, Result};

// to init the db
pub fn init_db() -> Result<()> {
    // initialising the table
    let lore = Connection::open("lore.db");
}

// start the session
pub fn session_init() {}

// save the commands
pub fn save_command() {}

// end the session
pub fn end_session() {}
