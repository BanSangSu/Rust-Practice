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
use garden_project::{Garden, SeedPacket, sow, harvest};
fn filesystem_hierarchy() {
    println!("Welcome to the Garden Project!");
    
    // Create some seed packets
    let seeds = vec![
        SeedPacket::new("Carrot"),
        SeedPacket::new("Tomato"),
        SeedPacket::new("Lettuce"),
    ];

    // Sow the seeds
    println!("Sowing seeds...");
    sow(seeds.clone());

    let mut garden = Garden::new();
    for seed in &seeds {
        garden.plant(&seed.name);
    }

    // Simulate time passing
    println!("Time passes... The plants are ready to harvest!");

    // Harvest the garden
    println!("Harvesting...");
    harvest(&mut garden);

    println!("Garden project complete!");
}



fn main() {
    filesystem_hierarchy();
    // module();
}
