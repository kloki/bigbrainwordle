use std::io;

use app::App;
use data::load_words;
use entropy::brain::Brain;
use ratatui::{
    backend::CrosstermBackend,
    terminal::{Terminal, Viewport},
    TerminalOptions,
};
mod app;
mod data;
mod entropy;
mod text;
fn main() -> io::Result<()> {
    let words = load_words();
    let brain = Brain::new(words);
    let mut app = App::new(brain);
    println!("\n");

    crossterm::terminal::enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(9),
        },
    )?;

    app.run(&mut terminal)?;
    crossterm::terminal::disable_raw_mode()?;

    println!("\n\n\n\n\n");

    Ok(())
}
