use foo::Foo;
use things::*;

mod bar;
mod foo;
mod things;

fn main() {
    let mut barstool = Barstool {};
    let foostool = Foo::new(&mut barstool);

    println!("{}", foostool.use_bar());

    let mut bartab = Bartab {};
    let footab = Foo::new(&mut bartab);

    println!("{}", footab.use_bar());
}
