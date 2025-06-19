use std::{error::Error, io::stdout};

use app::App;
use ratatui::crossterm::{event::{DisableMouseCapture, EnableMouseCapture}, ExecutableCommand};

mod canvas_widget;
mod selection_widget;
mod topbar_widget;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = ratatui::init();
    let mut app = App::default();
    stdout().execute(EnableMouseCapture)?;
    app.run(&mut term)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(ratatui::try_restore()?)
}
