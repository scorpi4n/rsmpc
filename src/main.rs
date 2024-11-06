use std::{
    io,
    sync::mpsc::{self, TryRecvError},
    thread,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use state::{Message, Model};
use ui::Screen;

mod state;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ui::init()?;
    let mut state = Model::default();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || -> io::Result<()> {
        loop {
            let event = event::read()?;
            if let Some(msg) = handle_event(event) {
                tx.send(msg).unwrap()
            }
        }
    });

    while !state.should_quit {
        ui::draw(&mut terminal, &mut state)?;

        match rx.try_recv() {
            Ok(msg) => state.update(msg),
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
    }

    ui::restore()?;

    Ok(())
}

// It would be best to move this function elsewhere, though I'm not sure where yet
fn handle_event(event: Event) -> Option<Message> {
    if let Event::Key(KeyEvent { code, .. }) = event {
        match code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('j') => Some(Message::SelectNext),
            KeyCode::Char('k') => Some(Message::SelectPrevious),
            // NOTE: I might forgo implementing song insertions on the normal screen/globally
            KeyCode::Char('I') => todo!("insert song at front of queue"),
            KeyCode::Char('i') => todo!("insert song before selected song"),
            KeyCode::Char('A') => todo!("insert song at end of queue"),
            KeyCode::Char('a') => todo!("insert song after selected song"),
            KeyCode::Char('p') => Some(Message::TogglePause),
            KeyCode::Char('d') => todo!("delete selected"),
            KeyCode::Esc => todo!("deselect"),

            KeyCode::Char('1') => Some(Message::ChangeScreen(Screen::NowPlaying)),
            KeyCode::Char('2') => Some(Message::ChangeScreen(Screen::Library)),

            _ => None,
        }
    } else {
        None
    }
}
