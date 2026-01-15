use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, Table, Tabs, Widget},
};
use time::{OffsetDateTime, Weekday};

use crate::{api_schema::MensaMenu, util::list_weekdays};

#[derive(Debug)]
pub struct WeekdayWidget {
    current_weekday: Weekday,
    pub selected_weekday: Weekday,
}

impl WeekdayWidget {
    pub fn new() -> WeekdayWidget {
        let today = match OffsetDateTime::now_local() {
            Ok(it) => it,
            Err(_) => OffsetDateTime::now_utc(),
        }
        .weekday();
        WeekdayWidget {
            current_weekday: today,
            selected_weekday: today,
        }
    }

    pub fn sel_next(&mut self) {
        self.selected_weekday = self.selected_weekday.next();
    }

    pub fn render_tabs(&self, area: Rect, buf: &mut Buffer, mensa_menu: &Vec<MensaMenu>) {
        let mut rows = Vec::new();
        for mensa in mensa_menu {
            rows.append(&mut mensa.to_table_rows());
        }
        Table::new(
            rows,
            [
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ],
        )
        .block(
            Block::bordered()
                .title_bottom(Line::from(" <q> Quit "))
                .title_bottom(Line::from(" <Tab> Switch Tab ").right_aligned()),
        )
        .render(area, buf);
    }
}

impl Widget for &WeekdayWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let weekdays = list_weekdays(self.current_weekday);
        let mut i = 0;
        for day in &weekdays {
            if day != &self.selected_weekday {
                i += 1;
            } else {
                break;
            }
        }

        Tabs::new(
            list_weekdays(self.current_weekday)
                .iter()
                .map(|w| w.to_string()),
        )
        .select(Some(i as usize))
        .render(area, buf);
    }
}

impl Default for WeekdayWidget {
    fn default() -> Self {
        WeekdayWidget::new()
    }
}
