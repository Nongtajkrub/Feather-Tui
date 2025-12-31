use std::usize;

use crossterm::cursor;

use crate::renderer::Renderer;
use crate::error::FtuiResult;
use crate::util::Coordinate;
use crate::util::Rect;
use crate::util::Positional;
use crate::util::Circular;
use crate::util::Fillable;
use crate::util::Rectangle;
use crate::util::Point;
use crate::util::Circle;
use crate::util::Dimension;
use crate::util::Renderable;
use crate::util::RenderableMut;

pub struct Custom {
    buffer: Vec<Vec<char>>,
    width: u16,
    height: u16,
}

impl Renderable<Custom> for Point {
    fn render(&self, container: &mut Custom) -> FtuiResult<()> {
        if !container.is_inbound(self.x(), self.y()) {
            return Ok(());
        }

        container.buf_set(self.x(), self.y(), '█');
        Ok(())
    }
}

impl Renderable<Custom> for Rectangle {
    fn render(&self, container: &mut Custom) -> FtuiResult<()> {
        let start_x = self.x().max(0);
        let start_y = self.y().max(0);
        let end_x = self.x() + self.w() as Coordinate;
        let end_y = self.y() + self.h() as Coordinate;

        if end_x < 0 || end_y < 0 {
            return Ok(());
        }

        if self.is_fill() {
            for cursor_y in start_y..end_y {
                for cursor_x in start_x..end_x {
                    container.buf_set(cursor_x, cursor_y, '█');
                }
            }
        } else {
            todo!("Implement Line Drawing First!");
        }

        Ok(())
    }
}

impl Renderable<Custom> for Circle {
    fn render(&self, container: &mut Custom) -> FtuiResult<()> {
        let x = self.x() as i32;
        let y = self.y() as i32;
        let r = self.r() as i32;
        let mut relative_y = -r;
        let mut decision_param = relative_y;

        for relative_x in 0..r {
            if decision_param > 0 {
                relative_y += 1;
                decision_param += 2 * (relative_x + relative_y) + 1;
            } else {
                decision_param += (2 * relative_x) + 1 
            }

            use crate::util::Coordinate as C;

            let _ = container
                .blit(Point::new((x + relative_x) as C, (y + relative_y) as C));
            let _ =container
                .blit(Point::new((x - relative_x) as C, (y + relative_y) as C));
            let _ = container
                .blit(Point::new((x + relative_x) as C, (y - relative_y) as C));
            let _ = container
                .blit(Point::new((x - relative_x) as C, (y - relative_y) as C));
            let _ = container
                .blit(Point::new((x + relative_y) as C, (y + relative_x) as C));
            let _ = container
                .blit(Point::new((x - relative_y) as C, (y + relative_x) as C));
            let _ =container
                .blit(Point::new((x + relative_y) as C, (y - relative_x) as C));
            let _ = container
                .blit(Point::new((x - relative_y) as C, (y - relative_x) as C));
        }

        Ok(())
    }
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
            .map(|_| (0..width).map(|_| 'X').collect())
            .collect()
    }

    #[inline]
    pub(crate) fn is_inbound_x(&self, x: Coordinate) -> bool {
        self.width as Coordinate >= x && x >= 0
    }

    #[inline]
    pub(crate) fn is_inbound_y(&self, y: Coordinate) -> bool {
        self.height as Coordinate >= y && y >= 0
    }

    #[inline]
    pub(crate) fn is_inbound(&self, x: Coordinate, y: Coordinate) -> bool {
        self.is_inbound_x(x) && self.is_inbound_y(y)
    }

    #[inline]
    pub(crate) fn buf_set(&mut self, x: Coordinate, y: Coordinate, c: char) {
        self.buffer[y as usize][x as usize] = c;
    }

    #[inline]
    pub fn blit<R>(&mut self, surface: R) -> FtuiResult<()> 
    where 
        R: Renderable<Custom>,
    {
        surface.render(self)
    }
}

impl RenderableMut<Renderer> for Custom {
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
