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


// Interior Mutability https://google.github.io/comprehensive-rust/borrowing/interior-mutability.html
use std::cell::{Cell, RefCell};

fn interior_mutability() {
    // Note that `cell` is NOT declared as mutable.
    let cell = Cell::new(5);

    cell.set(123);
    println!("{}", cell.get());

    let refcell = RefCell::new(5);

    {
        let mut cell_ref = refcell.borrow_mut();
        *cell_ref = 123;

        // This triggers an error at runtime.
        // let other = cell.borrow();
        // println!("{}", *other);
    }

    println!("{refcell:?}");
}


//////
// Exercise: Health Statistics https://google.github.io/comprehensive-rust/borrowing/exercise.html
// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]


// #![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height, visit_count: 0, last_blood_pressure: None }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        self.visit_count += 1;
        let bp = measurements.blood_pressure;
        let report = HealthReport{
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change: measurements.height - self.height,
            blood_pressure_change: match self.last_blood_pressure {
                Some(lbp) => {
                    Some((bp.0 as i32 - lbp.0 as i32, bp.1 as i32 - lbp.1 as i32))
                }
                None => None,
            },
        };
        self.height = measurements.height;
        self.last_blood_pressure = Some(bp);
        report
    }
}

fn health_statistics_exercise() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name, bob.age);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.visit_count, 0);
    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);
    assert!((report.height_change - 0.9).abs() < 0.00001);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    assert_eq!(report.height_change, 0.0);
}


fn main() {
    health_statistics_exercise();

    // borrowing_a_value();
    // borrowing_checking();
    // borrow_errors();
    // interior_mutability();
}
