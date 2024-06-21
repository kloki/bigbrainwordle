use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Terminal,
    widgets::{Paragraph, Widget, Wrap},
    Frame,
};

use crate::entropy::{
    brain::Brain,
    feedback::{Feedback, FeedbackType},
};

#[derive(PartialEq, Eq)]
pub enum AppState {
    Playing,
    Won,
    Lost,
    Failed,
}

pub struct App {
    brain: Brain,
    row: usize,
    column: usize,
    pub feedbacks: [[Option<FeedbackType>; 5]; 6],
    current: [char; 5],
    state: AppState,
}

impl App {
    pub fn new(brain: Brain) -> Self {
        let current = brain.suggest(false).expect("No words to suggest");
        Self {
            brain,
            row: 0,
            column: 0,
            feedbacks: [[None; 5]; 6],
            current,
            state: AppState::Playing,
        }
    }
    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> io::Result<()> {
        loop {
            term.draw(|f| self.draw(f))?;
            if let Event::Key(key) = event::read()? {
                match (key.code, self.row, self.column) {
                    (KeyCode::Char('q'), _, _) => return Ok(()),
                    (KeyCode::Esc, _, _) => return Ok(()),
                    (KeyCode::Char('g'), r, c) if c < 5 => {
                        self.feedbacks[r][c] = Some(FeedbackType::Correct(self.current[c]));
                        self.column += 1;
                    }
                    (KeyCode::Char('y'), r, c) if c < 5 => {
                        self.feedbacks[r][c] = Some(FeedbackType::WrongPosition(self.current[c]));
                        self.column += 1;
                    }
                    (KeyCode::Char(' '), r, c) if c < 5 => {
                        self.feedbacks[r][c] = Some(FeedbackType::Wrong(self.current[c]));
                        self.column += 1;
                    }
                    (KeyCode::Backspace, r, c) if c > 0 => {
                        self.feedbacks[r][c - 1] = None;
                        self.column -= 1;
                    }
                    (KeyCode::Enter, r, 5) if r < 5 => {
                        self.process_feedback();
                        self.column = 0;
                        self.row += 1;
                    }
                    _ => {}
                }
            }

            if self.state != AppState::Playing {
                //draw again for the last time
                term.draw(|f| self.draw(f))?;
                return Ok(());
            }
        }
    }

    pub fn process_feedback(&mut self) {
        let feedback = Feedback::new([
            self.feedbacks[self.row][0].unwrap(),
            self.feedbacks[self.row][1].unwrap(),
            self.feedbacks[self.row][2].unwrap(),
            self.feedbacks[self.row][3].unwrap(),
            self.feedbacks[self.row][4].unwrap(),
        ]);
        if feedback.is_correct() {
            self.state = AppState::Won;
            return;
        }

        if self.row == 5 {
            self.state = AppState::Lost;
            return;
        }
        self.brain.prune(feedback);
        match self.brain.suggest(self.row == 5) {
            Ok(word) => self.current = word,
            Err(_) => self.state = AppState::Failed,
        }
        if self.brain.done() {
            self.feedbacks[self.row + 1] = [Some(FeedbackType::Correct('a')); 5];
            self.state = AppState::Won;
        }
    }

    pub fn header(&self) -> impl Widget {
        Paragraph::new(" BigBrainWordle 󰧑 ").style(Style::default().fg(Color::Green))
    }

    pub fn instuctions(&self) -> impl Widget {
        let content = match self.state {
            AppState::Playing => format!(
                "Next try: '{}'",
                self.current.iter().collect::<String>()
            ),
            AppState::Won => format!("The correct word is '{}'",
                self.current.iter().collect::<String>()
            ),
            AppState::Lost =>
                "Lost! We didn't have enough guesses".to_string(),
            AppState::Failed =>
                "None the words I know match the feedback. Either we made a mistake or the word is not in my dictionary. Please try again.".to_string(),
        };

        Paragraph::new(content).wrap(Wrap { trim: true })
    }

    pub fn board(&self) -> impl Widget {
        let board = self
            .feedbacks
            .iter()
            .map(|row| {
                row.iter()
                    .map(|feedback| feedback.map_or('⬛', |f| f.block()))
                    .collect::<String>()
            })
            .enumerate()
            .map(|(i, x)| {
                if i == self.row {
                    format!(">{}", x)
                } else {
                    format!(" {}", x)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        Paragraph::new(board)
    }
    pub fn draw(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Length(8)])
            .split(f.size());

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(15), Constraint::Length(30)])
            .split(layout[1]);
        f.render_widget(self.header(), layout[0]);
        f.render_widget(self.instuctions(), body[1]);
        f.render_widget(self.board(), body[0]);
    }
}
