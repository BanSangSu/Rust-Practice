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

//////
// Exercise: Modules for a GUI Library
pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label { label: label.to_owned() }
    }
}

pub struct Button {
    label: Label,
}

impl Button {
    fn new(label: &str) -> Button {
        Button { label: Label::new(label) }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window { title: title.to_owned(), widgets: Vec::new() }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        // Add 4 paddings for borders
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut inner = String::new();
        for widget in &self.widgets {
            widget.draw_into(&mut inner);
        }

        let inner_width = self.inner_width();

        // TODO: Change draw_into to return Result<(), std::fmt::Error>. Then use the
        // ?-operator here instead of .unwrap().
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        writeln!(buffer, "| {:^inner_width$} |", &self.title).unwrap();
        writeln!(buffer, "+={:=<inner_width$}=+", "").unwrap();
        for line in inner.lines() {
            writeln!(buffer, "| {:inner_width$} |", line).unwrap();
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 8 // add a bit of padding
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = self.width();
        let mut label = String::new();
        self.label.draw_into(&mut label);

        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.lines().map(|line| line.chars().count()).max().unwrap_or(0)
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        writeln!(buffer, "{}", &self.label).unwrap();
    }
}

fn exercise_modules_for_a_gui_library() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new("Click me!")));
    window.draw();
}

fn main() {
    exercise_modules_for_a_gui_library()


    
    // use_super_self();
    // visibility_and_encapsulation();
    // visibility();
    // filesystem_hierarchy();
    // module();
}
