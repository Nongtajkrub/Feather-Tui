//! Feather-Tui is a simple terminal UI library designed to provide building blocks for text-based user interfaces. It started life as a small C library in my school management system project, aiming to offer an easy-to-use UI framework for terminal applications. Now, I’m rewriting it in Rust to learn the language and (hopefully) improve both performance and maintainability.

/// Core building blocks for constructing user interfaces.
pub mod components;
/// A generic callback handler for executing functions with stored arguments.
pub mod callback;
/// A generic trigger handler for evaluating conditions based on stored arguments.
pub mod trigger;
/// Used within a `Container` to navigate and select `Option`. 
pub mod selector;
/// Acts as a layout manager for the UI elements.
pub mod container;
/// Responsible for rendering the UI to the terminal.
pub mod renderer;
/// Handles user input, non-blocking key events, and key code conversions with crossterm.
pub mod input;

mod     util;
mod     errmsg;

pub use components as cpn;
pub use callback   as cbk;
pub use trigger    as trg;
pub use selector   as slc;
pub use container  as con;
pub use renderer   as ren;
pub use input      as inp;

use errmsg         as emg;
