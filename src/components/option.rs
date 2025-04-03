use crate::{cbk, err::{FtuiError, FtuiResult}};
use unicode_segmentation::UnicodeSegmentation;

/// A UI component representing an interactive option in a `Container`. 
/// `Option` components are displayed in the order they are added to the
/// `Container`. To make options selectable, a `Selector` must also be
/// initialized for the `Container`.
///
/// # Usage
///
/// The `Option` component is used within a `Container` to provide interactive  
/// choices. Each option requires an associated callback function that defines  
/// what happens when the option is selected.
///
/// # Notes
/// - A `Selector` is required to navigate and select options.
/// - Each `Option` should have an associated `Callback` function.
///
/// # Example
/// 
/// ```rust
/// use feather_tui as tui;
///
/// // Define a callback function that exits the program when invoked 
/// tui::cbk_new_callback_func!(quit_option_callback, _arg, {
///    std::process::exit(0);
/// });
///
/// // Create a Callback
/// let callback = tui::cbk::Callback::no_arg(quit_option_callback);
///
/// // Create an option labeled "Quit"
/// let option = tui::cpn::opt::Option::new("Quit", callback)?;
///
/// // Create a container and add the option
/// let mut container = tui::con::Container::new();
/// container.add_option(option);
///
/// // Set the selector for the container
/// // (Assuming a selector is created elsewhere)
/// container.set_selector(selector);
/// ```
pub struct Option {
    label: String,
    len: usize,
    line: u16,
    selc_on: bool,
    callback: cbk::Callback,
}

impl Option {
    /// Creates a new `Option` with the specified label and callback.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text displayed for this option.
    /// - `callback`: A `Callback` that will be invoked when the option is selected. 
    ///
    /// # Returns
    /// `Ok(Option)`: A new `Option` instance.
    /// `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// // Define a callback function that exits the program when invoked 
    /// tui::tui_cbk_new_callback_func!(quit_option_callback, _arg, {
    ///    std::process::exit(0);
    /// });
    ///
    /// // Create a Callback that stores the number 5.
    /// let callback = tui::cbk::Callback::no_arg(quit_option_callback);
    ///
    /// // Create an Option labeled "Quit".
    /// // When selected, it exit the program.
    /// let option = tui::cpn::opt::Option::new("Quit", callback)?;
    /// ```
    pub fn new(label: &str, callback: cbk::Callback) -> FtuiResult<Self> {
        if label.is_empty() {
            return Err(FtuiError::OptionLabelEmpty);
        }

        Ok(Option {
            label: label.to_string(),
            len: label.graphemes(true).count(),
            line: 0,
            selc_on: false,
            callback,
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

    pub(crate) fn callback(&self) -> &cbk::Callback {
        return &self.callback;
    }

    pub(crate) fn set_selc_on(&mut self, value: bool) {
        self.selc_on = value;
    }
}
