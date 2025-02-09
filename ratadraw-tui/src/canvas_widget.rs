use std::{collections::HashSet, hash::Hash};

use ratatui::{ crossterm::event::{MouseButton, MouseEvent, MouseEventKind}, layout::{Position, Rect}, symbols, widgets::{Block, Widget}};

pub struct MyCell {
    x: u16,
    y: u16,
    val: &'static str
}

impl PartialEq for MyCell {
    fn eq(&self, other: &Self) -> bool {
        other.x == self.x && other.y == self.y && self.val == other.val
    }
}

impl Hash for MyCell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Eq for MyCell {
}

impl Into<Position> for &MyCell{
    fn into(self) -> Position {
        (self.x,self.y).into()
    }
}

pub struct DrawingCanvas {
    full_region: Option<Rect>,
    actual_region: Option<Rect>,
    cells: HashSet<MyCell>
}

impl DrawingCanvas {
    pub(crate) fn new()-> DrawingCanvas{
        DrawingCanvas { 
            full_region: None,
            actual_region: None,
            cells: HashSet::default()
        }
    }

    pub(crate) fn listen(&mut self, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(x) if x == MouseButton::Left => {
                self.cells.insert(MyCell { x: event.column, y: event.row, val: symbols::block::FULL });
            },
            MouseEventKind::Drag(x) if x == MouseButton::Left => {
                self.cells.insert(MyCell { x: event.column, y: event.row, val: symbols::block::FULL });
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
            if let Some(cell) = buf.cell_mut(c){
                cell.set_symbol(c.val);
            }
        }
    }
}
