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
use std::io::Read;
use std::{fs, io};

fn try_operator() {
    fn read_username(path: &str) -> Result<String, io::Error> {
        let username_file_result = fs::File::open(path);
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(err) => return Err(err),
        };

        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(err) => Err(err),
        }

    }

     fs::write("config.dat", "alice").unwrap();
     let username = read_username("config.dat");
     println!("username or error: {username:?}");

}



fn main() {
    try_operator();
    // result();
    // panics();
}
