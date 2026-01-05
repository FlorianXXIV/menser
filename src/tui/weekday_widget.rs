use ratatui::widgets::Widget;
use time::Weekday;

pub struct WeekdayWidget {
    current_weekday: Weekday,
    selected_weekday: Weekday,
}

impl Widget for WeekdayWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}

