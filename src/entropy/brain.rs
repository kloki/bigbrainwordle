use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::entropy::{
    feedback::{Feedback, FeedbackType},
    solver::suggest_word,
};

pub struct Brain {
    pub options: Vec<Word>,
    pub valid: Vec<Word>,
}

pub type Word = [char; 5];

const OPENER: Word = ['t', 'a', 'r', 'e', 's'];

impl Brain {
    pub fn new(data_set: Vec<Word>) -> Self {
        Self {
            options: data_set.clone(),
            valid: data_set,
        }
    }

    pub fn suggest(&self, last_round: bool) -> Result<Word, &'static str> {
        if self.options.is_empty() {
            return Err("No possible anwers?");
        }
        if self.options.len() == 1 {
            return Ok(self.options[0]);
        }
        if last_round {
            return Ok(*self
                .options
                .choose(&mut rand::thread_rng())
                .expect("No possible anwers?"));
        }
        if self.options.contains(&OPENER) {
            return Ok(OPENER);
        }
        Ok(suggest_word(&self.options, &self.valid))
    }

    pub fn done(&self) -> bool {
        self.options.len() == 1
    }

    pub fn prune(&mut self, feedback: Feedback) {
        // These character should be in this posistion or not
        let mut correct_chars: Vec<(char, usize)> = Vec::new();
        let mut excluded_chars: Vec<(char, usize)> = Vec::new();

        // Wrong guesses do not only restrict their own posistion
        // else they would be "wrongprosition"
        let mut excluded_chars_infered: Vec<char> = Vec::new();
        let mut exclude_mask: Vec<usize> = Vec::new();

        // Correct and Wrong positiong count minimal amount of characters of the
        // same type in the answer
        let mut counts: HashMap<char, usize> = HashMap::with_capacity(5);

        for (i, guess) in feedback.items.iter().enumerate() {
            match guess {
                FeedbackType::Correct(c) => {
                    correct_chars.push((*c, i));
                    let char_count = counts.get(c).unwrap_or(&0);
                    counts.insert(*c, char_count + 1);
                }
                FeedbackType::WrongPosition(c) => {
                    excluded_chars.push((*c, i));
                    exclude_mask.push(i);
                    let char_count = counts.get(c).unwrap_or(&0);
                    counts.insert(*c, char_count + 1);
                }
                FeedbackType::Wrong(c) => {
                    excluded_chars.push((*c, i));
                    excluded_chars_infered.push(*c);
                    exclude_mask.push(i)
                }
            }
        }

        self.options = self
            .options
            .iter()
            .filter(|x| {
                for (c, i) in correct_chars.iter() {
                    if *c != x[*i] {
                        return false;
                    }
                }

                for (c, i) in excluded_chars.iter() {
                    if *c == x[*i] {
                        return false;
                    }
                }

                for c in excluded_chars_infered.iter() {
                    if *counts.get(c).unwrap_or(&0) == 0 {
                        for i in exclude_mask.iter() {
                            if *c == x[*i] {
                                return false;
                            }
                        }
                    }
                }
                for (c, count) in counts.iter() {
                    let x_count = x.iter().filter(|xc| *xc == c).count();
                    if x_count < *count {
                        return false;
                    }
                }
                true
            })
            .copied()
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_non_correct() {
        let mut brain = Brain::new(vec![['a', 'a', 'a', 'a', 'b'], ['b', 'a', 'a', 'a', 'b']]);
        brain.prune(Feedback::new([
            FeedbackType::Correct('a'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
        ]));
        assert_eq!(brain.options.len(), 1)
    }
    #[test]
    fn test_remove_non_excluded() {
        let mut brain = Brain::new(vec![['a', 'a', 'x', 'a', 'b'], ['a', 'a', 'a', 'a', 'b']]);
        brain.prune(Feedback::new([
            FeedbackType::Correct('a'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
        ]));
        assert_eq!(brain.options.len(), 1)
    }
    #[test]
    fn test_remove_exclude() {
        let mut brain = Brain::new(vec![['a', 'a', 'x', 'a', 'b'], ['a', 'b', 'b', 'a', 'b']]);
        brain.prune(Feedback::new([
            FeedbackType::Correct('a'),
            FeedbackType::WrongPosition('a'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
        ]));
        assert_eq!(brain.options.len(), 1)
    }
    #[test]
    fn test_wrong_posistion_conflict() {
        let mut brain = Brain::new(vec![
            ['b', 'a', 'n', 'n', 'n'],
            ['b', 'n', 'n', 'a', 'n'],
            ['b', 'n', 'a', 'n', 'n'],
            ['b', 'n', 'a', 'n', 'x'],
        ]);
        brain.prune(Feedback::new([
            FeedbackType::Correct('b'),
            FeedbackType::WrongPosition('a'),
            FeedbackType::Wrong('a'),
            FeedbackType::Wrong('x'),
            FeedbackType::Wrong('x'),
        ]));
        assert_eq!(brain.options.len(), 1)
    }
}
