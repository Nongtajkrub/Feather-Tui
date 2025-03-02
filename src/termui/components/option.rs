pub struct Option {
    label: String,
    line: u16,
    selc_on: bool,
}

impl Option {
    pub fn new(label: String) -> Option {
        Option {
            label,
            line: 0,
            selc_on: false
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

    pub fn set_selc_on(&mut self, value: bool) {
        self.selc_on = value;
    }
}
