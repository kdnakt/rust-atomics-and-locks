use std::marker::PhantomData;
use std::cell::Cell;

#[derive(Debug)]
struct X { // not sync
    handle: i32, // sync
    _not_sync: PhantomData<Cell<()>>, // not sync
}

fn main() {
    println!("Hello, world!");
    let x = X {
        handle: 1,
        _not_sync: PhantomData
    };
    println!("{:?}", x);
}
