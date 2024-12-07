


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

fn main() {
    box_type();
}
