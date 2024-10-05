use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget, Wrap},
    Frame, Terminal,
};
use std::io;

use crate::{
    entropy::{
        brain::Brain,
        feedback::{Feedback, FeedbackType},
    },
    text,
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

pub fn red_text(text: &str) -> Text {
    Text::styled(text, Style::default().fg(Color::Red))
}

pub fn message<'a>(text: (&'a str, &'a str), highlight: String) -> Text<'a> {
    Text::from(Line::from(vec![
        Span::raw(text.0),
        Span::styled(highlight, Style::default().fg(Color::Green)),
        Span::raw(text.1),
    ]))
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

    pub fn current_word(&self) -> String {
        self.current.iter().collect::<String>()
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
                    (KeyCode::Enter, _, 5) => {
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

        self.brain.prune(feedback);
        match self.brain.suggest(self.row == 4) {
            Ok(word) => self.current = word,
            Err(_) => self.state = AppState::Failed,
        }

        if self.brain.done() && self.row != 5 {
            self.feedbacks[self.row + 1] = [Some(FeedbackType::Correct('a')); 5];
            self.state = AppState::Won;
        } else if self.row == 5 {
            self.state = AppState::Lost;
        }
    }

    pub fn header(&self) -> impl Widget {
        Paragraph::new("BigBrainWordle 󰧑").style(Style::default().fg(Color::Green))
    }

    pub fn instuctions(&self) -> impl Widget {
        let content = match self.state {
            AppState::Playing => match self.row {
                0 => message(text::OPENING, self.current_word()),
                5 => message(text::CLOSING, self.current_word()),
                _ => message(
                    text::suggestion_text(self.brain.options.len()),
                    self.current_word(),
                ),
            },
            AppState::Won => message(text::WON, self.current_word()),
            AppState::Lost => red_text(text::LOST),
            AppState::Failed => red_text(text::FAILED),
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
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(15), Constraint::Length(50)])
            .split(f.area());

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2), Constraint::Length(8)])
            .split(layout[1]);
        f.render_widget(self.header(), right[0]);
        f.render_widget(self.instuctions(), right[1]);
        f.render_widget(self.board(), layout[0]);
    }
}
