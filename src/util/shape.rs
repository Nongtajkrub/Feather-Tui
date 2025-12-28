pub(crate) trait Rect {
    fn x(&self) -> u16;
    fn y(&self) -> u16;
    fn w(&self) -> u16;
    fn h(&self) -> u16;

    fn is_inbound(&self, bound_w: u16, bound_h: u16) -> bool {
        self.x() > bound_w || self.h() > bound_h
    }
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
