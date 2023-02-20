use std::marker::PhantomData;
use std::cell::Cell;
use std::thread;
use std::rc::Rc;

#[derive(Debug)]
struct X { // not sync
    handle: i32, // sync
    _not_sync: PhantomData<Cell<()>>, // not sync
}

#[derive(Debug)]
struct Y {
    // raw pointers are neither Send nor Sync
    p: *mut i32,
}

// opt in both traits with unsafe impl
unsafe impl Send for Y {}
unsafe impl Sync for Y {}

fn main() {
    println!("Hello, world!");
    let x = X {
        handle: 1,
        _not_sync: PhantomData
    };
    println!("{:?}", x);

    let y = Y {
        p: &mut 1,
    };
    println!("{:?}", y);

    // Compiler will stop if you try to move something not `Send` into another thread
    let a = Rc::new(123);
    // thread::spawn(move || {
    //     dbg!(a);
    // });
    // -> error[E0277]: `Rc<i32>` cannot be sent between threads safely
}
