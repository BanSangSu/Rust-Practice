//  Borrowing a Value https://google.github.io/comprehensive-rust/borrowing/shared.html
#[derive(Debug)]
struct Point(i32, i32);

fn borrowing_a_value() {
    fn add(p1: &Point, p2: &Point) -> Point {
        // Point(p1.0 + p2.0, p1.1 + p2.1)
        let p = Point(p1.0 + p2.0, p1.1 + p2.1);
        println!("&p.0: {:p}", &p.0);
        p
    }

    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);
    println!("{p1:?} + {p2:?} = {p3:?}");
}


// Borrow Checking https://google.github.io/comprehensive-rust/borrowing/borrowck.html
fn borrowing_checking() {
    // let x_ref = {
    //     let x = 10;
    //     &x // cause error: borrowed value does not live long enough
    // };
    // println!("x: {x_ref}");

    // let mut a: i32 = 10;
    // let b: &i32 = &a;
    // // {
    // //     let c: &mut i32 = &mut a; // cause error: mutable borrow occurs here
    // //     *c = 20;
    // // }
    // println!("a: {a}");
    // println!("b: {b}");

    let mut a: i32 = 10;
    let b: &i32 = &a;
    println!("b: {b}");
    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }
    println!("a: {a}");
}


// Borrow Errors https://google.github.io/comprehensive-rust/borrowing/examples.html
fn borrow_errors() {
    // let mut vec = vec![1, 2, 3, 4, 5];
    // let elem = &vec[2];
    // vec.push(6); // cause error: mutable borrow occurs here
    // println!("{elem}");

    // let mut vec2 = vec![1, 2, 3, 4, 5];
    // for element in &vec2 {
    //     vec2.push(element * 2); // cause error: mutable borrow occurs here
    // }
    
}

fn main() {
    borrowing_a_value();
    borrowing_checking();
    borrow_errors();
}
