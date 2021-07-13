use crate::bar::Bar;

pub struct Foo<'a> {
    bar: Box<dyn Bar + 'a>,
}

impl<'a> Foo<'a> {
    pub fn new(bar: &'a mut dyn Bar) -> Self {
        Self { bar: Box::new(bar) }
    }

    pub fn use_bar(&self) -> u8 {
        self.bar.value()
    }
}
