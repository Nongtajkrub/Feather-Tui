use crate::tui::cbk;

pub struct Option {
    label: String,
    line: u16,
    selc_on: bool,
    callback: cbk::Callback,
}

impl Option {
    pub fn new(label: &str, callback: cbk::Callback) -> Option {
        Option {
            label: label.to_string(),
            line: 0,
            selc_on: false,
            callback,
        }
    }

    pub fn set_line(&mut self, line: u16) {
        self.line = line;
    }

    pub fn line(&self) -> u16 {
        return self.line;
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }

    pub fn selc_on(&self) -> bool {
        return self.selc_on;
    }

    pub fn callback(&self) -> &cbk::Callback {
        return &self.callback;
    }

    pub fn set_selc_on(&mut self, value: bool) {
        self.selc_on = value;
    }
}
