pub struct Text {
    label: String,
    line: u16,
}

impl Text {
    pub fn new(label: String) -> Text {
        Text {
            label,
            line: 0
        }
    }

    pub fn set_line(&mut self, line: u16) {
        self.line = line;
    }
}
