use ratatui::{layout::Flex, prelude::*, widgets::*};

#[derive(Default)]
pub struct Header;

impl Widget for Header {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Block::new()
            .borders(Borders::BOTTOM)
            .border_style(Color::DarkGray)
            .render(area, buf);

        let horizontal_layout = Layout::horizontal([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .flex(Flex::SpaceBetween);

        let [top_half_area, bottom_half_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(area);

        let [top_left, top_center, top_right] = horizontal_layout.areas(top_half_area);
        let [bottom_left, bottom_center, bottom_right] = horizontal_layout.areas(bottom_half_area);

        Line::raw("0:00/0:00").render(top_left, buf);
        Line::raw("Example Title")
            .centered()
            .render(top_center, buf);
        Line::raw("Volume: 50%")
            .right_aligned()
            .render(top_right, buf);
        Line::raw("[paused]").render(bottom_left, buf);
        Line::from_iter([
            Span::styled("Example Artist", Color::Blue),
            Span::raw(" - "),
            Span::styled("Example Album", Color::Cyan),
            Span::raw(" (1912-6-23)"),
        ])
        .centered()
        .render(bottom_center, buf);
        Line::raw("[---c--]")
            .right_aligned()
            .render(bottom_right, buf);
    }
}

pub struct ProgressBar;

impl Widget for ProgressBar {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        LineGauge::default()
            .label("00:00")
            .ratio(0.5)
            .filled_style(Color::Green)
            .render(area, buf);
    }
}
