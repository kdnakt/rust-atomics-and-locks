use std::thread;
fn main() {
    static X: &'static [i32; 3] = &[1, 2, 3];

    // dbg! macro will print expression and result to stderr
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));
}
