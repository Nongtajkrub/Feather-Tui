/// A UI component that serves as the header of a `Container`. 
pub mod header;
/// A UI component representing an interactive option in a `Container`. 
pub mod option;
/// A UI component representing a text element in a `Container`.
pub mod text;
pub mod seperator;
pub mod selector;

pub use header    as hed;
pub use option    as opt;
pub use text      as txt;
pub use seperator as sep;
pub use selector  as slc;

pub use hed::Header;
pub use opt::Option;
pub use txt::{Text, TextFlags};
pub use sep::{Separator, SeparatorStyle};
pub use slc::Selector;
