//! Feather-Tui is a simple terminal UI library designed to provide building blocks
//! for text-based user interfaces. It started life as a small C library in my
//! school management system project, aiming to offer an easy-to-use UI framework
//! for terminal applications. Now, I’m rewriting it in Rust to learn the language
//! and (hopefully) improve both performance and maintainability.
//!
//! # Features
//!
//! - `shorten_mod_name` shortened aliases for common modules to reduce verbosity.
//!     - `components` -> `cpn`
//!     - `container` -> `con`
//!     - `list` -> `lis`
//!     - `renderer` -> `ren`
//!     - `input` -> `inp`
//!     - `error` -> `err`
//! 
//! - `reduce_abstraction` flattens module paths to make the API more direct.
//!     - `feather_tui::components::Option` → `feather_tui::Option`
//!     - `feather_tui::renderer::Renderer` → `feather_tui::Renderer`
//!     - `feather_tui::input::key_char` → `feather_tui::key_char
//!     - ...

/// Core building blocks for constructing user interfaces.
pub mod components;
/// Acts as a layout manager for the UI elements.
pub mod container;
pub mod document;
pub mod list;
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
pub use container as con;

#[cfg(feature = "shorten_mod_name")]
pub use list as lst;

#[cfg(feature = "shorten_mod_name")]
pub use document as doc;

#[cfg(feature = "shorten_mod_name")]
pub use renderer as ren;

#[cfg(feature = "shorten_mod_name")]
pub use input as inp;

#[cfg(feature = "shorten_mod_name")]
pub use error as err;

// Reducing abstraction.

#[cfg(feature = "reduce_abstraction")]
pub use components::{Option, Text, TextFlags, Separator, SeparatorStyle};

#[cfg(feature = "reduce_abstraction")]
pub use container::{Container, ContainerBuilder};

#[cfg(feature = "reduce_abstraction")]
pub use list::{List, ListBuilder};

#[cfg(feature = "reduce_abstraction")]
pub use document::{Document, DocumentBuilder};

#[cfg(feature = "reduce_abstraction")]
pub use renderer::{ready, unready, Renderer};

#[cfg(feature = "reduce_abstraction")]
pub use input::{line, key, keycode_to_char, key_char};

#[cfg(feature = "reduce_abstraction")]
pub use error::{FtuiError, FtuiResult};
