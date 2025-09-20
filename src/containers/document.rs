use std::path::Path;
use std::fs;

use crate::components::Text;
use crate::components::TextFlags;
use crate::error::FtuiResult;
use crate::renderer::Renderer;

/// A specialized variant of `Container` for displaying long-form text.  
/// The `Document` supports text wrapping and scrolling, making it suitable  
/// for content such as stories, logs, or multi-line descriptions.
///
/// # Usage
/// Use `Document` when you need to present lengthy text with proper  
/// wrapping and navigation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    header: Option<Text>,
    footer: Option<Text>,
    data: String,
    offset: usize,
    flags: TextFlags,
    style: Vec<&'static str>,
}

impl Document {
    pub(crate) fn new() -> Self {
        Self {
            header: None,
            footer: None,
            data: String::new(),
            offset: 0,
            flags: TextFlags::NONE,
            style: Vec::new(), 
        }
    }

    /// Attempts to scroll the `Document` up by one position.
    ///
    /// # Returns
    /// - `true` if the `Document` was successfully scrolled up.
    /// - `false`: The `Document` fail to scroll up (already at the top). 
    ///
    /// # Example
    /// ```rust
    /// // Create a new `Document`.
    /// let mut doc = DocumentBuilder::new().build();
    ///
    /// // Initially at the top, so scrolling up does nothing.
    /// assert_eq!(doc.scroll_up(), false);
    /// ```
    pub fn scroll_up(&mut self) -> bool {
        if self.offset != 0 {
            self.offset -= 1;
            true
        } else {
            false
        }
    }

    /// Attempts to scroll the `Document` down by one position.
    ///
    /// # Returns
    /// - `true` If the `Document` was successfully scrolled down.
    /// - `false`: The `Document` fail to scroll down (already at the bottom). 
    ///
    /// # Example
    /// ```rust
    /// // Create a new `Document`.
    /// let mut list = DocumentBuilder::new()
    ///     .content(...)
    ///     .build();
    ///
    /// // The list can scroll down since it's not at the bottom yet.
    /// assert_eq!(list.scroll_down(), true);
    /// ```
    #[inline]
    pub fn scroll_down(&mut self) -> bool {
        // Bounds checking is done in the `Renderer`.
        self.offset += 1;
        true
    }

    #[inline]
    pub(crate) fn offset_ensure_in_bound(&mut self, bound: usize) {
        self.offset = self.offset.min(bound);
    }

    pub(crate) fn header(&self) -> &Option<Text> {
        &self.header
    }

    pub(crate) fn footer(&self) -> &Option<Text> {
        &self.footer
    }

    pub(crate) fn header_mut(&mut self) -> &mut Option<Text> {
        &mut self.header
    }

    pub(crate) fn footer_mut(&mut self) -> &mut Option<Text> {
        &mut self.footer
    }

    pub(crate) fn data(&self) -> &str {
        &self.data
    }

    pub(crate) fn style(&self) -> &[&'static str] {
        &self.style
    }

    pub(crate) fn offset(&self) -> usize {
        self.offset
    }
}

/// `DocumentBuilder` is used to create `Document` instances using the builder pattern.
/// This allows for a flexible and readable way to construct `Document` with different
/// options by chaining method calls.
///
/// # Example
/// ```rust
/// DocumentBuilder::new()
///     .header(...)?
///     .content(...)
///     .flags(...)?
///     .footer(...)?
///     .build();
/// ```
pub struct DocumentBuilder {
    document: Document,
}

