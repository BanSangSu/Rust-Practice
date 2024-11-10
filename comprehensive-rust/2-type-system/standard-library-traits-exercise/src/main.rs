// Agenda https://google.github.io/comprehensive-rust/std-traits.html



// Comparisons https://google.github.io/comprehensive-rust/std-traits/comparisons.html
fn comparisions() {
    struct Key {
        id:u32,
        metadata: Option<String>,
    }
    // impl PartialEq for Key {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.id == other.id
    //     }
    // } 
    // Same above
    impl PartialEq <u32> for Key {
        fn eq(&self, other: &u32) -> bool {
            self.id == *other
        }
    }

    use std::cmp::Ordering;
    #[derive(Eq, PartialEq)]
    struct Citation {
        author: String,
        year: u32,
    }
    impl PartialOrd for Citation {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.author.partial_cmp(&other.author) {
                Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
                author_ord => author_ord,
            }
        }
    }

    let key1 = Key{ id:10, metadata: Some(String::from("B"))};
    let key2 = Key{ id:10, metadata: Some(String::from("B"))};
    let citation1 = Citation{ author: String::from("good"), year: 12};
    let citation2 = Citation{ author: String::from("good"), year: 12};
    println!("{}", key1.eq(&key2.id));
    println!("{:?}", citation1.partial_cmp(&citation2));
}



// Operators https://google.github.io/comprehensive-rust/std-traits/operators.html
fn operators() {
    #[derive(Debug, Copy, Clone)]
}

fn main() {
    comparisions();
    operators();
}
