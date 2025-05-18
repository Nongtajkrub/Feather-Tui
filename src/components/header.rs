use crate::error::{FtuiError, FtuiResult};
use unicode_segmentation::UnicodeSegmentation;

/// A UI component that serves as the header of a `Container`. It is displayed 
/// at the top of the `Container`.
///
/// # Usage
/// provide a title or context for the container.
///
/// # Derives
/// `Debug`, `Clone`, `PartialEq`, `Eq`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    label: String,
    len: usize,
}

impl Header {
    /// Creates a new `Header` with the specified label.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text displayed in the header.
    ///
    /// # Returns
    /// `Ok(Header)`: A new `Header` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a Header with the label "Welcome".
    /// let _ = Header::new("Welcome")?;
    /// ```
    pub fn new(label: &str) -> FtuiResult<Self> {
        if label.is_empty() {
            return Err(FtuiError::HeaderLabelEmpty);
        }

        Ok(Header { 
            label: label.to_string(), 
            len: label.graphemes(true).count(),
        })
    }

    pub(crate) fn len(&self) -> usize {
        return self.len;
    }

    pub(crate) fn label(&self) -> &String {
        return &self.label;
    }
}
