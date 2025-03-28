use crate::err::{FtuiError, FtuiResult};

/// A UI component that serves as the header of a `Container`. It is displayed 
/// at the top of the `Container` and is typically used to provide a title or
/// context for the container.
///
/// # Usage
///
/// The `Header` component is required to create a `Container` object.
///
/// # Derives
///
/// `Debug`, `Clone`, `PartialEq`, `Eq`
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Create a header with the text "Welcome"
/// let header = tui::cpn::hed::Header::new("Welcome")?;
///
/// // Set the header for a container
/// let mut container = tui::con::Container::new();
/// container.set_header(header);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    label: String,
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
    /// use feather_tui as tui;
    ///
    /// // Create a Header with the label "Welcome".
    /// let header = tui::cpn::hed::Header::new("Welcome")?;
    /// ```
    pub fn new(label: &str) -> FtuiResult<Self> {
        if label.is_empty() {
            return Err(FtuiError::HeaderLabelEmpty);
        }

        Ok(Header { 
            label: label.to_string(), 
        })
    }

    pub fn len(&self) -> usize {
        return self.label.len();
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }
}
