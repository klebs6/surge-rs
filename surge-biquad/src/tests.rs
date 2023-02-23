crate::ix!();

/// Here is an example test case that generates a sine wave with frequency 440 Hz and amplitude
/// 0.5, passes it through the biquad filter, and checks that the output has the same frequency and
/// amplitude:
/// 
/// In this test case, we generate a sine wave with a frequency of 440 Hz and an amplitude of 0.5,
/// which is then passed through the biquad filter. We then check that the output has a frequency
/// close to 440 Hz and an amplitude close to 0.5. The tolerances used in the `assert` statements
/// may need to be adjusted depending on the expected accuracy of the filter.
///
#[test]
fn test_biquad_filter() {

    let sample_rate = 44100.0;
    let frequency = 440.0;
    let amplitude = 0.5;
    let duration = 1.0;
    let num_samples = (duration * sample_rate) as usize;

    // Generate input sine wave
    let input = (0..num_samples)
        .map(|i| amplitude * f32::sin(2.0 * std::f32::consts::PI * frequency * i as f32 / sample_rate))
        .collect::<Vec<f32>>();

    // Create biquad filter with cutoff frequency at 500 Hz
    let srunit = SampleRateHandle::new(sample_rate);
    let tuner = TunerHandle::new(&srunit);
    let tables = TablesHandle::new(&srunit);
    let mut biquad = BiquadFilter::new(&tuner, &tables, &srunit);
    biquad.coeff_instantize();
    biquad.coeff_process();

    // Filter input signal
    let mut output = vec![0.0; num_samples];
    unsafe {
        biquad.process_block_mono(input.as_ptr(), output.as_mut_ptr(), num_samples as i32);
    }

    // Check that output has the same frequency and amplitude as input
    let output_frequency = surge_math::detect_frequency(&output, sample_rate);

    let output_amplitude = output.iter().map(|x| x.abs()).sum::<f32>() / num_samples as f32;

    assert!((output_frequency - frequency).abs() < 1.0, "Output frequency is incorrect");
    assert!((output_amplitude - amplitude).abs() < 0.05, "Output amplitude is incorrect");
}
