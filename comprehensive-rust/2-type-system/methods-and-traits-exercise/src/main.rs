// methods
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self. name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self (covered later)
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn methods() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);

}


// traits
trait Pet {
    fn talk(&self) -> String;
    
    // fn greet(&self) -> String;
    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

fn traits() {
    let fido = Dog { name: String::from("Fido"), age: 5};
    fido.greet();
}


// supertraits (trait inheritance)
trait Animal {
    fn leg_count(&self) -> u32;
}

trait Mammal: Animal {
    fn name(&self) -> String;
}

struct Cat(String);

impl Animal for Cat {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Mammal for Cat {
    fn name(&self) -> String {
        self.0.clone()
    }
}

fn supertraits() {
    let kitten = Cat(String::from("Kong"));
    println!("{} has {} legs", kitten.name(), kitten.leg_count());
}


// associated_types (output types)
#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared;
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

fn associated_types() {
    println!("{:?}", Meters(10).multiply(&Meters(20)));
}


// Deriving
#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn deriving() {
    let p1 = Player::default();
    let mut p2 = p1.clone();
    p2.name = String::from("EldurScrollz");
    println!("{p1:?} vs. {p2:?}");
}


// Exercise: Logger Trait 

pub trait Logger {
    fn log(&self, verbosity: u8, message: &str);
}

struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, verbosity: u8, message: &str) {
        println!("verbosity={verbosity}: {message}");
    }
}


struct VerbosityFilter { 
    max_verbosity: u8,
    inner: StdoutLogger,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn exercise_logger_trait() {    
    let logger = VerbosityFilter { max_verbosity: 3, inner: StdoutLogger};
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}

fn main() {
    exercise_logger_trait();

//     methods();
//     traits();
//     supertraits();
//     associated_types();
//     deriving();
}
