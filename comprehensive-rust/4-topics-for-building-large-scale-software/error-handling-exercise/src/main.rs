// Panics https://google.github.io/comprehensive-rust/error-handling/panics.html
use std::panic;

fn panics() {
    // let v = vec![10, 20, 30];
    // dbg!(v[100]); // index out of bounds error

    let result = panic::catch_unwind(|| "No problem here!");
    dbg!(result);

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    dbg!(result);
    
}

// it doesn't work, but I don't know why...
// in Cargo.toml
// [profile.dev]
// panic = 'abort'


// Result https://google.github.io/comprehensive-rust/error-handling/result.html
// use std::fs::File;
// use std::io::Read;

// fn result() {
//     let file: Result<File, std::io::Error> = File::open("diary.txt");
//     match file {
//         Ok(mut file) => {
//             let mut contents = String::new();
//             if let Ok(bytes) = file.read_to_string(&mut contents) {
//                 println!("Dear diary: {contents} ({bytes} bytes)");
//             } else {
//                 println!("Could not read file content");
//             }
//         }
//         Err(err) => {
//             println!("The diary could not be opened: {err}");
//         }
//     }
// }

// Try Operator https://google.github.io/comprehensive-rust/error-handling/try.html
// use std::io::Read;
// use std::{fs, io};

// fn try_operator() {
//     fn read_username(path: &str) -> Result<String, io::Error> {
//         // let username_file_result = fs::File::open(path);
//         // let mut username_file = match username_file_result {
//         //     Ok(file) => file,
//         //     Err(err) => return Err(err),
//         // };

//         // let mut username = String::new();
//         // match username_file.read_to_string(&mut username) {
//         //     Ok(_) => Ok(username),
//         //     Err(err) => Err(err),
//         // }

//         // Simplified version
//         let mut username = String::new();
//         fs::File::open(path)?.read_to_string(&mut username)?;
//         Ok(username)
        
//     }

//     //  fs::write("config.dat", "alice").unwrap();
//      let username = read_username("config.dat");
//      println!("username or error: {username:?}");

// }


// Try Conversions https://google.github.io/comprehensive-rust/error-handling/try-conversions.html
// use std::error::Error;
// use std::io::Read;
// use std::{fmt, fs, io};

// fn try_conversion() {
//     #[derive(Debug)]
//     enum ReadUsernameError {
//         IoError(io::Error),
//         EmptyUsername(String),
//     }
    
//     impl Error for ReadUsernameError {}
    
//     impl fmt::Display for ReadUsernameError {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             match self {
//                 Self::IoError(e) => write!(f, "I/O error: {e}"),
//                 Self::EmptyUsername(path) => write!(f, "Found no username in {path}"),
//             }
//         }
//     }

//     impl From<io::Error> for ReadUsernameError {
//         fn from(err: io::Error) -> Self {
//             Self::IoError(err)
//         }
//     }

//     fn read_username(path: &str) -> Result<String, ReadUsernameError> {
//         let mut username = String::with_capacity(100);
//         fs::File::open(path)?.read_to_string(&mut username)?;
//         if username.is_empty() {
//             return Err(ReadUsernameError::EmptyUsername(String::from(path)));
//         }
//         Ok(username)
//     }

//     //std::fs::write("config.dat", "").unwrap();
//     let username = read_username("config.dat");
//     println!("username or error: {username:?}");
// }


// Dynamic Error Types https://google.github.io/comprehensive-rust/error-handling/error.html
use std::error::Error;
use std::fs;
use std::io::Read;

fn dynamic_error_types() {
    fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
        let mut count_str = String::new();
        fs::File::open(path)?.read_to_string(&mut count_str)?;
        let count:i32 = count_str.parse()?;
        Ok(count)
    }

    fs::write("count.dat", "1i3").unwrap();
    match read_count("count.dat") {
        Ok(count) => println!("Count: {count}"),
        Err(err) => println!("Error: {err}"),
    }
}

fn main() {
    dynamic_error_types();
    // try_conversion();
    // try_operator();
    // result();
    // panics();
}
