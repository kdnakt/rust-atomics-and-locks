fn main() {
    let x = 1;
    let mut y = 2;
    f(&x, &mut y);

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf);
    let index = buf.trim().parse::<usize>().unwrap();

    match index {
        0 => println!("index is zero"),
        1 => println!("index is one"),
        _ => g(index),
    }

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

fn g(index: usize) {
    println!("g: start");
    if index == 3 {
        println!("g: index = 3");
    }
    println!("g: end");
}