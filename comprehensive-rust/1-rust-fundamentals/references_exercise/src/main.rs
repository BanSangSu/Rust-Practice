// Dangling refrences will be forbided.
// fn x_axis(x: &i32) -> &(i32, i32) {
//     let point = (*x, 0);
//     return &point;
// }


// A shared reference does not allow modifying the value it refers to, even if that value was mutable.
fn shared_references () {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}

fn exclusive_references() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 100;
    println!("point: {point:?}");
}

fn slices() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    a[2] = 100;

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}

fn strings() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2 {s2}");
    s2.push_str(s1);
    println!("s2 {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3 {s3}");

    println!("{:?}", b"abc");
    println!("{:?}", &[97, 98, 99]);

    println!(r#"<a href="link.html">link</a>"#);
    println!("<a href=\"link.html\">link</a>");
}

// exercise
fn exercise_geometry() {
    fn magnitude(v: &[f64; 3]) -> f64 {
        let mut mag: f64 = 0.0;
        for coord in v {
            mag += coord.powf(2.0);
        }
        mag.sqrt()
    }

    fn normalize(v: &mut [f64; 3]) {
        let mag = magnitude(v);
        for coord in v {
            *coord /= mag;
        }
    }

    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

fn main() {

    exercise_geometry();

    // shared_references();
    // exclusive_references();
    // slices();
    // strings();
}
