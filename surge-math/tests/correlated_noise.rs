use surge_imports::*;
use surge_math::*;

#[test] fn test_rand01() {
    let val = rand01();
    assert!(val >= 0.0 && val < 1.0);
}

#[test] fn test_rand11() {
    let val = rand11();
    assert!(val >= -1.0 && val < 1.0);
}

#[test] fn test_correlated_noise() {
    let lastval = 0.0;
    let correlation = 0.5;
    let noise = correlated_noise(lastval, correlation);
    assert!(noise >= -1.0 && noise <= 1.0);
}

#[test] fn test_correlated_noise_mk2() {
    let lastval = 0.0;
    let correlation = 0.5;
    let noise = correlated_noise_mk2(lastval, correlation);
    assert!(noise >= -1.0 && noise <= 1.0);
}

#[test] fn test_drift_noise() {
    let lastval = 0.0;
    let noise = drift_noise(lastval);
    assert!(noise >= -1.0 && noise <= 1.0);
}

#[test] fn test_correlated_noise_o2() {
    let lastval = 0.0;
    let lastval2 = 0.0;
    let correlation = 0.5;
    let noise = correlated_noise_o2(lastval, lastval2, correlation);
    assert!(noise >= -1.0 && noise <= 1.0);
}

#[test] fn test_correlated_noise_o2mk2() {
    let lastval = 0.0;
    let lastval2 = 0.0;
    let correlation = 0.5;
    let noise = correlated_noise_o2mk2(lastval, lastval2, correlation);
    assert!(noise >= -1.0 && noise <= 1.0);
}

/// Helper function to calculate the mean of an iterator
fn mean<I>(iter: I) -> f64
where
    I: IntoIterator<Item = f64>,
{
    let iter = iter.into_iter();
    let (sum, count) = iter.fold((0.0, 0), |(sum, count), val| (sum + val, count + 1));
    sum / count as f64
}

/// Helper function to calculate the standard deviation of an iterator
fn stddev<I>(iter: I, mean: f64) -> f64
where
    I: IntoIterator<Item = f64>,
{
    let (sum_of_squares, count) = iter.into_iter().fold((0.0, 0), |(acc, cnt), val| {
        (acc + (val - mean).powi(2), cnt + 1)
    });
    (sum_of_squares / count as f64).sqrt()
}

fn test_noisey<F: Float, NoiseFn>(
    mut noise_fn:     NoiseFn, 
    num_samples:      usize, 
    stddev_threshold: f64
)
where
    NoiseFn: FnMut() -> F,
{
    let mut noise_values = Vec::new();

    for _ in 0..num_samples {
        let noise = noise_fn();
        noise_values.push(noise);
    }

    // Assuming `mean` and `stddev` are modified to work with f64 directly
    let mean = mean(noise_values.iter().cloned().filter_map(|f| f.to_f64()));
    let stddev = stddev(noise_values.iter().cloned().filter_map(|f| f.to_f64()), mean);
    assert!(
        stddev > stddev_threshold,
        "Standard deviation too low: {:.2}",
        stddev
    );
}

/// In the `test_correlated_noise_noisey` test, we generate
/// a sequence of 1000 correlated noise values and calculate
/// the mean and standard deviation of the sequence. 
///
/// We then check if the standard deviation is greater than
/// a certain threshold (0.1 in this case) to ensure the
/// noise is actually "noisy". 
///
/// You can add similar tests for the other noise functions
/// by adapting this test.
///
#[test]
fn test_correlated_noise_noisey() {
    let mut lastval = 0.0;
    let correlation = 0.5;
    let num_samples = 1000;
    test_noisey(|| {
        let noise = correlated_noise(lastval, correlation);
        lastval = noise;
        noise
    }, num_samples, 0.1);
}

#[test]
fn test_correlated_noise_mk2_noisey() {
    let mut lastval: f32 = 0.0;
    let correlation = 0.5;
    let num_samples = 1000;
    test_noisey(|| {
        let noise = correlated_noise_mk2(lastval, correlation) as f32;
        lastval = noise;
        noise
    }, num_samples, 0.1);
}

/*
#[test]
fn test_drift_noise_noisey() {
    let mut lastval: f32 = 0.0;
    let num_samples = 1000;
    test_noisey(|| {
        let noise = drift_noise(lastval) as f32;
        lastval = noise;
        noise
    }, num_samples, 0.1);
}
*/

#[test]
fn test_correlated_noise_o2_noisey() {
    let mut lastval: f32 = 0.0;
    let mut lastval2 = 0.0;
    let correlation = 0.5;
    let num_samples = 1000;
    test_noisey(|| {
        let noise = correlated_noise_o2(lastval, lastval2, correlation) as f32;
        lastval2 = lastval;
        lastval = noise;
        noise
    }, num_samples, 0.1);
}

#[test]
fn test_correlated_noise_o2mk2_noisey() {
    let mut lastval: f32 = 0.0;
    let mut lastval2 = 0.0;
    let correlation = 0.5;
    let num_samples = 1000;
    test_noisey(|| {
        let noise = correlated_noise_o2mk2(lastval, lastval2, correlation) as f32;
        lastval2 = lastval;
        lastval = noise;
        noise
    }, num_samples, 0.1);
}
