use std::io::SeekFrom;

use crate::termui::components::{
    header::Header,
    option::Option,
    text::Text,
};

pub struct Container {
    header: std::option::Option<Header>,
    options: Vec<Option>,
    texts: Vec<Text>,
    component_count: u16,
}

impl Container { 
    pub fn new() -> Container {
        Container {
            header: None,
            options: Vec::new(),
            texts: Vec::new(),
            component_count: 0,
        }
    }

    pub fn set_header(&mut self, header: Header) {
        self.header = Some(header);
    }

    pub fn add_option(&mut self, option: Option) {
        self.options.push(option);
        self.options.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn add_text(&mut self, text: Text) {
        self.texts.push(text);
        self.texts.last_mut().unwrap().set_line(self.component_count);
        self.component_count += 1;
    }

    pub fn header(&self) -> &std::option::Option<Header>{
        return &self.header;
    }

    pub fn option(&self) -> &Vec<Option> {
        return &self.options;
    }

    pub fn texts(&self) -> &Vec<Text> {
        return &self.texts;
    }
}
