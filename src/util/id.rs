use num_integer::Integer;
use std::ops::AddAssign;

pub struct IdGenerator<T>
where 
    T: Integer + Copy + AddAssign + PartialEq + Eq,
{
    id: T,
}

impl<T> IdGenerator<T>
where 
    T: Integer + Copy + AddAssign + PartialEq + Eq,
{
    pub fn new() -> Self {
        IdGenerator {
            id: T::zero(),
        }
    }

    pub fn get_id(&mut self) -> T {
        self.id += T::one();
        self.id
    }
}
