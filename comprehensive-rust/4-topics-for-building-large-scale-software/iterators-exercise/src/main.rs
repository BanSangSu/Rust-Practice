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


// IntoIterator https://google.github.io/comprehensive-rust/iterators/intoiterator.html
fn into_iterator() {
    struct Grid {
        x_coords: Vec<u32>,
        y_coords: Vec<u32>,
    }

    // impl IntoIterator for Grid {
    //     type Item = (u32, u32);
    //     type IntoIter = GridIter;
    //     fn into_iter(self) -> GridIter {
    //         GridIter { grid: self, i: 0, j: 0 }
    //     }
    // }
    // struct GridIter {
    //     grid: Grid,
    //     i: usize,
    //     j: usize,
    // }  
    
    // impl Iterator for GridIter {
    //     type Item = (u32, u32);

    //     fn next(&mut self) -> Option<(u32, u32)> {
    //         if self.i >= self.grid.x_coords.len() {
    //             self.i = 0;
    //             self.j += 1;
    //             if self.j >= self.grid.y_coords.len() {
    //                 return None;
    //             }
    //         }
    //         let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
    //         self.i += 1;
    //         res   
    //     }
    // }

    /// reference method
    impl<'a> IntoIterator for &'a Grid { 
        type Item = (u32, u32);
        type IntoIter = GridRefIter<'a>;
        fn into_iter(self) -> GridRefIter<'a> {
            GridRefIter { grid: self, i: 0, j: 0 }
        }
    }
    
    struct GridRefIter<'a> {
        grid: &'a Grid,
        i: usize,
        j: usize,
    }

    impl<'a> Iterator for GridRefIter<'a> {
        type Item = (u32, u32);

        fn next(&mut self) -> Option<(u32, u32)> {
            if self.i >= self.grid.x_coords.len() {
                self.i = 0;
                self.j += 1;
                if self.j >= self.grid.y_coords.len() {
                    return None;
                }
            }
            let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
            self.i += 1;
            res   
        }
    }


    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    // for (x, y) in grid {
    //     println!("point = {x}, {y}");
    // }

    for (x, y) in &grid {
        println!("point = {x}, {y}");
    }
    println!("One more!");
    for (x, y) in &grid { // Reference method makes it possible.
        println!("point = {x}, {y}");
    }
}


//////
// Exercise: Iterator Method Chaining https://google.github.io/comprehensive-rust/iterators/exercise.html

/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    let a = values.iter();
    let b = values.iter().cycle().skip(offset);
    a.zip(b).map(|(a, b)| *b - *a).collect()
}

#[test]
fn test_offset_one() {
    assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
    assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
    assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
    assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
    assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
    assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
    assert_eq!(offset_differences(1, vec![0]), vec![0]);
    assert_eq!(offset_differences(1, vec![1]), vec![0]);
    let empty: Vec<i32> = vec![];
    assert_eq!(offset_differences(1, empty), vec![]);
}


fn main() {
    // into_iterator();
    // collect();
    // iterator_helper_methods();
    // iterator_trait();
    // motivating_iterators();
}
