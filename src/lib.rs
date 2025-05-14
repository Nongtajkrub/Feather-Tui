//! Feather-Tui is a simple terminal UI library designed to provide building blocks
//! for text-based user interfaces. It started life as a small C library in my
//! school management system project, aiming to offer an easy-to-use UI framework
//! for terminal applications. Now, I’m rewriting it in Rust to learn the language
//! and (hopefully) improve both performance and maintainability.

/// Core building blocks for constructing user interfaces.
pub mod components;
/// A generic callback handler for executing functions with stored arguments.
pub mod callback;
/// A generic trigger handler for evaluating conditions based on stored arguments.
pub mod trigger;
/// Acts as a layout manager for the UI elements.
pub mod container;
/// Responsible for rendering the UI to the terminal.
pub mod renderer;
/// Handles user input, non-blocking key events, and key code conversions with crossterm.
pub mod input;
/// Provides custom error types and a result type alias for error handling in `Feather-TUI`.
pub mod error;

mod     util;

pub use components as cpn;
pub use callback   as cbk;
pub use trigger    as trg;
pub use container  as con;
pub use renderer   as ren;
pub use input      as inp;
pub use error      as err;

#[cfg(feature = "reduce_abstraction")]
pub use cpn::{Header, Option, Text, TextFlags, Separator, SeparatorStyle, Selector};

#[cfg(feature = "reduce_abstraction")]
pub use cbk::Callback;

#[cfg(feature = "reduce_abstraction")]
pub use trg::Trigger;

#[cfg(feature = "reduce_abstraction")]
pub use con::{Container, ContainerBuilder};

#[cfg(feature = "reduce_abstraction")]
pub use ren::Renderer;

#[cfg(feature = "reduce_abstraction")]
pub use inp::{line, key, keycode_to_char, key_char};

#[cfg(feature = "reduce_abstraction")]
pub use err::{FtuiError, FtuiResult};
