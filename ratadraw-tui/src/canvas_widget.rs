use ratatui::{buffer::Cell, crossterm::event::MouseEvent, layout::Rect, widgets::{canvas::Canvas, Block, Widget}};


pub struct DrawingCanvas {
    full_region: Option<Rect>,
    actual_region: Option<Rect>,
    cells: Vec<Cell>
}

impl DrawingCanvas {
    pub(crate) fn new()-> DrawingCanvas{
        DrawingCanvas { 
            full_region: None,
            actual_region: None,
            cells: Vec::default()
        }
    }

    pub(crate) fn listen(&mut self, x: MouseEvent) {
        todo!()
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
    }
}
