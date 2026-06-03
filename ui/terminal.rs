// This will generate the GUI for the terminal.
use ratatui::{
    Frame,
    widgets::Paragraph,
};

pub fn ui(frame: &mut Frame<'_>) {
    let text = Paragraph::new("Hello Loree ");

    frame.render_widget(
        text,
        frame.area(),
    );
}
