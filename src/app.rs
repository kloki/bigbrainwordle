use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::{
    entropy::{
        brain::{Brain, Word},
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
    pub feedbacks: [[FeedbackType; 5]; 6],
    current: [char; 5],
    state: AppState,
    no_emoji: bool,
}

pub fn red_text<'a>(text: &'a str) -> Text<'a> {
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
    pub fn new(brain: Brain, no_emoji: bool) -> Self {
        let current = brain.suggest(false).expect("No words to suggest");
        Self {
            brain,
            row: 0,
            column: 0,
            feedbacks: [[FeedbackType::Empty; 5]; 6],
            current,
            state: AppState::Playing,
            no_emoji,
        }
    }

    pub fn current_word(&self) -> String {
        self.current.iter().collect::<String>()
    }
    pub fn run_autosolve(
        &mut self,
        solution: Word,
        term: &mut DefaultTerminal,
    ) -> io::Result<()> {
        term.draw(|f| self.draw(f))?;

        loop {
            let feedback = Feedback::from_guess(&self.current, &solution);

            for i in 0..5 {
                self.feedbacks[self.row][i] = feedback.items[i];
            }
            self.column = 5;
            term.draw(|f| self.draw(f))?;

            self.process_feedback();

            if self.state != AppState::Playing {
                break;
            }

            self.column = 0;
            self.row += 1;

            term.draw(|f| self.draw(f))?;
        }

        term.draw(|f| self.draw(f))?;

        loop {
            if let Event::Key(_) = event::read()? {
                return Ok(());
            }
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            term.draw(|f| self.draw(f))?;
            if let Event::Key(key) = event::read()? {
                match (key.code, self.row, self.column, &self.state) {
                    (_, _, _, state) if state != &AppState::Playing => return Ok(()),
                    (KeyCode::Char('q'), _, _, _) => return Ok(()),
                    (KeyCode::Esc, _, _, _) => return Ok(()),
                    (KeyCode::Char('g'), r, c, _) if c < 5 => {
                        self.feedbacks[r][c] = FeedbackType::Correct(self.current[c]);
                        self.column += 1;
                    }
                    (KeyCode::Char('y'), r, c, _) if c < 5 => {
                        self.feedbacks[r][c] = FeedbackType::WrongPosition(self.current[c]);
                        self.column += 1;
                    }
                    (KeyCode::Char(' '), r, c, _) if c < 5 => {
                        self.feedbacks[r][c] = FeedbackType::Wrong(self.current[c]);
                        self.column += 1;
                    }
                    (KeyCode::Backspace, r, c, _) if c > 0 => {
                        self.feedbacks[r][c - 1] = FeedbackType::Empty;
                        self.column -= 1;
                    }
                    (KeyCode::Enter, _, 5, _) => {
                        self.process_feedback();
                        self.column = 0;
                        self.row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn process_feedback(&mut self) {
        let feedback = Feedback::new(self.feedbacks[self.row]);
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
            for i in 0..5 {
                self.feedbacks[self.row + 1][i] = FeedbackType::Correct(self.current[i]);
            }
            self.state = AppState::Won;
        } else if self.row == 5 {
            self.state = AppState::Lost;
        }
    }

    pub fn header(&self) -> impl Widget {
        Paragraph::new("BigBrainWordle").style(Style::default().fg(Color::Green))
    }

    pub fn instuctions(&self) -> impl Widget {
        let content = match self.state {
            AppState::Playing => match self.row {
                0 => message(text::opening_text(self.no_emoji), self.current_word()),
                5 => message(text::closing_text(self.no_emoji), self.current_word()),
                _ => message(
                    text::suggestion_text(self.brain.options.len(), self.no_emoji),
                    self.current_word(),
                ),
            },
            AppState::Won => message(text::won_text(self.no_emoji), self.current_word()),
            AppState::Lost => red_text(text::lost_text(self.no_emoji)),
            AppState::Failed => red_text(text::failed_text(self.no_emoji)),
        };

        Paragraph::new(content).wrap(Wrap { trim: true })
    }

    pub fn board(&self) -> impl Widget {
        let lines: Vec<Line> = self
            .feedbacks
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let prefix = if i == self.row { ">" } else { " " };
                let mut spans = vec![Span::raw(prefix)];
                for feedback in row.iter() {
                    spans.push(feedback.to_widget(self.no_emoji));
                }
                Line::from(spans)
            })
            .collect();
        Paragraph::new(lines)
    }
    pub fn draw(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(18), Constraint::Length(50)])
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
