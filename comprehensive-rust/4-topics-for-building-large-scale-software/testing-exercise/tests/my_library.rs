// Integration Tests
// use my_library::init;

// #[test]
// fn test_init() {
//     assert!(init().is_ok());
// }

#[test]
fn test_example() {
    assert_eq!(2 + 2, 4);
}


/// Shortens a string to the given length.
///
/// ```
/// # use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}