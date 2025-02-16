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
    result();
    // panics();
}
