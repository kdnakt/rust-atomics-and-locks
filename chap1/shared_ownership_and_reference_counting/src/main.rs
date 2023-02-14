use std::thread;
fn main() {
    static X: &'static [i32; 3] = &[1, 2, 3];

    // dbg! macro will print expression and result to stderr
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));

    // never drop and deallocate it
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || dbg!(x));
    thread::spawn(move || dbg!(x));
}
