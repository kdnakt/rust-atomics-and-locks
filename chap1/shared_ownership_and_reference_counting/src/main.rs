use std::thread;
use std::rc::Rc;
use std::sync::Arc;
fn main() {
    static X: &'static [i32; 3] = &[1, 2, 3];

    // dbg! macro will print expression and result to stderr
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));

    // never drop and deallocate it
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));

    // reference counting
    let a = Rc::new([1, 2, 3]);
    let b = a.clone(); // increment a counter stored next to the contained value

    assert_eq!(a.as_ptr(), b.as_ptr());
    // same allocation, they share ownership!

    // thread::spawn(move || dbg!(b));
    // compile error: Rc cannot be sent between threads safely

    // use std::sync::Arc, atomically reference counted, instead
    let a = Arc::new([1, 2, 3]); // count starts at 1
    let b = a.clone(); // increment to 2 but the same allocation

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
    // last thread to drop Arc will drop and deallocate the array
}
