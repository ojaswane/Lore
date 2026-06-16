use super::schema;
use rusqlite::{Connection, Result};

// to init the db
pub fn init_db() -> Result<()> {
    // initialising the table
    let conn = Connection::open("lore.db")?;

    // create the table

    Ok(())
}

// start the session
pub fn session_init() {}

// save the commands
pub fn save_command() {}

// end the session
pub fn end_session() {}
