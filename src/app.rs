use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use mpd::Client;
use ratatui::widgets::ListState;

use crate::{
    fps_to_duration,
    ui::{CurrentScreen, Tui},
};

pub struct App {
    should_quit: bool,
    current_screen: CurrentScreen,
    state: State,
}

pub struct State {
    pub client: Client,
    pub list_state: ListState,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.should_quit {
            let _ = terminal.draw(|frame| {
                frame.render_stateful_widget(&self.current_screen, frame.area(), &mut self.state)
            });

            if event::poll(fps_to_duration(60))? {
                let event = event::read()?;
                self.handle_event(event)?;
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> io::Result<()> {
        if let Event::Key(KeyEvent { code: key_code, .. }) = event {
            match key_code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Char('j') => self.state.list_state.select_next(),
                KeyCode::Char('k') => self.state.list_state.select_previous(),
                // NOTE: I might forgo implementing song insertions on the normal screen/globally
                KeyCode::Char('I') => todo!("insert song at front of queue"),
                KeyCode::Char('i') => todo!("insert song before selected song"),
                KeyCode::Char('A') => todo!("insert song at end of queue"),
                KeyCode::Char('a') => todo!("insert song after selected song"),
                KeyCode::Char('p') => {
                    let _ = self.state.client.toggle_pause();
                }
                KeyCode::Char('d') => {
                    if let Some(index) = self.state.list_state.selected() {
                        let index = index as u32;
                        // WARN: forcing this range looks like bad practice
                        let _ = self.state.client.delete(index..index + 1);
                    }
                }
                KeyCode::Esc => self.state.list_state.select(None),
                _ => (),
            };
        };

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_quit: false,
            current_screen: CurrentScreen::NowPlaying,
            state: State {
                client: Client::connect("127.0.0.1:6600").expect("Failed to connect to MPD"),
                list_state: ListState::default(),
            },
        }
    }
}
