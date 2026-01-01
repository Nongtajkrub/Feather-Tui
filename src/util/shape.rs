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

pub(crate) trait Segment {
    fn start(&self) -> (Coordinate, Coordinate);
    fn end(&self) -> (Coordinate, Coordinate);
}

pub(crate) trait Fillable {
    fn is_fill(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle {
    x: Coordinate,
    y: Coordinate,
    w: u16,
    h: u16,
    is_fill: bool,
}

impl Rectangle {
    pub fn new(x: Coordinate, y: Coordinate, w: u16, h: u16, fill: bool) -> Self {
        Self {
            x,
            y,
            w,
            h,
            is_fill: fill,
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

impl Fillable for Rectangle {
    fn is_fill(&self) -> bool {
        self.is_fill
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Circle {
    x: Coordinate,
    y: Coordinate,
    r: u16,
    is_fill: bool,
}

impl Circle {
    pub fn new(x: Coordinate, y: Coordinate, r: u16, fill: bool) -> Self {
        Self {
            x,
            y,
            r,
            is_fill: fill,
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

impl Fillable for Circle {
    fn is_fill(&self) -> bool {
        self.is_fill
    }
}

pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(
        x1: Coordinate, y1: Coordinate, x2: Coordinate, y2: Coordinate
    ) -> Self {
        Self {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }
}

impl Segment for Line {
    fn start(&self) -> (Coordinate, Coordinate) {
        (self.start.x(), self.start.y())
    }

    fn end(&self) -> (Coordinate, Coordinate) {
        (self.end.x(), self.end.y())
    }
}
