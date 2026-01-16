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
