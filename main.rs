// Lets start our contribution by creating first a terminal emulator using ratatui
use ratatui::DefaultTerminal;
use crossterm::event::{self , Event , KeyCode}; // this is the library we will be using to handle the events of the terminal
use anyhow::Result; // this is the error handling library we will be using
use crate::core::{io::system_io, pty::shell, state::output_shell};
use std::sync::{Arc, Mutex};

mod ui;
mod core;


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
    let mut text = String::new();
    let (reader, _writer) = system_io(master.as_ref())?;
    let _handle = output_shell(reader, output.clone());

    loop{
        // This will draw the Ui to show onto the terminal and show the output as we want
        terminal.draw(|frame| {
            ui::terminal::ui(frame , &mut text);
        })?;

        // to match the events (To match the keys to be pressed)
        if let Event::Key(key) = event::read()? {
            match key.code {
                // add the character to the text
                KeyCode::Char('c') => {
                    text.push('c');
                }

                //remove the charecter 
                KeyCode::Backspace => {
                    text.pop();
                }

                //exit app
                KeyCode::Esc => {
                    break;
                }

                //rest , remain same
                _ => {}
            }
        }

    }

    Ok(())
}   