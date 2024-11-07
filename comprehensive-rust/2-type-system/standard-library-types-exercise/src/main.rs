


/// Documentation https://google.github.io/comprehensive-rust/std-types/docs.html#documentation

fn documentation() {
    /*!
     * ref. https://doc.rust-lang.org/reference/comments.html
     */

    //! This module contains functionality relating to divisibility of integers.
    //! /*! */ and //! are used in inner line (funcfion or something.)
    

    /// Determine whether the first argument is divisible by the second argument.
    ///
    /// If the second argument is zero, the result is false.
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs == 0
    }
    println!("{}", is_divisible_by(1,55));
}


// Option https://google.github.io/comprehensive-rust/std-types/option.html
fn option() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut position: Option<usize> = name.find('é');
    println!("find returned {position:?}");
    assert_eq!(position.unwrap(), 14); 
    position = name.find('Z');
    println!("find returned {position:?}");
    assert_eq!(position.expect("Character not found"), 0);
}


// Result https://google.github.io/comprehensive-rust/std-types/result.html
use std::fs::File;
use std::io::Read;
fn result() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}


// String https://google.github.io/comprehensive-rust/std-types/string.html
fn string() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let mut s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}", s3.len(), s3.chars().count());

    use std::ops::Deref;
    let s4 = s1.deref();
    let s5 = &*s1;
    println!("s4: {}, s5 = {}", s4, s5);
}


// Vec https://google.github.io/comprehensive-rust/std-types/vec.html
fn vec() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());
    println!("v2: len = {:?},", v2);

    // Canonical macro to initialize a vector with elements.
    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    // Retains only the elements specified by the predicate (in here even elements).
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Removes consecutive repeated elements
    v3.dedup();
    println!("{v3:?}");
}



// https://google.github.io/comprehensive-rust/std-types/hashmap.html
use std::collections::HashMap;

fn hashmap(){
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn", 207);
    page_counts.insert("Grimms' Fairy Tales", 751);
    page_counts.insert("Pride and Prejudice", 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");
    
    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone")
        .unwrap_or(&336);
    println!("{pc1}");

    let pc2 = page_counts
        .entry("The Hunger Games")
        .or_insert(374);
    println!("{pc2}");
    // println!("{pc1}, {pc2}"); error, mutalbe borrow occurs with immutable borrow later used.


    let page_counts_1 = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);
    println!("{page_counts_1:#?}");
    println!("{:#?}", page_counts_1.keys());
}


fn main() {
    // documentation();
    // option();
    // result();
    // string();
    // vec();
    hashmap();
}
