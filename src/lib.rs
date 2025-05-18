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

// Shorten modules name.

#[cfg(feature = "shorten_mod_name")]
pub use components as cpn;

#[cfg(feature = "shorten_mod_name")]
pub use callback as cbk;

#[cfg(feature = "shorten_mod_name")]
pub use trigger as trg;

#[cfg(feature = "shorten_mod_name")]
pub use container as con;

#[cfg(feature = "shorten_mod_name")]
pub use renderer as ren;

#[cfg(feature = "shorten_mod_name")]
pub use input as inp;

#[cfg(feature = "shorten_mod_name")]
pub use error as err;

// Reducing abstraction.

#[cfg(feature = "reduce_abstraction")]
pub use components::{Header, Option, Text, TextFlags, Separator, SeparatorStyle, Selector};

#[cfg(feature = "reduce_abstraction")]
pub use callback::Callback;

#[cfg(feature = "reduce_abstraction")]
pub use trigger::Trigger;

#[cfg(feature = "reduce_abstraction")]
pub use container::{Container, ContainerBuilder};

#[cfg(feature = "reduce_abstraction")]
pub use renderer::{ready, unready, Renderer};

#[cfg(feature = "reduce_abstraction")]
pub use input::{line, key, keycode_to_char, key_char};

#[cfg(feature = "reduce_abstraction")]
pub use error::{FtuiError, FtuiResult};
