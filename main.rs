// Lets start our contribution by creating first a terminal emulator using ratatui
use ratatui::DefaultTerminal;
use color_eyre::eyre::Result; // this is the error handling library we will be using
use crossterm::event::{self , Event , KeyCode}; // this is the library we will be using to handle the events of the terminal
mod ui;
mod core;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    //initialising the terminal 
    let terminal = ratatui::init(); // crossterm is a backed for ratatui which can support windows

    // calling the app function to run the GUI 
    app(terminal)?;
    Ok(())

}

fn app(mut terminal: DefaultTerminal) -> Result<()> {

    let mut text = String::new();

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