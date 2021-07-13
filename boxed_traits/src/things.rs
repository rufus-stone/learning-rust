use crate::bar::Bar;

pub struct Barstool {}

impl Bar for Barstool {
    fn value(&self) -> u8 {
        6
    }
}

pub struct Bartab {}

impl Bar for Bartab {
    fn value(&self) -> u8 {
        4
    }
}
