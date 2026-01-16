use crate::util::geometry::Coordinate;
use crate::util::geometry::AddPropertiesManager;
use crate::util::geometry::AddProperties;
use crate::util::geometry::Positional;
use crate::util::geometry::Segment;
use crate::util::geometry::HasProperties;
use crate::util::geometry::Point;

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
