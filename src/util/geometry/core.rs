pub type Coordinate = i32;

pub(crate) trait Rect {
    fn w(&self) -> u16;
    fn h(&self) -> u16;
}

pub(crate) trait Positional {
    fn x(&self) -> Coordinate;
    fn y(&self) -> Coordinate;
}

pub(crate) trait Circular {
    fn r(&self) -> u16;
}

pub(crate) trait Segment {
    fn start(&self) -> (Coordinate, Coordinate);
    fn end(&self) -> (Coordinate, Coordinate);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddProperties {
    Rotate(i32),
    Fill,
}

impl AddProperties {
    pub(crate) fn slot(&self) -> AddPropertySlot {
        match self {
            Self::Rotate(_) => AddPropertySlot::Rotate,
            Self::Fill => AddPropertySlot::Fill,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AddPropertySlot {
    Rotate = 0,
    Fill = 1,
}

impl AddPropertySlot {
    pub(crate) const COUNT: usize = 2;
}

type AddPropertiesArray = [Option<AddProperties>; AddPropertySlot::COUNT];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AddPropertiesManager {
    properties: AddPropertiesArray,
}

impl AddPropertiesManager {
    pub(crate) fn new() -> Self {
        Self {
            properties: std::array::from_fn(|_| None),
        }
    }

    #[inline]
    pub(crate) fn apply_iter<I>(&mut self, properties: I)
    where 
        I: IntoIterator<Item = AddProperties>
    {
        properties.into_iter().for_each(|prop| self.apply(prop));
    }

    #[inline]
    pub(crate) fn apply(&mut self, property: AddProperties) {
        let slot = property.slot();
        self.properties[slot as usize] = Some(property);
    }

    #[inline]
    pub(crate) fn get(&self, slot: AddPropertySlot) -> &Option<AddProperties> {
        &self.properties[slot as usize]
    }

    #[inline]
    pub(crate) fn is_exist(&self, slot: AddPropertySlot) -> bool {
        self.properties[slot as usize].is_some()
    }
}

pub(crate) trait HasProperties {
    fn props(&self) -> &AddPropertiesManager; 
}
