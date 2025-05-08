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
    /// use feather_tui as tui;
    /// 
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // Using `TextFlags::NONE` with TextFlags::COLOR_RED results in an error.
    ///     tui::cpn::txt::Text::new(
    ///         "Label",
    ///         tui::cpn::txt::TextFlags::NONE |
    ///         tui::cpn::txt::TextFlags::COLOR_RED)?;
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
    /// use feather_tui as tui;
    /// 
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // Setting both `COLOR_BLUE` and `COLOR_RED` results in an error.
    ///     tui::cpn::txt::Text::new(
    ///         "Label",
    ///         tui::cpn::txt::TextFlags::COLOR_BLUE |
    ///         tui::cpn::txt::TextFlags::COLOR_RED)?;
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
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // Creating a header with an empty label results in an error.
    ///     tui::cpn::hed::Header::new("")?;
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
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // Creating an option with an empty label results in an error.
    ///     // Assuming the callback is created elsewhere.
    ///     tui::cpn::opt::Option::new("", callback)?;
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
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // Create a container with an option component. 
    ///     let mut container = tui::con::Container::new()
    ///         .with_option("Option!", callback)?;
    ///     
    ///     // Attempting to call the loop method without a selector
    ///     container.looper()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("The container's looper method requires a Selector.")]
    ContainerLooperNoSelector,

    /// Occurs when calling `Container::selector_mut` on a container that does
    /// not have a `Selector`.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // Create a container without a selector. 
    ///     let mut container = tui::con::Container::new();
    ///     
    ///     // Attempting to call `selector_mut` on a container without a selector
    ///     // results in an error.
    ///     container.selector_mut()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("Container doesnot have a Selector.")]
    ContainerNoSelector,

    #[error("Fail to find option by ID")]
    ContainerNoOptionById,

    /// Occurs when attempting to call the `Renderer::render` method with a container
    /// that exceeds the dimensions of the renderer. There are two cases where a
    /// container is considered "too big":
    /// 
    /// 1. A component's label is longer than the renderer's width.
    /// 2. The total number of components exceeds the renderer's height.
    /// 
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     let mut container = tui::con::Container::new()
    ///         .with_header("Header!")?
    ///         .with_text("Label", None)?;
    ///
    ///     // This will cause an error because the label "Header!" is 7 characters
    ///     // long, which is wider than the renderer width of 5.
    ///     let mut renderer = tui::ren::Renderer::new(5, 10);
    ///     renderer.render(&mut container)?;
    ///
    ///     // This will cause an error because the container has 2 components,
    ///     // but the renderer can only display 1 line (height = 1).
    ///     let mut renderer = tui::ren::Renderer::new(10, 1);
    ///     renderer.render(&mut container)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[error("Container is bigger than what the renderer can accommodate.")]
    RendererContainerTooBig,

    /// Occurs when functions in the `input` module fail. Affected functions 
    /// include `line`, `key`, and `key_char`. This enum wraps an error message
    /// from `std::io::Error`.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    /// 
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     // This function may return an error if an I/O operation fails.
    ///     tui::inp::line("Prompt")?;
    /// 
    ///     // This function may return an error if an I/O operation fails.
    ///     tui::inp::key()?;
    /// 
    ///     // This function may return an error if an I/O operation fails.
    ///     tui::inp::key_char()?;
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
    /// use feather_tui as tui;
    ///
    /// // When creating a trigger using the no_arg constructor, the argument
    /// // will be set to None.
    /// tui::trg::Trigger::no_arg(trigger_func);
    ///
    /// tui::trg_new_trigger_func!(trigger_func, arg, {
    ///     // An error occurs because arg is None.
    ///     tui::trg::cast_arg::<T>(arg)?;
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
    /// use feather_tui as tui;
    ///
    /// // Creating a trigger with an argument of 5, which is a u32.
    /// tui::trg::Trigger::new(trigger_func, 5u32);
    /// 
    /// tui::trg_new_trigger_func!(trigger_func, arg, {
    ///     // An error occurs because arg is a u32, but we're attempting to cast 
    ///     // it to a char.
    ///     tui::trg::cast_arg::<char>(arg)?;
    /// });
    /// ```
    #[error("Trigger function argument type mismatch unable to cast to the expected type.")]
    TriggerCastArgWrongType,

    #[error("")]
    SelectorNoTriggers,

    /// Occurs when calling the `callback::cast_arg` function with an argument 
    /// that is a `None`.
    ///
    /// # Notes
    /// - The callback function argument has the type `&Option<Box<dyn Any>>`.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // When creating a callback using the no_arg constructor, the argument
    /// // will be set to None.
    /// tui::cbk::Callback::no_arg(callback_func);
    ///
    /// tui::cbk_new_callback_func!(callback_func, arg, {
    ///     // An error occurs because arg is None.
    ///     tui::cbk::cast_arg::<T>(arg)?;
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
    /// use feather_tui as tui;
    ///
    /// // Creating a callback with an argument of 5, which is a u32.
    /// tui::cbk::Callback::new(callback_func, 5u32);
    /// 
    /// tui::callback_new_callback_func!(callback_func, arg, {
    ///     // An error occurs because arg is a u32, but we're attempting to cast 
    ///     // it to a char.
    ///     tui::cbk::cast_arg::<char>(arg)?;
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
/// use feather_tui as tui;
///
/// // Variants of the same type are considered equal.
/// assert_eq!(
///     tui::err::FtuiError::TextFlagNoneWithOther,
///     tui::err::FtuiError::TextFlagNoneWithOther);
///
/// // Variants of different types are not equal.
/// assert_ne!(
///     tui::err::FtuiError::TextFlagNoneWithOther,
///     tui::err::FtuiError::TextFlagMultipleColor);
///
/// // StdInputOutputError variants are treated as equal even if their inner errors differ.
/// use std::io::{Error, ErrorKind};
/// use tui::err::FtuiError;
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
            (RendererContainerTooBig, RendererContainerTooBig) => true,
            (TriggerCastArgNoArgument, TriggerCastArgNoArgument) => true,
            (TriggerCastArgWrongType, TriggerCastArgWrongType) => true,
            (SelectorNoTriggers, SelectorNoTriggers) => true,
            (CallbackCastArgNoArgument, CallbackCastArgNoArgument) => true,
            (CallbackCastArgWrongType, CallbackCastArgWrongType) => true,
            (StdInputOutputError(_), StdInputOutputError(_)) => true,

            _ => false,
        }
    }
}

/// A convenient alias for `Resul`t<T, FtuiError>`.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // A main function that returns a Result<(), FtuiError>.
/// fn main() -> tui::err::FtuiResult<()> {
///     Ok(())
/// }
// ```
pub type FtuiResult<T> = Result<T, FtuiError>;
