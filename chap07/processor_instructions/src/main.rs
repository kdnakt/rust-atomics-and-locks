pub fn add_ten(num: &mut i32) {
    *num += 10;
}

fn main() {
    println!("Hello, world!");

    let mut x = 1;
    add_ten(&mut x);
    println!("{x}");
}
