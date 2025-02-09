use ratatui::{ crossterm::event::{MouseButton, MouseEvent, MouseEventKind}, layout::{Position, Rect}, widgets::{Block, Widget}};

pub struct MyCell {
    x: u16,
    y: u16,
    val: &'static str
}

impl Into<Position> for &MyCell{
    fn into(self) -> Position {
        (self.x,self.y).into()
    }
}

pub struct DrawingCanvas {
    full_region: Option<Rect>,
    actual_region: Option<Rect>,
    cells: Vec<MyCell>
}

impl DrawingCanvas {
    pub(crate) fn new()-> DrawingCanvas{
        DrawingCanvas { 
            full_region: None,
            actual_region: None,
            cells: Vec::default()
        }
    }

    pub(crate) fn listen(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(x) if x == MouseButton::Left => {
                self.cells.push(MyCell { x: event.column, y: event.row, val: "A" });
            },
            MouseEventKind::Drag(x) if x == MouseButton::Left => {
                self.cells.push(MyCell { x: event.column, y: event.row, val: "D" });
            }
            _ =>{}
        };
    }

    fn generate_rect(&mut self, big_rect: Rect)-> Rect{
        match self.full_region {
            Some(x)=>{
                if x != big_rect {
                    self.update_regions(big_rect)
                }
            }
            _ => self.update_regions(big_rect),
        };
        self.actual_region.unwrap()
    }

    fn update_regions(&mut self, big_rect: Rect) {
        self.full_region = Some(big_rect);
        self.actual_region = Some(Rect{
            x : big_rect.x,
            y : big_rect.y + 1,
            width : big_rect.width ,
            height : big_rect.height -1
        })
    }
}

impl Widget for &mut DrawingCanvas {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let area = self.generate_rect(area);
        let block = Block::bordered().title("Canvas");
        //let inner_area = block.inner(area);
        block.render(area, buf);

        for c in self.cells.iter() {
            let cell = buf.cell_mut(c).unwrap();
            cell.set_symbol(c.val);
        }
    }
}
