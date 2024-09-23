use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::{
    fps_to_duration,
    tui::{screens::NowPlayingScreen, Tui},
};

#[derive(Default)]
pub struct App {
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.should_quit {
            let _ = terminal.draw(|frame| frame.render_widget(&NowPlayingScreen, frame.area()));

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
