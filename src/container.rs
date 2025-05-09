use crate::{
    cbk::Callback, trg::Trigger, cpn, error::{FtuiError, FtuiResult}, slc::Selector,
    ren::Renderer, util::id::IdGenerator
};
use std::option::Option;

/// `Container` acts as a layout manager for the UI elements (headers, options,
/// text, and selector).
///
/// # Usage
///
/// The `Renderer` requires a `Container` object to render the UI to the terminal.  
/// Components can be added to the container using method chaining, allowing for a  
/// structured and organized layout.
///
/// # Example
/// ```rust
/// use feather_tui as tui;
///
/// // Define a container with a header, two options, a text component, 
/// // and a selector.
/// let mut container = tui::con::Container::new()
///     .with_header("Header")?
///     .with_option("Option1", tui::cbk::Callback::new(callback_func, arg))? 
///     .with_option("Option2", tui::cbk::Callback::new(callback_func, arg))?
///     .with_text(
///         "Text",
///         tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |
///         tui::cpn::txt::TextFlags::ALIGN_RIGHT)?
///     .with_selector(
///         tui::sel::Selector::new(
///             tui::trg::Trigger::new(up_trig_func, arg),
///             tui::trg::Trigger::new(down_trig_func, arg),
///             tui::trg::Trigger::new(selc_trig_func, arg)));
///
/// // The container can then be passed to a `Renderer` for display.
/// ```
pub struct Container {
    id_generator: IdGenerator<u16>,
    header: Option<cpn::Header>,
    options: Vec<cpn::Option>,
    selector: Option<Selector>,
    texts: Vec<cpn::Text>,
    separators: Vec<cpn::Separator>,
    component_count: u16,
}

impl Container { 
    pub fn new() -> Container {
        Container {
            id_generator: IdGenerator::new(),
            header: None,
            options: vec![],
            selector: None,
            texts: vec![],
            separators: vec![],
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
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     let mut container = tui::con::Container::new();
    ///
    ///     // Re-render the UI if an update occurred.
    ///     if container.looper()? {
    ///         render();
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn looper(&mut self) -> FtuiResult<bool> {
        if self.options.len() > 0 {
            return match self.selector.as_mut() {
                Some(selector) => Ok(selector.looper(&mut self.options)?),
                None => Err(FtuiError::ContainerLooperNoSelector),
            };
        }

        Ok(false)
    }

    /// Assigns a header component for the container.
    ///
    /// # Parameters 
    /// - `header`: The `Header` to assign to the container.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     let mut container = tui::con::Container::new();
    ///
    ///     let header = tui::cpn::Header::new("Header")?;
    ///
    ///     // Assign the header to the container.
    ///     container.set_header(header);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_header(&mut self, header: cpn::Header) {
        self.header = Some(header);
        self.component_count += 1;
    }

    /// Assigns a selector to the container for handling user navigation.
    ///
    /// # Parameters 
    /// - `selector`: The `Selector` to assign to the container.
    ///
    /// # Example
    /// ```rust
    /// use feather_tui as tui;
    ///
    /// fn main() -> tui::err::FtuiResult<()> {
    ///     let mut container = tui::con::Container::new();
    ///
    ///     // Assume up_trig, down_trig, and selc_trig are defined elsewhere.
    ///     let selector = tui::slc::Selector::new(up_trig, down_trig, selc_trig);
    ///
    ///     // Set the selector for the container.
    ///     container.set_selector(selector);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = Some(selector);
    }

    // Return added option ID.
    pub fn add_option(&mut self, mut option: cpn::Option) -> u16 {
        if self.options.len() == 0 {
            option.set_selc_on(true);
        }

        let id = self.id_generator.get_id();
        option.set_id(id);

        self.options.push(option);
        self.options.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;

        id
    }

