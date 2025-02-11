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


fn main() {
}
