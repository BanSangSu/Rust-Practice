// Types and Values
fn hello_world() {
    println!("Hello, 🌍!");
}

fn variables_values() {
    let x: i32 = 10;
    let y: f32 = 10e2;
    let z: char = 'a';
    let a: i32 = 10i32; // let a = 10_i32;
    println!("x: {x}, y: {y}, z: {z}, a: {a}");

    // if let mut x: i32, the code below will be run.
    // x = 20;
    // println!("x: {x}");
}

fn arithmetic() {
    fn interproduct(a: i32, b: i32, c: i32) -> i32 {
        return a * b + b * c + c * a;
    }

    println!("reuslt {}", interproduct(120, 100, 248));
}

fn type_inference() {
    fn takes_u32(x: u32) {
        println!("u32: {x}");
    }
    
    fn takes_i8(y: i8) {
        println!("i8: {y}");
    }

    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
}

fn fibo() {
    fn fib(n: u32) -> u32 {
        if n < 2 {
            return n;
        } else {
            return fib(n-1) + fib(n-2);
        }
    }

    let n = 5;
    println!("fib({n}) = {}", fib(n));

}


//
// Control Flow Basics
fn if_expressions() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    let size =  if x < 20 {"small"} else {"large"};
    println!("size: {size}");
}

fn loops() {
    fn while_loop() {
        let mut x = 200;
        
        while x >= 10 {
            x /= 2;
        }
        println!("Final x: {x}");
    }

    fn for_loop() {
        for x in 1..5 {
            println!("x: {x}");
        }

        for y in 1..=5 {
            println!("y: {y}");
        }

        for elem in [1, 2, 3, 4, 5] {
            println!("elem: {elem}");
        }
    }

    fn loop_loop() {
        let mut i = 0;
        loop {
            i += 1;
            println!("{i}");
            if i > 100 {
                break;
            }
        }
    }

    while_loop();
    for_loop();
    loop_loop();
}

fn break_and_contiue() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i & 1 != 1 {
            continue;
        }
        println!("{}", i);
    }
}

fn labels() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
            'label: {
                break 'label;
                println!("This line gets skipped");
            }
        }
    }
    print!("elements searched: {elements_searched}");
}

fn blocks_and_scopes_and_shadowing() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");


    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

fn functions() {
    fn gcd(a: u32, b: u32,) -> u32 {
        if b > 0 {
            gcd(b, a % b)
        } else {
            a
        }
    }

    println!("gcd: {}", gcd(143, 52));
}

fn macros() {
    fn factorial(n: u32) -> u32 {
        let mut product = 1;
        for i in 1..=n {
            product *= dbg!(i);
        }
        product
    }

    fn fizzbuzz(n: u32) -> u32 {
        todo!()
    }

    let n = 4;
    println!("{n}! = {}", factorial(n));
    fizzbuzz(12);
}


//
// Exercise: Collatz Sequence
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n & 1 != 1 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len

    // let mut cnt = 0;
    // 'seq: loop {
    //     cnt += 1;
    //     if n == 1 {
    //         break 'seq;
    //     } else if n & 1 == 1 {
    //         // odd
    //         n = 3 * n + 1;
    //     } else if n & 1 == 0 {
    //         // even
    //         n /= 2;
    //     }
    // }
    // cnt
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
fn collatz_sequence() {

    println!("Length: {}", collatz_length(11));
}


fn main() {
    
    collatz_sequence();
    
    //

    // if_expressions();

    // loops();

    // break_and_contiue();

    // labels();

    // blocks_and_scopes_and_shadowing();

    // functions();

    // macros();


    //

    // hello_world();

    // variables_values();
    
    // arithmetic();

    // type_inference();

    // fibo();
}
