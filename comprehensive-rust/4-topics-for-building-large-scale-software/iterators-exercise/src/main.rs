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
}


fn main() {
    
    iterator_trait();
    motivating_iterators();
}
