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

    /// Cccurs when performing an operation on a `List` container using an element
    /// index that does not exist.
    ///
    /// # Example
    /// ```rust
    /// // Create a simple `List` container.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add elements to the list.
    /// list.add(...)?;
    /// list.add(...)?;
    ///
    /// // Attempt to access an out-of-bounds index, which will trigger this error.
    /// list.at(100)?; 
    /// ```
    #[error("List index is out of bound.")]
    ListIndexOutOfBound,

    /// Occurs when attempting to find the index of an element in a `List`
    /// by its ID, but no element with the specified ID exists.
    ///
    /// # Example
    /// ```rust
    /// // Create a simple `List` container.
    /// let mut list = ListBuilder::new().build();
    ///
    /// // Add an element to the list and store its ID.
    /// let id = list.add(...)?;
    ///
    /// // Attempt to find an element by a non-existent ID.
    /// list.find(id + 1)?; 
    /// ```
    #[error("No element found with the specified ID.")]
    ListFailToFindElement,

    #[error("")]
    DimensionsTerminalToSmall,

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

    #[error("")]
    CustomContainerBlitOutOfBound,

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
            (ContainerNoComponentById, ContainerNoComponentById) => true,
            (ListIndexOutOfBound, ListIndexOutOfBound) => true,
            (RendererContainerTooBig, RendererContainerTooBig) => true,
            (StdInputOutputError(_), StdInputOutputError(_)) => true,
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
