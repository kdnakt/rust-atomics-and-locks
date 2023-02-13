use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];
    // let mut numbers = vec![1, 2, 3];
    // the above line results in compile error: cannot borrow `numbers` as mutable
    thread::scope(|s| {
        s.spawn(|| {
            // numbers.push(1);
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        }); // the scope ends, threads automatically joined
    });
}
