use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use crate::core::engine::Comment;

pub struct TuiState<'a> {
    query: String,
    results: Vec<&'a Comment<'a>>,
    selected: usize,
    should_quit: bool,
}

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // setup
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_tui_loop(&mut terminal);

    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_tui_loop(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<(), Box<dyn std::error::Error>> {
    
}