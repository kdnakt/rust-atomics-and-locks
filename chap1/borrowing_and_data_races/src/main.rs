fn main() {
    let x = 1;
    let mut y = 2;
    f(&x, &mut y);
}

fn f(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    // compiler can conclude this will never happen, so remove the expressions after that.
    if before != after {
        panic!("never happens");
    }
    println!("end of f()");
}
