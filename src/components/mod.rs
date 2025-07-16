/// A UI component representing an interactive option in a `Container`. 
pub(crate) mod option;
/// A UI component representing a text element in a `Container`.
pub(crate) mod text;
/// A UI component that acts as a separator typically a horizontal line.
pub(crate) mod seperator;

pub use option::{Option, OptionsManager};
pub use text::{Text, TextFlags, TextsManager};
pub use seperator::{Separator, SeparatorStyle};
