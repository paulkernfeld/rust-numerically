//! Implement a 2D point object using Rust tuples.
//!
//! Although we're not using the `+` and `*` operators to represent addition and multiplication, we
//! will be able to do that in a future exercise. There are also ways to make the types a bit more
//! readable.
fn add(point1: (f64, f64), point2: (f64, f64)) -> (f64, f64) {
    unimplemented!("Fill me in")
}

fn mul(point: (f64, f64), scalar: f64) -> (f64, f64) {
    unimplemented!("Fill me in")
}

fn main() {
    let point1 = (1.0, 2.0);
    let point2 = (3.0, 4.0);
    let point3 = add(point1, point2);
    assert_eq!(point3, (4.0, 6.0));
    let negative_point1 = mul(point1, -1.0);
    assert_eq!(add(point3, negative_point1), point2);
}
