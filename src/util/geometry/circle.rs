use crate::util::geometry::Coordinate;
use crate::util::geometry::AddPropertiesManager;
use crate::util::geometry::AddProperties;
use crate::util::geometry::Positional;
use crate::util::geometry::Circular;
use crate::util::geometry::HasProperties;

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

