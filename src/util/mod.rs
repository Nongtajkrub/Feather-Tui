pub(crate) mod ansi;
pub(crate) mod id;
pub(crate) mod number;

mod color;
pub use color::Colors;

mod dimension;
pub use dimension::Dimension;

mod shape;
pub(crate) use shape::Rect;
pub(crate) use shape::Positional;
pub(crate) use shape::Circular;
pub(crate) use shape::Fillable;
pub(crate) use shape::Segment;
pub use shape::Coordinate;
pub use shape::Rectangle;
pub use shape::Point;
pub use shape::Circle;
pub use shape::Line;

mod traits;
pub(crate) use traits::Renderable;
pub(crate) use traits::RenderableMut;
