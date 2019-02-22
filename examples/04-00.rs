//! Implement these functions to compute the mean and variance of a fixed-size list of
//! floats.
//!
//! This exercise should be paired with part 1 of Chapter 4 of The Rust Programming Language. It is
//! probably not a good idea to write Rust functions that directly return their inputs, but this is
//! the best way to accomplish our goal without using references.
//!
//! First computing the mean then computing the variance is not necessarily the most efficient way
//! or accurate way to get these values. See:
//! https://www.johndcook.com/blog/2008/09/26/comparing-three-methods-of-computing-standard-deviation/
//!
//! Although it's not usually a good idea to compare floating point numbers exactly, the results
//! here can be represented exactly in binary.
struct MeanResult {
    mean: f64,
    values: [f64; 100],
}

fn mean(values: [f64; 100]) -> MeanResult {
    unimplemented!()
}

struct VarianceResult {
    variance: f64,
    values: [f64; 100],
}

fn variance(values: [f64; 100], mean: f64) -> VarianceResult {
    unimplemented!()
}

fn main() {
    let values = {
        let mut values = [0.0; 100];
        for i in 0..100 {
            values[i] = i as f64;
        }
        values
    };

    // Compute and check the mean
    let MeanResult {
        mean: the_mean,
        values,
    } = mean(values);
    assert_eq!(the_mean, 49.5);

    // Compute and check the variance
    let the_variance = variance(values, the_mean).variance;
    assert_eq!(the_variance, 833.25)
}
