// Lets start our contribution by creating first a terminal emulator using ratatui
use crate::core::{io::system_io, pty::shell, state::output_shell};
use anyhow::Result; // this is the error handling library we will be using
use crossterm::event::{self, Event, KeyCode}; // this is the library we will be using to handle the events of the terminal
use crossterm::{cursor, execute};
use ratatui::DefaultTerminal;
use std::io::Write;
use std::io::stdout;
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

// app state to track which screen is showing
enum AppMode {
    Terminal,
    Search,
    AiPanel,
}

fn main() -> Result<()> {
    //cursor rendering from crossterm backend
    execute!(stdout(), cursor::SetCursorStyle::BlinkingBar)?;
    // Initializing the db
    let conn = db::storage::init_db()?;
    let session_id = db::storage::session_init(&conn, "lore")?;

    //initialising the terminal
    let terminal = ratatui::init(); // crossterm is a backed for ratatui which can support windows

    // calling the app function to run the GUI
    app(terminal, &conn, session_id)?;

    // end session
    execute!(stdout(), cursor::SetCursorStyle::DefaultUserShape)?;
    db::storage::end_session(conn, session_id)?;
    ratatui::restore();

    Ok(())
}

// TODO : ADD the Db inserting and deletion logic
fn app(mut terminal: DefaultTerminal, conn: &rusqlite::Connection, session_id: i64) -> Result<()> {
    let (master, _child) = shell()?;
    // let output = Arc::new(Mutex::new(String::new()));

    // this is the ANSI parser to parse the output from shell into the text
    let parser = Arc::new(Mutex::new(vt100::Parser::new(24, 80, 0)));

    let (reader, mut writer) = system_io(master.as_ref())?;
    let _handle = output_shell(reader, parser.clone());

    //checking if the user idle or typing for cursor animation
    let mut last_key_node = std::time::Instant::now();
    let idle_min = std::time::Duration::from_millis(500);
    let mut is_idle = true;

    let mut mode = AppMode::Terminal;

    loop {
        let (current_text, cursor_pos) = {
            let parser_lock = parser.lock().unwrap();
            let screen = parser_lock.screen();
            let text = screen.rows(0, 80).collect::<Vec<String>>().join("\n");
            let (crow, ccol) = screen.cursor_position();
            (text, (crow, ccol))
        };

        terminal.draw(|frame| match mode {
            AppMode::Terminal => {
                ui::terminal::ui(frame, &current_text, cursor_pos);
            }
            AppMode::Search => {
                ui::terminal::ui(frame, &current_text, cursor_pos);
                ui::search::ui(frame, &search_state);
            }
            AppMode::AiPanel => {
                ui::ai::ui(frame, &current_text, cursor_pos, &ai_state);
            }
        })?;

        if !is_idle && last_key_node.elapsed() > idle_min {
            is_idle = true;
            let _ = execute!(stdout(), cursor::SetCursorStyle::BlinkingBar);
        }

        // to match the events (To match the keys to be pressed)

        if event::poll(std::time::Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                // went from idle to typing
                if is_idle {
                    is_idle = false;
                    execute!(stdout(), cursor::SetCursorStyle::SteadyBlock)?;
                }
                last_key_node = std::time::Instant::now();

                match mode {
                    AppMode::Terminal => match key.code {
                        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            mode = AppMode::Search;
                        }
                        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            mode = AppMode::AiPanel;
                        }
                        KeyCode::Char(c) => {
                            write!(writer, "{c}")?;
                            writer.flush()?;
                        }
                        KeyCode::Enter => {
                            write!(writer, "\r")?;
                            writer.flush()?;
                        }
                        KeyCode::Backspace => {
                            writer.write_all(&[127])?;
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
                    },
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
