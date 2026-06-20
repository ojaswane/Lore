use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub struct SearchState {
    pub query: String,
}

pub enum Filter {
    All,
    Today,
    ThisWeek,
    Errors,
    Project,
}
