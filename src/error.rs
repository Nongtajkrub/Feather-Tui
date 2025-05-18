use thiserror::Error;
use std::io;

/// An `enum` representing all possible errors that can occur in `Feather-TUI`.
///
/// # Derives
/// `thiserror::Error`, `Debug`
///
/// # PartialEq Implementation
/// This is necessary because the `StdInputOutputError` variant wraps a value of
/// type `std::io::Error`, which does not implement `PartialEq`. As a result, we
/// implement `PartialEq` manually, comparing all variants normally while treating
/// any two `StdInputOutputError` values as equal regardless of their internal
/// `io::Error`.
#[repr(u8)]
#[derive(Error, Debug)]
pub enum FtuiError {
    /// Occurs when `TextFlags::NONE` is used together with other flags.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Using `TextFlags::NONE` with TextFlags::COLOR_RED results in an error.
    ///     Text::new("Label", TextFlags::NONE | TextFlags::COLOR_RED)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    #[error("TextFlags::NONE cannot be combined with other TextFlags.")]
    TextFlagNoneWithOther,

    /// Occurs when multiple color flags are set for a `Text` component.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Setting both `COLOR_BLUE` and `COLOR_RED` results in an error.
    ///     Text::new("Label", TextFlags::COLOR_BLUE | TextFlags::COLOR_RED)?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    #[error("TextFlags cannot contain multiple color.")]
    TextFlagMultipleColor,

    /// Occurs when attempting to create a `Header` component with an empty label.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Creating a header with an empty label results in an error.
    ///     Header::new("")?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    #[error("A Header label cannot be empty.")]
    HeaderLabelEmpty,

    /// Occurs when attempting to create an `Option` component with an empty label.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Creating an option with an empty label results in an error.
    ///     // Assuming the callback is created elsewhere.
    ///     Option::new("", ...)?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    #[error("An Option label cannot be empty.")]
    OptionLabelEmpty,

    /// Occurs when calling `Container::loop` on a container that has `Option`
    /// components but does not have a `Selector`.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Create a container with an option component. 
    ///     let mut container = ContainerBuilder::new() 
    ///         .option(...)?;
    ///     
    ///     // Attempting to call the loop method without a selector
    ///     container.looper()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("The container's looper method requires a Selector.")]
    ContainerLooperNoSelector,

    /// Occurs when attempting to use `Container` functionality that
    /// requires a `Selector`, but the `Container` does not have one.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Create a container without a selector. 
    ///     let mut container = ContainerBuilder::new().build();
    ///     
    ///     // Attempting to call `selector_mut` on a container without a selector
    ///     // results in the error.
    ///     container.selector_mut()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("Container doesnot have a Selector.")]
    ContainerNoSelector,

    /// Occurs when attempting to query a component by its ID, but no such
    /// component exists in the container.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Create an empty container.
    ///     let mut container = ContainerBuilder::new().build();
    ///
    ///     // Attempt to query using a non-existent ID.
    ///     // This results in the error.
    ///     container.option(172)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("Failed to query for component by its ID")]
    ContainerNoComponentById,

    /// Occurs when attempting to use `Selector` functionality that
    /// requires triggers, but the `Selector` does not have one.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     // Create a `Selector` component with no triggers.
    ///     let mut selector = Selector::no_triggers();
    ///
    ///     // Attempting to use functionality that requires triggers.
    ///     // This results in the error.
    ///     selector.up_trig_mut()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("Selector does not have triggers.")]
    SelectorNoTriggers,

    /// Occurs when attempting to call the `Renderer::render` method with a container
    /// that exceeds the dimensions of the renderer. There are two cases where a
    /// container is considered "too big":
    /// 
    /// 1. A component's label is longer than the renderer's width.
    /// 2. The total number of components exceeds the renderer's height.
    /// 
    /// # Example
    /// ```rust
    /// fn main() -> FtuiResult<()> {
    ///     let mut container = ContainerBuilder::new()
    ///         .header("Header!")?
    ///         .text("Label", None)?;
    ///
    ///     // This will cause an error because the label "Header!" is 7 characters
    ///     // long, which is wider than the renderer width of 5.
    ///     let mut renderer = Renderer::new(5, 10);
    ///     renderer.render(&mut container)?;
    ///
    ///     // This will cause an error because the container has 2 components,
    ///     // but the renderer can only display 1 line (height = 1).
    ///     let mut renderer = Renderer::new(10, 1);
    ///     renderer.render(&mut container)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("Container is bigger than what the renderer can accommodate.")]
    RendererContainerTooBig,

