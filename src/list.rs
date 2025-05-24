use crate::{
    components::{Header, Text, TextFlags}, error::FtuiResult,
    util::id::IdGenerator
};

pub struct List {
    header: Header,
    elements: Vec<Text>,
    offset: usize,
    id_generator: IdGenerator<u16>, 
}

impl List {
    pub fn new(header: &str) -> FtuiResult<Self> {
        Ok(List {
            header: Header::new(header)?,
            elements: vec![],
            offset: 0,
            id_generator: IdGenerator::new(),
        })
    }

    pub fn add(
        &mut self, label: &str, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<u16> {
        let mut text = Text::new(label, flags)?;
        let id = self.id_generator.get_id();

        text.set_id(id);
        self.elements.push(text);

        Ok(id)
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
