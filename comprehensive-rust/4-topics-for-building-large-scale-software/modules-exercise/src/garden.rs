pub struct Garden {
    pub plants: Vec<String>,
}

impl Garden {
    pub fn new() -> Self {
        Garden { plants: Vec::new() }
    }

    pub fn plant(&mut self, seed_name: &str) {
        println!("Planting seed: {}", seed_name);
        self.plants.push(seed_name.to_string());
    }

    pub fn harvest(&mut self) -> Vec<String> {
        println!("Harvesting plants...");
        let harvested = self.plants.clone();
        self.plants.clear();
        harvested
    }
}