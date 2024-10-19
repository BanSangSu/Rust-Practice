use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        
        write!(f, "{}: {:.3}°{} {:.3}°{}",
            self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Colour {
    red: u8,
    green: u8,
    blue: u8,
}

struct RGB {
    red: u32,
    green: u32,
    blue: u32,
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:0>6X}", self.red, self.green, self.blue, ((self.red*65536)+(self.green*256)+self.blue))
    }
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for colour in [
        Colour { red: 128, green: 255, blue: 90 },
        Colour { red: 0, green: 3, blue: 254 },
        Colour { red: 0, green: 0, blue: 0 },
    ] {
        println!("{:?}", colour);
    }

    /*
        Activity

        1. Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:

        RGB (128, 255, 90) 0x80FF5A
        RGB (0, 3, 254) 0x0003FE
        RGB (0, 0, 0) 0x000000

        Three hints if you get stuck:

            The formula for calculating a color in the RGB color space is: RGB = (R*65536)+(G*256)+B , (when R is RED, G is GREEN and B is BLUE). For more see RGB color format & calculation.
            You may need to list each color more than once.
            You can pad with zeros to a width of 2 with :0>2.

     */

    // 1.
    for rgb in [
        RGB { red: 128, green: 255, blue: 90 },
        RGB { red: 0, green: 3, blue: 254 },
        RGB { red: 0, green: 0, blue: 0 },
    ] {
        println!("{}", rgb);
    }
}