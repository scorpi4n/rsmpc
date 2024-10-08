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





    }
}
