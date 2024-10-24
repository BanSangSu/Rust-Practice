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

fn main() {
    matching_values();
    structs();
    enums();
}
