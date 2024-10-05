use app::App;
use data::load_words;
use entropy::brain::Brain;
use ratatui::{TerminalOptions, Viewport};
use std::io;

mod app;
mod data;
mod entropy;
mod text;
fn main() -> io::Result<()> {
    let words = load_words();
    let brain = Brain::new(words);
    let mut app = App::new(brain);
    println!("\n");

    let mut terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(9),
    })?;

    app.run(&mut terminal)?;

    ratatui::try_restore()?;

    println!("\n\n\n\n\n");

    Ok(())
}
