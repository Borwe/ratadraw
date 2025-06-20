cfg_if::cfg_if!{
    if #[cfg(target_arch = "wasm32")] {
        use ratzilla::DomBackend;
    }else if #[cfg(not(target_arch = "wasm32"))] {
        use std::io::stdout;
        use ratatui::{crossterm::{event::{DisableMouseCapture,
            EnableMouseCapture}, ExecutableCommand}, DefaultTerminal};
    }
}

use std::{cell::RefCell, error::Error, rc::Rc};

use app::App;
use ratatui::{prelude::Backend, Terminal};

mod canvas_widget;
mod selection_widget;
mod topbar_widget;
mod app;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
fn get_terminal()-> std::io::Result<DefaultTerminal>{
    use ratatui::DefaultTerminal;

    Ok(ratatui::init())
}


#[cfg(target_arch = "wasm32")]
fn get_terminal()-> std::io::Result<Terminal<DomBackend>>{
    Terminal::new(DomBackend::new()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let term = Rc::from(RefCell::new(get_terminal()?));
    let app = Rc::from(RefCell::new(App::default()));

    #[cfg(not(target_arch = "wasm32"))] 
    {
        stdout().execute(EnableMouseCapture)?;
        app.borrow_mut().run(term)?;
        stdout().execute(DisableMouseCapture)?;
        Ok(ratatui::try_restore()?)
    }

    #[cfg(target_arch = "wasm32")] 
    {
        App::run(app.clone(), term.clone());
        Ok(())
    }
}
