use data::load_words;

use crate::entropy::{
    brain::{Brain, Word},
    feedback::{Feedback, FeedbackType},
};
mod data;
mod entropy;
pub fn print_emoji(result: &Feedback) {
    print!("          ");
    for i in result.items.iter() {
        match i {
            FeedbackType::Correct(_) => print!("🟩",),
            FeedbackType::WrongPosition(_) => print!("🟨"),
            FeedbackType::Wrong(_) => print!("⬜"),
        }
    }
    println!(" ");
}

pub fn get_use_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_feedback(guess: Word) -> Feedback {
    println!("Enter feedback('_/y/g'): ");
    let raw = get_use_input().chars().collect::<Vec<char>>();
    if raw.len() != 5 {
        println!("FeedbackType must be 5 characters long");
        return get_feedback(guess);
    }
    let items: [FeedbackType; 5] = guess
        .iter()
        .enumerate()
        .map(|(i, c)| match raw[i] {
            'g' => FeedbackType::Correct(*c),
            'y' => FeedbackType::WrongPosition(*c),
            _ => FeedbackType::Wrong(*c),
        })
        .collect::<Vec<FeedbackType>>()
        .try_into()
        .expect("invalid feedback");
    Feedback::new(items)
}

fn main() {
    let words = load_words();
    let mut brain = Brain::new(words);

    println!("Wordle Helper\n");
    for i in 0..6 {
        let guess = brain.suggest(i == 5).unwrap();
        if brain.options.len() == 1 {
            println!(
                "Word found: {}",
                brain.options[0].iter().collect::<String>()
            );
            break;
        } else if i == 5 {
            println!("Last guess: {}", guess.iter().collect::<String>());
        } else {
            println!("Suggestion: {}\n", guess.iter().collect::<String>());
        }

        if i != 5 {
            println!("\n");
            let feedback = get_feedback(guess);
            print_emoji(&feedback);
            brain.prune(feedback);
        }
    }
}
