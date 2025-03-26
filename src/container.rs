use crate::{cbk, cpn, emg, error::FtuiError, slc};

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
///     .with_header("Header")
///     .with_option("Option1", tui::cbk::Callback::new(callback_func, arg)) 
///     .with_option("Option2", tui::cbk::Callback::new(callback_func, arg))
///     .with_text(
///         "Text",
///         tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |
///         tui::cpn::txt::TextFlags::ALIGN_RIGHT)
///     .with_selector(
///         tui::sel::Selector::new(
///             tui::trg::Trigger::new(up_trig_func, arg),
///             tui::trg::Trigger::new(down_trig_func, arg),
///             tui::trg::Trigger::new(selc_trig_func, arg)));
///
/// // The container can then be passed to a `Renderer` for display.
/// ```
pub struct Container {
    header: Option<cpn::hed::Header>,
    options: Vec<cpn::opt::Option>,
    selector: Option<slc::Selector>,
    texts: Vec<cpn::txt::Text>,
    component_count: u16,
}

impl Container { 
    pub fn new() -> Container {
        Container {
            header: None,
            options: Vec::new(),
            selector: None,
            texts: Vec::new(),
            component_count: 0,
        }
    }

    // return whether an update occure
    pub fn looper(&mut self) -> Result<bool, FtuiError> {
        return match self.selector.as_mut() {
            Some(selector) => Ok(selector.looper(&mut self.options)),
            None => Err(FtuiError::ContainerNoSelector),
        };
    }

    pub fn set_header(&mut self, header: cpn::hed::Header) {
        self.header = Some(header);
        self.component_count += 1;
    }

    pub fn set_selector(&mut self, selector: slc::Selector) {
        self.selector = Some(selector);
    }

    pub fn add_option(&mut self, mut option: cpn::opt::Option) {
        if self.options.len() == 0 {
            option.set_selc_on(true);
        }

        self.options.push(option);
        self.options.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn add_text(&mut self, text: cpn::txt::Text) {
        self.texts.push(text);
        self.texts.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn with_header(mut self, label: &str) -> Result<Self, FtuiError> {
        self.set_header(cpn::hed::Header::new(label)?);
        Ok(self)
    }

    pub fn with_option(
        mut self, label: &str, callback: cbk::Callback
    ) -> Result<Self, FtuiError> {
        self.add_option(cpn::opt::Option::new(label, callback)?);
        Ok(self)
    }

    pub fn with_text(
        mut self, label: &str, flags: impl Into<Option<cpn::txt::TextFlags>>
    ) -> Result<Self, FtuiError> {
        self.add_text(cpn::txt::Text::new(label, flags)?);
        Ok(self)
    }

    pub fn with_selector(mut self, selector: slc::Selector) -> Self {
        self.set_selector(selector);
        self
    } 

    pub fn header(&self) -> &Option<cpn::hed::Header>{
        return &self.header;
    }

    pub fn options(&self) -> &Vec<cpn::opt::Option> {
        return &self.options;
    }

    pub fn texts(&self) -> &Vec<cpn::txt::Text> {
        return &self.texts;
    }

    pub fn texts_mut(&mut self) -> &mut Vec<cpn::txt::Text> {
        return &mut self.texts;
    }

    pub fn selector_mut(&mut self) -> &mut slc::Selector {
        return self.selector.as_mut().expect(emg::NO_SELETOR_ERRMSG);
    }
}
