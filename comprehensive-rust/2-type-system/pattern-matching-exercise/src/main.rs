#[rustfmt::skip]
fn matching_values() {
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("Something else"),
    }

    let opt = Some(1234);
    match opt {
        outer @ Some(inner) => {
            println!("outer: {outer:?}, inner: {inner}");
        }
        None => {}
    }
}

fn structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    #[rustfmt::skip]
    let foo = Foo { x: (5,4), y: 2};
    match foo {
        Foo { x: (1,b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i } => println!("y = 2, x = {i:?}"),
        Foo { y, .. } => println!("y = {y}, other fields were ignored"),
    }
}

fn enums() {
    enum Result {
        OK(i32),
        Err(String),
        // Wait(String), it causes non-exhaustive patterns when there is no this enum in match below.
    }

    fn divide_in_two(n: i32) -> Result {
        if n % 2 == 0 {
            Result::OK(n/2)
        } else {
            Result::Err(format!("cannot divide {n} into two equal parts"))
        }
    }

    let n = 102;
    match divide_in_two(n) {
        Result::OK(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}


// let control flow
use std::time::Duration;

fn let_control_flow() {
    // if let expressions
    fn sleep_for(secs: f32) {
        if let Ok(duration) = Duration::try_from_secs_f32(secs) {
            std::thread::sleep(duration);
            println!("slept for {duration:?}");
        }
    }

    sleep_for(-10.0);
    sleep_for(0.8);


    // if let expressions (with else)
    fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
        if let Some(s) = maybe_string {
            if let Some(first_byte_char) = s.chars().next() {
                if let Some(digit) = first_byte_char.to_digit(16) {
                    return Ok(digit)
                } else {
                    return Err(String::from("not a hex digit"));
                }
            } else {
                return Err(String::from("got empty string"));
            }
        } else {
            return Err(String::from("got None"));
        }
    }
    // let else expressions - rewritten version of above code
    fn hex_or_die_trying_rewritten_version(maybe_string: Option<String>) -> Result<u32, String> {
        let Some(s) = maybe_string else {
            return Err(String::from("got None"));
        };

        let Some(first_byte_char) = s.chars().next()  else {
            return Err(String::from("got empty string"));
        };

        let Some(digit) = first_byte_char.to_digit(16) else {
            return Err(String::from("not a hex digit"));
        };

        return Ok(digit);
    }

    println!("result: {:?}", hex_or_die_trying(Some(String::from("bad"))));
    println!("result: {:?}", hex_or_die_trying_rewritten_version(Some(String::from("good"))));


    // while let expressions
    fn while_let_1() {
        let mut optional = Some(0);

        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }
    } 
    fn while_let_2() {
        let mut name = String::from("Comprehensive Rust 🦀");
        while let Some(c) = name.pop() {
            println!("character: {c}");
        }
    }
    while_let_1();
    while_let_2();

}



// exercise expression evaluation

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

fn eval(e: Expression) -> Result<i64, String> {
    match e {
        Expression::Op {op, left, right} => {
            let left = match eval(*left) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            let right = match eval(*right) {
                Ok(v) => v,
                e @ Err(_) => return e,
            };
            Ok(match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => {
                    if right == 0 {
                        return Err(String::from("division by zero"));
                    } else {
                        left / right
                    }
                },
            })
        }
        Expression::Value(v) => Ok(v),
    }
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("division by zero"))
    );
}

fn exercise_expression_evaluation() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };
    println!("expr: {:?}", expr);
    println!("result: {:?}", eval(expr));
}

fn main() {
    // use cargo test to test the exercise
    exercise_expression_evaluation();

    // matching_values();
    // structs();
    // enums();
    // let_control_flow();
}
