// Unsafe Rust https://google.github.io/comprehensive-rust/unsafe-rust/unsafe.html
fn unsafe_rust() {
    let mut num = 5;

    // let r1 = &raw const num;
    // let r2 = &raw mut num;
    let r1 = &num as *const i32; // Create a raw const pointer
    let r2 = &mut num as *mut i32; // Create a raw mutable pointer

    println!("r1={:p}, r2={:p}", r1, r2);
    unsafe {
        println!("Value through r1={}, r2={}", *r1, *r2);
    }
}



fn main() {
    unsafe_rust();
}
