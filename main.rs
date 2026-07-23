// Lets start our contribution by creating first a terminal emulator using ratatui
use crate::core::{io::system_io, pty::shell, state::output_shell};
use anyhow::Result; // this is the error handling library we will be using
use crossterm::event::{self, Event, KeyCode, KeyModifiers}; // this is the library we will be using to handle the events of the terminal
use crossterm::{cursor, execute};
use ratatui::DefaultTerminal;
use reqwest::Request;
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

fn is_app_shortcut(modifiers: KeyModifiers) -> bool {
    modifiers.contains(KeyModifiers::SUPER) || modifiers.contains(KeyModifiers::CONTROL)
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
    db::storage::end_session(&conn, session_id)?;
    ratatui::restore();

    Ok(())
}

// TODO : ADD Ollama implementations
fn app(mut terminal: DefaultTerminal, conn: &rusqlite::Connection, session_id: i64) -> Result<()> {
    let (master, _child) = shell()?;

    // this is the ANSI parser to parse the output from shell into the text
    let parser = Arc::new(Mutex::new(vt100::Parser::new(24, 80, 0)));

    let (reader, mut writer) = system_io(master.as_ref())?;
    let _handle = output_shell(reader, parser.clone());

    //checking if the user idle or typing for cursor animation
    let mut last_key_node = std::time::Instant::now();
    let idle_min = std::time::Duration::from_millis(500);
    let mut is_idle = true;

    let mut mode = AppMode::Terminal;
    let mut search_state = ui::search::SearchState {
        query: String::new(),
        results: vec![],
        selected: 0,
        filter: ui::search::Filter::All,
    };
    let ai_state = ui::ai_panel::AiState {
        context: String::new(),
        explanation: String::new(),
        fix: String::new(),
        what_it_does: String::new(),
    };

    // Helpers to add save the commands
    let mut current_command = String::new();
    let mut command_started_at: Option<std::time::Instant> = None;

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
                ui::ai_panel::ui(frame, &current_text, cursor_pos, &ai_state);
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
                        KeyCode::Char('l') if is_app_shortcut(key.modifiers) => {
                            mode = AppMode::Search;
                        }
                        KeyCode::Char('e') if is_app_shortcut(key.modifiers) => {
                            mode = AppMode::AiPanel;

                            // tigger Ai request
                            let response = core::ai::ai::explain_error(
                                &last_command,
                                &last_output,
                                last_exit_code,
                            )
                            .await;

                            ai_state.context =
                                format!("{} → exit {}", last_command, last_exit_code);
                            ai_state.explanation = response.explanation;
                            ai_state.fix = response.fix;
                            ai_state.what_it_does = response.what_it_does;
                        }
                        KeyCode::Char(c) => {
                            if command_started_at.is_none() {
                                command_started_at = Some(std::time::Instant::now());
                            }
                            current_command.push(c);

                            write!(writer, "{c}")?;
                            writer.flush()?;
                        }
                        KeyCode::Enter => {
                            write!(writer, "\r")?;
                            writer.flush()?;

                            let command = std::mem::take(&mut current_command);

                            if !command.is_empty() {
                                let duration_ms = command_started_at
                                    .map(|t| t.elapsed().as_millis() as i64)
                                    .unwrap_or(0);
                                let dir = std::env::current_dir()
                                    .map(|path| path.display().to_string())
                                    .unwrap_or_else(|_| String::from("."));

                                db::storage::save_command(
                                    conn,
                                    session_id,
                                    &command,
                                    &dir,
                                    &current_text,
                                    0,
                                    duration_ms,
                                )?;
                            }

                            command_started_at = None;
                        }
                        KeyCode::Backspace => {
                            // handelling pop
                            current_command.pop();

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
                    AppMode::Search => match key.code {
                        KeyCode::Esc => {
                            mode = AppMode::Terminal;
                        }
                        KeyCode::Up => {
                            if search_state.selected > 0 {
                                search_state.selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if search_state.selected < search_state.results.len().saturating_sub(1)
                            {
                                search_state.selected += 1;
                            }
                        }
                        KeyCode::Char(c) => {
                            search_state.query.push(c);
                        }
                        KeyCode::Backspace => {
                            search_state.query.pop();
                        }
                        KeyCode::Tab => {
                            search_state.filter = search_state.filter.next();
                        }
                        _ => {}
                    },
                    AppMode::AiPanel => match key.code {
                        KeyCode::Esc => {
                            mode = AppMode::Terminal;
                        }
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
                terminal.draw(|frame| match mode {
                    AppMode::Terminal => {
                        ui::terminal::ui(frame, &current_text, cursor_pos);
                    }
                    AppMode::Search => {
                        ui::terminal::ui(frame, &current_text, cursor_pos);
                        ui::search::ui(frame, &search_state);
                    }
                    AppMode::AiPanel => {
                        ui::ai_panel::ui(frame, &current_text, cursor_pos, &ai_state);
                    }
                })?;
            }
        }
    }

    Ok(())
}
