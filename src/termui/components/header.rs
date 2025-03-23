pub struct Header {
    label: String,
}

impl Header {
    pub fn new(label: &str) -> Header {
        Header { 
            label: label.to_string(), 
        }
    }

    pub fn len(&self) -> usize {
        return self.label.len();
    }

    pub fn label(&self) -> &String {
        return &self.label;
    }
}
