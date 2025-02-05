#[derive(Clone)]
pub struct SeedPacket {
    pub name: String,
}

impl SeedPacket {
    pub fn new(name: &str) -> Self {
        SeedPacket { name: name.to_string() }
    }
}