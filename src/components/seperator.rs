#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeperatorStyle {
    Solid,
    Medium,
    Thin,
    Double,
    Custom(char),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seperator {
    line: u16,
    style: SeperatorStyle,
}

impl Seperator {
    pub fn new(style: SeperatorStyle) -> Self {
        Seperator {
            line: 0,
            style,
        }
    }

    pub(crate) fn set_line(&mut self, line: u16) {
        self.line = line; 
    }

    pub(crate) fn line(&self) -> u16 {
        self.line
    }

    pub(crate) fn style(&self) -> SeperatorStyle {
        self.style
    }
}
