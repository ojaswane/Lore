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

fn render_topbar(frame: &mut Frame, area: ratatui::layout::Rect) {
    let line = Line::from(vec![
        Span::styled("● ", Style::default().fg(Color::Green)),
        Span::styled("Memory Active", Style::default().fg(Color::White)),
        // version of the app
        Span::raw("                                                "),
        Span::styled("V 0.1.0", Style::default().fg(Color::White)),
    ]);

    //rendering the line
    frame.render_widget(Paragraph::new(line), area)
}

fn render_body(frame: &mut Frame, area: ratatui::layout::Rect) {}

fn render_bottombar(frame: &mut Frame, area: ratatui::layout::Rect) {}

// Ui for main terminal
pub fn ui(frame: &mut Frame<'_>, text: &str, cursor_pos: (u16, u16)) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // terminal body
            Constraint::Length(1), // status bar
        ])
        .split(frame.area());

    // terminal output
    frame.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::White)),
        chunks[0],
    );

    // cursor
    let (row, col) = cursor_pos;
    if col < chunks[0].width && row < chunks[0].height {
        frame.set_cursor_position((chunks[0].x + col, chunks[0].y + row));
    }
}
