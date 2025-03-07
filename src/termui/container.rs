use std::option;

use crate::tui::sel;

use crate::tui::{
    cpn::hed,
    cpn::opt,
    cpn::txt,
    emg,
};

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
