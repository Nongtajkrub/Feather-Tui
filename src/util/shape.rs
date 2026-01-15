use std::usize;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddProperties {
    Rotate(i32),
    Fill,
}

impl AddProperties {
    pub(crate) fn slot(&self) -> AddPropertySlot {
        match self {
            Self::Rotate(_) => AddPropertySlot::Rotate,
            Self::Fill => AddPropertySlot::Fill,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AddPropertySlot {
    Rotate = 0,
    Fill = 1,
}

impl AddPropertySlot {
    pub(crate) const COUNT: usize = 2;
}

type AddPropertiesArray = [Option<AddProperties>; AddPropertySlot::COUNT];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AddPropertiesManager {
    properties: AddPropertiesArray,
}

impl AddPropertiesManager {
    fn new() -> Self {
        Self {
            properties: std::array::from_fn(|_| None),
        }
    }

    #[inline]
    pub(crate) fn apply_iter<I>(&mut self, properties: I)
    where 
        I: IntoIterator<Item = AddProperties>
    {
        properties.into_iter().for_each(|prop| self.apply(prop));
    }

    #[inline]
    pub(crate) fn apply(&mut self, property: AddProperties) {
        let slot = property.slot();
        self.properties[slot as usize] = Some(property);
    }

    #[inline]
    pub(crate) fn get(&self, slot: AddPropertySlot) -> &Option<AddProperties> {
        &self.properties[slot as usize]
    }

    #[inline]
    pub(crate) fn is_exist(&self, slot: AddPropertySlot) -> bool {
        self.properties[slot as usize].is_some()
    }
}

macro_rules! impl_apply_prop_methods {
    () => {
        #[inline]
        pub fn apply_iter<I>(mut self, props: I) -> Self
        where 
            I: IntoIterator<Item = AddProperties>
        {
            self.properties.apply_iter(props);
            self
        }

        #[inline]
        pub fn apply(mut self, props: AddProperties) -> Self {
            self.properties.apply(props);
            self
        }
    };
}

pub(crate) trait HasProperties {
    fn props(&self) -> &AddPropertiesManager; 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rectangle {
    x: Coordinate,
    y: Coordinate,
    w: u16,
    h: u16,
    properties: AddPropertiesManager,
}

impl Rectangle {
    pub fn new(
        x: Coordinate, y: Coordinate, w: u16, h: u16
    )-> Self {
        Self {
            x,
            y,
            w,
            h,
            properties: AddPropertiesManager::new(),
        }
    }

    impl_apply_prop_methods!();
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

impl HasProperties for Rectangle {
    fn props(&self) -> &AddPropertiesManager {
        &self.properties
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
    properties: AddPropertiesManager,
}

impl Circle {
    pub fn new(x: Coordinate, y: Coordinate, r: u16, fill: bool) -> Self {
        Self {
            x,
            y,
            r,
            is_fill: fill,
            properties: AddPropertiesManager::new(),
        }
    }

    impl_apply_prop_methods!();
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

impl HasProperties for Circle {
    fn props(&self) -> &AddPropertiesManager {
        &self.properties
    }
} 

pub struct Line {
    start: Point,
    end: Point,
    properties: AddPropertiesManager,
}

impl Line {
    pub fn new(
        x1: Coordinate, y1: Coordinate, x2: Coordinate, y2: Coordinate
    ) -> Self {
        Self {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
            properties: AddPropertiesManager::new(),
        }
    }

    impl_apply_prop_methods!();
}

impl Segment for Line {
    fn start(&self) -> (Coordinate, Coordinate) {
        (self.start.x(), self.start.y())
    }

    fn end(&self) -> (Coordinate, Coordinate) {
        (self.end.x(), self.end.y())
    }
}

impl HasProperties for Line {
    fn props(&self) -> &AddPropertiesManager {
        &self.properties
    }
}
