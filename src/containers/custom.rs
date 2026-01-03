use crate::renderer::Renderer;
use crate::error::FtuiResult;
use crate::util::Coordinate;
use crate::util::Rect;
use crate::util::Positional;
use crate::util::Circular;
use crate::util::Segment;
use crate::util::Fillable;
use crate::util::Rectangle;
use crate::util::Point;
use crate::util::Circle;
use crate::util::Line;
use crate::util::Dimension;
use crate::util::Renderable;
use crate::util::RenderableMut;
use crate::util::Turtle;
use crate::util::TurtleAction;

pub struct Custom {
    buffer: Vec<Vec<char>>,
    width: u16,
    height: u16,
    cursor: char,
}

impl Custom {
    pub fn new(dimension: Dimension) -> Self {
        Self {
            buffer: Self::create_buffer(dimension.width(), dimension.height()),
            width: dimension.width(),
            height: dimension.height(),
            cursor: '█',
        }
    }

    #[inline]
    pub fn set_cursor(&mut self, c: char) {
        self.cursor = c;
    }

    #[inline]
    pub fn blit<R>(&mut self, surface: R) -> FtuiResult<()> 
    where 
        R: Renderable<Custom>,
    {
        surface.render(self)
    }

    #[inline]
    fn create_buffer(width: u16, height: u16) -> Vec<Vec<char>> {
        (0..height)
            .map(|_| (0..width).map(|_| ' ').collect())
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
    pub(crate) fn is_overflowing(&self, x: Coordinate, y: Coordinate) -> bool {
        (self.width as Coordinate) < x || (self.height as Coordinate) < y
    }

    #[inline]
    pub(crate) fn buf_set(&mut self, x: Coordinate, y: Coordinate, c: char) {
        self.buffer[y as usize][x as usize] = c;
    }

    #[inline]
    pub(crate) fn dimension(&mut self) -> (u16, u16) {
        (self.width, self.height)
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

impl Renderable<Custom> for Point {
    fn render(&self, surface: &mut Custom) -> FtuiResult<()> {
        if !surface.is_inbound(self.x(), self.y()) {
            return Ok(());
        }

        surface.buf_set(self.x(), self.y(), surface.cursor);
        Ok(())
    }
}

impl Renderable<Custom> for Rectangle {
    fn render(&self, surface: &mut Custom) -> FtuiResult<()> {
        if surface.is_overflowing(self.x(), self.y()) {
            return Ok(());
        } 

        let start_x = self.x().max(0);
        let start_y = self.y().max(0);
        let (width, height) = surface.dimension();
        let end_x = (self.x() + self.w() as Coordinate).min(width as Coordinate);
        let end_y = (self.y() + self.h() as Coordinate).min(height as Coordinate);

        if end_x < 0 || end_y < 0 {
            return Ok(());
        }

        if self.is_fill() {
            for cursor_y in start_y..end_y {
                for cursor_x in start_x..end_x {
                    surface.buf_set(cursor_x, cursor_y, surface.cursor);
                }
            }
        } else {
            let end_x_uncap = self.x() + self.w() as Coordinate;
            let end_y_uncap = self.y() + self.h() as Coordinate;

            // Avoid drawing edge if it suppose to clip.
            if surface.is_inbound_x(self.x()) {
                surface.blit(Line::new(start_x, start_y, start_x, end_y))?;
            }
            if surface.is_inbound_y(self.y()) {
                surface.blit(Line::new(start_x, start_y, end_x, start_y))?;
            }
            if surface.is_inbound_x(end_x_uncap) {
                surface.blit(Line::new(end_x - 1, start_y, end_x - 1, end_y - 1))?;
            }
            if surface.is_inbound_y(end_y_uncap) {
                surface.blit(Line::new(start_x, end_y - 1, end_x, end_y - 1))?;
            }
        }

        Ok(())
    }
}

impl Renderable<Custom> for Circle {
    fn render(&self, surface: &mut Custom) -> FtuiResult<()> {
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

            surface.blit(Point::new((x + relative_x) as C, (y + relative_y) as C))?;
            surface.blit(Point::new((x - relative_x) as C, (y + relative_y) as C))?;
            surface.blit(Point::new((x + relative_x) as C, (y - relative_y) as C))?;
            surface.blit(Point::new((x - relative_x) as C, (y - relative_y) as C))?;
            surface.blit(Point::new((x + relative_y) as C, (y + relative_x) as C))?;
            surface.blit(Point::new((x - relative_y) as C, (y + relative_x) as C))?;
            surface.blit(Point::new((x + relative_y) as C, (y - relative_x) as C))?;
            surface.blit(Point::new((x - relative_y) as C, (y - relative_x) as C))?;
        }

        Ok(())
    }
}

impl Renderable<Custom> for Line {
    fn render(&self, surface: &mut Custom) -> FtuiResult<()> {
        let (x1, y1) = self.start();
        let (x2, y2) = self.end();
        let dx = x2 - x1;
        let dy = y2 - y1;
        
        let step = std::cmp::max(dx.abs(), dy.abs());
        if step != 0 {
            let step_x = dx as f32 / step as f32;
            let step_y = dy as f32 / step as f32;
            let mut cursor_x = x1 as f32;
            let mut cursor_y = y1 as f32;

            for _ in 0..=step {
                surface.blit(
                    Point::new(
                        cursor_x.round() as Coordinate,
                        cursor_y.round() as Coordinate))?;

                cursor_x += step_x;
                cursor_y += step_y;
            }
        } else {
            surface.blit(Point::new(x1, y1))?;
        }

        Ok(())
    }
}

impl Renderable<Custom> for Turtle {
    fn render(&self, surface: &mut Custom) -> FtuiResult<()> {
        for action in self.actions() {
            use TurtleAction::*;

            match action {
                DrawLine((x1, y1), (x2, y2)) =>
                    surface.blit(Line::new(*x1, *y1, *x2, *y2))?,
                SetPen(c) => surface.set_cursor(*c),
            }
        }

        Ok(())
    }
}
