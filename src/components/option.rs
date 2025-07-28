use crate::error::{FtuiError, FtuiResult};
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn new(label: impl ToString) -> Self {
        let label = label.to_string();

        Option {
            len: label.graphemes(true).count(),
            label: label,
            id: 0,
            line: 0,
            selc_on: false,
            is_selc: false,
        }
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
 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsManager {
    components: Vec<Option>,
    selector_on: usize,
}

impl OptionsManager {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
            selector_on: 0,
        }
    }

    pub(crate) fn add(&mut self, component: Option) {
        self.components.push(component);
    }

    /// Query an `Option` component by its ID (`O(n)` lookup).
    ///
    /// # Parameters
    /// - `id`: The ID of the `Option` component to query.
    ///
    /// # Returns
    /// - `Ok(&Option)`: A reference to the `Option` component.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // A mutable `u16` to store the ID of a `Option` component.
    /// let mut option_id = 0;
    ///
    /// let container = ContainerBuilder::new()
    ///     .option_id(..., &mut option_id)?
    ///     .build();
    ///
    /// // Query the option by its ID.
    /// container.option(option_id)?;
    /// ```
    #[inline]
    pub fn query(&self, id: u16) -> FtuiResult<&Option> {
        self.components.iter()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    /// Query an `Option` component by its ID (`O(n)` lookup).
    ///
    /// # Parameters
    /// - `id`: The ID of the `Option` component to query.
    ///
    /// # Returns
    /// - `Ok(&Option)`: A mutable reference to the `Option` component.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // A mutable `u16` to store the ID of a `Option` component.
    /// let mut option_id = 0;
    ///
    /// let container = ContainerBuilder::new()
    ///     .option_id(..., &mut option_id)?
    ///     .build();
    ///
    /// // Query the option by its ID.
    /// container.option_mut(option_id)?;
    /// ```
    #[inline]
    pub fn query_mut(&mut self, id: u16) -> FtuiResult<&mut Option> {
        self.components.iter_mut()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    /// Attempts to move the `Selector` up by one position, if possible.
    ///
    /// # Returns
    /// - `Ok(true)`: The selector moved up successfully.
    /// - `Ok(false)`: The selector could not move (already at the top).
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a container with two `Option`s component and a `Selector`.
    /// let mut container = ContainerBuilder::new()
    ///     .option(...)? // This is where the `Selector` will be when initialize.
    ///     .option(...)?
    ///     .selector_no_triggers()
    ///     .build();
    ///
    /// // The `Selector` cannot move up since it is at the top.
    /// assert_eq!(container.selector_up()?, false);
    /// ```
    #[inline]
    pub fn selector_up(&mut self) -> bool {
        if self.selector_on == 0 {
            return false;
        }

        // move the selector up
        self.components[self.selector_on].set_selc_on(false);
        self.selector_on -= 1;
        self.components[self.selector_on].set_selc_on(true);

        true
    }

    /// Attempts to move the `Selector` down by one position, if possible.
    ///
    /// # Returns
    /// - `Ok(true)`: The selector moved down successfully.
    /// - `Ok(false)`: The selector could not move (already at the bottom).
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a container with two `Option`s component and a `Selector`.
    /// let mut container = ContainerBuilder::new()
    ///     .option(...)? // This is where the `Selector` will be when initialize.
    ///     .option(...)?
    ///     .selector_no_triggers()
    ///     .build();
    ///
    /// // The `Selector` can move up since it is not at the bottom.
    /// assert_eq!(container.selector_up()?, true);
    /// ```
    #[inline]
    pub fn selector_down(&mut self) -> bool {
        if self.selector_on == self.components.len() - 1 {
            return false;
        }

        // move selector down
        self.components[self.selector_on].set_selc_on(false);
        self.selector_on += 1;
        self.components[self.selector_on].set_selc_on(true);

        true
    }

    /// Attempts to select the `Option` that the `Selector` is currently on. 
    /// This operation should always succeed unless an error occurs internally.
    ///
    /// # Returns
    /// - `Ok(true)`: The selection was successful.
    /// - `Err(FtuiError)`: An error occurred during selection.
    ///
    /// # Example
    /// ```rust
    /// // Create a container with on `Option` components and a `Selector`.
    /// let mut container = ContainerBuilder::new()
    ///     .option(...)? // The `Selector` starts at this `Option`.
    ///     .selector_no_triggers()
    ///     .build();
    ///
    /// // Selecting the current `Option` is always possible unless an error occurs.
    /// assert_eq!(container.selector_select()?, true);
    /// ```
    #[inline]
    pub fn selector_select(&mut self) -> bool {
        if self.components.is_empty() {
            return false;
        }

        self.components[self.selector_on].set_is_selc(true);
        true
    }

    pub(crate) fn comps(&self) -> &[Option] {
        &self.components
    }
}
