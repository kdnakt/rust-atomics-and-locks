use std::cell::Cell;
fn main() {
    println!("Hello, world!");
    
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
