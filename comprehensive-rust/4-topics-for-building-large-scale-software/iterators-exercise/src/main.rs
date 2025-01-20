// Motivating Iterators https://google.github.io/comprehensive-rust/iterators/motivation.html

fn motivating_iterators() {
    let array = [2, 4, 6, 8];
    for elem in array {
        println!("for elem: {}", elem);
    }

    let mut i = 0;
    while i < array.len() {
        let elem = array[i];
        println!("while elem: {}", elem);
        i += 1;    
    }

    for elem in &array {
        println!("for ref elem: {}", elem);
    }
}


fn main() {
    motivating_iterators();
}
