fn main() {
    let x = 1;
    let mut y = 2;
    f(&x, &mut y);

    let index = 3;
    let a = [123, 456, 789];
    let b = unsafe { a.get_unchecked(index) };
    println!("{:?} {b}", a); // [123, 456, 789] 1829694064

    // println!("{}", a[3]);
    // compile error: index out of bounds
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
