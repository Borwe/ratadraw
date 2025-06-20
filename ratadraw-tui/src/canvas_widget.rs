cfg_if::cfg_if!{
    if #[cfg(target_arch = "wasm32")] {
        use crate::utils::MouseState;
    } else if #[cfg(not(target_arch = "wasm32"))] {
        use ratatui::crossterm::event::{MouseButton, MouseEvent, MouseEventKind};
    }
}


use std::{collections::HashSet, hash::Hash};

use ratatui::{
    layout::{Position, Rect},
    symbols,
    widgets::{Block, Widget},
};

pub struct MyCell {
    x: u16,
    y: u16,
    val: &'static str,
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

impl Eq for MyCell {}

impl Into<Position> for &MyCell {
    fn into(self) -> Position {
        (self.x, self.y).into()
    }
}

type CellGroup = Vec<MyCell>;

#[derive(Default)]
pub struct DrawingCanvas {
    full_region: Option<Rect>,
    actual_region: Option<Rect>,
    cells: Vec<CellGroup>,
    undone_cells: Vec<CellGroup>,
}

impl DrawingCanvas {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn listen(&mut self, pos: Position, state: MouseState) {

        //match state {
        //    MouseState::Pressed => {},
        //    MouseState::Released => {},
        //    MouseState::None => {},
        //}
        //match event.kind {
        //    MouseEventKind::Down(x) if x == MouseButton::Left => {
        //        //clear all items in undone queue as we've overwritten them
        //        self.undone_cells.clear();
        //        let mut new_group = CellGroup::with_capacity(200);
        //        new_group.push(MyCell {
        //            x: event.column,
        //            y: event.row,
        //            val: symbols::block::FULL,
        //        });
        //        self.cells.push(new_group);
        //    }
        //    MouseEventKind::Drag(x) if x == MouseButton::Left => {
        //        let vec = self.cells.last_mut().unwrap();
        //        vec.push(MyCell {
        //            x: event.column,
        //            y: event.row,
        //            val: symbols::block::FULL,
        //        });
        //    }
        //    _ => {}
        //};
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn listen(&mut self, event: MouseEvent) {

        match event.kind {
            MouseEventKind::Down(x) if x == MouseButton::Left => {
                //clear all items in undone queue as we've overwritten them
                self.undone_cells.clear();
                let mut new_group = CellGroup::with_capacity(200);
                new_group.push(MyCell {
                    x: event.column,
                    y: event.row,
                    val: symbols::block::FULL,
                });
                self.cells.push(new_group);
            }
            MouseEventKind::Drag(x) if x == MouseButton::Left => {
                let vec = self.cells.last_mut().unwrap();
                vec.push(MyCell {
                    x: event.column,
                    y: event.row,
                    val: symbols::block::FULL,
                });
            }
            _ => {}
        };
    }

    //get region usable
    fn generate_rect(&mut self, big_rect: Rect) -> Rect {
        match self.full_region {
            Some(x) => {
                if x != big_rect {
                    self.update_regions(big_rect)
                }
            }
            _ => self.update_regions(big_rect),
        };
        self.actual_region.unwrap()
    }

    //update region usable
    fn update_regions(&mut self, big_rect: Rect) {
        self.full_region = Some(big_rect);
        self.actual_region = Some(Rect {
            x: big_rect.x,
            y: big_rect.y + 1,
            width: big_rect.width,
            height: big_rect.height - 1,
        })
    }

    pub(crate) fn undo(&mut self) {
        if let Some(undone) = self.cells.pop() {
            self.undone_cells.push(undone);
        }
    }

    pub(crate) fn redo(&mut self) {
        if let Some(redo) = self.undone_cells.pop() {
            self.cells.push(redo);
        }
    }
}

impl Widget for &mut DrawingCanvas {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let area = self.generate_rect(area);
        let block = Block::bordered().title("Canvas");
        //let inner_area = block.inner(area);
        block.render(area, buf);

        for cgroup in self.cells.iter() {
            for c in cgroup.iter() {
                if let Some(cell) = buf.cell_mut(c) {
                    cell.set_symbol(c.val);
                }
            }
        }
    }
}
