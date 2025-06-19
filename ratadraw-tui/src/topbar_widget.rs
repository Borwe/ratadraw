use std::default;

use ratatui::{
    layout::Constraint,
    style::{Color, Stylize},
    text::Text,
    widgets::{Cell, Row, StatefulWidget, Table, Widget},
};

#[derive(Default, PartialEq)]
enum TopBarItem {
    #[default]
    None,
    SetBackground,
    Quit,
    Undo,
    Redo,
    Export,
}

#[derive(Default)]
pub(crate) struct TopBarWidget {
    item_selected: TopBarItem,
    set_clear_count: usize,
}

impl TopBarWidget {
    pub(crate) fn select_exit(&mut self) {
        self.item_selected = TopBarItem::Quit;
    }

    pub(crate) fn select_undo(&mut self) {
        self.item_selected = TopBarItem::Undo;
    }

    pub(crate) fn select_redo(&mut self) {
        self.item_selected = TopBarItem::Redo;
    }

    ///try clear selection if any after 120 frames
    pub(crate) fn clear_selection(&mut self) {
        if self.set_clear_count % 120 == 0 {
            self.item_selected = TopBarItem::None;
        }

        if self.item_selected != TopBarItem::None {
            self.set_clear_count += 1;
        }
    }
}

impl Widget for &TopBarWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut bg = Text::from("[b] Set Background");
        let mut quit = Text::from("[q] Quit");
        let mut undo = Text::from("[u] Undo");
        let mut redo = Text::from("[r] Redo");
        let mut export = Text::from("[e] Export Drawing");

        match self.item_selected {
            TopBarItem::None => (),
            TopBarItem::SetBackground => bg = bg.bg(Color::Green),
            TopBarItem::Quit => quit = quit.bg(Color::Green),
            TopBarItem::Undo => undo = undo.bg(Color::Green),
            TopBarItem::Redo => redo = redo.bg(Color::Green),
            TopBarItem::Export => export = export.bg(Color::Green),
        }

        let row_items = vec![bg, undo, redo, export, quit];
        let row = Row::new(row_items);
        let table = Table::new(
            vec![row],
            [
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
                Constraint::Min(5),
            ],
        );
        ratatui::widgets::Widget::render(&table, area, buf);
    }
}
