// Lets start our contribution by creating first a terminal emulator using ratatui
use ratatui::{DefaultTerminal, Terminal};
use color_eyre::eyre::Result; // this is the error handling library we will be using
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Application code goes here ˝
    // ratatui::run(app);
    
    app();
    Ok(())
}

fn app() {
    ui::terminal::ui(); // this should print hello lore to terminal 
}