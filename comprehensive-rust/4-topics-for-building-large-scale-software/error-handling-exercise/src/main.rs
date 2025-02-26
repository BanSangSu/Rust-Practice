// Panics https://google.github.io/comprehensive-rust/error-handling/panics.html
// use std::panic;

// fn panics() {
//     // let v = vec![10, 20, 30];
//     // dbg!(v[100]); // index out of bounds error

//     let result = panic::catch_unwind(|| "No problem here!");
//     dbg!(result);

//     let result = panic::catch_unwind(|| {
//         panic!("oh no!");
//     });
//     dbg!(result);
    
// }

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
// use std::error::Error;
// use std::fs;
// use std::io::Read;

// fn dynamic_error_types() {
//     fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
//         let mut count_str = String::new();
//         fs::File::open(path)?.read_to_string(&mut count_str)?;
//         let count:i32 = count_str.parse()?;
//         Ok(count)
//     }

//     fs::write("count.dat", "1i3").unwrap();
//     match read_count("count.dat") {
//         Ok(count) => println!("Count: {count}"),
//         Err(err) => println!("Error: {err}"),
//     }
// }


// thiserror https://google.github.io/comprehensive-rust/error-handling/thiserror.html
// use std::io::Read;
// use std::{fs, io};
// use thiserror::Error;

// fn thiserror() {
//     #[derive(Debug, Error)]
//     enum ReadUsernameError {
//         #[error("I/O error: {0}")]
//         IoError(#[from] io::Error),
//         #[error("Found no username in {0}")]
//         EmptyUsername(String),
//     }

//     fn read_username(path: &str) -> Result<String, ReadUsernameError> {
//         let mut username = String::with_capacity(100);
//         fs::File::open(path)?.read_to_string(&mut username)?;
//         if username.is_empty() {
//             return Err(ReadUsernameError::EmptyUsername(String::from(path)));
//         }
//         Ok(username)
//     }

//     //fs::write("config.dat", "").unwrap();
//     match read_username("config.dat") {
//         Ok(username) => println!("Username: {username}"),
//         Err(err) => println!("Error: {err:?}"),
//     }
// }


// anyhow https://google.github.io/comprehensive-rust/error-handling/anyhow.html
// use anyhow::{bail, Context, Result};
// use std::fs;
// use std::io::Read;
// use thiserror::Error;

// #[derive(Clone, Debug, Eq, Error, PartialEq)]
// #[error("Found no username in {0}")]
// struct EmptyUsernameError(String);

// fn anyhow() {
//     fn read_username(path: &str) -> Result<String> {
//         let mut username = String::with_capacity(100);
//         fs::File::open(path)
//             .with_context(|| format!("Failed to open {path}"))?
//             .read_to_string(&mut username)
//             .context("Failed to read")?;
//         if username.is_empty() {
//             bail!(EmptyUsernameError(path.to_string()));
//         }
//         Ok(username)
//     }

//     //fs::write("config.dat", "").unwrap();
//     match read_username("config.dat") {
//         Ok(username) => println!("Username: {username}"),
//         Err(err) => println!("Error: {err:?}"),
//     }
// }


//////
// Exercise: Rewriting with Result https://google.github.io/comprehensive-rust/error-handling/exercise.html

/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

// The original implementation of the expression evaluator. Update this to
// return a `Result` and produce an error when dividing by 0.
fn eval(e: Expression) -> Result<i64, DivideByZeroError> {
    match e {
        Expression::Op { op, left, right } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => if right == 0 {
                    return Err(DivideByZeroError);
                } else {
                    left / right
                },
            })
        }
        Expression::Value(v) => Ok(v),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(DivideByZeroError)
        );
    }

    #[test]
    fn test_ok() {
        let expr = Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(20)),
            right: Box::new(Expression::Value(10)),
        };
        assert_eq!(eval(expr), Ok(10));
    }
}

fn main() {

    // anyhow(); // add anyhow = "1.0" below [dependencies] in Cargo.toml
    // thiserror(); // add thiserror = "1.0" below [dependencies] in Cargo.toml
    // dynamic_error_types();
    // try_conversion();
    // try_operator();
    // result();
    // panics();
}
