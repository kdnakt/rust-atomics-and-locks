use std::cell::Cell;
use std::cell::RefCell;
fn main() {
    println!("Hello, world!");

    // Cell and RefCell can only be used within a single thread
    let v = RefCell::new(vec![0, 1, 2]);
    f(&v);
    println!("{:?}", v.borrow());

    let v = Cell::new(vec![1, 2, 3]);
    f_push(&v);
    println!("{:?}", v.take());

    let a = Cell::new(1);
    f_cell(&a, &a);
}

fn f_cell(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        panic!("might happen");
    }
    println!("not panic");
}

fn f_push(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(1);
}
