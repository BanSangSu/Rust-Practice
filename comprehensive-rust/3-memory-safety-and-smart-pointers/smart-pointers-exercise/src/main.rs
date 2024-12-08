// Box<T> https://google.github.io/comprehensive-rust/smart-pointers/box.html
# [derive(Debug)]
enum List<T> {
    Element(T, Box<List<T>>),
    Nil,
}
fn box_type() {
    let five = Box::new(5);
    println!("five: {}", *five);

    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}


// Rc https://google.github.io/comprehensive-rust/smart-pointers/rc.html
use std::rc::Rc;


fn reference_counted() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    
    println!("a: {a}");
    println!("b: {b}");
    println!("Strong count of a: {}", Rc::strong_count(&a));
    println!("Strong count of b: {}", Rc::strong_count(&b));
}


fn main() {
    box_type();
    reference_counted();
}
