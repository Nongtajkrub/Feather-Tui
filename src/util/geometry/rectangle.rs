use crate::util::geometry::Coordinate;
use crate::util::geometry::AddPropertiesManager;
use crate::util::geometry::AddProperties;
use crate::util::geometry::Positional;
use crate::util::geometry::Rect;
use crate::util::geometry::HasProperties;

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

