use crate::{
    components::{Header, Text, TextFlags}, error::{FtuiError, FtuiResult},
    util::id::IdGenerator,
};

pub struct List {
    header: Option<Header>,
    elements: Vec<Text>,
    offset: usize,
    default_flags: Option<TextFlags>,
    number: bool,
    id_generator: IdGenerator<u16>,
}

impl List {
    pub(crate) fn new() -> Self {
        List {
            header: None,
            elements: vec![],
            offset: 0,
            default_flags: None,
            number: false,
            id_generator: IdGenerator::new(),
        }
    }

    pub fn add(
        &mut self, label: &str, flags: impl Into<Option<TextFlags>>
    ) -> FtuiResult<u16> {
        let flags: Option<TextFlags> = flags.into();

        let id = self.id_generator.get_id(); 

        match flags {
            Some(flags) if flags.contains(TextFlags::ALIGN_BOTTOM) =>
                return Err(FtuiError::TextFlagAlignBottomWithListElement),
            Some(flags) => self.elements.push(Text::with_id(label, flags, id)?),
            None => self.elements.push(Text::with_id(label, self.default_flags, id)?),
        }

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

    pub(crate) fn header(&self) -> &Option<Header> {
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
    
    pub(crate) fn is_number(&self) -> bool {
        self.number
    }
}

pub struct ListBuilder {
    list: List,
}

impl ListBuilder {
    pub fn new() -> Self {
        ListBuilder { list: List::new(), }
    }

    pub fn header_expl(mut self, header: Header) -> Self {
        self.list.header = Some(header);
        self
    }

    #[inline]
    pub fn header(self, label: &str) -> FtuiResult<Self> {
        Ok(self.header_expl(Header::new(label)?))
    }

    pub fn default_flags(mut self, flags: TextFlags) -> FtuiResult<Self> {
        Text::ensure_compatible_flags(&flags)?;
        self.list.default_flags = Some(flags);
        Ok(self)
    }

    pub fn number(mut self) -> Self {
        self.list.number = true;
        self
    }

    pub fn build(self) -> List {
        self.list
    }
}
