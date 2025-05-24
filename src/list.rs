use crate::{
    components::{Header, Text, TextFlags}, error::{FtuiResult, FtuiError},
};

pub struct List {
    header: Header,
    elements: Vec<Text>,
    offset: usize,
}

impl List {
    pub fn new(header: &str) -> FtuiResult<Self> {
        Ok(List {
            header: Header::new(header)?,
            elements: vec![],
            offset: 0,
        })
    }

    pub fn add(
        &mut self, label: &str, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<()> {
        let flags: Option<TextFlags> = flags.into();

        if flags.is_some_and(|flags| flags.contains(TextFlags::ALIGN_BOTTOM)) {
            return Err(FtuiError::TextFlagAlignBottomWithListElement);
        }

        self.elements.push(Text::new(label, flags)?);
        Ok(())
    }

    pub fn scroll_down(&mut self) -> bool {
        if self.offset < self.elements.len() - 1 {
            self.offset += 1;
            true
        } else {
            false
        }
    }

    pub fn scroll_up(&mut self) -> bool {
        if self.offset != 0 {
            self.offset -= 1;
            true
        } else {
            false
        }
    }

    pub(crate) fn header(&self) -> &Header {
        &self.header
    }

    pub(crate) fn elements_mut(&mut self) -> &mut [Text] {
        &mut self.elements
    }

    pub(crate) fn offset(&self) -> usize {
        self.offset
    }

    pub(crate) fn len(&self) -> usize {
        self.elements.len()
    }
}
