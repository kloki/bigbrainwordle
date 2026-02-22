use app::App;
use clap::Parser;
use data::load_words;
use entropy::brain::{Brain, Word};
use ratatui::{TerminalOptions, Viewport};
use std::io;

mod app;
mod data;
mod entropy;
mod text;

#[derive(serde::Deserialize)]
struct NytResponse {
    solution: String,
}

/// A cli tool that helps you solve the daily wordle
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// A 5-letter word to autosolve against
    #[arg(long, conflicts_with_all = ["nyt_today", "nyt_date"])]
    autosolve: Option<String>,

    /// Fetch today's NYT puzzle and autosolve
    #[arg(long, conflicts_with_all = ["autosolve", "nyt_date"])]
    nyt_today: bool,

    /// Fetch a specific date's NYT puzzle and autosolve (format: YYYY-MM-DD)
    #[arg(long, conflicts_with_all = ["autosolve", "nyt_today"])]
    nyt_date: Option<String>,
}

fn fetch_nyt_solution(date: &str) -> Result<Word, Box<dyn std::error::Error>> {
    let url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", date);
    let body = ureq::get(&url).call()?.into_string()?;
    let response: NytResponse = serde_json::from_str(&body)?;
    let chars: Vec<char> = response.solution.chars().collect();
    if chars.len() != 5 {
        return Err("Solution is not 5 characters".into());
    }
    Ok([chars[0], chars[1], chars[2], chars[3], chars[4]])
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let words = load_words();
    let brain = Brain::new(words);

    let solution = if let Some(word) = cli.autosolve {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() != 5 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Autosolve word must be exactly 5 letters",
            ));
        }
        Some([chars[0], chars[1], chars[2], chars[3], chars[4]])
    } else if cli.nyt_today {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        eprintln!("Fetching today's NYT Wordle...");
        Some(fetch_nyt_solution(&date).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to fetch NYT puzzle: {}", e),
            )
        })?)
    } else if let Some(date) = cli.nyt_date {
        eprintln!("Fetching NYT Wordle for {}...", date);
        Some(fetch_nyt_solution(&date).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to fetch NYT puzzle: {}", e),
            )
        })?)
    } else {
        None
    };

    let mut app = App::new(brain);

    println!("\n");

    let mut terminal = ratatui::try_init_with_options(TerminalOptions {
        viewport: Viewport::Inline(9),
    })?;

    if let Some(solution) = solution {
        app.run_autosolve(solution, &mut terminal)?;
    } else {
        app.run(&mut terminal)?;
    }

    ratatui::try_restore()?;

    println!("\n\n\n\n\n");

    Ok(())
}
