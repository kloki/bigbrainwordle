use crate::entropy::brain::Word;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FeedbackType {
    Correct(char),
    WrongPosition(char),
    Wrong(char),
}

impl FeedbackType {
    pub fn block(&self) -> char {
        match self {
            FeedbackType::Correct(_) => '🟩',
            FeedbackType::WrongPosition(_) => '🟨',
            FeedbackType::Wrong(_) => '⬜',
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub enum FB {
    C,
    WP,
    W,
}

impl From<FeedbackType> for FB {
    fn from(value: FeedbackType) -> Self {
        match value {
            FeedbackType::Correct(_) => FB::C,
            FeedbackType::WrongPosition(_) => FB::WP,
            FeedbackType::Wrong(_) => FB::W,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Feedback {
    pub items: [FeedbackType; 5],
}

impl Feedback {
    pub fn new(items: [FeedbackType; 5]) -> Self {
        Feedback { items }
    }

    pub fn from_guess(guess: &Word, solution: &Word) -> Self {
        let mut answer = [FeedbackType::Wrong('a'); 5];
        let mut used_for_wrong_pos = [false; 5];
        'outer: for i in 0..5 {
            if solution[i] == guess[i] {
                answer[i] = FeedbackType::Correct(guess[i]);
            } else {
                for ii in 0..5 {
                    if !used_for_wrong_pos[ii]
                        && guess[ii] != solution[ii]
                        && guess[i] == solution[ii]
                    {
                        answer[i] = FeedbackType::WrongPosition(guess[i]);
                        used_for_wrong_pos[ii] = true;
                        continue 'outer;
                    }
                }
                answer[i] = FeedbackType::Wrong(guess[i]);
            }
        }
        Feedback::new(answer)
    }

    pub fn mask(&self) -> [FB; 5] {
        [
            self.items[0].into(),
            self.items[1].into(),
            self.items[2].into(),
            self.items[3].into(),
            self.items[4].into(),
        ]
    }

    pub fn is_correct(&self) -> bool {
        self.items.iter().all(|x| match x {
            FeedbackType::Correct(_) => true,
            _ => false,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guessing_simple() {
        let fb = Feedback::from_guess(&['m', 'o', 'd', 'e', 'm'], &['m', 'a', 'n', 'a', 's']);

        assert_eq!(
            fb.items,
            [
                FeedbackType::Correct('m'),
                FeedbackType::Wrong('o'),
                FeedbackType::Wrong('d'),
                FeedbackType::Wrong('e'),
                FeedbackType::Wrong('m'),
            ]
        );
    }

    #[test]
    fn test_guessing_simple_wrong_position() {
        let fb = Feedback::from_guess(&['q', 'o', 'd', 'e', 'm'], &['m', 'a', 'n', 'a', 's']);
        assert_eq!(
            fb.items,
            [
                FeedbackType::Wrong('q'),
                FeedbackType::Wrong('o'),
                FeedbackType::Wrong('d'),
                FeedbackType::Wrong('e'),
                FeedbackType::WrongPosition('m'),
            ]
        )
    }

    #[test]
    fn test_guessing_simple_wrong_position_duplicate() {
        let fb = Feedback::from_guess(&['q', 'o', 'd', 'm', 'm'], &['m', 'a', 'n', 'a', 's']);
        assert_eq!(
            fb.items,
            [
                FeedbackType::Wrong('q'),
                FeedbackType::Wrong('o'),
                FeedbackType::Wrong('d'),
                FeedbackType::WrongPosition('m'),
                FeedbackType::Wrong('m'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_wrong_position_duplicate() {
        let fb = Feedback::from_guess(&['a', 'x', 'a', 'x', 's'], &['m', 'a', 'n', 'a', 's']);
        assert_eq!(
            fb.items,
            [
                FeedbackType::WrongPosition('a'),
                FeedbackType::Wrong('x'),
                FeedbackType::WrongPosition('a'),
                FeedbackType::Wrong('x'),
                FeedbackType::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_some_wrong_position_duplicate() {
        let fb = Feedback::from_guess(&['m', 'a', 'a', 'x', 's'], &['m', 'a', 'n', 'a', 's']);
        assert_eq!(
            fb.items,
            [
                FeedbackType::Correct('m'),
                FeedbackType::Correct('a'),
                FeedbackType::WrongPosition('a'),
                FeedbackType::Wrong('x'),
                FeedbackType::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_switched() {
        let fb = Feedback::from_guess(&['m', 'a', 'n', 's', 'a'], &['m', 'a', 'n', 'a', 's']);
        assert_eq!(
            fb.items,
            [
                FeedbackType::Correct('m'),
                FeedbackType::Correct('a'),
                FeedbackType::Correct('n'),
                FeedbackType::WrongPosition('s'),
                FeedbackType::WrongPosition('a'),
            ]
        )
    }
}
