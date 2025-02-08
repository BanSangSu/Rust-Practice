// Modules https://google.github.io/comprehensive-rust/modules/modules.html
fn module() {
    mod foo {
        pub fn do_something() {
            println!("In the foo module");
        }
    }

    mod bar {
        pub fn do_something() {
            println!("In the bar module");
        }
    }

    foo::do_something();
    bar::do_something();

}

// Filesystem Hierarchy https://google.github.io/comprehensive-rust/modules/filesystem.html
// use garden_project::{Garden, SeedPacket, sow, harvest};
// fn filesystem_hierarchy() {
//     println!("Welcome to the Garden Project!");
    
//     // Create some seed packets
//     let seeds = vec![
//         SeedPacket::new("Carrot"),
//         SeedPacket::new("Tomato"),
//         SeedPacket::new("Lettuce"),
//     ];

//     // Sow the seeds
//     println!("Sowing seeds...");
//     sow(seeds.clone());

//     let mut garden = Garden::new();
//     for seed in &seeds {
//         garden.plant(&seed.name);
//     }

//     // Simulate time passing
//     println!("Time passes... The plants are ready to harvest!");

//     // Harvest the garden
//     println!("Harvesting...");
//     harvest(&mut garden);

//     println!("Garden project complete!");
// }

// Visibility https://google.github.io/comprehensive-rust/modules/visibility.html
fn visibility() {
    mod outer {
        fn private() {
            println!("outer::private");
        }
    
        pub fn public() {
            println!("outer::public");
        }
        
        pub mod inner {
        // pub(in crate) mod inner {
        // pub(crate) mod inner {
        // pub(super) mod inner {
        // mod inner {
            fn private() {
                println!("outer::inner::private");
            }
    
            pub fn public() {
                println!("outer::inner::public");
                super::private();
            }        
        }
    }
    outer::public();
    outer::inner::public();
}

// Visibility and Encapsulation https://google.github.io/comprehensive-rust/modules/encapsulation.html
fn visibility_and_encapsulation() {
    use outer::Foo;
    mod outer {
        pub struct Foo {
            pub val: i32,
            is_big: bool,
        }

        impl Foo {
            pub fn new(val: i32) -> Self {
                Self { val, is_big: val > 100}
            }
        }

        pub mod inner {
            use super::Foo;

            pub fn print_foo(foo: &Foo) {
                println!("Is {} big? {}", foo.val, foo.is_big);
            }
        }
    }

    let foo = Foo::new(42);
    println!("foo.val = {}", foo.val);
    // let foo = Foo { val: 42, is_big: true };

    outer::inner::print_foo(&foo);
    // println!("Is {} big? {}", foo.val, foo.is_big);
}

// use, super, self https://google.github.io/comprehensive-rust/modules/paths.html
use garden_project::Garden;
fn use_super_self() {
    let seeds = vec![
        "Apple",
        "Grape",
        "Watermelon",
    ];

    let mut garden = Garden::new();
    for seed in seeds {
        garden.plant(&seed);
    }
    println!("zzZ");
    let produce = garden.harvest();
    for item in produce {
        println!("Harvested: {}", item);
    }
    println!(">3<");
}

fn main() {
    use_super_self();
    // visibility_and_encapsulation();
    // visibility();
    // filesystem_hierarchy();
    // module();
}
