// This will generate the GUI for the terminal.
use ratatui::{
    Widget::Paragraph,
    Frame,
}

pub fn ui(frame: &mut Frame){
    let text = Paragraph::new("Hello Loree ");

    frame.render_widget(
        text,
        frame.area()
    );
};
