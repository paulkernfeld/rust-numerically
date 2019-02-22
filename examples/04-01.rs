//! This is similar to exercise `04-00` but more concise and less weird because we are using
//! references and borrowing.
fn mean(values: &[f64; 100]) -> f64 {
    unimplemented!()
}

fn variance(values: &[f64; 100], mean: f64) -> f64 {
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
    let the_mean = mean(&values);
    assert_eq!(the_mean, 49.5);

    // Compute and check the variance
    let the_variance = variance(&values, the_mean);
    assert_eq!(the_variance, 833.25)
}
