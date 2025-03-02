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
}
