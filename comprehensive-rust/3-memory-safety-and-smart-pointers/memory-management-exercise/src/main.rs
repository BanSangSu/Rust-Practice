
// Review of Program Memory https://google.github.io/comprehensive-rust/memory-management/review.html
fn review_programme_memory() {
    let s1 = String::from("Helllo");
    println!("{}", s1);

    let mut s2 = String::from("Goten Tag");
    s2.push(' ');
    s2.push_str("world");

    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s2);
        println!("capacity = {capacity}, ptr = {ptr:#x}, len = {len}");
    }
}

// Approaches to Memory Management https://google.github.io/comprehensive-rust/memory-management/approaches.html
// There is no code.
// Rust's onwership, borrowing, and tools (Rc, Box, etc) model can handle pointer like C alloc and C++'s smart pointers.


// Ownership https://google.github.io/comprehensive-rust/memory-management/ownership.html#ownership
struct Point(i32, i32);
fn ownership() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1); // Cause error not found p.1 in this scope
}



fn main() { 
    review_programme_memory();
    ownership();
}
