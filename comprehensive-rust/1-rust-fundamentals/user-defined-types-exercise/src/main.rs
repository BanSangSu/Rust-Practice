

fn named_structs() {
    struct Person {
        name: String,
        age: u8,
    }

    fn describe(person: &Person) {
        println!("{} is {} years old", person.name, person.age);
    }


    let mut susu = Person { name: String::from("Susu"), age: 27};
    describe(&susu);

    susu.age = 29;
    describe(&susu);

    let name = String::from("Ssic");
    let age = 40;
    let ssic = Person {name, age};
    describe(&ssic);

    let gogi = Person { name: String::from("Gogi"), ..ssic };
    describe(&gogi);
}

fn tuple_structs() {
    struct Point(i32, i32);

    let p = Point(10, 85);
    println!("({}, {})", p.0, p.1);
}


// Enums
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport {x: i32, y: i32,},
}

#[repr(u32)]
enum Bar {
    A, // 0
    B = 10000,
    C, // 10001
}


use std::mem::transmute;
macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

fn enums() {
    let player_move: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {player_move:?}");

    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);


    unsafe {
        println!("bool:");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Option<bool>:");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Option<Option<bool>>:");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Option<&i32>:");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
    }
}

fn consts() {
    const DIGET_SIZE: usize = 3;
    const ZERO: Option<u8> = Some(42);

    fn compute_digest(text: &str) -> [u8; DIGET_SIZE] {
        let mut digest = [ZERO.unwrap_or(0); DIGET_SIZE];
        for (idx, &b) in text.as_bytes().iter().enumerate() {
            digest[idx % DIGET_SIZE] = digest[idx % DIGET_SIZE].wrapping_add(b);
        }
        digest
    }
    let digest = compute_digest("HiHi");
    println!("digest: {digest:?}");

}

fn main() {
    named_structs();
    tuple_structs();
    enums();
    consts();
}
