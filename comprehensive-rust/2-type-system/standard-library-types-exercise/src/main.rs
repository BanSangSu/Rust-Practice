


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


fn main() {
    documentation();
    println!("Hello, world!");
}
