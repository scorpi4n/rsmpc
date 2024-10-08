use mpd::Client;
use ratatui::widgets::ListState;

use crate::ui::CurrentScreen;

/// Represents the application's state
#[derive(Default, Debug)]
pub struct Model {
    pub should_quit: bool,
    pub client: Client,
    pub list_state: ListState,
    pub current_screen: CurrentScreen,
    pub library_sceen_directory: String,
}

impl Model {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Quit => self.should_quit = true,
            Message::TogglePause => {
                let _ = self.client.toggle_pause();
            }
            Message::SelectNext => self.list_state.select_next(),
            Message::SelectPrevious => self.list_state.select_previous(),
            Message::ChangeScreen(screen) => self.current_screen = screen,
        };
    }
}

#[derive(Debug)]
pub enum Message {
    Quit,
    TogglePause,
    SelectNext,
    SelectPrevious,
    ChangeScreen(CurrentScreen),
}
