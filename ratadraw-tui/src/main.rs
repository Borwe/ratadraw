use std::{default, error::Error, io::stdout, time::Duration};

use canvas_widget::DrawingCanvas;
use ratatui::{
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind, MouseEventKind,
        },
        ExecutableCommand,
    },
    layout::{Position, Rect},
    prelude::Backend,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    Terminal,
};
use topbar_widget::TopBarWidget;

mod canvas_widget;
mod topbar_widget;

#[derive(Default)]
struct App {
    exit: bool,
    mouse_position: Option<Position>,
    topbar: TopBarWidget,
    canvas: DrawingCanvas,
}

impl App {
    fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            term.draw(|f| {
                f.render_widget(&self.topbar, f.area());
                f.render_widget(&mut self.canvas, f.area());
                f.render_widget(&self, f.area());
            })?;
            self.topbar.clear_selection();
            self.listen()?
        }
        Ok(())
    }

    fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        match ratatui::crossterm::event::poll(Duration::from_millis(5))? {
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
            Event::Mouse(x) => {
                self.handle_hover(x);
                self.canvas.listen(x);
            }
            _ => {}
        }
    }

    fn handle_hover(&mut self, x: event::MouseEvent) {
        if x.kind == MouseEventKind::Moved {
            self.mouse_position = Some(Position {
                x: x.column,
                y: x.row,
            });
        }
    }
}

impl Widget for &&mut App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if let Some(pos) = self.mouse_position.clone() {
            if let Some(cell) = buf.cell_mut(pos) {
                cell.set_bg(ratatui::style::Color::LightGreen);
            }
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
