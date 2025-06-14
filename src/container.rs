use crate::{
    callback::Callback, components as cpn, error::{FtuiError, FtuiResult},
    renderer::Renderer, trigger::Trigger, util::id::IdGenerator
};

pub trait ContainerTrait {
    fn draw(&mut self, width: u16, height: u16) -> FtuiResult<()>;
    fn draw_fullscreen(&mut self) -> FtuiResult<()>;
    fn draw_expl(&mut self, renderer: &mut Renderer) -> FtuiResult<()>;
    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()>;
}

/// `Container` is a data structure used to store and organize UI components,
/// including `Header`, `Option`, `Text`, `Separator`, and `Selector`.
/// It is created using a `ContainerBuilder`.
///
/// # Usage
/// - Handle UI events with the `looper` method.
/// - Render the UI using a `Renderer` (recommended).
/// - Alternatively, use the `draw` or `draw_fullscreen` methods.
/// - Access `Option` components by ID using `option` and `option_mut`.
/// - Access `Text` components by ID using `text` and `text_mut`.
/// - Navigate using `selector_up`, `selector_down`, and `selector_select`.
pub struct Container {
    id_generator: IdGenerator<u16>,
    header: Option<cpn::Header>,
    options: Vec<cpn::Option>,
    texts: Vec<cpn::Text>,
    separators: Vec<cpn::Separator>,
    selector: Option<cpn::Selector>,
    component_count: u16,
}

impl Container {
    /// Constructs a new `Container`. 
    ///
    /// # Returns
    /// `Container`: A new instance of `Container`.
    ///
    /// # Example
    /// ```rust
    /// let _ = Container::new();
    /// ```
    pub(crate) fn new() -> Container {
        Container {
            id_generator: IdGenerator::new(),
            header: None,
            options: vec![],
            texts: vec![],
            separators: vec![],
            selector: None,
            component_count: 0,
        }
    }

