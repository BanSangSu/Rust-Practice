
// Review of Program Memory https://google.github.io/comprehensive-rust/memory-management/review.html
fn review_programme_memory() {
    let s1 = String::from("Helllo");
    println!("{}", s1);

    let mut s2 = String::from("Goten Tag");
    s2.push(' ');
    s2.push_str("world");

    unsafe {
        let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s2);
        println!("capacity = {capacity}, ptr = {ptr:#x}, len = {len}");
    }
}

// Approaches to Memory Management https://google.github.io/comprehensive-rust/memory-management/approaches.html
// There is no code.
// Rust's onwership, borrowing, and tools (Rc, Box, etc) model can handle pointer like C alloc and C++'s smart pointers.


// Ownership https://google.github.io/comprehensive-rust/memory-management/ownership.html#ownership
// struct Point(i32, i32);
fn ownership() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // println!("y: {}", p.1); // Cause error not found p.1 in this scope
}

// Move Semantics https://google.github.io/comprehensive-rust/memory-management/move.html
fn move_semantics() {
    let s1: String = String::from("Cool!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}"); // Error: value borrowed here after move

    fn say_yo(name: String) {
        println!("Yo, {name}!");
    }

    let name = String::from("GiGi");
    say_yo(name);
    // say_yo(name);
}


// Clone https://google.github.io/comprehensive-rust/memory-management/clone.html
fn clone() {
    fn call_me(name: String) {
        println!("Call me {name}!");
    } 

    let name = String::from("BoBo");
    call_me(name.clone());
    call_me(name);
}


// Copy Types https://google.github.io/comprehensive-rust/memory-management/copy-types.html
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn copy_types() {
    let x = 31;
    let y = x;
    println!("x: {x}"); // would not be accessible if not Copy
    println!("y: {y}");

    let p1 = Point(2, 6);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}


// The Drop Trait https://google.github.io/comprehensive-rust/memory-management/drop.html
fn drop_trait() {
    struct Droppable {
        name: &'static str,
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            println!("Dropping {}", self.name);
        }
    }

    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    // a.drop();
    drop(a);
    println!("Exiting main");
}


/////
// Exercise: Builder Type https://google.github.io/comprehensive-rust/memory-management/exercise.html

#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Clone, Debug)]
struct Dependency {
    name: String,
    version_expression: String,
}

/// A representation of a software package.
#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    /// Return a representation of this package as a dependency, for use in
    /// building other packages.
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expression: self.version.clone(),
        }
    }
}

/// A builder for a Package. Use `build()` to create the `Package` itself.
struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            version: "0.1".into(),
            authors: Vec::new(), // (same) == vec![]
            dependencies: Vec::new(),
            language: None,
        })
    }

    /// Set the package version.
    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    /// Set the package authors.
    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors.into();
        self
    }

    /// Add an additional dependency.
    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    /// Set the language. If not set, language defaults to None.
    fn language(mut self, language: Language) -> Self {
        self.0.language = language.into();
        self
    }

    fn build(self) -> Package {
        self.0
    }
}

fn exercise_builder_type() {
    let base64 = PackageBuilder::new("base64").version("0.13").build();
    println!("base64: {base64:?}");
    let log =
        PackageBuilder::new("log").version("0.4").language(Language::Rust).build();
    println!("log: {log:?}");
    let serde = PackageBuilder::new("serde")
        .authors(vec!["djmitche".into()])
        .version(String::from("4.0"))
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    println!("serde: {serde:?}");

}


fn main() {
    exercise_builder_type();


    // review_programme_memory();
    // ownership();
    // move_semantics();
    // clone();
    // copy_types();
    // drop_trait();
}

