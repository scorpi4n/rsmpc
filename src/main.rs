use std::{io, net::TcpStream};

use app::App;

mod app;
mod config;
mod tui;

fn main() -> io::Result<()> {
    let _stream = TcpStream::connect("127.0.0.1:6600").expect("Failed to connect to MPD");

    let mut terminal = tui::init()?;
    let result = App::default().run(&mut terminal);
    tui::restore()?;
    result

    // todo!("implement rsmpc");
}
