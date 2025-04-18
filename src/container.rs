use crate::{cbk::Callback, cpn, error::{FtuiError, FtuiResult}, slc::Selector, ren::Renderer};
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
            header: None,
            options: vec![],
            selector: None,
            texts: vec![],
            separators: vec![],
            component_count: 0,
        }
    }

    // return whether an update occure
    pub fn looper(&mut self) -> FtuiResult<bool> {
        if self.options.len() > 0 {
            return match self.selector.as_mut() {
                Some(selector) => Ok(selector.looper(&mut self.options)?),
                None => Err(FtuiError::ContainerLooperNoSelector),
            };
        }

        Ok(false)
    }

    pub fn set_header(&mut self, header: cpn::Header) {
        self.header = Some(header);
        self.component_count += 1;
    }

    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = Some(selector);
    }

    pub fn add_option(&mut self, mut option: cpn::Option) {
        if self.options.len() == 0 {
            option.set_selc_on(true);
        }

        self.options.push(option);
        self.options.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
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

    pub fn with_header(mut self, label: &str) -> FtuiResult<Self> {
        self.set_header(cpn::Header::new(label)?);
        Ok(self)
    }

    pub fn with_option(
        mut self, label: &str, callback: Callback
    ) -> FtuiResult<Self> {
        self.add_option(cpn::Option::new(label, callback)?);
        Ok(self)
    }

    pub fn with_text(
        mut self, label: &str, flags: impl Into<Option<cpn::TextFlags>>
    ) -> FtuiResult<Self> {
        self.add_text(cpn::Text::new(label, flags)?);
        Ok(self)
    }

    pub fn with_separator_normal(mut self, style: cpn::SeparatorStyle) -> Self {
        self.add_separator(cpn::Separator::normal(style));
        self
    }

    #[deprecated(
        since = "3.1.0",
        note = "Typo in method name use `with_separator_dotted` instead")]
    pub fn with_selector_dotted(mut self, style: cpn::SeparatorStyle) -> Self {
        self.add_separator(cpn::Separator::dotted(style));
        self
    }

    pub fn with_separator_dotted(mut self, style: cpn::SeparatorStyle) -> Self {
        self.add_separator(cpn::Separator::dotted(style));
        self
    }

    pub fn with_selector(mut self, selector: Selector) -> Self {
        self.set_selector(selector);
        self
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

    pub(crate) fn header(&self) -> &Option<cpn::Header>{
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

    pub(crate) fn component_count(&self) -> u16 {
        self.component_count
    }
}
