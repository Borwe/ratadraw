use std::{error::Error, time::Duration};

use ratatui::{crossterm::event::{self, Event, KeyEventKind}, prelude::Backend, text::Line, widgets::{Block, Paragraph}, Terminal};
use topbar_widget::TopBarWidget;

mod topbar_widget;

struct App {
    exit: bool,
    topbar: TopBarWidget
}

impl App {
    fn new()-> Self{
        Self{
            exit: false,
            topbar: TopBarWidget::default()
        }
    }

    fn run<B: Backend>(&mut self, term: &mut Terminal<B>)-> Result<(), Box<dyn Error>>{
        while !self.exit {
            term.draw(|f|{
                f.render_widget(&self.topbar, f.area());
            })?;
            self.listen()?
        }
        Ok(())
    }

    fn listen(&mut self)-> Result<(),Box<dyn Error>>{
        match ratatui::crossterm::event::poll(Duration::from_millis(13))?{
            true => Ok(self.handle_event(event::read()?)) ,
            false => Ok(())
        }
    }

    fn handle_event(&mut self, event: Event){
        match event {
            Event::Key(x) => {
                match x.kind {
                    KeyEventKind::Press => {
                        match x.code {
                            event::KeyCode::Char('q') => { self.exit = true},
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = ratatui::init();
    let mut app = App::new();
    app.run(&mut term)?;
    ratatui::restore();
    Ok(ratatui::try_restore()?)
}
