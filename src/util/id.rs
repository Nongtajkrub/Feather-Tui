pub type GeneratedId = u32;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IdGenerator {
    id: GeneratedId,
}

impl IdGenerator {
    pub(crate) fn new() -> Self {
        IdGenerator {
            id: 0,
        }
    }

    pub(crate) fn get_id(&mut self) -> GeneratedId {
        self.id += 1;
        self.id
    }
}
