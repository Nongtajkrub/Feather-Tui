//! Feather-Tui is a simple terminal UI library designed to provide building blocks
//! for text-based user interfaces. It started life as a small C library in my
//! school management system project, aiming to offer an easy-to-use UI framework
//! for terminal applications. Now, I’m rewriting it in Rust to learn the language
//! and (hopefully) improve both performance and maintainability.

/// Core building blocks for constructing user interfaces.
pub mod components;
/// Acts as a layout manager for the UI elements.
pub mod containers;
/// Responsible for rendering the UI to the terminal.
pub mod renderer;
/// Handles user input, non-blocking key events, and key code conversions with crossterm.
pub mod input;
/// Provides custom error types and a result type alias for error handling in `Feather-TUI`.
pub mod error;
pub mod terminal;

mod     util;
