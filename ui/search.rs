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
    Project,
}

pub struct SearchResult {
    pub output: String,
    pub cwd: String,
    pub command: String,
    pub exit_code: i32,
    pub time_ago: String,
}
