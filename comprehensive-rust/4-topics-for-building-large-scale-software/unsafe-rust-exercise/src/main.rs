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


// Unions https://google.github.io/comprehensive-rust/unsafe-rust/unions.html
fn unions() {
    #[repr(C)]
    union MyUnion {
        i: u8,
        b: bool,
    }

    let u = MyUnion { i: 42};
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b }); // Undefined behavior!
}


// Unsafe Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions.html
// no code

// Unsafe Rust Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions/rust.html

// #[deny(unsafe_op_in_unsafe_fn)]
// #[warn(unsafe_op_in_unsafe_fn)] // only warning
fn unsafe_rust_functions() {
    /// Swaps the values pointed to by the given pointers.
    ///
    /// # Safety
    ///
    /// The pointers must be valid, properly aligned, and not otherwise accessed for
    /// the duration of the function call.
    unsafe fn swap(a: *mut u8, b: *mut u8) {
        // unsafe { // do it when we use #[deny(unsafe_op_in_unsafe_fn)].
            let temp = *a;
            *a = *b;
            *b = temp;
        // }
    }

    let mut a = 42;
    let mut b = 66;

    // SAFETY: The pointers must be valid, aligned and unique because they came
    // from references.
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}

// Unsafe External Functions https://google.github.io/comprehensive-rust/unsafe-rust/unsafe-functions/extern-c.html
use std::ffi::c_char;

fn unsafe_external_functions () {
    unsafe extern "C" {
        // `abs` doesn't deal with pointers and doesn't have any safety requirements.
        safe fn abs(input: i32) -> i32;
    
        /// # Safety
        ///
        /// `s` must be a pointer to a NUL-terminated C string which is valid and
        /// not modified for the duration of this function call.
        unsafe fn strlen(s: *const c_char) -> usize;
    }

    println!("Absolute value of -3 according to C: {}", abs(-3));

    unsafe {
        // SAFETY: We pass a pointer to a C string literal which is valid for
        // the duration of the program.
        println!("String length: {}", strlen(c"String".as_ptr()));
    }
}



fn main() {
    unsafe_external_functions();

    // unsafe_rust_functions();
    // unions();
    // mutable_static_variables();
    // dereferencing_raw_pointers(); 
    // unsafe_rust();
}
