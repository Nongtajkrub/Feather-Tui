use crate::components as cpn;
use crate::error::FtuiResult;
use crate::error::FtuiError;
use crate::renderer::Renderer;
use crate::util::Colors;
use crate::util::id::IdGenerator;
use crate::util::id::GeneratedId;
use crate::util::RenderableMut;
use crate::util::Renderable;

/// A general container used to store and organize UI components,
/// including `Header`, `Option`, `Text`, and `Separator`. It is created using
/// a `GeneralBuilder`.
///
/// # Usage
/// - Handle UI events with the `looper` method.
/// - Render the UI using a `Renderer` (recommended).
/// - Alternatively, use the `draw` or `draw_fullscreen` methods.
/// - Access `Option` components by ID using `option` and `option_mut`.
/// - Access `Text` components by ID using `text` and `text_mut`.
/// - Navigate using `selector_up`, `selector_down`, and `selector_select`.
#[derive(Debug, PartialEq, Eq)]
pub struct General {
    id_generator: IdGenerator,
    header: Option<cpn::Text>,
    footer: Option<cpn::Text>,
    options: cpn::OptionsManager,
    texts: cpn::TextsManager,
    separators: Vec<cpn::Separator>,
    component_count: u16,
}

impl General {
    /// Constructs a new `General`. 
    ///
    /// # Returns
    /// `General`: A new instance of `General`.
    ///
    /// # Example
    /// ```rust
    /// let _ = General::new();
    /// ```
    pub(crate) fn new() -> General {
        General {
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
    pub(crate) fn add_option(&mut self, mut option: cpn::Option) -> GeneratedId {
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
    pub(crate) fn add_text(&mut self, mut text: cpn::Text) -> GeneratedId {
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

    pub fn options_mut(&mut self) -> &mut cpn::OptionsManager {
        &mut self.options
    }
}

/// `GeneralBuilder` is used to create `General` instances using the builder
/// pattern. This allows for a flexible and readable way to construct complex
/// containers by chaining method calls.
///
/// # Example
/// ```rust
/// // Create a container with a header, two options, a separator, some text,
/// // and a selector.
/// let container: General = GeneralBuilder::new()
///     .header(...)?
///     .option(...)
///     .option(...)
///     .separator_normal(...)
///     .text(...)?
///     .selector(...)?
///     .build();
/// ```
pub struct GeneralBuilder {
    container: General,
}

impl Into<General> for GeneralBuilder {
    fn into(self) -> General {
        self.container
    }
}

impl GeneralBuilder {
    /// Constructs a new `GeneralBuilder`. 
    ///
    /// # Return
    /// `GeneralBuilder`: A new instance of `GeneralBuilder`.
    ///
    /// # Example
    /// ```rust
    /// let _ = GeneralBuilder::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        GeneralBuilder { container: General::new(), }
    }

    /// Sets the header for the `General`.
    ///
    /// # Notes
    /// The header behaves similarly to a `Text` component and can display
    /// styled text using the provided flags.
    ///
    /// # Parameters
    /// - `label`: An `impl ToString` is the the text to display in the header.
    /// - `flags`: An optional set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Returns
    /// - `Ok(GeneralBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a header with the label "Welcome" in red.
    /// GeneralBuilder::new()
    ///     .header("Welcome", TextFlags::COLOR_RED)?;
    /// ```
    #[inline]
    pub fn header(
        mut self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.set_header(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    /// Sets the footer for the `General`.
    ///
    /// # Notes
    /// The footer behaves similarly to a `Text` component and can display
    /// styled text using the provided flags.
    ///
    /// # Parameters
    /// - `label`: An `impl ToString` is the the text to display in the footer.
    /// - `flags`: An optional set of `TextFlags` combined using the bitwise OR operator.
    ///
    /// # Returns
    /// - `Ok(GeneralBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Sets a footer with the label "q -> exit" in red.
    /// GeneralBuilder::new()
    ///     .footer("q -> exit", TextFlags::COLOR_RED)?;
    /// ```
    #[inline]
    pub fn footer(
        mut self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.set_footer(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    /// Adds an `Option` component to the `General`.
    ///
    /// # Parameters
    /// - `label`: A `&str` representing the text displayed for this option.
    /// - `callback`: An optional `Callback` invoked when the option is selected.
    ///
    /// # Returns
    /// - `Ok(GeneralBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Add an `Option` component with the label "Option" and no `Callback`.
    /// GeneralBuilder::new()
    ///     .option("Option", None);
    /// ```
    #[inline]
    pub fn option(mut self, label: impl ToString) -> Self {
        self.container.add_option(cpn::Option::new(label));
        self
    }

    /// Adds an `Option` component to the `General` and stores its ID.
    ///
    /// # Parameters
    /// - `label`: The text displayed for this option.
    /// - `callback`: An optional `Callback` invoked when the option is selected.
    /// - `store_id`: A `&mut u16` to store the created `Option` component ID.
    /// 
    /// # Returns
    /// - `Ok(GeneralBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    /// 
    /// # Example
    /// ```rust
    /// let mut id = 0u16;
    ///
    /// // Add an `Option` labeled "Option" with no `Callback`,
    /// // storing the generated ID in `id`.
    /// GeneralBuilder::new()
    ///     .option_id("Option", None, &mut id);
    /// ```
    #[inline]
    pub fn option_id(mut self, label: impl ToString, store_id: &mut GeneratedId) -> Self {
        *store_id = self.container.add_option(cpn::Option::new(label)); 
        self
    }

    #[inline]
    pub fn option_highligh(mut self, color: Colors) -> Self {
        self.container.options_mut().set_highlight(color);
        self
    }

    /// Adds a `Text` component to the `General`.
    /// 
    /// # Parameters
    /// - `label`: A `&str` representing the text to display.
    /// - `flags`: A set of `TextFlags`, combined using the bitwise OR operator.
    ///
    /// # Notes
    /// - This is what bitwise OR operator look like -> `flag1 | flag2 | flag3 ...`
    ///
    /// # Returns
    /// - `Ok(GeneralBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// // Create a `Text` component labeled "Text", right-aligned and with
    /// // a magenta background.
    /// GeneralBuilder::new()
    ///     .text("Text", TextFlags::ALIGN_RIGHT | TextFlags::COLOR_MAGENTA_BACK)?;
    /// ```
    #[inline]
    pub fn text(
        mut self, label: impl ToString, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.add_text(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    /// Adds a `Text` component to the `General` and stores its ID.
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
    /// - `Ok(GeneralBuilder)`: Returns `self`.
    /// - `Err(FtuiError)`: Returns an error.
    ///
    /// # Example
    /// ```rust
    /// let mut id = 0u16;
    ///
    /// // Create a `Text` component labeled "Text", right-aligned and with
    /// // a magenta background. storing the generated ID in `id`.
    /// GeneralBuilder::new()
    ///     .text(
    ///         "Text",
    ///         TextFlags::ALIGN_RIGHT | TextFlags::COLOR_MAGENTA_BACK, &mut id)?;
    /// ```
    #[inline]
    pub fn text_id(
        mut self, 
        label: impl ToString,
        flags: impl Into<Option<cpn::TextFlags>>, store_id: &mut GeneratedId
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
    /// - `GeneralBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Add a normal separator with a solid style.
    /// GeneralBuilder::new()
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
    /// - `GeneralBuilder`: Returns `self`.
    ///
    /// # Example
    /// ```rust
    /// // Add a dotted separator with a solid style.
    /// GeneralBuilder::new()
    ///     separator_dotted(SeparatorStyle::Solid);
    /// ```
    #[inline]
    pub fn separator_dotted(mut self, style: cpn::SeparatorStyle) -> Self {
        self.container.add_separator(cpn::Separator::dotted(style));
        self
    }

    /// Renders the current `General` directly to the terminal without
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
    /// GeneralBuilder::new()
    ///     .header(...)?
    ///     .option(...)
    ///     .instant_draw(Renderer::new(...))?;
    /// ```
    pub fn instant_draw(mut self, mut renderer: impl AsMut<Renderer>) -> FtuiResult<()> {
        renderer.as_mut().draw(&mut self.container)
    }

    /// Finalizes the construction of a `General`. This method should be called
    /// after all desired components have been added using the builder pattern.
    /// It consumes `self` and returns the completed `General`.
    ///
    /// # Returns
    /// - `General`: Returns the created `General`.
    ///
    /// # Example
    /// ```rust
    /// let container: General = GeneralBuilder::new()
    ///     .header(...)?
    ///     .option(...)
    ///     .option(...)
    ///     .separator_normal(...)
    ///     .text(...)?
    ///     .selector(...)?
    ///     .build(); // Finalize and retrieve the constructed container.
    /// ```
    pub fn build(self) -> General {
        self.container
    }
}

impl RenderableMut<Renderer> for General {
    fn render(&mut self, renderer: &mut Renderer) -> FtuiResult<()> {
        let (_, height) = renderer.get_dimensions();

        if self.component_count > height {
            return Err(FtuiError::RendererContainerTooBig);
        }

        renderer.clear();

        if let Some(header) = &mut self.header {
            header.render(renderer)?;
        }

        self.options.render(renderer)?;
        self.texts.render(renderer)?;
        
        for seperator in self.separators.iter_mut() {
            seperator.render(renderer)?;
        }

        if let Some(footer) = &mut self.footer {
            renderer.render_text_as_footer(footer)?;
        }

        Ok(())
    }
}
