mod rectangle;
pub use rectangle::Rectangle;

mod line;
pub use line::Line;

mod point;
pub use point::Point;

mod circle;
pub use circle::Circle;

mod turtle;
pub use turtle::Turtle;
pub(crate) use turtle::TurtleAction;

mod core;

pub(crate) use core::Rect;
pub(crate) use core::Positional;
pub(crate) use core::Circular;
pub(crate) use core::Segment;
pub(crate) use core::HasProperties;
pub(crate) use core::AddPropertySlot;
pub(crate) use core::AddPropertiesManager;

pub use core::AddProperties;
pub use core::Coordinate;

pub(crate) mod linalg;
