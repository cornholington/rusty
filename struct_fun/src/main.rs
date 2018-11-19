#[derive(Debug, Default)]
pub struct Foo {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Bar {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

fn main() {
    let foo = Foo { x: 1, y: 2 };
    println!("foo = {:?}", foo);
    // doesn't work, <sigh>
    //    let bar = Bar { z: 3, ..foo };
    //    println!("bar = {:?}", bar);
    let foo1 = Foo {
        x: 1,
        ..Foo::default()
    };
    println!("foo1 = {:?}", foo1);
}
