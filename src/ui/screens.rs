use ratatui::{prelude::*, widgets::List};

use crate::state::Model;

use super::widgets::{Header, ProgressBar};

#[derive(Default)]
pub struct NowPlayingScreen;

impl StatefulWidget for &NowPlayingScreen {
    type State = Model;

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

#[derive(Default)]
pub struct LibraryScreen;

impl StatefulWidget for &LibraryScreen {
    type State = Model;

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
                    .listfiles(&state.library_sceen_directory)
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|(item, _)| item == "file" || item == "directory")
                    .map(|(_, name)| name),
            )
            .highlight_symbol("> "),
            body_area,
            buf,
            &mut state.list_state,
        );
        ProgressBar.render(footer_area, buf, state);
    }
}
