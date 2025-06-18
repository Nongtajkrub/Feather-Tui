use crate::{error::{FtuiError, FtuiResult}};
use unicode_segmentation::UnicodeSegmentation;

/// A UI component representing an interactive option in a `Container`. 
/// `Option` components are displayed in the order they are added to the
/// `Container`. To make options selectable, a `Selector` must also be
/// initialized for the `Container`.
///
/// # Usage
///
/// The `Option` component is used within a `Container` to provide interactive  
/// choices. 
///
/// # Notes
/// - A `Selector` component is required to navigate and select options.
pub struct Option {
    label: String,
    len: usize,
    line: u16,
    id: u16,
    selc_on: bool,
    is_selc: bool,
}

impl Option {
    /// Creates a new `Option` with the specified label and callback.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text displayed for this option.
    /// - `callback`: A `Callback` to invoked when the option is selected (optional). 
    ///
    /// # Returns
    /// `Ok(Option)`: A new `Option` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Define a callback function that quits the program when invoked.
    /// cbk_new_callback_func!(quit_option_callback, _arg, {
    ///     std::process::exit(0);
    /// });
    /// 
    /// // Create a `Callback` with no arguments.
    /// let callback = Callback::no_arg(quit_option_callback);
    /// 
    /// // Create an `Option` component labeled "Quit".
    /// // When selected, it exits the program.
    /// let _ = Option::new("Quit", callback)?;
    /// 
    /// // Create an `Option` component labeled "Nothing".
    /// // This option has no associated callback.
    /// // You can detect its selection using the `is_selc()` method.
    /// let _ = Option::new("Nothing", None)?;
    /// ```
    pub fn new(label: &str) -> FtuiResult<Self> {
        if label.is_empty() {
            return Err(FtuiError::OptionLabelEmpty);
        }

        Ok(Option {
            label: label.to_string(),
            len: label.graphemes(true).count(),
            id: 0,
            line: 0,
            selc_on: false,
            is_selc: false,
        })
    }

    pub(crate) fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    pub(crate) fn line(&self) -> u16 {
        return self.line;
    }

    pub(crate) fn label(&self) -> &String {
        return &self.label;
    }

    pub(crate) fn len(&self) -> usize {
        return self.len;
    }

    pub(crate) fn selc_on(&self) -> bool {
        return self.selc_on;
    }

    pub(crate) fn set_selc_on(&mut self, value: bool) {
        self.selc_on = value;
    }

    /// Returns whether the `Option` component was selected. This method acts
    /// like a latch or semaphore in multithreading contexts. It returns the
    /// current state of the `is_selc` flag and then resets it to `false`. 
    /// This method is useful for `Option` components with no `Callback`.
    ///
    /// # Notes 
    /// Imagine the following timeline:
    ///
    /// Time (ms):    0        500         2000          ...
    ///               |---------|------------|------------>
    /// is_selc:      |  false  |    true    |    false
    ///
    /// At time 500ms, some internal event sets `is_selc = true`.
    /// When `is_selc()` is called (e.g., at 2000ms), it returns `true`
    /// and immediately resets the flag to `false`.
    ///
    /// # Returns
    /// - `true`: if the option was selected since the last check.
    /// - `false`: otherwise.
    ///
    /// # Example
    /// ```rust
    /// // Create an `Option` component with no callback.
    /// let mut option = Option::new(..., None)?;
    ///
    /// // Check if the option was selected.
    /// if option.is_selc() {
    ///     // Perform an action.
    ///     todo!();
    /// }
    /// ```
    pub fn is_selc(&mut self) -> bool {
        std::mem::take(&mut self.is_selc)
    }

    pub(crate) fn set_is_selc(&mut self, value: bool) {
        self.is_selc = value;
    }

    pub(crate) fn id(&self) -> u16 {
        self.id
    }

    pub(crate) fn set_id(&mut self, value: u16) {
        self.id = value;
    }
}
