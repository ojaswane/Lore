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
    let chunks = Layout::Default()
        .direction()
        .constraints([
            Constraint::Length(1), // top bar
            Constraint::Min(1),    // body
            Constraint::Length(1), // bottom bar
        ])
        .split(area);

    render_topbar(frame, chunks[0]);
    render_body(frame, chunks[1]);
    render_bottombar(frame, chunks[2]);
}

fn render_topbar(frame: &mut Frame, area: ratatui::layout::Rect) {}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect) {}

fn render_bottombar(frame: &mut Frame, area: ratatui::layout::Rect) {}

// pub fn ui(frame: &mut Frame<'_>, text: &str, cursor_pos: (u16, u16)) {
//     // we use the frame and update it for some time and it will render the TUI

//     let paragraph = Paragraph::new(text.to_string()).block(
//         Block::default()
//             .title("Lore Terminal")
//             .borders(Borders::ALL),
//     );

//     frame.render_widget(paragraph, frame.area());

//     // rendering the cursor
//     let area = frame.area();
//     let (row, col) = cursor_pos;

//     if col < area.width && row < area.height {
//         frame.set_cursor_position((area.x + col + 1, area.y + row + 1));
//     }
// }
