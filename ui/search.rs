// search will be activated when event key is cmd+s or ctrl+s
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub struct SearchState {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub selected: usize,
    pub filter: Filter,
}

pub enum Filter {
    All,
    Today,
    ThisWeek,
    Errors,
    Project, // showing which project are currently working on
}

pub struct SearchResult {
    pub output: String,
    pub cwd: String,
    pub command: String,
    pub exit_code: i32,
    pub time_ago: String,
}

pub fn ui(frame: &mut Frame, state: &SearchState) {
    let area = frame.area();

    // overlay pannel
    let panel = centered_rect(85, 75, area);

    // clear the background behind panel
    frame.render_width(Clear, panel);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // search bar
            Constraint::Length(3), // filter chips
            Constraint::Min(1),    // results
            Constraint::Length(2), // footer hints
        ])
        .split(panel);

    render_search_bar(frame, chunks[0], &state.query);
    render_filters(frame, chunks[1], &state.filter);
    render_results(frame, chunks[2], &state.results, state.selected);
    render_search_footer(frame, chunks[3], state.results.len());
}

fn render_search_bar(frame: &mut Frame, area: Rect, query: &str) {
    let line = Line::from(vec![
        Span::styled(" 🔍 ", Style::default().fg(Color::Rgb(80, 80, 100))),
        Span::styled(
            query,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "█",
            Style::default() // cursor
                .fg(Color::Rgb(124, 58, 237)),
        ),
        Span::raw("          "),
        Span::styled(
            " esc ",
            Style::default()
                .fg(Color::Rgb(80, 80, 100))
                .bg(Color::Rgb(30, 30, 40)),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(line).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(50, 50, 70)))
                .style(Style::default().bg(Color::Rgb(15, 15, 22))),
        ),
        area,
    );
}

fn render_filters(frame: &mut Frame, area: Rect, filters: &Filter) {
    let chips = vec!["all", "today", "this week", "errors", "~/lore/Lore"];
    let mut spans = vec![Span::raw("        ")];

    for chip in chips {
        let is_active = matches!(
            (chip, filters),
            ("all", Filter::All)
                | ("today", Filter::Today)
                | ("this week", Filter::ThisWeek)
                | ("errors", Filter::Errors)
        );

        let style = if is_active {
            Style::default()
                .fg(Color::Rgb(167, 139, 250))
                .bg(Color::Rgb(40, 20, 80))
        } else {
            Style::default().fg(Color::Rgb(80, 80, 100))
        };

        spans.push(Span::styled(format!(" {} ", chip), style));
        spans.push(Span::raw("  "));
    }

    frame.render_widget(
        Paragraph::new(spans).style(Style::default().bg(Color::Rgb(12, 12, 18))),
        area,
    )
}

fn render_results(frame: &mut Frame, area: Rect, result: &[SearchResult], selected: usize) {
    let mut lines: Vec<Line> = vec![];

    for (i, result) in result.iter().enumerate() {
        let is_selected = i == selected;

        let bg = if is_selected {
            Color::Rgb(25, 20, 45)
        } else {
            Color::Rgb(12, 12, 18)
        };

        let exit_style = if result.exit_code == 0 {
            Style::default()
                .fg(Color::Rgb(74, 222, 128))
                .bg(Color::Rgb(20, 40, 25))
        } else {
            Style::default()
                .fg(Color::Rgb(248, 113, 113))
                .bg(Color::Rgb(40, 15, 15))
        };

        let exit_label = if result.exit_code == 0 {
            format!(" exit {} ", result.exit_code)
        } else {
            format!(" exit {} ", result.exit_code)
        };

        // command line
        lines.push(Line::from(vec![
            Span::raw(if is_selected { "▌ " } else { "  " }),
            Span::styled(
                &result.command,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
                    .bg(bg),
            ),
            Span::raw("    "),
            Span::styled(exit_label, exit_style),
            Span::raw("  "),
            Span::styled(
                &result.time_ago,
                Style::default().fg(Color::Rgb(80, 80, 100)),
            ),
        ]));

        // output preview
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(&result.output, Style::default().fg(Color::Rgb(80, 80, 100))),
        ]));

        // dir
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(&result.dir, Style::default().fg(Color::Rgb(60, 60, 80))),
        ]));

        lines.push(Line::from(""));
    }

    frame.render_widget(
        Paragraph::new(lines).style(Style::default().bg(Color::Rgb(12, 12, 18))),
        area,
    );
}

fn render_search_footer(frame: &mut Frame, area: Rect, result: usize) {
    let line = Line::from(vec![
        Span::styled(
            " ↑↓ ",
            Style::default()
                .fg(Color::Rgb(80, 80, 100))
                .bg(Color::Rgb(25, 25, 35)),
        ),
        Span::styled(" navigate  ", Style::default().fg(Color::Rgb(60, 60, 80))),
        Span::styled(
            " ↵ ",
            Style::default()
                .fg(Color::Rgb(80, 80, 100))
                .bg(Color::Rgb(25, 25, 35)),
        ),
        Span::styled(" run again  ", Style::default().fg(Color::Rgb(60, 60, 80))),
        Span::styled(
            " tab ",
            Style::default()
                .fg(Color::Rgb(80, 80, 100))
                .bg(Color::Rgb(25, 25, 35)),
        ),
        Span::styled(" copy", Style::default().fg(Color::Rgb(60, 60, 80))),
        Span::raw("                              "),
        Span::styled(
            format!("{} results", count),
            Style::default().fg(Color::Rgb(60, 60, 80)),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(Color::Rgb(12, 12, 18))),
        area,
    );
}
