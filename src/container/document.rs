use crate::{components::{TextFlags, Text}, error::FtuiResult, renderer::Renderer};
use std::{fs, path::Path};

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

pub struct DocumentBuilder {
    document: Document,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
        } 
    }

    pub fn header(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.document.header = Some(Text::new(label.to_string(), flags)?);
        Ok(self)
    }

    pub fn footer(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.document.footer = Some(Text::new(label.to_string(), flags)?);
        Ok(self)
    }

    pub fn flags(mut self, flags: TextFlags) -> FtuiResult<Self> {
        flags.ensure_compatibility()?;
        self.document.flags = flags;
        self.document.style = flags.resolve_ansi();
        Ok(self)
    }

    pub fn content(mut self, data: impl ToString) -> Self {
        self.document.data = data.to_string(); 
        self
    }

    pub fn from_file(mut self, path: impl AsRef<Path>) -> FtuiResult<Self> {
        self.document.data = fs::read_to_string(path.as_ref())?.trim().to_owned(); 
        Ok(self)
    }

    pub fn instant_draw(self, mut renderer: impl AsMut<Renderer>) -> FtuiResult<()> {
        renderer.as_mut().draw(self.document)
    }

    pub fn build(self) -> Document {
        self.document
    }
}