    /// Updates the container and returns whether a change occurred.
    ///
    /// # Returns
    /// - `Ok(bool)`: Returns whether an update occurred.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// fn render() {
    ///     todo!();
    /// }
    ///
    /// // Re-render the UI if an update occurred.
    /// if container.looper()? {
    ///     render();
    /// }
    /// ```
    pub fn looper(&mut self) -> FtuiResult<bool> {
        if self.options.is_empty() {
            return Ok(false);
        }

        self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerLooperNoSelector)?
            .looper(&mut self.options)
            .or_else(|e| match e {
                FtuiError::SelectorNoTriggers => Ok(false),
                _ => Err(e),
            })
    }

    pub(crate) fn set_header(&mut self, header: cpn::Header) {
        self.header = Some(header);
        self.component_count += 1;
    }

    pub(crate) fn set_selector(&mut self, selector: cpn::Selector) {
        self.selector = Some(selector);
    }

    // Return added Option ID.
    pub(crate) fn add_option(&mut self, mut option: cpn::Option) -> u16 {
        let id = self.id_generator.get_id();
        option.set_id(id);
        option.set_line(self.component_count);

        if self.options.is_empty() {
            option.set_selc_on(true);
        }

        self.options.push(option);
        self.component_count += 1;

        id
    }

    // Return added Text ID.
    pub(crate) fn add_text(&mut self, mut text: cpn::Text) -> u16 {
        let id = self.id_generator.get_id();
        text.set_id(id);
        text.set_line(self.component_count);

        self.texts.push(text);
        self.component_count += 1;

        id
    }

    pub(crate) fn add_separator(&mut self, mut separator: cpn::Separator) {
        separator.set_line(self.component_count);
        self.separators.push(separator);
        self.component_count += 1;
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
    pub fn option(&self, id: u16) -> FtuiResult<&cpn::Option> {
        self.options.iter()
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
    pub fn option_mut(&mut self, id: u16) -> FtuiResult<&mut cpn::Option> {
        self.options.iter_mut()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    /// Query an `Text` component by its ID (`O(n)` lookup).
    ///
    /// # Parameters
    /// - `id`: The ID of the `Text` component to query.
    ///
    /// # Returns
    /// - `Ok(&Option)`: A reference to the `Text` component.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // A mutable `u16` to store the ID of a `Text` component.
    /// let mut text_id: u16 = 0;
    ///
    /// let container = ContainerBuilder::new()
    ///     .text_id(..., &mut text_id)?
    ///     .build();
    ///
    /// // Query the option by its ID.
    /// container.text(text_id)?;
    /// ```
    #[inline]
    pub fn text(&self, id: u16) -> FtuiResult<&cpn::Text> {
        self.texts.iter()
            .find(|text| text.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    /// Query an `Text` component by its ID (`O(n)` lookup).
    ///
    /// # Parameters
    /// - `id`: The ID of the `Text` component to query.
    ///
    /// # Returns
    /// - `Ok(&Option)`: A mutable reference to the `Text` component.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // A mutable `u16` to store the ID of a `Text` component.
    /// let mut text_id: u16 = 0;
    ///
    /// let container = ContainerBuilder::new()
    ///     .text_id(..., &mut text_id)?
    ///     .build();
    ///
    /// // Query the option by its ID.
    /// container.text_mut(text_id)?;
    /// ```
    #[inline]
    pub fn text_mut(&mut self, id: u16) -> FtuiResult<&mut cpn::Text> {
        self.texts.iter_mut()
            .find(|text| text.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    /// Returns a mutable reference to the `Selector` component, 
    /// if the `Container` has one.
    /// 
    /// # Returns
    /// - `Ok(&mut Selector)`: A mutable reference to the `Selector` component.
    /// - `Err(FtuiError)`: Return an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a container with a `Selector`.
    /// let mut container = ContainerBuilder::new()
    ///     .selector_no_triggers()
    ///     .build();
    ///
    /// // Get a mutable reference to the `Selector`.
    /// container.selector_mut()?;
    /// ```
    #[inline]
    pub fn selector_mut(&mut self) -> FtuiResult<&mut cpn::Selector> {
        self.selector.as_mut()
            .ok_or(FtuiError::ContainerNoSelector)
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
    pub fn selector_up(&mut self) -> FtuiResult<bool> {
        Ok(self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)?
            .up(&mut self.options))
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
    pub fn selector_down(&mut self) -> FtuiResult<bool> {
        Ok(self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)?
            .down(&mut self.options))
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
    pub fn selector_select(&mut self) -> FtuiResult<bool> {
        Ok(self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)?
            .select(&mut self.options)?)
    }

    pub(crate) fn component_count(&self) -> u16 {
        self.component_count
    }

    pub(crate) fn header(&self) -> &Option<cpn::Header> {
        &self.header
    }

    pub(crate) fn options(&self) -> &[cpn::Option] {
        &self.options
    }

    pub(crate) fn texts_mut(&mut self) -> &mut [cpn::Text] {
        &mut self.texts
    }

    pub(crate) fn separators(&self) -> &[cpn::Separator] {
        &self.separators
    }
}

/// `ContainerBuilder` is used to create `Container` instances using the builder
/// pattern. This allows for a flexible and readable way to construct complex
/// containers by chaining method calls.
///
/// # Example
/// ```rust
/// // Create a container with a header, two options, a separator, some text,
/// // and a selector.
/// let container: Container = ContainerBuilder::new()
///     .header(...)?
///     .option(...)?
///     .option(...)?
///     .separator_normal(...)
///     .text(...)?
///     .selector(...)?
///     .build();
/// ```
pub struct ContainerBuilder {
    container: Container,
}

impl ContainerBuilder {
    /// Constructs a new `ContainerBuilder`. 
    ///
    /// # Return
    /// `ContainerBuilder`: A new instance of `ContainerBuilder`.
    ///
    /// # Example
    /// ```rust
    /// let _ = ContainerBuilder::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        ContainerBuilder { container: Container::new(), }
    }

    /// Explicitly sets a `Header` component for the `Container`. Unlike the `header`
    /// method, which takes a label and internally constructs a `Header`, this 
    /// method allows you to directly provide a preconstructed `Header` component.
    ///
    /// # Parameters
    /// - `header`: A `Header` component.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Header` component.
    /// let header = Header::new(...)?;
    ///
    /// // Set a preconstructed `Header` component.
    /// ContainerBuilder::new()
    ///     .header_expl(header);
    /// ```
    pub fn header_expl(mut self, header: cpn::Header) -> Self {
        self.container.set_header(header);
        self
    }

    /// Sets a `Header` component for the `Container`.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text to display in the header.
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a `Header` component with the label "Welcome".
    /// ContainerBuilder::new()
    ///     .header("Welcome")?;
    /// ```
    #[inline]
    pub fn header(self, label: &str) -> FtuiResult<Self> {
        Ok(self.header_expl(cpn::Header::new(label)?))
    }

    /// Explicitly add an `Option` component to the `Container`. Unlike the `option`
    /// method, which takes a label and an optional callback to internally constructs
    /// an `Option`, this method allows you to directly provide a preconstructed 
    /// `Option` component.
    ///
    /// # Parameters
    /// - `option`: An `Option` component.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create an `Option` component.
    /// let option = Option::new(...)?;
    ///
    /// // Set a preconstructed `Option` component.
    /// ContainerBuilder::new()
    ///     .option_expl(option);
    /// ```
    pub fn option_expl(mut self, option: cpn::Option) -> Self {
        self.container.add_option(option);
        self
    }

    /// Adds an `Option` component to the `Container`.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text displayed for this option.
    /// - `callback`: An optional `Callback` invoked when the option is selected.
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Add an `Option` component with the label "Option" and no `Callback`.
    /// ContainerBuilder::new()
    ///     .option("Option", None)?;
    /// ```
    #[inline]
    pub fn option(
        self, label: &str, callback: impl Into<Option<Callback>>
    ) -> FtuiResult<Self> {
        Ok(self.option_expl(cpn::Option::new(label, callback)?))
    }

    /// Explicitly add an `Option` component to the `Container` and stores its ID. 
    /// Unlike the `option_id` method, which takes a label and an optional callback
    /// to internally constructs an `Option`, this method allows you to directly
    /// provide a preconstructed `Option` component.
    ///
    /// # Parameters
    /// - `option`: An `Option` component.
    /// - `store_id`: A `&mut u16` to store the created `Option` component ID.
    /// 
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    /// 
    /// # Example
    /// ```rust
    /// let mut id = 0u16;
    ///
    /// // Create an `Option` component.
    /// let option = Option::new(...)?;
    ///
    /// // Add the create `Option` component and storing the generated ID in `id`.
    /// ContainerBuilder::new()
    ///     .option_id(option, &mut id)?;
    /// ```
    pub fn option_id_expl(
        mut self, option: cpn::Option, store_id: &mut u16
    ) -> Self {
        *store_id = self.container.add_option(option);
        self
    }

    /// Adds an `Option` component to the `Container` and stores its ID.
    ///
    /// # Parameters
    /// - `label`: The text displayed for this option.
    /// - `callback`: An optional `Callback` invoked when the option is selected.
    /// - `store_id`: A `&mut u16` to store the created `Option` component ID.
    /// 
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    /// 
    /// # Example
    /// ```rust
    /// let mut id = 0u16;
    ///
    /// // Add an `Option` labeled "Option" with no `Callback`,
    /// // storing the generated ID in `id`.
    /// ContainerBuilder::new()
    ///     .option_id("Option", None, &mut id)?;
    /// ```
    #[inline]
    pub fn option_id(
        self,
        label: &str, callback: impl Into<Option<Callback>>, store_id: &mut u16
    ) -> FtuiResult<Self> {
        Ok(self.option_id_expl(cpn::Option::new(label, callback)?, store_id))
    }

    /// Explicitly add a `Text` component to the `Container`. Unlike the `text`
    /// method, which takes a label and flags to internally constructs an `Text`,
    /// this method allows you to directly provide a preconstructed `Text` component.
    ///
    /// # Parameters
    /// - `text`: A `Text` component.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create an `Text` component.
    /// let text = Text::new(...)?;
    ///
    /// // Set a preconstructed `Text` component.
    /// ContainerBuilder::new()
    ///     .text_expl(text);
    /// ```
    pub fn text_expl(mut self, text: cpn::Text) -> Self {
        self.container.add_text(text);
        self
    }

    /// Adds a `Text` component to the `Container`.
    /// 
    /// # Parameters
    /// - `label`: A `&str` representing the text to display.
    /// - `flags`: A set of `TextFlags`, combined using the bitwise OR operator.
    ///
    /// # Notes
    /// - This is what bitwise OR operator look like -> `flag1 | flag2 | flag3 ...`
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Text` component labeled "Text", right-aligned and with
    /// // a magenta background.
    /// ContainerBuilder::new()
    ///     .text("Text", TextFlags::ALIGN_RIGHT | TextFlags::COLOR_MAGENTA_BACK)?;
    /// ```
    #[inline]
    pub fn text(
        self, label: &str, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        Ok(self.text_expl(cpn::Text::new(label, flags)?))
    }

    /// Explicitly add a `Text` component to the `Container` and stores its ID. 
    /// Unlike the `text_id` method, which takes a label and flags to internally
    /// constructs an `Text`, this method allows you to directly provide a
    /// preconstructed `Text` component.
    ///
    /// # Parameters
    /// - `text`: An `Text` component.
    /// - `store_id`: A `&mut u16` to store the created `Option` component ID.
    /// 
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    /// 
    /// # Example
    /// ```rust
    /// let mut id = 0u16;
    ///
    /// // Create an `Text` component.
    /// let text = Text::new(...)?;
    ///
    /// // Add the create `Text` component and storing the generated ID in `id`.
    /// ContainerBuilder::new()
    ///     .text_id_expl(text, &mut id)?;
    /// ```
    pub fn text_id_expl(mut self, text: cpn::Text, store_id: &mut u16) -> Self {
        *store_id = self.container.add_text(text);
        self
    }

    /// Adds a `Text` component to the `Container` and stores its ID.
    /// 
    /// # Parameters
    /// - `label`: A `&str` representing the text to display.
    /// - `flags`: A set of `TextFlags`, combined using the bitwise OR operator.
    /// - `store_id`: A `&mut u16` to store the created `Text` component ID.
    ///
    /// # Notes
    /// - This is what bitwise OR operator look like -> `flag1 | flag2 | flag3 ...`
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// let mut id = 0u16;
    ///
    /// // Create a `Text` component labeled "Text", right-aligned and with
    /// // a magenta background. storing the generated ID in `id`.
    /// ContainerBuilder::new()
    ///     .text(
    ///         "Text",
    ///         TextFlags::ALIGN_RIGHT | TextFlags::COLOR_MAGENTA_BACK, &mut id)?;
    /// ```
    #[inline]
    pub fn text_id(
        self, 
        label: &str, flags: impl Into<Option<cpn::TextFlags>>, store_id: &mut u16
    ) -> FtuiResult<Self> {
        Ok(self.text_id_expl(cpn::Text::new(label, flags)?, store_id))
    }

    /// Explicitly add a normal `Separator` component to the `Container`. 
    /// Unlike the `Separator` method, which takes a style and internally 
    /// constructs a `Separator`, this method allows you to directly provide a
    /// preconstructed `Separator` component.
    ///
    /// # Parameters
    /// - `separator`: A `Separator` component.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create a normal `Separator` component.
    /// let separator = Separator::normal(...)?;
    ///
    /// // Set a preconstructed `Header` component.
    /// ContainerBuilder::new()
    ///     .separator_normal_expl(separator);
    /// ```
    pub fn separator_normal_expl(mut self, separator: cpn::Separator) -> Self {
        self.container.add_separator(separator);
        self
    }

    /// Add a standard (non-dotted) `Separator` with the given style.
    ///
    /// # Parameters
    /// - `style`: The visual style of the separator, specified as a `SeparatorStyle`.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Add a normal separator with a solid style.
    /// ContainerBuilder::new()
    ///     separator_normal(SeparatorStyle::Solid);
    /// ```
    #[inline]
    pub fn separator_normal(self, style: cpn::SeparatorStyle) -> Self {
        self.separator_normal_expl(cpn::Separator::normal(style))
    }

    /// Explicitly add a dotted `Separator` component to the `Container`. 
    /// Unlike the `Separator` method, which takes a style and internally 
    /// constructs a `Separator`, this method allows you to directly provide a
    /// preconstructed `Separator` component.
    ///
    /// # Parameters
    /// - `separator`: A `Separator` component.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create a dotted `Separator` component.
    /// let separator = Separator::dotted(...)?;
    ///
    /// // Set a preconstructed `Header` component.
    /// ContainerBuilder::new()
    ///     .separator_dotted_expl(separator);
    /// ```
    pub fn separator_dotted_expl(mut self, separator: cpn::Separator) -> Self {
        self.container.add_separator(separator);
        self
    }

    /// Add a dotted `Separator` with the given style.
    ///
    /// # Parameters
    /// - `style`: The visual style of the separator, specified as a `SeparatorStyle`.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Add a dotted separator with a solid style.
    /// ContainerBuilder::new()
    ///     separator_dotted(SeparatorStyle::Solid);
    /// ```
    #[inline]
    pub fn separator_dotted(self, style: cpn::SeparatorStyle) -> Self {
        self.separator_dotted_expl(cpn::Separator::dotted(style))
    }

    /// Explicitly sets a `Selector` component for the `Container`. Unlike the `selector`
    /// method, which takes triggers and internally constructs a `selector`, this 
    /// method allows you to directly provide a preconstructed `Selector` component.
    ///
    /// # Parameters
    /// - `selector`: A `Selector` component.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Header` component.
    /// let selector = Selector::new(...)?;
    ///
    /// // Set a preconstructed `Selector` component.
    /// ContainerBuilder::new()
    ///     .selector_expl(selector);
    /// ```
    pub fn selector_expl(mut self, selector: cpn::Selector) -> Self {
        self.container.set_selector(selector);
        self
    }

    /// Set a `Selector` for the `Container` to handle user navigation.
    ///
    /// # Parameters 
    /// - `up_trig`: A `Trigger` that moves the selector up when activated.
    /// - `down_trig`: A `Trigger` that moves the selector down when activated.
    /// - `selc_trig`: A `Trigger` that confirms the selection when activated.
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    /// 
    /// # Example
    /// ```rust
    /// // A `Trigger` that always activate.
    /// tui::trg_new_trigger_func!(always_true_trigger, _arg, {
    ///     Ok(true)
    /// });
    /// 
    /// // A `Trigger` that never activate .
    /// tui::trg_new_trigger_func!(always_false_trigger, _arg, {
    ///     Ok(false)
    /// });
    ///
    /// // Set a `Selector` that always moves down.
    /// ContainerBuilder::new()
    ///     .selector(
    ///         Trigger::no_arg(always_false_trigger),
    ///         Trigger::no_arg(always_true_trigger),
    ///         Trigger::no_arg(always_false_trigger),
    ///     );
    /// ```
    #[inline]
    pub fn selector(
        self, up_trig: Trigger, down_trig: Trigger, selc_trig: Trigger
    ) -> Self {
        self.selector_expl(cpn::Selector::new(up_trig, down_trig, selc_trig))
    }

    /// Sets a `Selector` with no `Trigger`s for the `Container`.
    ///
    /// In this case, navigation must be handled manually using the following methods:
    /// - `Container::selector_up`
    /// - `Container::selector_down`
    /// - `Container::selector_select`
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Set a `Selector` with no `Trigger`s.
    /// ContainerBuilder::new()
    ///     .selector_no_triggers();
    /// ```
    pub fn selector_no_triggers(mut self) -> Self {
        self.container.set_selector(cpn::Selector::no_triggers());
        self
    }

    /// Finalizes the construction of a `Container`. This method should be called
    /// after all desired components have been added using the builder pattern.
    /// It consumes `self` and returns the completed `Container`.
    ///
    /// # Returns
    /// - `Container`: Returns the created `Container`.
    ///
    /// # Example
    /// ```rust
    /// let container: Container = ContainerBuilder::new()
    ///     .header(...)?
    ///     .option(...)?
    ///     .option(...)?
    ///     .separator_normal(...)
    ///     .text(...)?
    ///     .selector(...)?
    ///     .build(); // Finalize and retrieve the constructed container.
    /// ```
    pub fn build(self) -> Container {
        self.container
    }
}

impl ContainerTrait for Container {
    /// Renders the `Container` using a temporary `Renderer`. This method is ideal
    /// for quick, one-off renderings where performance isn't critical. Internally,
    /// it creates a new `Renderer` with the given dimensions and uses it to draw
    /// the `Container`. If you need to render multiple times, consider using a
    /// persistent `Renderer` for better performance.
    ///
    ///
    /// # Parameters
    /// - `width`: The width of the rendering area.
    /// - `height`: The height of the rendering area.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Good for quick one-off rendering like this.
    /// fn render_ui() -> FtuiResult<()> {
    ///     ContainerBuilder::new()
    ///         .header(...)?
    ///         .text(...)?
    ///         .build()
    ///         .draw(40, 20)?; // Render with dimensions 40x20.
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    fn draw(&mut self, width: u16, height: u16) -> FtuiResult<()> {
        Renderer::new(width, height).simple_draw_container(self)
    }

    /// Renders the `Container` using a temporary `Renderer`. This method is ideal
    /// for quick, one-off renderings where performance isn't critical. Internally,
    /// it creates a new fullscreen `Renderer` and uses it to draw the `Container`.
    /// If you need to render multiple times, consider using a persistent `Renderer`
    /// for better performance.
    ///
    /// # Returns
    /// - `Ok(())`: Returns nothing.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Good for quick one-off rendering like this.
    /// fn render_ui() -> FtuiResult<()> {
    ///     ContainerBuilder::new()
    ///         .header(...)?
    ///         .text(...)?
    ///         .build()
    ///         .draw_fullscreen()?; // Render in fullscreen.
    ///
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    fn draw_fullscreen(&mut self) -> FtuiResult<()> {
        Renderer::fullscreen()?.simple_draw_container(self)
    }

    #[inline]
    fn draw_expl(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        renderer.simple_draw_container(self)
    }

    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        renderer.clear();
        renderer.render_container(self)
    }
}
