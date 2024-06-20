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
        Paragraph::new("BigBrainWordle 󰧑 ")
            .centered()
            .style(Style::default().fg(Color::Green))
    }

    pub fn instuctions(&self) -> impl Widget {
        Paragraph::new("Lets start with tares").centered()
    }

    pub fn board(&self) -> impl Widget {
        Paragraph::new("▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️\n▫️▫️▫️▫️▫️").centered()
    }
    pub fn draw(&self, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(5),
            ])
            .split(f.size());
        f.render_widget(self.header(), layout[0]);
        f.render_widget(self.instuctions(), layout[1]);
        f.render_widget(self.board(), layout[2]);
    }
}
