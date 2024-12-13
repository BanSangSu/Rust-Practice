// Box<T> https://google.github.io/comprehensive-rust/smart-pointers/box.html
# [derive(Debug)]
enum List<T> {
    Element(T, Box<List<T>>),
    Nil,
}
fn box_type() {
    let five = Box::new(5);
    println!("five: {}", *five);

    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}


// Rc https://google.github.io/comprehensive-rust/smart-pointers/rc.html
use std::rc::Rc;


fn reference_counted() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    
    println!("a: {a}");
    println!("b: {b}");
    println!("Strong count of a: {}", Rc::strong_count(&a));
    println!("Strong count of b: {}", Rc::strong_count(&b));
}


// Owned Trait Objects https://google.github.io/comprehensive-rust/smart-pointers/trait-objects.html

fn owned_trait_objects() {
    struct Dog {
        name: String,
        age: i8,
    }
    struct Cat {
        lives: i8,
    }
    
    trait Pet {
        fn talk(&self) -> String;
    }
    
    impl Pet for Dog {
        fn talk(&self) -> String {
            format!("Woof, my name is {}!", self.name)
        }
    }
    
    impl Pet for Cat {
        fn talk(&self) -> String {
            String::from("Miau!")
        }
    }
    
    let pets: Vec<Box<dyn Pet>> = vec! [
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
        ];

        for pet in pets {
            println!("Hello, who are you? {}", pet.talk());
        }
        
        println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
        println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
        println!("{}", std::mem::size_of::<&dyn Pet>());
        println!("{}", std::mem::size_of::<Box<dyn Pet>>());
    }
    
    
    
//////
// Exercise: Binary Tree https://google.github.io/comprehensive-rust/smart-pointers/exercise.html
/// A node in the binary tree.
use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// A container storing a set of values, using a binary tree.
///
/// If the same value is added multiple times, it is only stored once.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

// Implement `new`, `insert`, `len`, and `has` for `Subtree`.
impl<T: Ord> Substree<T> {
    fn new() -> Self {
        Self(None)
    }

    fn insert(&mut self, value: T) {
        match &mut self.0 {
            None => self.0 = Some(Box::new(Node::new(value))),
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.insert(value),
                Ordering::Equal => {},
                Ordering::Greater => n.right.insert(value),
            },
        }
    }

    fn has(&self, value: &T) -> bool {
        match &mut self.0 {
            None => false,
            Some(n) => match value.cmp(&n.value) {
                Ordering::Less => n.left.has(value),
                Ordering::Equal => true,
                Ordering::Greater => n.right.has(value),
            },
        }
    }

    fn len(&self) -> usize {
        match &mut self.0 {
            None => 0,
            Some(n) => 1 + n.left.len() + n.right.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}


fn main() {
    // box_type();
    // reference_counted();
    // owned_trait_objects();
}
