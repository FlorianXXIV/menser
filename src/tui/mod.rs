use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    text::Text,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

const APP_TITLE: &str = "Menser";

mod weekday_widget;

#[derive(Debug, Default)]
pub struct TUI {
    exit: bool,
}

impl TUI {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) => self.handle_key(key_event),
            _ => {}
        }
        Ok(())
    }

    fn handle_key(&mut self, key_event: KeyEvent) {
        if key_event.is_press() {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                _ => {}
            }
        }
    }
}

impl Widget for &TUI {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(Text::from("MENSER"))
            .block(Block::bordered())
            .render(area, buf);
    }
}
