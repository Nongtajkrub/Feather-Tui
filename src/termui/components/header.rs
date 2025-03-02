pub struct Header {
    label: String,
}

impl Header {
    pub fn new(label: String) -> Header {
        Header { label }
    }

    pub fn len(&self) -> usize {
        return self.label.len();
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }
}
