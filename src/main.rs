#[macro_use] mod tui;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut terminal, rx) = setup_terminal!();

    restore_terminal!(terminal);

    Ok(())
}