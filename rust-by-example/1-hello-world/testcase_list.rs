use::std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            // write!(f, "{}", v)?;
            /*
                Activity

                1. Try changing the program so that the index of each element in the vector is also printed. The new output should look like this:

                    [0: 1, 1: 2, 2: 3]
             */
            // 1. 
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
}