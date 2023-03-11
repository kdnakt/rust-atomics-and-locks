
fn f(a: &mut i32, b: &mut i32) {
    *a += 1;
    *b += 1;
    *a += 1;
}

fn main() {
    println!("Hello, world!");

    let mut a = 10;
    let mut b = 20;

    f(&mut a, &mut b);

    println!("a={a}, b={b}");
}
