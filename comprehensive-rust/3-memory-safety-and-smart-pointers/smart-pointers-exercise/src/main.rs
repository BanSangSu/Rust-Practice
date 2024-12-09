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


// Owned Trait Objects https://google.github.io/comprehensive-rust/smart-pointers/trait-objects.html
fn owned_trait_objects() {
    struct Dog {
        name: String,
        age: i8,
    }
    struct Cat {
        lives: i8,
    }

    trait Pet {
        fn talk(&self) -> String;
    }

    impl Pet for Dog {
        fn talk(&self) -> String {
            format!("Woof, my name is {}!", self.name)
        }
    }

    impl Pet for Cat {
        fn talk(&self) -> String {
            String::from("Miau!")
        }
    }

    let pets: Vec<Box<dyn Pet>> = vec! [
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];

    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }

    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());
}



fn main() {
    box_type();
    reference_counted();
    owned_trait_objects();
}
