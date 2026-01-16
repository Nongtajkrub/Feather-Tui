use crate::util::geometry::Coordinate;
use crate::util::geometry::AddPropertiesManager;
use crate::util::geometry::AddProperties;
use crate::util::geometry::Positional;
use crate::util::geometry::HasProperties;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point {
    x: Coordinate,
    y: Coordinate,
    properties: AddPropertiesManager,
}

impl Point {
    pub fn new(x: Coordinate, y: Coordinate) -> Self {
        Self {
            x,
            y,
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

impl Positional for Point {
    fn x(&self) -> Coordinate {
        self.x
    }

    fn y(&self) -> Coordinate {
        self.y
    }
}

impl HasProperties for Point {
    fn props(&self) -> &AddPropertiesManager {
        &self.properties
    }
}

