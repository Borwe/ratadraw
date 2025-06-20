cfg_if::cfg_if!{
    if #[cfg(target_arch = "wasm32")] {
        use ratzilla::event::{KeyCode, KeyEvent};
        use ratzilla::WebRenderer;
        use ratzilla::DomBackend;
        use crate::utils::MouseState;
    }else if #[cfg(not(target_arch = "wasm32"))] {
        use ratatui::{crossterm::event::{self, Event, KeyEventKind, MouseEventKind}};
    }
}
use std::{error::Error, time::Duration, rc::Rc, cell::RefCell};

use ratatui::{layout::{Position, Rect}, prelude::Backend, widgets::Widget, Terminal};

use crate::{canvas_widget::DrawingCanvas, topbar_widget::TopBarWidget};

#[derive(Default)]
pub(crate) struct App {
    exit: bool,
    mouse_position: Rc<Option<Position>>,
    topbar: TopBarWidget,
    canvas: DrawingCanvas,
}

impl App {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn run(me: Rc<RefCell<Self>>, term: Rc<RefCell<Terminal<DomBackend>>>) -> Result<(), Box<dyn Error>> {
        use std::ops::DerefMut;

        use ratatui::Frame;

        App::listen(me.clone(), term.clone());


        let me_clone = me.clone();
        let term: Box<Terminal<DomBackend>> = unsafe {
            Box::from_raw(term.as_ptr())
        };
        term.draw_web(Box::new(move |f: &mut Frame| {
            f.render_widget(&me_clone.borrow().topbar, f.area());
            f.render_widget(&mut me_clone.borrow_mut().canvas, f.area());
            f.render_widget(&me_clone.borrow_mut().deref_mut(), f.area());
        }));

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn run<B: Backend + Sized + 'static>(&mut self,
        term: Rc<RefCell<Terminal<B>>>) -> Result<(), Box<dyn Error>> {

        while !self.exit {
            term.borrow_mut().draw(|f| {
                f.render_widget(&self.topbar, f.area());
                f.render_widget(&mut self.canvas, f.area());
                f.render_widget(&self, f.area());
            })?;
            self.topbar.clear_selection();
            self.listen()?
        }
        Ok(())
    }


    /** For web, sets up listener and handler*/
    #[cfg(target_arch = "wasm32")]
    fn listen(me: Rc<RefCell<App>>, term: Rc<RefCell<Terminal<DomBackend>>>) {
        use ratzilla::{utils::{self, is_mobile}, WebRenderer};
        use web_sys::{wasm_bindgen::{prelude::Closure, JsCast}, HtmlElement};

        let window = web_sys::window().expect("Couldn't get window context");
        let document = window.document().expect("Coudln't get dcument from window'");
        //disable highlighting by mouse
        document.body().and_then(|body|{
            body.style().set_property("user-select", "none").unwrap();
            Some(())
        });
        let (width, height): (u16, u16)= match is_mobile() {
            true => {
                (window.screen().expect("Couldn't get screen")
                    .width().unwrap().try_into().unwrap(),
                window.screen().expect("Couldn't get screen")
                        .height().unwrap().try_into().unwrap())
            },
            false => (u16::try_from(window.inner_width()
                .unwrap().as_f64().unwrap() as usize).unwrap(),
                u16::try_from(window.inner_height()
                    .unwrap().as_f64().unwrap() as usize).unwrap())
        };

        let me_c1 = me.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent|{
                let x: u32 = event.x().try_into().expect("Coudln't get x coordinate");
                let y: u32 = event.y().try_into().expect("Coudln't get x coordinate");
                let size = match utils::is_mobile(){
                    true => utils::get_screen_size(),
                    false => utils::get_window_size(),
                };

                let gaps_in_x = width/size.width;
                let gaps_in_y = height/size.height;
                let x_i: u16 = x as u16 /gaps_in_x;
                let y_i: u16 = y as u16 /gaps_in_y;
                let position = Position::new(x_i, y_i);
                me_c1.borrow_mut().handle_hover(position);
            }) as Box<dyn FnMut(_)>);

        document.set_onmousemove(Some(closure.as_ref().unchecked_ref()));

        std::mem::forget(closure);

        term.borrow().on_key_event(move |event|{
            me.borrow_mut().handle_event(event);
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_hover(&mut self, pos: Position) {
        self.canvas.listen(pos, MouseState::None);
        self.mouse_position = Rc::new(Some(pos));
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_event(&mut self, event: KeyEvent) {

        match event.code {
            KeyCode::Char('q') => {
                self.topbar.select_exit();
                self.exit = true
            }
            KeyCode::Char('u') => {
                self.topbar.select_undo();
                self.canvas.undo()
            }
            KeyCode::Char('r') => {
                self.topbar.select_redo();
                self.canvas.redo()
            }
            _ => {}
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn listen(&mut self) -> Result<(), Box<dyn Error>> {
        match ratatui::crossterm::event::poll(Duration::from_millis(5))? {
            true => Ok(self.handle_event(event::read()?)),
            false => Ok(()),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(not(target_arch = "wasm32"))]
    fn handle_hover(&mut self, x: event::MouseEvent) {
        if x.kind == MouseEventKind::Moved {
            self.mouse_position = Rc::new(Some(Position {
                x: x.column,
                y: x.row,
            }));
        }
    }
}

impl Widget for &&mut App {
    fn render(self, _: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if let Some(pos) = self.mouse_position.as_ref() {
            if let Some(cell) = buf.cell_mut(pos.clone().to_owned()) {
                cell.set_bg(ratatui::style::Color::LightGreen);
            }
        }
    }
}
