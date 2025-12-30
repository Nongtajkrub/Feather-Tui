pub type Coordinate = i32;

pub(crate) trait Rect {
    fn w(&self) -> u16;
    fn h(&self) -> u16;
}

pub(crate) trait Positional {
    fn x(&self) -> Coordinate;
    fn y(&self) -> Coordinate;
}

pub(crate) trait Circular {
    fn r(&self) -> u16;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    x: Coordinate,
    y: Coordinate,
    w: u16,
    h: u16,
}

impl Rectangle {
    pub fn new(x: Coordinate, y: Coordinate, w: u16, h: u16) -> Self {
        Self {
            x,
            y,
            w,
            h,
        }
    }
}

impl Positional for Rectangle {
    fn x(&self) -> Coordinate {
        self.x
    }

    fn y(&self) -> Coordinate {
        self.y
    }
}

impl Rect for Rectangle {
    fn w(&self) -> u16 {
        self.w
    }

    fn h(&self) -> u16 {
        self.h
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    x: Coordinate,
    y: Coordinate,
}

impl Point {
    pub fn new(x: Coordinate, y: Coordinate) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Positional for Point {
    fn x(&self) -> Coordinate {
        self.x
    }

    fn y(&self) -> Coordinate {
        self.y
    }
}

pub struct Circle {
    x: Coordinate,
    y: Coordinate,
    r: u16,
}

impl Circle {
    pub fn new(x: Coordinate, y: Coordinate, r: u16) -> Self {
        Self {
            x,
            y,
            r,
        }
    }
}

impl Positional for Circle {
    fn x(&self) -> Coordinate {
        self.x
    }

    fn y(&self) -> Coordinate {
        self.y
    }
}

impl Circular for Circle {
    fn r(&self) -> u16 {
        self.r
    }
}
