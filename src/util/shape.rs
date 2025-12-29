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
