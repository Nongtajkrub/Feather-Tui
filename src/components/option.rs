use crate::cbk;

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
/// // Define a callback function that exits the program when triggered
/// tui::tui_cbk_new_callback_func!(quit_option_callback, _arg, {
///    std::process::exit(0);
/// });
///
/// // Create an option labeled "Quit"
/// let option =
///     tui::cpn::opt::Option::new(
///         "Quit", tui::cbk::Callback::new(quit_option_callback, 0));
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
    line: u16,
    selc_on: bool,
    callback: cbk::Callback,
}

impl Option {
    pub fn new(label: &str, callback: cbk::Callback) -> Option {
        Option {
            label: label.to_string(),
            line: 0,
            selc_on: false,
            callback,
        }
    }

    pub fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    pub fn line(&self) -> u16 {
        return self.line;
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }

    pub fn selc_on(&self) -> bool {
        return self.selc_on;
    }

    pub fn callback(&self) -> &cbk::Callback {
        return &self.callback;
    }

    pub fn set_selc_on(&mut self, value: bool) {
        self.selc_on = value;
    }
}
