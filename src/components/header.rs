/// A UI component that serves as the header of a `Container`. It is displayed 
/// at the top of the `Container` and is typically used to provide a title or
/// context for the container.
///
/// # Usage
///
/// The `Header` component is required to create a `Container` object.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Create a header with the text "Welcome"
/// let header = tui::cpn::hed::Header::new("Welcome");
///
/// // Set the header for a container
/// let mut container = tui::con::Container::new();
/// container.set_header(header);
/// ```
pub struct Header {
    label: String,
}

impl Header {
    /// Creates a new `Header` with the specified label.
    ///
    /// # Parameters
    /// * `label`: A `&str` representing the text displayed in the header.
    ///
    /// # Returns
    /// A new `Header` instance.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Create a Header with the label "Welcome".
    /// let header = tui::cpn::hed::Header::new("Welcome");
    /// ```
    pub fn new(label: &str) -> Header {
        Header { 
            label: label.to_string(), 
        }
    }

    pub fn len(&self) -> usize {
        return self.label.len();
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }
}
