use mpd::State as PlaybackState;
use ratatui::{layout::Flex, prelude::*, widgets::*};

use crate::state::Model;

use super::format_time;

#[derive(Default)]
pub struct Header;

impl StatefulWidget for Header {
    type State = Model;

    // WARN: using the client in a component's render method sends a lot of requests to the MPD
    // server, so I want to cache responses for about a second
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let status = state.client.status().expect("Failed to fetch MPD status");
        let playback_state_str = match status.state {
            PlaybackState::Stop => "stopped",
            PlaybackState::Play => "playing",
            PlaybackState::Pause => "paused",
        };
        let elapsed_playback_str = status.time.map(format_time).unwrap_or_default();
        let current_song = state
            .client
            .currentsong()
            .expect("Failed to fetch current song from MPD");

        Block::new()
            .borders(Borders::BOTTOM)
            .border_style(Color::DarkGray)
            .render(area, buf);

        // TODO: make this resize better, it looks awful at smaller widths
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

        Line::raw(elapsed_playback_str).render(top_left, buf);
        Line::raw(current_song.as_ref().map_or(String::default(), |song| {
            song.title.clone().expect("no title in metadata")
        }))
        .centered()
        .render(top_center, buf);
        Line::raw(format!("Volume: {}%", status.volume))
            .right_aligned()
            .render(top_right, buf);
        Line::raw(format!("[{}]", playback_state_str)).render(bottom_left, buf);
        // TODO: Use string templating with subst, currently looks weird without a song playing
        Line::from_iter([
            Span::styled(
                current_song.as_ref().map_or(String::default(), |song| {
                    song.artist.clone().expect("no artist in metadata")
                }),
                Color::Blue,
            ),
            Span::raw(" - "),
            Span::styled(
                current_song.as_ref().map_or("", |song| {
                    song.tags
                        .iter()
                        .find(|&(k, _v)| k.to_lowercase() == "album")
                        .map(|(_k, v)| v)
                        .expect("no album in metadata")
                }),
                Color::Cyan,
            ),
            Span::raw(format!(
                " ({})",
                current_song.as_ref().map_or("", |song| {
                    song.tags
                        .iter()
                        .find(|&(k, _v)| k.to_lowercase() == "date")
                        .map(|(_k, v)| v)
                        .expect("no date in metadata")
                })
            )),
        ])
        .centered()
        .render(bottom_center, buf);
        // TODO: replace with data from state
        Line::raw("[---c--]")
            .right_aligned()
            .render(bottom_right, buf);
    }
}

pub struct ProgressBar;

impl StatefulWidget for ProgressBar {
    type State = Model;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let status = state.client.status().unwrap();
        let (elapsed_time_label, elapsed_time_ratio) = status
            .time
            .map(|(elapsed_time, total_time)| {
                let elapsed_time_label = {
                    let elapsed_minutes = elapsed_time.as_secs() / 60;

                    let prefix = '0';
                    let mut elapsed_seconds = (elapsed_time.as_secs() % 60).to_string();
                    elapsed_seconds.insert(0, prefix);
                    let elapsed_seconds = elapsed_seconds[elapsed_seconds.len() - 2..].to_string();

                    format!("{elapsed_minutes}:{elapsed_seconds}")
                };
                let elapsed_time_ratio = elapsed_time.as_secs_f64() / total_time.as_secs_f64();

                (elapsed_time_label, elapsed_time_ratio)
            })
            .unwrap_or((String::from("0:00"), f64::default()));

        LineGauge::default()
            .label(elapsed_time_label)
            .ratio(elapsed_time_ratio)
            .filled_style(Color::Green)
            .render(area, buf);
    }
}
