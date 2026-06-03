// This will generate the GUI for the terminal.
use ratatui::{
    Frame,
    widgets::Paragraph,
};

pub fn ui(frame: &mut Frame<'_>) { // we use the frame and update it for some time and it will render the TUI 
    let text = Paragraph::new("Hello Loree ");

    frame.render_widget(
        text,
        frame.area(),
    );
}
