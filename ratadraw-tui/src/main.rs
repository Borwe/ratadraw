use std::{default, error::Error, io::stdout, time::Duration};

use canvas_widget::DrawingCanvas;
use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
        ExecutableCommand,
    },
    layout::Rect,
    prelude::Backend,
    text::Line,
    widgets::{Block, Paragraph},
    Terminal,
};
use topbar_widget::TopBarWidget;

mod canvas_widget;
mod topbar_widget;

#[derive(Default)]
struct App {
    exit: bool,
    mouse_position: Option<Rect>,
    topbar: TopBarWidget,
    canvas: DrawingCanvas,
}

impl App {
    fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            term.draw(|f| {
                f.render_widget(&self.topbar, f.area());
                f.render_widget(&mut self.canvas, f.area());
            })?;
            self.topbar.clear_selection();
            self.listen()?
        }
        Ok(())
    }

    fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        match ratatui::crossterm::event::poll(Duration::from_millis(13))? {
            true => Ok(self.handle_event(event::read()?)),
            false => Ok(()),
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(x) => match x.kind {
                KeyEventKind::Press => match x.code {
                    event::KeyCode::Char('q') => {
                        self.topbar.select_exit();
                        self.exit = true
                    }
                    event::KeyCode::Char('u') => {
                        self.topbar.select_undo();
                        self.canvas.undo()
                    }
                    event::KeyCode::Char('r') => {
                        self.topbar.select_redo();
                        self.canvas.redo()
                    }
                    _ => {}
                },
                _ => {}
            },
            Event::Mouse(x) => self.canvas.listen(x),
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = ratatui::init();
    let mut app = App::default();
    stdout().execute(EnableMouseCapture)?;
    app.run(&mut term)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(ratatui::try_restore()?)
}
