use crate::renderer::Renderer;
use crate::renderer::RenderableContainer;
use crate::error::FtuiResult;
use crate::util::Rect;
use crate::util::Rectangle;
use crate::util::Dimension;

pub struct Custom {
    buffer: Vec<Vec<char>>,
    width: u16,
    height: u16,
}

impl Custom {
    pub fn new(dimension: Dimension) -> Self {
        Self {
            buffer: Self::create_buffer(dimension.width(), dimension.height()),
            width: dimension.width(),
            height: dimension.height(),
        }
    }

    #[inline]
    fn create_buffer(width: u16, height: u16) -> Vec<Vec<char>> {
        (0..height)
            .map(|_| (0..width).map(|_| 'x').collect())
            .collect()
    }

    pub fn square(&mut self, shape: Rectangle) {
        if !shape.is_inbound(self.width, self.height) {
            return;
        }

        let max_width = (shape.x() + shape.w()).min(self.width) as usize;
        let max_height = (shape.y() + shape.h()).min(self.height) as usize;

        for cursor_y in (shape.y() as usize)..max_height {
            for cursor_x in (shape.x() as usize)..max_width {
                self.buffer[cursor_y][cursor_x] = '█';
            }
        }
    }
}

impl RenderableContainer for Custom {
    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        let (r_width, r_height) = renderer.get_dimensions();
        let max_height = self.height.min(r_height) as usize;
        let max_width = self.width.min(r_width) as usize;

        for i in 0..max_height {
            renderer.line_mut(i)
                .edit_iter(self.buffer[i][0..max_width].iter().copied(), 0);
        }

        Ok(())
    }
}