    /// Occurs when functions in the `input` module fail. Affected functions 
    /// include `line`, `key`, and `key_char`. This enum wraps an error
    /// from `std::io::Error`.
    ///
    /// # Example
    /// ```rust
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // This function may return an error if an I/O operation fails.
    ///     line("Prompt")?;
    /// 
    ///     // This function may return an error if an I/O operation fails.
    ///     key()?;
    /// 
    ///     // This function may return an error if an I/O operation fails.
    ///     key_char()?;
    /// 
    ///     Ok(())
    /// }
    /// ```
    #[error("Std Input Output Error: {0}")]
    StdInputOutputError(#[from] io::Error),

    /// Occurs when calling the `trigger::cast_arg` function with an argument 
    /// that is a `None`.
    ///
    /// # Notes
    /// - The trigger function argument has the type `&Option<Box<dyn Any>>`.
    ///
    /// # Example
    /// ```rust
    /// // When creating a trigger using the no_arg constructor, the argument
    /// // will be set to None.
    /// Trigger::no_arg(trigger_func);
    ///
    /// trg_new_trigger_func!(trigger_func, arg, {
    ///     // An error occurs because arg is None.
    ///     trg::cast_arg::<T>(arg)?;
    /// });
    ///
    /// ```
    #[error("Trigger function does not have an argument available for casting.")]
    TriggerCastArgNoArgument,

    /// Occurs when calling the `trigger::cast_arg` function with an argument of
    /// the wrong type.
    ///
    /// # Notes
    /// - The trigger function argument has the type `&Option<Box<dyn Any>>`.
    ///
    /// # Example
    /// ```rust
    /// // Creating a trigger with an argument of 5, which is a u32.
    /// Trigger::new(trigger_func, 5u32);
    /// 
    /// trg_new_trigger_func!(trigger_func, arg, {
    ///     // An error occurs because arg is a u32, but we're attempting to cast 
    ///     // it to a char.
    ///     trg::cast_arg::<char>(arg)?;
    /// });
    /// ```
    #[error("Trigger function argument type mismatch unable to cast to the expected type.")]
    TriggerCastArgWrongType,

    /// Occurs when calling the `callback::cast_arg` function with an argument 
    /// that is a `None`.
    ///
    /// # Notes
    /// - The callback function argument has the type `&Option<Box<dyn Any>>`.
    ///
    /// # Example
    /// ```rust
    /// // When creating a callback using the no_arg constructor, the argument
    /// // will be set to None.
    /// cbk::Callback::no_arg(callback_func);
    ///
    /// cbk_new_callback_func!(callback_func, arg, {
    ///     // An error occurs because arg is None.
    ///     cbk::cast_arg::<T>(arg)?;
    /// });
    ///
    /// ```
    #[error("Callback function does not have an argument available for casting.")]
    CallbackCastArgNoArgument,

    /// Occurs when calling the `callback::cast_arg` function with an argument of
    /// the wrong type.
    ///
    /// # Notes
    /// - The callback function argument has the type `&Option<Box<dyn Any>>`.
    ///
    /// # Example
    /// ```rust
    /// // Creating a callback with an argument of 5, which is a u32.
    /// Callback::new(callback_func, 5u32);
    /// 
    /// callback_new_callback_func!(callback_func, arg, {
    ///     // An error occurs because arg is a u32, but we're attempting to cast 
    ///     // it to a char.
    ///     cbk::cast_arg::<char>(arg)?;
    /// });
    /// ```
    #[error("Callback function argument type mismatch unable to cast to the expected type.")]
    CallbackCastArgWrongType,
}

/// Implementation of the `PartialEq` trait for the `FtuiError` enum. This is necessary
/// because the `StdInputOutputError` variant wraps a value of type `std::io::Error`,
/// which does not implement `PartialEq`. As a result, we implement `PartialEq`
/// manually, comparing all variants normally while treating any two `StdInputOutputError`
/// values as equal regardless of their internal `io::Error`.
///
/// # Examples
///
/// ```rust
/// // Variants of the same type are considered equal.
/// assert_eq!(
///     FtuiError::TextFlagNoneWithOther, FtuiError::TextFlagNoneWithOther);
///
/// // Variants of different types are not equal.
/// assert_ne!(
///     FtuiError::TextFlagNoneWithOther, FtuiError::TextFlagMultipleColor);
///
/// // StdInputOutputError variants are treated as equal even if their inner errors differ.
/// use std::io::{Error, ErrorKind};
/// 
/// assert_eq!(
///     FtuiError::StdInputOutputError(Error::from(ErrorKind::NotFound)),
///     FtuiError::StdInputOutputError(Error::from(ErrorKind::PermissionDenied)));
/// ```
impl PartialEq for FtuiError {
    fn eq(&self, other: &Self) -> bool {
        use FtuiError::*;

        match (self, other) {
            (TextFlagNoneWithOther, TextFlagNoneWithOther) => true,
            (TextFlagMultipleColor, TextFlagMultipleColor) => true,
            (HeaderLabelEmpty, HeaderLabelEmpty) => true,
            (OptionLabelEmpty, OptionLabelEmpty) => true,
            (ContainerLooperNoSelector, ContainerLooperNoSelector) => true,
            (ContainerNoSelector, ContainerNoSelector) => true,
            (ContainerNoComponentById, ContainerNoComponentById) => true,
            (SelectorNoTriggers, SelectorNoTriggers) => true,
            (RendererContainerTooBig, RendererContainerTooBig) => true,
            (StdInputOutputError(_), StdInputOutputError(_)) => true,
            (TriggerCastArgNoArgument, TriggerCastArgNoArgument) => true,
            (TriggerCastArgWrongType, TriggerCastArgWrongType) => true,
            (CallbackCastArgNoArgument, CallbackCastArgNoArgument) => true,
            (CallbackCastArgWrongType, CallbackCastArgWrongType) => true,

            _ => false,
        }
    }
}

/// A convenient alias for `Result<T, FtuiError>`.
///
/// # Example
/// ```rust
/// // A main function that returns a Result<(), FtuiError>.
/// fn main() -> FtuiResult<()> {
///     Ok(())
/// }
// ```
pub type FtuiResult<T> = Result<T, FtuiError>;
