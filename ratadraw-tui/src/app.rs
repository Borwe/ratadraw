use std::{error::Error, time::Duration};

use ratatui::{crossterm::event::{self, Event, KeyEventKind, MouseEventKind}, layout::{Position, Rect}, prelude::Backend, widgets::Widget, Terminal};

use crate::{canvas_widget::DrawingCanvas, topbar_widget::TopBarWidget};



#[derive(Default)]
pub struct App {
    exit: bool,
    mouse_position: Option<Position>,
    topbar: TopBarWidget,
    canvas: DrawingCanvas,
}

impl App {
    pub fn run<B: Backend>(&mut self, term: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
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
    fn render(self, _: Rect, buf: &mut ratatui::prelude::Buffer)
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
