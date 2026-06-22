// Ai pannel will be activated when user will press cmd+e or ctrl+e (to explain the error)
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub struct AiState {
    pub context: String,
    pub explanation: String,
    pub fix: String,
    pub what_it_does: String,
}

pub fn ui(frame: &mut Frame, text: &str, cursor_pos: (u16, u16), ai_state: &AiState) {
    // split terminal and ai panel
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(1),     // terminal
            Constraint::Length(45), // ai panel
        ])
        .split(frame.area());

    crate::ui::terminal::ui(frame, text, cursor_pos); // creates the ui for the terminal from terminal.rs
    // ai panel on right
    render_ai_panel(frame, chunks[1], ai_state);
}

fn render_ai_panel(frame: &mut Frame, area: Rect, state: &AiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // header
            Constraint::Length(4), // context block
            Constraint::Min(1),    // explanation + fix
            Constraint::Length(3), // ask input
        ])
        .split(area);

    // header
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" ● ", Style::default().fg(Color::Rgb(167, 139, 250))),
            Span::styled("lore ai", Style::default().fg(Color::Rgb(180, 180, 200))),
            Span::raw("                    "),
            Span::styled(" × ", Style::default().fg(Color::Rgb(80, 80, 100))),
        ]))
        .style(Style::default().bg(Color::Rgb(14, 14, 20))),
        chunks[0],
    );

    // context
    frame.render_widget(
        Paragraph::new(vec![
            Line::from(Span::styled(
                "  CONTEXT",
                Style::default().fg(Color::Rgb(80, 80, 100)),
            )),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    &state.context,
                    Style::default().fg(Color::Rgb(248, 113, 113)),
                ),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(40, 40, 55))),
        )
        .style(Style::default().bg(Color::Rgb(18, 14, 28))),
        chunks[1],
    );

    // explanation + fix
    let body_lines = vec![
        Line::from(Span::styled(
            "  EXPLANATION",
            Style::default().fg(Color::Rgb(124, 58, 237)),
        )),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                &state.explanation,
                Style::default().fg(Color::Rgb(200, 200, 220)),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  SUGGESTED FIX",
            Style::default().fg(Color::Rgb(74, 222, 128)),
        )),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                &state.fix,
                Style::default()
                    .fg(Color::Rgb(74, 222, 128))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                " run it ",
                Style::default()
                    .fg(Color::Rgb(74, 222, 128))
                    .bg(Color::Rgb(20, 40, 25)),
            ),
            Span::raw("  "),
            Span::styled(
                " copy ",
                Style::default()
                    .fg(Color::Rgb(80, 80, 100))
                    .bg(Color::Rgb(25, 25, 35)),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  WHAT THIS DOES",
            Style::default().fg(Color::Rgb(124, 58, 237)),
        )),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                &state.what_it_does,
                Style::default().fg(Color::Rgb(160, 160, 180)),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(body_lines).style(Style::default().bg(Color::Rgb(14, 14, 20))),
        chunks[2],
    );

    // ask input
    frame.render_widget(
        Paragraph::new(Line::from(vec![Span::styled(
            " ask anything about this...",
            Style::default().fg(Color::Rgb(60, 60, 80)),
        )]))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(40, 40, 55))),
        )
        .style(Style::default().bg(Color::Rgb(14, 14, 20))),
        chunks[3],
    );
}
