// TODO: remove allow
#![allow(clippy::single_match, clippy::default_constructed_unit_structs)]

use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    prelude::*,
    widgets::*,
    Terminal,
};

use self::widgets::{Header, ProgressBar};

pub mod screens;
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

#[derive(Default)]
pub struct App {
    should_quit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.should_quit {
            let _ = terminal.draw(|frame| frame.render_widget(&(*self), frame.area()));
            if event::poll(fps_to_duration(60)).unwrap() {
                self.handle_events()?;
            }
        }

        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => self.should_quit = true,
            Event::Key(KeyEvent {
                code: KeyCode::Char('j'),
                ..
            }) => todo!("update state to select next item in queue"),
            Event::Key(KeyEvent {
                code: KeyCode::Char('k'),
                ..
            }) => todo!("update state to select previous item in queue"),
            // silently ignore any other events
            _ => (),
        }

        Ok(())
    }
}

impl Widget for &App {
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

fn fps_to_duration(fps: u16) -> Duration {
    Duration::from_millis((1_000 / fps).into())
}

#[cfg(test)]
mod tests {
    use crate::ui::fps_to_duration;

    use super::*;

    #[test]
    fn test_fps_to_duration() {
        assert_eq!(fps_to_duration(30), Duration::from_millis(33));
        assert_eq!(fps_to_duration(60), Duration::from_millis(16));
        assert_eq!(fps_to_duration(90), Duration::from_millis(11));
        assert_eq!(fps_to_duration(120), Duration::from_millis(8));
        assert_eq!(fps_to_duration(144), Duration::from_millis(6));
        assert_eq!(fps_to_duration(240), Duration::from_millis(4));
    }
}
