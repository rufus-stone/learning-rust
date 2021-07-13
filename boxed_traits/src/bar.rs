/// This is the basic trait
pub trait Bar {
    fn value(&self) -> u8;
}

/// Why is this necessary to make structs that impl Bar work with Box::new() ???
impl<'a, B: Bar + ?Sized> Bar for &'a mut B {
    fn value(&self) -> u8 {
        (**self).value()
    }
}
