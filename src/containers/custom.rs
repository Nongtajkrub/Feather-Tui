use std::usize;

use crate::renderer::Renderer;
use crate::renderer::RenderableContainer;
use crate::error::FtuiResult;
use crate::util::Dimension;

pub struct Custom {
    buffer: Vec<Vec<char>>,
    width: u16,
    height: u16,
}

/// Implementation detail, not intended for direct use.
/// 
/// This trait is automatically implemented for shapes types.
pub trait RasterizableShape {
    /// Implementation detail. Use `Custom::blit` instead.
    fn rasterize(&self, container: &mut Custom) -> FtuiResult<()>;
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

    #[inline]
    pub(crate) fn is_inbound(&self, x: u16, y: u16) -> bool {
        self.width >= x && self.height >= y
    }

    #[inline]
    pub(crate) fn get_dimensions(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    #[inline]
    pub(crate) fn buf_set(&mut self, x: u16, y: u16, c: char) {
        self.buffer[y as usize][x as usize] = c;
    }

    #[inline]
    pub(crate) fn buffer(&self) -> &[Vec<char>] {
        &self.buffer
    }

    #[inline]
    pub fn blit<R>(&mut self, shape: R) -> FtuiResult<()> 
    where 
        R: RasterizableShape,
    {
        shape.rasterize(self)
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
