use crate::{components::{TextFlags, Text}, error::FtuiResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    header: Option<Text>,
    footer: Option<Text>,
    data: String,
    flags: TextFlags,
    style: Vec<&'static str>,
}

impl Document {
    pub(crate) fn new() -> Self {
        Self {
            header: None,
            footer: None,
            data: String::new(),
            flags: TextFlags::NONE,
            style: Vec::new(), 
        }
    }

    pub(crate) fn header(&self) -> &Option<Text> {
        &self.header
    }

    pub(crate) fn footer(&self) -> &Option<Text> {
        &self.footer
    }

    pub(crate) fn header_mut(&mut self) -> &mut Option<Text> {
        &mut self.header
    }

    pub(crate) fn footer_mut(&mut self) -> &mut Option<Text> {
        &mut self.footer
    }

    pub(crate) fn data(&self) -> &str {
        &self.data
    }

    pub(crate) fn style(&self) -> &[&'static str] {
        &self.style
    }
}

pub struct DocumentBuilder {
    document: Document,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
        } 
    }

    pub fn header(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.document.header = Some(Text::new(label.to_string(), flags)?);
        Ok(self)
    }

    pub fn footer(
        mut self, label: impl ToString, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<Self> {
        self.document.footer = Some(Text::new(label.to_string(), flags)?);
        Ok(self)
    }

    pub fn flags(mut self, flags: TextFlags) -> FtuiResult<Self> {
        flags.ensure_compatibility()?;
        self.document.flags = flags;
        self.document.style = flags.resolve_ansi();
        Ok(self)
    }

    pub fn data(mut self, data: impl ToString) -> Self {
        self.document.data = data.to_string(); 
        self
    }

    pub fn build(self) -> Document {
        self.document
    }
}
