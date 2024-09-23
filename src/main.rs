use std::{io, net::TcpStream, time::Duration};

use app::App;

mod app;
mod config;
mod ui;

fn main() -> io::Result<()> {
    let _stream = TcpStream::connect("127.0.0.1:6600").expect("Failed to connect to MPD");

    let mut terminal = ui::init()?;
    let result = App::default().run(&mut terminal);
    ui::restore()?;
    result

    // todo!("implement rsmpc");
}

fn fps_to_duration(fps: u16) -> Duration {
    Duration::from_millis((1_000 / fps).into())
}

#[cfg(test)]
mod tests {
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
