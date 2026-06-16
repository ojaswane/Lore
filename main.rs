// Lets start our contribution by creating first a terminal emulator using ratatui
use crate::core::{io::system_io, pty::shell, state::output_shell};
use anyhow::Result; // this is the error handling library we will be using
use crossterm::event::{self, Event, KeyCode}; // this is the library we will be using to handle the events of the terminal
use ratatui::DefaultTerminal;
use std::io::Write;
use std::sync::{Arc, Mutex};

mod core;
mod db;
mod ui;

// app starts
// → init_db()
// → start_session()
// → loop
//     → user types command
//     → command runs
//     → capture output + exit code + duration
//     → save_command()
// → user exits
// → end_session()
// NOTE , THIS IS FOR BETTER UNDERSTANDING

fn main() -> Result<()> {
    // Initializing the db
    let conn = db::storage::init_db()?;
    let session_id = db::storage::session_init(&conn, "lore")?;

    //initialising the terminal
    let terminal = ratatui::init(); // crossterm is a backed for ratatui which can support windows

    // calling the app function to run the GUI
    app(terminal, &conn, session_id)?;

    // end session
    let end_session = db::storage::end_session(conn, session_id)?;
    ratatui::restore();

    Ok(())
}

fn app(mut terminal: DefaultTerminal, conn: &rusqlite::Connection, session_id: i64) -> Result<()> {
    let (master, _child) = shell()?;
    // let output = Arc::new(Mutex::new(String::new()));

    // this is the ANSI parser to parse the output from shell into the text
    let parser = Arc::new(Mutex::new(vt100::Parser::new(24, 80, 0)));

    let (reader, mut writer) = system_io(master.as_ref())?;
    let _handle = output_shell(reader, parser.clone());

    loop {
        let (current_text, cursor_pos) = {
            let parser_lock = parser.lock().unwrap();
            let screen = parser_lock.screen();
            let text = screen.rows(0, 80).collect::<Vec<String>>().join("\n");
            let (crow, ccol) = screen.cursor_position();
            (text, (crow, ccol))
        };

        terminal.draw(|frame| {
            ui::terminal::ui(frame, &current_text, cursor_pos);
        })?;

        // to match the events (To match the keys to be pressed)

        if event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        write!(writer, "{c}")?;
                        writer.flush()?;
                    }

                    KeyCode::Enter => {
                        write!(writer, "\r")?; // \r instead of \n for PTY
                        writer.flush()?;
                    }

                    KeyCode::Backspace => {
                        writer.write_all(&[127])?; // 127 = DEL, better than 8 for most shells
                        writer.flush()?;
                    }

                    KeyCode::Tab => {
                        write!(writer, "\t")?;
                        writer.flush()?;
                    }

                    KeyCode::Up => {
                        writer.write_all(b"\x1b[A")?;
                        writer.flush()?;
                    }

                    KeyCode::Down => {
                        writer.write_all(b"\x1b[B")?;
                        writer.flush()?;
                    }

                    KeyCode::Left => {
                        writer.write_all(b"\x1b[D")?;
                        writer.flush()?;
                    }

                    KeyCode::Right => {
                        writer.write_all(b"\x1b[C")?;
                        writer.flush()?;
                    }
                    KeyCode::Esc => break,

                    _ => {}
                }

                // Print immediately after keypress
                // heres how exactly this works : after typing the key the thread goes to sleep for 5 ms until the text is echoed into the shell
                std::thread::sleep(std::time::Duration::from_millis(5)); // wait for shell to echo back
                let (current_text, cursor_pos) = {
                    let parser_lock = parser.lock().unwrap();
                    let screen = parser_lock.screen();
                    let text = screen.rows(0, 80).collect::<Vec<String>>().join("\n");
                    let (crow, ccol) = screen.cursor_position();
                    (text, (crow, ccol))
                };
                terminal.draw(|frame| {
                    ui::terminal::ui(frame, &current_text, cursor_pos);
                })?;
            }
        }
    }

    Ok(())
}
