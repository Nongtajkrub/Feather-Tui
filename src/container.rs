use crate::{
    cbk::Callback, trg::Trigger, cpn, error::{FtuiError, FtuiResult}, slc::Selector,
    ren::Renderer, util::id::IdGenerator
};

/// `Container` is a data structure used to store and organize UI components,
/// including `Header`, `Option`, `Text`, `Separator`, and `Selector`.
/// It is created using a `ContainerBuilder`.
///
/// ---
///
/// # Usage
/// - Handle UI events with the `looper` method.
/// - Render the UI using a `Renderer` (recommended).
/// - Alternatively, use the `draw` or `draw_fullscreen` methods.
/// - Access `Option` components by ID using `option` and `option_mut`.
/// - Access `Text` components by ID using `text` and `text_mut`.
/// - Navigate using `selector_up`, `selector_down`, and `selector_select`.
///
/// ---
pub struct Container {
    id_generator: IdGenerator<u16>,
    header: Option<cpn::Header>,
    options: Vec<cpn::Option>,
    texts: Vec<cpn::Text>,
    separators: Vec<cpn::Separator>,
    selector: Option<Selector>,
    component_count: u16,
}

impl Container { 
    pub fn new() -> Container {
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
    /// ---
    ///
    /// # Returns
    /// - `Ok(bool)`: Returns whether an update occurred.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// ---
    ///
    /// # Example
    /// ```rust
    /// // Re-render the UI if an update occurred.
    /// if container.looper()? {
    ///     render();
    /// }
    /// ```
    ///
    /// ---
    pub fn looper(&mut self) -> FtuiResult<bool> {
        if self.options.len() > 0 {
            Ok(self.selector
                .as_mut()
                .ok_or(FtuiError::ContainerLooperNoSelector)?
                .looper(&mut self.options)?)
        } else {
            Ok(false)
        }
    }

    pub(crate) fn set_header(&mut self, header: cpn::Header) {
        self.header = Some(header);
        self.component_count += 1;
    }

    pub(crate) fn set_selector(&mut self, selector: Selector) {
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


    pub fn draw(&mut self, width: u16, height: u16) -> FtuiResult<()> {
        Renderer::new(width, height).simple_draw(self)?;
        Ok(())
    }

    pub fn draw_fullscreen(&mut self) -> FtuiResult<()> {
        Renderer::fullscreen()?.simple_draw(self)?;
        Ok(())
    }

    #[inline]
    pub fn option(&self, id: u16) -> FtuiResult<&cpn::Option> {
        self.options.iter()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    #[inline]
    pub fn option_mut(&mut self, id: u16) -> FtuiResult<&mut cpn::Option> {
        self.options.iter_mut()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    #[inline]
    pub fn text(&self, id: u16) -> FtuiResult<&cpn::Text> {
        self.texts.iter()
            .find(|text| text.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    #[inline]
    pub fn text_mut(&mut self, id: u16) -> FtuiResult<&mut cpn::Text> {
        self.texts.iter_mut()
            .find(|text| text.id() == id)
            .ok_or(FtuiError::ContainerNoComponentById)
    }

    #[inline]
    pub fn selector_mut(&mut self) -> FtuiResult<&mut Selector> {
        self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)
    }

    #[inline]
    pub fn selector_up(&mut self) -> FtuiResult<bool> {
        Ok(self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)?
            .move_up(&mut self.options))
    }

    #[inline]
    pub fn selector_down(&mut self) -> FtuiResult<bool> {
        Ok(self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)?
            .move_down(&mut self.options))
    }

    #[inline]
    pub fn selector_select(&mut self) -> FtuiResult<bool> {
        Ok(self.selector
            .as_mut()
            .ok_or(FtuiError::ContainerNoSelector)?
            .selc(&mut self.options)?)
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
/// ---
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
///
/// ---
pub struct ContainerBuilder {
    container: Container,
}

impl ContainerBuilder {
    #[inline]
    pub fn new() -> Self {
        ContainerBuilder { container: Container::new(), }
    }

    /// Sets a `Header` component for the `Container`.
    ///
    /// ---
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text to display in the header.
    ///
    /// ---
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// ---
    ///
    /// # Example
    /// ```rust
    /// // Sets a `Header` component with the label "Welcome".
    /// ContainerBuilder::new()
    ///     .header("Welcome")?;
    /// ```
    ///
    /// ---
    pub fn header(mut self, label: &str) -> FtuiResult<Self> {
        self.container.set_header(cpn::Header::new(label)?);
        Ok(self)
    }

    /// Adds an `Option` component to the `Container`.
    ///
    /// ---
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text displayed for this option.
    /// - `callback`: An optional `Callback` invoked when the option is selected.
    ///
    /// ---
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// ---
    ///
    /// # Example
    /// ```rust
    /// // Add an `Option` component with the label "Option" and no `Callback`.
    /// ContainerBuilder::new()
    ///     .option("Option", None)?;
    /// ```
    ///
    ///---
    pub fn option(
        mut self, label: &str, callback: impl Into<Option<Callback>>
    ) -> FtuiResult<Self> {
        self.container.add_option(cpn::Option::new(label, callback)?);
        Ok(self)
    }

    /// Adds a `Text` component to the `Container`.
    /// 
    /// ---
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text to display.
    /// - `flags`: A set of `TextFlags`, combined using the bitwise OR operator.
    ///
    /// ---
    ///
    /// # Notes
    /// - This is what bitwise OR operator look like -> `flag1 | flag2 | flag3 ...`
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// ---
    ///
    /// # Example
    /// ```rust
    /// // Create a `Text` component labeled "Text", right-aligned and with
    /// // a magenta background.
    /// ContainerBuilder::new()
    ///     .text("Text", TextFlags::ALIGN_RIGHT | TextFlags::COLOR_MAGENTA_BACK)?;
    /// ```
    ///
    /// --- 
    pub fn text(
        mut self, label: &str, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.add_text(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    /// Adds an `Option` component to the `Container` and stores its ID.
    ///
    /// ---
    /// 
    /// # Parameters
    /// - `label`: The text displayed for this option.
    /// - `callback`: An optional `Callback` invoked when the option is selected.
    /// - `store_id`: A `&mut u16` to store the created `Option` component ID.
    /// 
    /// ---
    ///
    /// # Returns
    /// - `Ok(ContainerBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error if creation or insertion fails.
    /// 
    /// ---
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
    ///
    /// ---
    pub fn option_id(
        mut self,
        label: &str, callback: impl Into<Option<Callback>>, store_id: &mut u16
    ) -> FtuiResult<Self> {
        *store_id = self.container.add_option(cpn::Option::new(label, callback)?);
        Ok(self)
    }

    pub fn text_id(
        mut self,
        label: &str,
        flags: impl Into<Option<cpn::TextFlags>>, store_id: &mut u16
    ) -> FtuiResult<Self> {
        *store_id = self.container.add_text(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    pub fn separator_normal(mut self, style: cpn::SeparatorStyle) -> Self {
        self.container.add_separator(cpn::Separator::normal(style));
        self
    }

    pub fn separator_dotted(mut self, style: cpn::SeparatorStyle) -> Self {
        self.container.add_separator(cpn::Separator::dotted(style));
        self
    }

    /// Set a `Selector` for the `Container` to handle user navigation.
    ///
    /// ---
    ///
    /// # Parameters 
    /// - `up_trig`: A `Trigger` that moves the selector up when activated.
    /// - `down_trig`: A `Trigger` that moves the selector down when activated.
    /// - `selc_trig`: A `Trigger` that confirms the selection when activated.
    ///
    /// ---
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    /// 
    /// ---
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
    ///
    /// ---
    pub fn selector(
        mut self, up_trig: Trigger, down_trig: Trigger, selc_trig: Trigger
    ) -> Self {
        self.container.set_selector(Selector::new(up_trig, down_trig, selc_trig));
        self
    }

    /// Sets a `Selector` with no `Trigger`s for the `Container`.
    ///
    /// ---
    ///
    /// In this case, navigation must be handled manually using the following methods:
    /// - `Container::selector_up`
    /// - `Container::selector_down`
    /// - `Container::selector_select`
    ///
    /// ---
    ///
    /// # Returns
    /// - `ContainerBuilder`: Returns `self`.
    ///
    /// ---
    ///
    /// # Example
    /// ```rust
    /// // Set a `Selector` with no `Trigger`s.
    /// ContainerBuilder::new()
    ///     .selector_no_triggers();
    /// ```
    ///
    /// ---
    pub fn selector_no_triggers(mut self) -> Self {
        self.container.set_selector(Selector::no_triggers());
        self
    }

    pub fn build(self) -> Container {
        self.container
    }
}
