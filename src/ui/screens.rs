use ratatui::{prelude::*, widgets::List};

use crate::app::State;

use super::widgets::{Header, ProgressBar};

#[derive(Default)]
pub struct NowPlayingScreen;

impl StatefulWidget for &NowPlayingScreen {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let [header_area, body_area, footer_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ])
        .areas(area);

        Header.render(header_area, buf, state);
        StatefulWidget::render(
            List::new(
                state
                    .client
                    .queue()
                    .unwrap()
                    .into_iter()
                    .map(|song| song.title.unwrap()),
            )
            .highlight_symbol("> "),
            body_area,
            buf,
            &mut state.list_state,
        );
        ProgressBar.render(footer_area, buf, state);
    }
}
