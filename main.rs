// Lets start our contribution by creating first a terminal emulator using ratatui
use ratatui::{DefaultTerminal, Terminal};
use color_eyre::eyre::Result; // this is the error handling library we will be using
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    //initialising the terminal 
    let mut terminal = ratatui::init();

    // calling the app function to run the GUI 
    app(terminal);
    Ok(())

}

fn app(mut terminal: DefaultTerminal) -> Result<()> {
    terminal.draw(|frame| {
        ui::terminal::ui(frame);
    })

    Ok(())
}   