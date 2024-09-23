use ratatui::{
    prelude::*,
    widgets::{List, ListState, Widget},
};

use super::widgets::{Header, ProgressBar};

#[derive(Default)]
pub struct NowPlayingScreen;

impl Widget for &NowPlayingScreen {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [header_area, body_area, footer_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ])
        .areas(area);

        // WARN: remove allow
        #[allow(clippy::default_constructed_unit_structs)]
        Header::default().render(header_area, buf);
        let mut list_state = ListState::default();
        list_state.select_first();
        StatefulWidget::render(
            List::new(["Song 1", "Song 2"]).highlight_symbol("> "),
            body_area,
            buf,
            &mut list_state,
        );
        ProgressBar.render(footer_area, buf);
    }
}
