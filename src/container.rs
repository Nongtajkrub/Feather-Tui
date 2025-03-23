use crate::sel;

use crate::{
    cpn::hed,
    cpn::opt,
    cpn::txt,
    emg,
};

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
/// // Define a container with a header, two options, a text component, and a selector.
/// let mut container = tui::con::Container::new()
///     .with_header(tui::cpn::hed::Header::new("Main Menu"))
///     .with_option(
///         tui::cpn::opt::Option::new(
///             "Option1",
///             tui::cbk::Callback::new(callback_func, 1u32)))
///     .with_option(
///         tui::cpn::opt::Option::new(
///             "Option2",
///             tui::cbk::Callback::new(callback_func, 2u32)))
///     .with_text(
///         tui::cpn::txt::Text::new(
///             "Text", 
///             tui::cpn::txt::TextFlags::COLOR_YELLOW_BACK |
///             tui::cpn::txt::TextFlags::ALIGN_RIGHT))
///     .with_selector(
///         tui::sel::Selector::new(
///             tui::trg::Trigger::new(up_trig_func, key_char),
///             tui::trg::Trigger::new(down_trig_func, key_char),
///             tui::trg::Trigger::new(selc_trig_func, key_char)));
///
/// // The container can then be passed to a `Renderer` for display.
/// ```
pub struct Container {
    header: std::option::Option<hed::Header>,
    options: Vec<opt::Option>,
    selector: std::option::Option<sel::Selector>,
    texts: Vec<txt::Text>,
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
    pub fn looper(&mut self) -> bool {
        return self.selector.as_mut().expect(emg::NO_SELETOR_ERRMSG)
            .looper(&mut self.options);
    }

    pub fn set_header(&mut self, header: hed::Header) {
        self.header = Some(header);
        self.component_count += 1;
    }

    pub fn set_selector(&mut self, selector: sel::Selector) {
        self.selector = Some(selector);
    }

    pub fn add_option(&mut self, mut option: opt::Option) {
        if self.options.len() == 0 {
            option.set_selc_on(true);
        }

        self.options.push(option);
        self.options.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn add_text(&mut self, text: txt::Text) {
        self.texts.push(text);
        self.texts.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn with_header(mut self, header: hed::Header) -> Self {
        self.set_header(header);
        self
    }

    pub fn with_option(mut self, option: opt::Option) -> Self {
        self.add_option(option);
        self
    }

    pub fn with_text(mut self, text: txt::Text) -> Self {
        self.add_text(text);
        self
    }

    pub fn with_selector(mut self, selector: sel::Selector) -> Self {
        self.set_selector(selector);
        self
    } 

    pub fn header(&self) -> &std::option::Option<hed::Header>{
        return &self.header;
    }

    pub fn options(&self) -> &Vec<opt::Option> {
        return &self.options;
    }

    pub fn texts(&self) -> &Vec<txt::Text> {
        return &self.texts;
    }

    pub fn texts_mut(&mut self) -> &mut Vec<txt::Text> {
        return &mut self.texts;
    }

    pub fn selector_mut(&mut self) -> &mut sel::Selector {
        return self.selector.as_mut().expect(emg::NO_SELETOR_ERRMSG);
    }
}
