use crate::{
    components::{self as cpn},
    error::FtuiResult, util::id::IdGenerator, renderer::Renderer};

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
#[derive(Debug, PartialEq, Eq)]
pub struct Container {
    id_generator: IdGenerator<u16>,
    header: Option<cpn::Text>,
    footer: Option<cpn::Text>,
    options: cpn::OptionsManager,
    texts: cpn::TextsManager,
    separators: Vec<cpn::Separator>,
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
            footer: None,
            options: cpn::OptionsManager::new(),
            texts: cpn::TextsManager::new(),
            separators: vec![],
            component_count: 0,
        }
    }

    pub(crate) fn set_header(&mut self, header: cpn::Text) {
        self.header = Some(header);
        self.component_count += 1;
    }
    
    #[inline]
    pub(crate) fn set_footer(&mut self, footer: cpn::Text) {
        self.footer = Some(footer);
    }

    // Return added Option ID.
    pub(crate) fn add_option(&mut self, mut option: cpn::Option) -> u16 {
        let id = self.id_generator.get_id();
        option.set_id(id);
        option.set_line(self.component_count);

        if self.options.comps().is_empty() {
            option.set_selc_on(true);
        }

        self.options.add(option);
        self.component_count += 1;

        id
    }

    // Return added Text ID.
    pub(crate) fn add_text(&mut self, mut text: cpn::Text) -> u16 {
        let id = self.id_generator.get_id();
        text.set_id(id);
        text.set_line(self.component_count);

        self.texts.add(text);
        self.component_count += 1;

        id
    }

    pub(crate) fn add_separator(&mut self, mut separator: cpn::Separator) {
        separator.set_line(self.component_count);
        self.separators.push(separator);
        self.component_count += 1;
    }

    pub fn options(&self) -> &cpn::OptionsManager {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut cpn::OptionsManager {
        &mut self.options
    }

    pub fn text(&self) -> &cpn::TextsManager {
        &self.texts
    }

    pub fn text_mut(&mut self) -> &mut cpn::TextsManager {
        &mut self.texts
    }

    pub(crate) fn option_comps(&self) -> &[cpn::Option] {
        &self.options.comps()
    }

    pub(crate) fn text_comps_mut(&mut self) -> &mut [cpn::Text] {
        self.texts.comps_mut()
    }

    pub(crate) fn component_count(&self) -> u16 {
        self.component_count
    }

    pub(crate) fn header_mut(&mut self) -> &mut Option<cpn::Text> {
        &mut self.header
    }

    pub(crate) fn footer_mut(&mut self) -> &mut Option<cpn::Text> {
        &mut self.footer
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

impl Into<Container> for ContainerBuilder {
    fn into(self) -> Container {
        self.container
    }
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
    pub fn header_expl(mut self, header: cpn::Text) -> Self {
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
    pub fn header(
        self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        Ok(self.header_expl(cpn::Text::new(label, flags)?))
    }

    pub fn footer_expl(mut self, footer: cpn::Text) -> Self {
        self.container.set_footer(footer);
        self
    }

    #[inline]
    pub fn footer(
        self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        Ok(self.footer_expl(cpn::Text::new(label, flags)?))
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
    pub fn option(self, label: impl ToString) -> FtuiResult<Self> {
        Ok(self.option_expl(cpn::Option::new(label)?))
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
    pub fn option_id(self, label: impl ToString, store_id: &mut u16) -> FtuiResult<Self> {
        Ok(self.option_id_expl(cpn::Option::new(label)?, store_id))
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
        self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
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
        label: impl ToString,
        flags: impl Into<Option<cpn::TextFlags>>, store_id: &mut u16
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

    pub fn instant_draw(self, renderer: &mut Renderer) -> FtuiResult<()> {
        renderer.draw(self.container)
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
