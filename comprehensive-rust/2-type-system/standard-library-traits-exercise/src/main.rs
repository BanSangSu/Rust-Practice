// Agenda https://google.github.io/comprehensive-rust/std-traits.html



// Comparisons https://google.github.io/comprehensive-rust/std-traits/comparisons.html
fn comparisions() {
    struct Key {
        id:u32,
        metadata: Option<String>,
    }
    // impl PartialEq for Key {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.id == other.id
    //     }
    // } 
    // Same above
    impl PartialEq <u32> for Key {
        fn eq(&self, other: &u32) -> bool {
            self.id == *other
        }
    }

    use std::cmp::Ordering;
    #[derive(Eq, PartialEq)]
    struct Citation {
        author: String,
        year: u32,
    }
    impl PartialOrd for Citation {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.author.partial_cmp(&other.author) {
                Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
                author_ord => author_ord,
            }
        }
    }

    let key1 = Key{ id:10, metadata: Some(String::from("B"))};
    let key2 = Key{ id:10, metadata: Some(String::from("B"))};
    let citation1 = Citation{ author: String::from("good"), year: 12};
    let citation2 = Citation{ author: String::from("good"), year: 12};
    println!("{}", key1.eq(&key2.id));
    println!("{:?}", citation1.partial_cmp(&citation2));
}



// Operators https://google.github.io/comprehensive-rust/std-traits/operators.html
fn operators() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl std::ops::Add for Point {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self { x: self.x + other.x, y: self.y + other.y }
        }
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{p1:?} +{p2:?} = {:?}", p1 + p2);
}


// From and Into https://google.github.io/comprehensive-rust/std-traits/from-and-into.html#from-and-into
fn from_and_into() {
    // from
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s}, {addr}, {one}, {bigger}");

    // into
    let s1: String = "hello".into();
    let addr1: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one1: i16 = true.into();
    let bigger1: i32 = 123_i16.into();
    println!("{s1}, {addr1}, {one1}, {bigger1}");
}


// Casting https://google.github.io/comprehensive-rust/std-traits/casting.html#casting
fn casting() {
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}



// Read and Write https://google.github.io/comprehensive-rust/std-traits/read-and-write.html
use std::io::{BufRead, BufReader, Read, Result, Write};

// Read & BufRead
fn read() -> Result<()> {

    fn count_lines<R: Read>(reader: R) -> usize {
        let buf_reader = BufReader::new(reader);
        buf_reader.lines().count()
    }

    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("lines in file: {}", count_lines(file));
    Ok(())
}

// Write
fn write() -> Result<()> {
    fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
        writer.write_all(msg.as_bytes())?;
        writer.write_all("\n".as_bytes())
    }

    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {buffer:?}");
    Ok(())
}


// The Default Trait https://google.github.io/comprehensive-rust/std-traits/default.html
fn default_trait() {
    #[derive(Debug, Default)]
    struct Derived {
        x: u32,
        y: String,
        z: Implemented,
    }

    #[derive(Debug)]
    struct Implemented(String);

    impl Default for Implemented {
        fn default() -> Self {
            Self("John Smith".into())
        }
    }

    let default_struct = Derived::default();
    println!("{default_struct:#?}");

    let almost_default_struct = 
        Derived { y: "Y is set!".into(), ..Derived::default() };
    println!("{almost_default_struct:#?}");
    
    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}


// Closures https://google.github.io/comprehensive-rust/std-traits/closures.html
fn closures() {
    fn apply_and_log(func: impl FnOnce(i32) -> i32, func_name: &str, input: i32) {
        println!("Calling {func_name}({input}): {}", func(input))
    }

    let n = 3;
    let add_3 = |x| x + n;
    apply_and_log(&add_3, "add_3", 10);
    apply_and_log(&add_3, "add_3", 20);

    let mut v = Vec::new();
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    apply_and_log(&mut accumulate, "accumulate", 4);
    apply_and_log(&mut accumulate, "accumulate", 5);

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    apply_and_log(multiply_sum, "multiply_sum", 3);


    fn make_greeter(prefix: String) -> impl Fn(&str) {
        return move |name| println!("{} {}", prefix, name);
    }

    let hi = make_greeter("Hi".to_string());
    hi("Greg");
}




// Exercise: ROT13 https://google.github.io/comprehensive-rust/std-traits/exercise.html#exercise-rot13

// use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

impl <R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let size = self.input.read(buf)?;
        for b in &mut buf[..size] {
            if b.is_ascii_alphabetic() {
                let base = if b.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
                *b = (*b - base + self.rot) % 26 + base;
            }
        }
        Ok(size)
    }
}

fn exercise_rot13() {

    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
        
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}


fn main() {
    exercise_rot13();

    // comparisions();
    // operators();
    // from_and_into();
    // casting();
    // read();
    // write();
    // default_trait();
    // closures();
}
