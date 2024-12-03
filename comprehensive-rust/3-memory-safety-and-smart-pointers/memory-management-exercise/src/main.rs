
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

// Move Semantics https://google.github.io/comprehensive-rust/memory-management/move.html
fn move_semantics() {
    let s1: String = String::from("Cool!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}"); // Error: value borrowed here after move

    fn say_yo(name: String) {
        println!("Yo, {name}!");
    }

    let name = String::from("GiGi");
    say_yo(name);
    // say_yo(name);
}


// Clone https://google.github.io/comprehensive-rust/memory-management/clone.html
fn clone() {
    fn call_me(name: String) {
        println!("Call me {name}!");
    } 

    let name = String::from("BoBo");
    call_me(name.clone());
    call_me(name);
}


fn main() { 
    review_programme_memory();
    ownership();
    move_semantics();
    clone();
}

