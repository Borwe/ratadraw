use ratatui::{layout::Constraint, text::Text, widgets::{Cell, Row, StatefulWidget, Table, Widget}};


#[derive(Default)]
pub(crate) struct TopBarWidget {}

impl Widget for &TopBarWidget{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let bg = Text::from("[b] Set Background");
        let quit = Text::from("[q] Quit");
        let export = Text::from("[e] Export Drawing");


        let row_items = vec![bg,quit,export];
        let row = Row::new(row_items);
        let table = Table::new(vec![row],
            [Constraint::Min(3),Constraint::Min(3),Constraint::Min(3)]);
        ratatui::widgets::Widget::render(&table, area, buf);
    }
}
