use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

pub fn welcome() {
    let area = Frame.area();

    // spliting the terminal into 3 parts , top , body and bottom
    let chunks = Layout::Default().directioan();
}

pub fn ui(frame: &mut Frame<'_>, text: &str, cursor_pos: (u16, u16)) {
    // we use the frame and update it for some time and it will render the TUI

    let paragraph = Paragraph::new(text.to_string()).block(
        Block::default()
            .title("Lore Terminal")
            .borders(Borders::ALL),
    );

    frame.render_widget(paragraph, frame.area());

    // rendering the cursor
    let area = frame.area();
    let (row, col) = cursor_pos;

    if col < area.width && row < area.height {
        frame.set_cursor_position((area.x + col + 1, area.y + row + 1));
    }
}
