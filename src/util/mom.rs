#[derive(Debug, PartialEq, Eq)]
pub enum Mom<'a, T> {
    Owned(T),
    Ref(&'a mut T),
}

impl<'a, T> Mom<'a, T> {
    pub fn as_mut(&mut self) -> &mut T {
        match self {
            Mom::Owned(val) => val,
            Mom::Ref(r) => *r,
        }
    }
}
