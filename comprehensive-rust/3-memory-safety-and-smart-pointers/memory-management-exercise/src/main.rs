
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
// struct Point(i32, i32);
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


// Copy Types https://google.github.io/comprehensive-rust/memory-management/copy-types.html
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn copy_types() {
    let x = 31;
    let y = x;
    println!("x: {x}"); // would not be accessible if not Copy
    println!("y: {y}");

    let p1 = Point(2, 6);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}


// The Drop Trait https://google.github.io/comprehensive-rust/memory-management/drop.html
fn drop_trait() {
    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Dropping {}", self.name);
        }
    }

    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    // a.drop();
    drop(a);
    println!("Exiting main");
}


fn main() { 
    review_programme_memory();
    ownership();
    move_semantics();
    clone();
    copy_types();
    drop_trait();
}