impl DocumentBuilder {
    /// Constructs a new `DocumentBuilder`. 
    ///
    /// # Return
    /// `DocumentBuilder`: A new instance of `DocumentBuilder`.
    ///
    /// # Example
    /// ```rust
    /// let _ = DocumentBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            document: Document::new(),
        } 
    }

    /// Sets a header for the `Document`.
    ///
    /// # Notes
    /// The header behaves similarly to a `Text` component and can display
    /// styled text using the provided flags.
    ///
    /// # Parameters
    /// - `label`: A type that impl `ToString` representing the text for the header.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Returns
    /// - `Ok(DocumentBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a header with the label "Welcome" and no style.
    /// DocumentBuilder::new()
    ///     .header("Welcome", None)?;
    /// ```
    pub fn header(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.document.header = Some(Text::new(label.to_string(), flags)?);
        Ok(self)
    }

    /// Sets a footer for the `Document`.
    ///
    /// # Notes
    /// The footer behaves similarly to a `Text` component and can display
    /// styled text using the provided flags.
    ///
    /// # Parameters
    /// - `label`: A type that impl `ToString` representing the text for the footer.
    /// - `flags`: A set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Returns
    /// - `Ok(DocumentBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a footer with the label "Welcome" and no style.
    /// DocumentBuilder::new()
    ///     .header("Welcome", None)?;
    /// ```
    pub fn footer(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.document.footer = Some(Text::new(label.to_string(), flags)?);
        Ok(self)
    }

    /// Sets the `TextFlags` to be used when for this document.
    ///
    /// # Parameters
    /// - `flags`: The `TextFlags` to apply to the document.
    ///
    /// # Returns
    /// - `Ok(DocumentBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Set a red color for.
    /// DocumentBuilder::new()
    ///     .flags(TextFlags::COLOR_RED)?;
    /// ```
    pub fn flags(mut self, flags: TextFlags) -> FtuiResult<Self> {
        flags.ensure_compatibility()?;
        self.document.flags = flags;
        self.document.style = flags.resolve_ansi();
        Ok(self)
    }

    /// Sets the content of the document.
    ///
    /// # Parameters
    /// - `data`: Any type that implements `ToString`, representing the document content.
    ///
    /// # Returns
    /// `DocumentBuilder`: Returns self.
    ///
    /// # Example
    /// ```
    /// DocumentBuilder::new()
    ///     .content("Hello, World");
    /// ```
    pub fn content(mut self, data: impl ToString) -> Self {
        self.document.data = data.to_string(); 
        self
    }

    /// Loads the contents of a file and sets it as the document content.
    ///
    /// # Parameters
    /// - `path`: A path to the file to be read.
    ///
    /// # Returns
    /// - `Ok(DocumentBuilder)`: Returns self.  
    /// - `Err(FtuiError)`: Returns an `io` error.  
    ///
    /// # Example
    /// ```
    /// let builder = DocumentBuilder::new()
    ///     .from_file("/path/to/file.txt")?;
    /// ```
    pub fn from_file(mut self, path: impl AsRef<Path>) -> FtuiResult<Self> {
        self.document.data = fs::read_to_string(path.as_ref())?.trim().to_owned(); 
        Ok(self)
    }

    /// Renders the current `Document` directly to the terminal without
    /// creating and returning a new one.
    ///
    /// # Parameters
    /// - `renderer`: A mutable type that implements `AsMut<Renderer>`.
    ///
    /// # Returns
    /// - `Ok(())`: Return nothing if the document was successfully drawn.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// DocumentBuilder::new()
    ///     .header(...)?
    ///     .instant_draw(Renderer::new(...))?;
    /// ```
    pub fn instant_draw(self, mut renderer: impl AsMut<Renderer>) -> FtuiResult<()> {
        renderer.as_mut().draw(self.document)
    }

    /// Finalizes the construction of a `Document`. This method should be called
    /// after all desired options have been set using the builder pattern.
    /// It consumes `self` and returns the completed `Document`.
    ///
    /// # Returns
    /// - `Document`: Returns the created `Document`.
    ///
    /// # Example
    /// ```rust
    /// DocumentBuilder::new()
    ///     .header(...)?
    ///     .content(...)
    ///     .flags(...)?
    ///     .footer(...)?
    ///     .build(); // Finalize and retrieve the constructed document.
    /// ```
    pub fn build(self) -> Document {
        self.document
    }
}
