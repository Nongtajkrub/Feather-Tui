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
        mut self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.set_header(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    #[inline]
    pub fn footer(
        mut self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.set_footer(cpn::Text::new(label, flags)?);
        Ok(self)
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
    pub fn option(mut self, label: impl ToString) -> FtuiResult<Self> {
        self.container.add_option(cpn::Option::new(label));
        Ok(self)
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
    pub fn option_id(mut self, label: impl ToString, store_id: &mut u16) -> FtuiResult<Self> {
        *store_id = self.container.add_option(cpn::Option::new(label)); 
        Ok(self)
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
        mut self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.add_text(cpn::Text::new(label, flags)?);
        Ok(self)
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
        mut self, 
        label: impl ToString,
        flags: impl Into<Option<cpn::TextFlags>>, store_id: &mut u16
    ) -> FtuiResult<Self> {
        *store_id = self.container.add_text(cpn::Text::new(label, flags)?);
        Ok(self)
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
    pub fn separator_normal(mut self, style: cpn::SeparatorStyle) -> Self {
        self.container.add_separator(cpn::Separator::normal(style));
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
    pub fn separator_dotted(mut self, style: cpn::SeparatorStyle) -> Self {
        self.container.add_separator(cpn::Separator::dotted(style));
        self
    }

    /// Renders the current `Container` directly to the terminal without
    /// creating and returning a new one.
    ///
    /// # Parameters
    /// - `renderer`: A mutable type that implements `AsMut<Renderer>`.
    ///
    /// # Returns
    /// - `Ok(())` if the container was successfully drawn.
    /// - `Err(FtuiError)` if rendering failed.
    ///
    /// # Example
    /// ```rust
    /// ContainerBuilder::new()
    ///     .header(...)?
    ///     .option(...)
    ///     .instant_draw(Renderer::new(...))?;
    /// ```
    pub fn instant_draw(self, mut renderer: impl AsMut<Renderer>) -> FtuiResult<()> {
        renderer.as_mut().draw(self.container)
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
