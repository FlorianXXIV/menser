use ratatui::widgets::{Paragraph, Tabs, Widget};
use time::{OffsetDateTime, Weekday};

pub struct WeekdayWidget {
    current_weekday: Weekday,
    selected_weekday: Weekday,
}

impl WeekdayWidget {
    /// Returns Vector of weekdays starting from the current weekday.
    pub fn list_weekdays() -> Vec<String> {
        OffsetDateTime::now_local()?.weekday()?;
    }
}

impl Widget for WeekdayWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Tabs::new(titles)
    }
}
