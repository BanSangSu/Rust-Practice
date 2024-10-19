struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main(){
    println!("{:?}", 13);
    println!("{1:?} {0:?} and {actor:?}",
            "Deep",
            "Learning",
            actor="You?");
    
    println!("{:?} printed?", Structure(3));
    println!("{:?} printed?", Deep(Structure(13)));

    let name = "Siu";
    let age = 28;
    let siu = Person {name, age};
    println!("{:#?}", siu);
}