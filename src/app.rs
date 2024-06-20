use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    terminal::Terminal,
    widgets::{Paragraph, Widget},
    Frame,
};

use crate::entropy::brain::Brain;
pub struct App {
    brain: Brain,
}

impl App {
    pub fn new(brain: Brain) -> Self {
        Self { brain }
    }
    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> io::Result<()> {
        loop {
            term.draw(|f| self.draw(f))?;
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }

    pub fn header(&self) -> impl Widget {
        Paragraph::new("BigBrainWordle 󰧑 ").style(Style::default().fg(Color::Green))
    }

    pub fn instuctions(&self) -> impl Widget {
        Paragraph::new("Lets start with tares")
    }

    pub fn board(&self) -> impl Widget {
        Paragraph::new("▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️")
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
