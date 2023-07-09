use surge_math::*;

use test::Bencher;

#[bench]
fn bench_rand01(b: &mut Bencher) {
    b.iter(|| rand01());
}

#[bench]
fn bench_rand11(b: &mut Bencher) {
    b.iter(|| rand11());
}

#[bench]
fn bench_correlated_noise(b: &mut Bencher) {
    let lastval = 0.0;
    let correlation = 0.5;
    b.iter(|| correlated_noise(lastval, correlation));
}

#[bench]
fn bench_correlated_noise_mk2(b: &mut Bencher) {
    let lastval = 0.0;
    let correlation = 0.5;
    b.iter(|| correlated_noise_mk2(lastval, correlation));
}

#[bench]
fn bench_drift_noise(b: &mut Bencher) {
    let lastval = 0.0;
    b.iter(|| drift_noise(lastval));
}

#[bench]
fn bench_correlated_noise_o2(b: &mut Bencher) {
    let lastval = 0.0;
    let lastval2 = 0.0;
    let correlation = 0.5;
    b.iter(|| correlated_noise_o2(lastval, lastval2, correlation));
}

#[bench]
fn bench_correlated_noise_o2mk2(b: &mut Bencher) {
    let lastval = 0.0;
    let lastval2 = 0.0;
    let correlation = 0.5;
    b.iter(|| correlated_noise_o2mk2(lastval, lastval2, correlation));
}
