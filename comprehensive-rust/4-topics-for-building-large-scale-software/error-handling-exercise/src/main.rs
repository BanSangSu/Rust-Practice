// Panics https://google.github.io/comprehensive-rust/error-handling/panics.html
use std::panic;

fn panics() {
    // let v = vec![10, 20, 30];
    // dbg!(v[100]); // index out of bounds error

    let result = panic::catch_unwind(|| "No problem here!");
    dbg!(result);

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    dbg!(result);
}

fn main() {
    panics();
}
