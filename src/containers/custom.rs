use std::usize;

use crate::renderer::Renderer;
use crate::renderer::RenderableContainer;
use crate::error::FtuiResult;
use crate::util::Rect;
use crate::util::Rectangle;
use crate::util::Dimension;

pub struct Custom {
    buffer: Vec<String>,
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
    fn create_buffer(width: u16, height: u16) -> Vec<String> {
        (0..height)
            .map(|_| std::iter::repeat(' ').take(width as usize).collect())
            .collect()
    }

    pub fn square(&mut self, shape: Rectangle) {
        if !shape.is_inbound(self.width, self.height) {
            return;
        }

        let max_replace_range = (shape.x() + shape.w()).min(self.width) as usize;

        for cursor_y in (shape.y() as usize)..(shape.h() as usize) {
            let replace_range = (shape.x() as usize)..max_replace_range;

            self.buffer.get_mut(cursor_y)
                .map(|y_buf| y_buf.replace_range(replace_range, "█"));
        }
    }
}

impl RenderableContainer for Custom {
    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        Ok(())
    }
}
