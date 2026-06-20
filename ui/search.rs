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

pub fn ui(frame: &mut Frame, state: &SearchState) {}
