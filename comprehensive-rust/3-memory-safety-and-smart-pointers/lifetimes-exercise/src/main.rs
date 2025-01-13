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

fn lifetimes_in_function_calls() {
    #[derive(Debug)]
    struct Point(i32, i32);
    
    fn cab_distance(p1: &Point, p2: &Point) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }

    fn find_nearest<'a>(points: &'a [Point], query: &Point) -> Option<&'a Point> {
    // fn find_nearest<'a, 'q>(points: &'a [Point], query: &'q Point) -> Option<&'q Point> { // This is a common source of errors with unsafe Rust. This won’t compile, demonstrating that the annotations are checked for validity by the compiler.
        let mut nearest = None;
        for p in points {
            if let Some((_, nearest_dist)) = nearest {
                let dist = cab_distance(p, query);
                if dist < nearest_dist {
                    nearest = Some((p, dist));
                }
            } else {
                nearest = Some((p, cab_distance(p, query)));
            };
        }
        nearest.map(|(p, _)| p)
    }

    let points = &[Point(1, 0), Point(1, 0), Point(-1, 0), Point(0, -1),];
    let nearest = {
        let query = Point(0, 2);
        find_nearest(points, &Point(0, 2))
    };
    println!("{:?}", nearest);
}

// Lifetimes in Data Structures https://google.github.io/comprehensive-rust/lifetimes/struct-lifetimes.html

fn lifetime_in_data_structures() {
    #[derive(Debug)]
    enum HighlightColor {
        Pink,
        Yellow,
    }

    #[derive(Debug)]
    struct Highlight<'document> {
        slice: &'document str,
        color: HighlightColor,
    }

    
}


fn main() {
    lifetime_in_data_structures();
    lifetimes_in_function_calls();
    lifetime_annotations();
}
