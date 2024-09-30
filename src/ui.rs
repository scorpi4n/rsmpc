use std::io::{self, Stdout};

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
