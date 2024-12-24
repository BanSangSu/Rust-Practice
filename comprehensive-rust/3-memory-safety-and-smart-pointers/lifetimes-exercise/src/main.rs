// Lifetime Annotations https://google.github.io/comprehensive-rust/lifetimes/lifetime-annotations.html

fn lifetime_annotations() {
    #[derive(Debug)]
    struct Point(i32, i32);

    fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
        if p1.0 < p2.0 {
            p1
        } else {
            p2
        }
    }

    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3 = left_most(&p1, &p2); // What is the lifetime of p3?
    println!("p3: {p3:?}");
}


// Lifetimes in Function Calls https://google.github.io/comprehensive-rust/lifetimes/lifetime-elision.html




fn main() {
    lifetime_annotations();
}
