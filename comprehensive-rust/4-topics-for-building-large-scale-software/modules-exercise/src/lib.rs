//! This module implements the garden library, including seed sowing and harvesting logic.
// Set the lib to use lib.rs in Cargo.toml

// Declare submodules using directories with `module.rs (garden.rs, seeds.rs)`.
mod garden;
mod seeds;

// Re-export types from submodules for easier access.
pub use garden::Garden;
pub use seeds::SeedPacket;


/// Sow the given seed packets into the garden.
pub fn sow(seeds: Vec<SeedPacket>) {
    println!("Sowing seeds into the garden...");
    for seed in seeds {
        println!("Sowing seed: {}", seed.name);
    }
}

/// Harvest the produce in the garden that is ready.
pub fn harvest(garden: &mut Garden) {
    println!("Harvesting produce from the garden...");
    let produce = garden.harvest();
    for item in produce {
        println!("Harvested: {}", item);
    }
}