use crate::containers::custom::RasterizableShape;
use crate::containers::Custom;
use crate::error::FtuiResult;
use crate::error::FtuiError;

pub(crate) trait Rect {
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    fn w(&self) -> u16;
    fn h(&self) -> u16;
}

pub(crate) trait Positional {
    fn x(&self) -> u16;
    fn y(&self) -> u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl Rectangle {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }
}

impl Rect for Rectangle {
    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
        self.y
    }

    fn w(&self) -> u16 {
        self.w
    }

    fn h(&self) -> u16 {
        self.h
    }
}

impl RasterizableShape for Rectangle {
    fn rasterize(&self, container: &mut Custom) -> FtuiResult<()> {
        if !container.is_inbound(self.x(), self.y()) {
            return Err(FtuiError::CustomContainerBlitOutOfBound);
        }

        let (width, height) = container.get_dimensions();

        let max_width = (self.x() + self.w()).min(width);
        let max_height = (self.y() + self.h()).min(height);

        for cursor_y in self.y()..max_height + 1 {
            for cursor_x in self.x()..max_width + 1 {
                container.buf_set(cursor_x - 1, cursor_y - 1, '█');
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Positional for Point {
    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
        self.y
    }
}

impl RasterizableShape for Point {
    fn rasterize(&self, container: &mut Custom) -> FtuiResult<()> {
        if !container.is_inbound(self.x(), self.y()) {
            return Err(FtuiError::CustomContainerBlitOutOfBound);
        }

        container.buf_set(self.x() - 1, self.y() - 1, '█');
        Ok(())
    }
}
