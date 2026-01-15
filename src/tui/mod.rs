use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    layout::{
        self,
        Constraint::{Fill, Length},
        Layout,
    },
    text::Text,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::{
    api_interactions::fetch_menus, api_schema::MensaMenu, tui::weekday_widget::WeekdayWidget,
};

const APP_TITLE: &str = "Menser";

mod weekday_widget;

#[derive(Debug, Default)]
pub struct TUI {
    exit: bool,
    weekday_w: WeekdayWidget,
    menues: Vec<MensaMenu>,
}

impl TUI {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.menues = match fetch_menus(&self.weekday_w.selected_weekday.to_string()) {
            Ok(menues) => menues,
            Err(_) => Vec::new(),
        };
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
                KeyCode::Tab => {
                    self.weekday_w.sel_next();
                    self.menues = match fetch_menus(&self.weekday_w.selected_weekday.to_string()) {
                        Ok(menues) => menues,
                        Err(_) => Vec::new(),
                    };
                }
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
        let layout = Layout::vertical([Length(1), Fill(1)]);
        let [tabs, para] = layout.areas(area);
        self.weekday_w.render(tabs, buf);
        self.weekday_w.render_tabs(para, buf, &self.menues);
    }
}
