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

// Dereferencing Raw Pointers https://google.github.io/comprehensive-rust/unsafe-rust/dereferencing.html
fn dereferencing_raw_pointers() {
    let mut s = String::from("careful!");

    // let r1 = &raw mut s; // experimental feature
    let r1 = &mut s as *mut String;
    let r2 = r1 as *const String;

    // SAFETY: r1 and r2 were obtained from references and so are guaranteed to
    // be non-null and properly aligned, the objects underlying the references
    // from which they were obtained are live throughout the whole unsafe
    // block, and they are not accessed either through the references or
    // concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = String::from("uhoh");
        println!("r2 is: {}", *r2);
    }

    // NOT SAFE. DO NOT DO THIS.
    /*
    let r3: &String = unsafe { &*r1 };
    drop(s);
    println!("r3 is: {}", *r3);
    */
}


// Mutable Static Variables https://google.github.io/comprehensive-rust/unsafe-rust/mutable-static.html
fn mutable_static_variables() {
    // static HELLO_WORLD: &str = "Hello, world!";
    
    // println!("HELLO_WORLD: {HELLO_WORLD}");

    static mut COUNTER: u32 = 0;

    fn add_to_counter(inc: u32) {
        // SAFETY: There are no other threads which could be accessing `COUNTER`.
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_counter(42);

    // SAFETY: There are no other threads which could be accessing `COUNTER`.
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}


fn main() {
    mutable_static_variables();
    // dereferencing_raw_pointers(); 
    // unsafe_rust();
}
