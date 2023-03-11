
fn f(a: &mut i32, b: &mut i32) {
    *a += 1;
    *b += 1;
    *a += 1;

    // compiler might reorder above like this, for faster execution
    // *a += 2;
    // *b += 1;

    // And Processor might execute the second addition before the first
    // possibly because *a had to be fetched from the main memory
    // and *b was available in a cache.
}

fn main() {
    println!("Hello, world!");

    let mut a = 10;
    let mut b = 20;

    f(&mut a, &mut b);

    println!("a={a}, b={b}");
}
