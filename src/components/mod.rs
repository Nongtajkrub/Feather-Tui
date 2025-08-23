/// A UI component representing an interactive option in a `Container`. 
pub(crate) mod option;
/// A UI component representing a text element in a `Container`.
pub(crate) mod text;
/// A UI component that acts as a separator typically a horizontal line.
pub(crate) mod seperator;

pub use option::Option;
pub use option::OptionsManager;
pub use text::Text;
pub use text::TextFlags;
pub use text::TextsManager;
pub use seperator::Separator;
pub use seperator::SeparatorStyle;