    pub fn add_text(&mut self, text: cpn::Text) {
        self.texts.push(text);
        self.texts.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn add_separator(&mut self, separator: cpn::Separator) {
        self.separators.push(separator);
        self.separators.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }


    #[inline]
    pub fn draw(&mut self, width: u16, height: u16) -> FtuiResult<()> {
        Renderer::new(width, height).simple_draw(self)?;
        Ok(())
    }

    #[inline]
    pub fn draw_fullscreen(&mut self) -> FtuiResult<()> {
        Renderer::fullscreen()?.simple_draw(self)?;
        Ok(())
    }

    #[inline]
    pub fn option(&self, id: u16) -> FtuiResult<&cpn::Option> {
        self.options.iter()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoOptionById)
    }

    #[inline]
    pub fn option_mut(&mut self, id: u16) -> FtuiResult<&mut cpn::Option> {
        self.options.iter_mut()
            .find(|option| option.id() == id)
            .ok_or(FtuiError::ContainerNoOptionById)
    }

    pub(crate) fn header(&self) -> &Option<cpn::Header> {
        return &self.header;
    }

    pub(crate) fn options(&self) -> &[cpn::Option] {
        return &self.options;
    }

    pub(crate) fn texts_mut(&mut self) -> &mut [cpn::Text] {
        return &mut self.texts;
    }

    pub(crate) fn separators(&self) -> &[cpn::Separator] {
        return &self.separators;
    }

    pub fn selector_mut(&mut self) -> FtuiResult<&mut Selector> {
        self.selector.as_mut().ok_or(FtuiError::ContainerNoSelector)
    }

    pub fn selector_up(&mut self) -> FtuiResult<bool> {
        let selector = self.selector.as_mut().ok_or(FtuiError::ContainerNoSelector)?;
        Ok(selector.move_up(&mut self.options))
    }

    pub fn selector_down(&mut self) -> FtuiResult<bool> {
        let selector = self.selector.as_mut().ok_or(FtuiError::ContainerNoSelector)?;
        Ok(selector.move_down(&mut self.options))
    }

    pub fn selector_select(&mut self) -> FtuiResult<bool> {
        let selector = self.selector.as_mut().ok_or(FtuiError::ContainerNoSelector)?;
        selector.selc(&mut self.options)?;
        Ok(true)
    }

    pub(crate) fn component_count(&self) -> u16 {
        self.component_count
    }
}

pub struct ContainerBuilderId {
    builder: ContainerBuilder,
    id: u16,
}

impl ContainerBuilderId {
    pub fn new(builder: ContainerBuilder, id: u16) -> Self {
        ContainerBuilderId {
            builder,
            id,
        }
    }

    pub fn done(self) -> ContainerBuilder {
        self.builder
    }

    pub fn store_id(self, at: &mut u16) -> ContainerBuilder {
        *at = self.id;
        self.builder
    }
}

pub struct ContainerBuilder {
    container: Container,
}

impl ContainerBuilder {
    #[inline]
    pub fn new() -> Self {
        ContainerBuilder { container: Container::new(), }
    }

    pub fn header(mut self, label: &str) -> FtuiResult<Self> {
        self.container.set_header(cpn::Header::new(label)?);
        Ok(self)
    }

    pub fn option(
        mut self, label: &str, callback: impl Into<Option<Callback>>
    ) -> FtuiResult<ContainerBuilderId> {
        let id = self.container.add_option(cpn::Option::new(label, callback)?);
        Ok(ContainerBuilderId::new(self, id))
    }

    pub fn text(
        mut self, label: &str, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.container.add_text(cpn::Text::new(label, flags)?);
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

    pub fn selector(
        mut self, up_trig: Trigger, down_trig: Trigger, selc_trig: Trigger
    ) -> Self {
        self.container.set_selector(Selector::new(up_trig, down_trig, selc_trig));
        self
    }

    pub fn selector_no_triggers(mut self) -> Self {
        self.container.set_selector(Selector::no_triggers());
        self
    }

    pub fn build(self) -> Container {
        self.container
    }
}
