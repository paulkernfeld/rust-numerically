//! This function should return x^y without using recursion.
//!
//! Also, don't use Rust's built-in `pow` function, that's too easy.
//!
//! Extra credit: how should this function deal with overflow? See
//! https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/
fn pow(x: u32, y: u32) -> Option<u32> {
    unimplemented!("Fill me in")
}

fn main() {
    assert_eq!(pow(0, 0), None);
    assert_eq!(pow(0, 1), Some(0));
    assert_eq!(pow(0, 2), Some(0));
    assert_eq!(pow(0, 3), Some(0));

    assert_eq!(pow(1, 0), Some(1));
    assert_eq!(pow(1, 1), Some(1));
    assert_eq!(pow(1, 2), Some(1));
    assert_eq!(pow(1, 3), Some(1));

    assert_eq!(pow(2, 0), Some(1));
    assert_eq!(pow(2, 1), Some(2));
    assert_eq!(pow(2, 2), Some(4));
    assert_eq!(pow(2, 3), Some(8));

    assert_eq!(pow(3, 0), Some(1));
    assert_eq!(pow(3, 1), Some(3));
    assert_eq!(pow(3, 2), Some(9));
    assert_eq!(pow(3, 3), Some(27));
}
