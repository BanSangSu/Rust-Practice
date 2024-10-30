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



fn main() {
    generic_functions();
    generic_data_types();
    generic_traits();
}
