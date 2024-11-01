// Generic Functions
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n & 1 == 0 {
        even
        // even + obb // cause type constraints? error
    } else{
        odd
    }
}

fn generic_functions() {
    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a string: {:?}", pick(28, "dog", "cat"));
}


// Generic Data Types
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn coords(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }

    fn set_x(&mut self, x:T) {
        self.x = x;
    }
}

fn generic_data_types() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("integer coords: {:?}, float coords: {:?}", integer.coords(), float.coords());

    // Try declaring a new variable let p = Point { x: 5, y: 10.0 };. Update the code to allow points that have elements of different types, by using two type variables, e.g., T and U.
    let p = Point { x: 5, y: 10.0 };
    println!("{p:?}");
    println!("coords: {:?}", p.coords());
}


// Generic Traits
#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}
impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

fn generic_traits() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    println!("{from_int:?}, {from_bool:?}");
}


// Trait Bounds
fn trait_bounds() {
    fn duplicate<T: Clone>(a: T) -> (T, T) {
        (a.clone(), a.clone())
    }

    fn copy<T>(a: T) -> (T, T)
    where
        T: Clone,
    {
        (a.clone(), a.clone())
    }

    #[derive(Debug)] // struct below is non clonable and non debugable.
    #[derive(Clone)] // Therefore we need to add derive attribute
    struct NonClonable;

    let foo = String::from("foo");
    let foo2 = String::from("foo"); // my opinion. we initiate foo2 as we can use one value to two or more function because of owenership
    let pair1 = duplicate(foo);
    let pair2 = copy(foo2);
    println!("{pair1:?}, {pair2:?}");

    let non_clonable = NonClonable;
    let non_clonable2 = NonClonable; // my opinion. we initiate foo2 as we can use one value to two or more function because of owenership
    let pass_non1 = duplicate(non_clonable);
    let pass_non2 = copy(non_clonable2);
    println!("{pass_non1:?}, {pass_non2:?}");
}



// impl Trait https://google.github.io/comprehensive-rust/generics/impl-trait.html
fn impl_trait() {
    // fn add_56_millions(x: impl Into<i32>) -> i32 {
    fn add_56_millions<T: Into<i32>>(x: T) -> i32 { // Syntactic sugar:
        x.into() + 56_000_000
    }

    fn pair_of(x: u32) -> impl std::fmt::Debug {
        (x + 1, x - 1)
    }
    
    let many = add_56_millions(56_i8);
    println!("{many}");
    let many_more = add_56_millions(10_000_000);
    println!("{many_more}");
    // let debuggable: () = pair_of(53); // to see the expected unit type
    // let debuggable: &dyn std::fmt::Debug = &pair_of(53); // found it... but it makes the code complicated...
    let debuggable = pair_of(53);
    println!("debuggable: {debuggable:?}");
}



// dyn Trait https://google.github.io/comprehensive-rust/generics/dyn-trait.html
fn dyn_trait() {
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
            String::from("Miau!!!")
        }
    }

    fn generic(pet: &impl Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }

    fn dynamic(pet: &dyn Pet) {
        println!("Hello, who are you? {}", pet.talk());
    }


    let cat = Cat { lives: 12 };
    let dog = Dog { name: String::from("Fido"), age:5 };

    generic(&cat);
    generic(&dog);
    
    dynamic(&cat);
    dynamic(&dog);
}



// Exercise: Generic min https://google.github.io/comprehensive-rust/generics/exercise.html
use std::cmp::Ordering;

// The code that I did firstly
// fn min<T: Ord>(a:T , b:T) -> T {
//     a.min(b)
// }
// another code
fn min<T: Ord> (l:T, r:T) -> T {
    match l.cmp(&r) {
        Ordering::Less | Ordering::Equal => l, 
        Ordering::Greater => r,
    }
}

fn exercise_generic_min() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}

fn main() {
    exercise_generic_min();

    // generic_functions();
    // generic_data_types();
    // generic_traits();
    // trait_bounds();
    // impl_trait();
    // dyn_trait();
}
