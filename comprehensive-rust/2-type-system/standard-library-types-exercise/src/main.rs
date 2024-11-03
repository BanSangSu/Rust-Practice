


/// Documentation https://google.github.io/comprehensive-rust/std-types/docs.html#documentation

fn documentation() {
    /*!
     * ref. https://doc.rust-lang.org/reference/comments.html
     */

    //! This module contains functionality relating to divisibility of integers.
    //! /*! */ and //! are used in inner line (funcfion or something.)
    

    /// Determine whether the first argument is divisible by the second argument.
    ///
    /// If the second argument is zero, the result is false.
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }
    println!("{}", is_divisible_by(1,55));
}


// Option https://google.github.io/comprehensive-rust/std-types/option.html
fn option() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut position: Option<usize> = name.find('é');
    println!("find returned {position:?}");
    assert_eq!(position.unwrap(), 14); 
    position = name.find('Z');
    println!("find returned {position:?}");
    assert_eq!(position.expect("Character not found"), 0);
}


// Result https://google.github.io/comprehensive-rust/std-types/result.html
use std::fs::File;
use std::io::Read;
fn result() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}

fn main() {
    // documentation();
    // option();
    result();
}
