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

// Iterator Trait https://google.github.io/comprehensive-rust/iterators/iterator.html
fn iterator_trait() {
    struct SliceIter<'s> {
        slice: &'s [i32],
        i:usize,
    }

    impl<'s> Iterator for SliceIter<'s> {
        type Item = &'s i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i == self.slice.len() {
                None
            }else {
                let next = &self.slice[self.i];
                self.i += 1;
                Some(next)
            }
        }
    }

    let slice = &[2, 4, 6, 8];
    let iter = SliceIter { slice, i: 0};
    for elem in iter {
        dbg!(elem);
    }
}

// Iterator Helper Methods https://google.github.io/comprehensive-rust/iterators/helpers.html
fn iterator_helper_methods() {
    let result: i32 = (1..=10) // Create a range from 1 to 10
        .filter(|x| x % 2 == 0) // Keep only even numbers
        .map(|x| x * x) // Square each number
        .sum(); // Sum up all the squared numbers

    println!("The sum of squares of even numbers from 1 to 10 is: {}", result);
}

// Collect https://google.github.io/comprehensive-rust/iterators/collect.html
fn collect() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");
}

fn main() {

    collect();
    // iterator_helper_methods();
    // iterator_trait();
    // motivating_iterators();
}
