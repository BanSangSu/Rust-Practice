// Unit Tests https://google.github.io/comprehensive-rust/testing/unit-tests.html
fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(first_word("Hello"), "Hello");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(first_word("Hello World"), "Hello");
    }
}

// Other Types of Tests https://google.github.io/comprehensive-rust/testing/other.html
// at my_library.rs in tests folder.

// Compiler Lints and Clippy https://google.github.io/comprehensive-rust/testing/lints.html
// fn compiler_lints_and_clippy() {
    // 
    // }
    
#[deny(clippy::cast_possible_truncation)]
// #![deny(clippy::cast_possible_truncation)]
fn main() {
    let mut x = 3;
    while (x < 70000) {
        x *= 2;
    }
    println!("X probably fits in a u16, right? {}", x as u16);
}
