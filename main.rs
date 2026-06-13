// Lets start our contribution by creating first a terminal emulator using ratatui
use crate::core::{io::system_io, pty::shell, state::output_shell};
use anyhow::Result; // this is the error handling library we will be using
use crossterm::event::{self, Event, KeyCode}; // this is the library we will be using to handle the events of the terminal
use ratatui::DefaultTerminal;
use std::io::Write;
use std::sync::{Arc, Mutex};

mod core;
mod ui;

fn main() -> Result<()> {
    //initialising the terminal
    let terminal = ratatui::init(); // crossterm is a backed for ratatui which can support windows

    // calling the app function to run the GUI
    app(terminal)?;
    Ok(())
}

fn app(mut terminal: DefaultTerminal) -> Result<()> {
    let (master, _child) = shell()?;
    let output = Arc::new(Mutex::new(String::new()));
    let (reader, mut writer) = system_io(master.as_ref())?;
    let _handle = output_shell(reader, output.clone());

    loop {
        let mut current_text = output.lock().unwrap().clone(); // This will draw the Ui to show onto the terminal and show the output as we want
        terminal.draw(|frame| {
            ui::terminal::ui(frame, &mut current_text);
        })?;

        // to match the events (To match the keys to be pressed)
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    write!(writer, "{c}")?;
                    writer.flush()?;
                }

                KeyCode::Enter => {
                    write!(writer, "\n")?;
                    writer.flush()?;
                }

                KeyCode::Backspace => {
                    writer.write_all(&[8])?;
                    writer.flush()?;
                }

                KeyCode::Esc => {
                    break;
                }

                _ => {}
            }
        }
    }

    Ok(())
}
