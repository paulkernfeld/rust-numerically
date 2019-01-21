//! Find the mean of an array of 3 values.
//!
//! Note that this function can't accept arrays with different numbers of elements. In future
//! exercises we'll explore different ways to process collections where the size is not known at
//! compile time.
//!
//! Extra credit: although it doesn't really matter for an array of this size, it's often better to
//! visit each value once.
fn mean(values: [f64; 3]) -> f64 {
    unimplemented!()
}

fn main() {
    assert_eq!(mean([-1.0, 0.0, 1.0]), 0.0);
    assert_eq!(mean([0.0, 2.0, 4.0]), 2.0);
    assert_eq!(mean([0.0, 2.0, 10.0]), 4.0);
}
