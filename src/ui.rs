use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, prelude::*, Terminal};

use crate::{app::State, ui::screens::NowPlayingScreen};

mod screens;
mod widgets;

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    io::stdout().execute(EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    Terminal::new(CrosstermBackend::new(io::stdout()))
}

pub fn restore() -> io::Result<()> {
    io::stdout().execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

pub enum CurrentScreen {
    NowPlaying,
}

impl StatefulWidget for &CurrentScreen {
    type State = State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        use CurrentScreen::*;

        match self {
            NowPlaying => NowPlayingScreen.render(area, buf, state),
        }
    }
}

fn format_time(time: (Duration, Duration)) -> String {
    let (elapsed_time, total_time) = time;
    let elapsed_minutes = elapsed_time.as_secs() / 60;
    let elapsed_seconds = elapsed_time.as_secs() % 60;
    let total_minutes = total_time.as_secs() / 60;
    let total_seconds = total_time.as_secs() % 60;

    format!("{elapsed_minutes}:{elapsed_seconds:02}/{total_minutes}:{total_seconds:02}",)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(
            format_time((Duration::default(), Duration::from_secs(182))),
            "0:00/3:02"
        );
    }
}
