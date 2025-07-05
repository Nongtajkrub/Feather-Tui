use num_integer::Integer;
use std::ops::AddAssign;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct IdGenerator<T>
where 
    T: Integer + Copy + AddAssign + PartialEq + Eq,
{
    id: T,
}

impl<T> IdGenerator<T>
where 
    T: Integer + Copy + AddAssign + PartialEq + Eq,
{
    pub(crate) fn new() -> Self {
        IdGenerator {
            id: T::zero(),
        }
    }

    pub(crate) fn get_id(&mut self) -> T {
        self.id += T::one();
        self.id
    }
}
