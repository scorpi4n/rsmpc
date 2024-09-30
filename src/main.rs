use std::{io, time::Duration};

use app::App;

mod app;
mod config;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ui::init()?;
    let mut app = App::default();
    let result = app.run(&mut terminal);
    ui::restore()?;
    result

    // todo!("implement rsmpc");
}

fn fps_to_duration(fps: u16) -> Duration {
    Duration::from_millis((1_000 / fps).into())
}

fn format_time(time: (Duration, Duration)) -> String {
    let (elapsed_time, total_time) = time;
    let elapsed_minutes = elapsed_time.as_secs() / 60;
    let elapsed_seconds = {
        let prefix = '0';
        let mut elapsed_seconds = (elapsed_time.as_secs() % 60).to_string();
        elapsed_seconds.insert(0, prefix);
        elapsed_seconds[elapsed_seconds.len() - 2..].to_string()
    };
    let total_minutes = total_time.as_secs() / 60;
    let total_seconds = {
        let prefix = '0';
        let mut total_seconds = (total_time.as_secs() % 60).to_string();
        total_seconds.insert(0, prefix);
        total_seconds[total_seconds.len() - 2..].to_string()
    };

    format!(
        "{}:{}/{}:{}",
        elapsed_minutes, elapsed_seconds, total_minutes, total_seconds
    )
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

    #[test]
    fn test_format_time() {
        assert_eq!(
            format_time((Duration::default(), Duration::from_secs(182))),
            "0:00/3:02"
        );
    }
}
