// goal: test that we pass all frequencies equally in gain
//
use surge_math::*;
use surge_imports::*;

#[test] fn test_allpass() {

    use crate::*;

    let mut allpass = AllpassFilter::<8>::default();
    let mut signal  = Signal::white_noise(1024);

    let power_freq_tolerance = 0.01;
    let initial_power_freq   = signal.average_power_freq_domain();

    println!("initial power (freq): {:?}", initial_power_freq);

    for i in 0..10 {

        println!("iter {}", i);

        signal = Signal {
            data: signal.data.map(|x| allpass.process(*x))
        };

        let power_freq  = signal.average_power_freq_domain();

        println!("allpassed power (freq): {:?}", power_freq);

        assert!((initial_power_freq - power_freq).abs() < power_freq_tolerance);
    }
}

#[test]
fn test_default_initialization() {
    let filter: AllpassFilter<8> = Default::default();
    assert_eq!(filter.get_wpos(), 0);
    assert_eq!(filter.get_a(), 0.3);
    assert_eq!(filter.get_buffer(), [0.0; 8]);
}

#[test]
fn test_update_wpos() {
    let mut filter: AllpassFilter<8> = Default::default();
    assert_eq!(filter.get_wpos(), 0);

    filter.update_wpos();
    assert_eq!(filter.get_wpos(), 1);

    filter.set_wpos(7);
    filter.update_wpos();
    assert_eq!(filter.get_wpos(), 0); // wrap around
}


#[test]
fn test_process() {
    let mut filter: AllpassFilter<8> = Default::default();

    // Process a sample through the filter
    let x = 1.0;
    let y = filter.process(x);

    // Check the output
    assert_eq!(y, 0.0 + x * filter.get_a());

    // Check the buffer state
    assert_eq!(filter.get_buffer_at(filter.get_wpos()), x);

    // Process another sample
    let x2 = 0.5;
    let y2 = filter.process(x2);
    let expected_y2 = filter.get_buffer_at(filter.get_wpos()) * filter.get_a();
    println!("Expected y2: {}, Actual y2: {}", expected_y2, y2);
    assert_eq!(y2, expected_y2);

    // Check the buffer state
    let expected_buffer_value = x2 + y2 * -filter.get_a();
    println!("Expected buffer value: {}, Actual buffer value: {}", expected_buffer_value, filter.get_buffer_at(filter.get_wpos()));
    assert_eq!(filter.get_buffer_at(filter.get_wpos()), expected_buffer_value);
}

#[test]
fn test_set_a() {
    let mut filter: AllpassFilter<8> = Default::default();
    filter.set_a(0.5);
    assert_eq!(filter.get_a(), 0.5);
}

#[test]
fn test_frequency_response() {
    const N: usize = 1024;
    let mut filter: AllpassFilter<N> = Default::default();

    // Generate white noise
    let mut rng = rand::thread_rng();
    let mut white_noise = vec![0.0; N];
    for sample in white_noise.iter_mut() {
        *sample = rng.gen::<f64>() * 2.0 - 1.0; // Random values between -1.0 and 1.0
    }

    // Process white noise through the filter
    let mut filtered_signal = vec![0.0; N];
    for i in 0..N {
        filtered_signal[i] = filter.process(white_noise[i]);
    }

    // Perform FFT on the filtered signal
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(N);
    let mut spectrum = filtered_signal.iter().map(|&x| Complex::new(x, 0.0)).collect::<Vec<_>>();
    fft.process(&mut spectrum);

    // Compute the magnitude of the frequency response
    let magnitude: Vec<f64> = spectrum.iter().map(|c| c.norm()).collect();

    // Verify that the magnitude response is approximately flat
    let avg_magnitude: f64 = magnitude.iter().copied().sum::<f64>() / magnitude.len() as f64;
    let tolerance = 0.1; // Allowable deviation from flat response

    for (i, mag) in magnitude.iter().enumerate() {
        if (mag - avg_magnitude).abs() >= tolerance {
            println!("Frequency bin {}: Magnitude {}, Average Magnitude {}", i, mag, avg_magnitude);
            assert!((mag - avg_magnitude).abs() < tolerance, "Magnitude response is not flat");
        }
    }
}
