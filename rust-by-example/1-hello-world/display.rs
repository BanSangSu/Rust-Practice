use std:fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0);
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1);
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y);
    }
}

fn main(){


    /*
        Activity

        After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:

        Expected print:
            Display: 3.3 + 7.2i
            Debug: Complex { real: 3.3, imag: 7.2 }

     */
}