crate::ix!();

/// Computes the phase increment for an LFO based on the number of samples and the LFO rate.
///
/// # Arguments
///
/// * `samples` - The number of samples in the audio buffer.
/// * `rate` - The rate of the LFO, expressed as a value between 0.0 and 1.0.
///
/// # Returns
///
/// A phase increment value for the LFO.
///
#[inline] pub fn lfo_phaseincrement(samples: i32 , mut rate: f32 ) -> f32 
{
    // Adjust the rate to a range of -1.0 to 0.0
   rate = 1.0 - rate;

    // Compute the phase increment using the adjusted rate and the number of samples
   samples as f32 * ENV_PHASEMULTI / (1.0 + LFO_RANGE * rate * rate * rate)
}

#[cfg(test)]
mod tests {

    use super::*;

    /// This test uses different values for the
    /// `samples` and `rate` parameters and compares
    /// the computed result with an expected result
    /// using the `assert!` macro. 
    ///
    /// The `abs()` method is used to calculate the
    /// absolute difference between the expected and
    /// computed results. 
    ///
    /// If the difference is less than a very small
    /// tolerance value (`0.000000001` in this case),
    /// the test passes.
    ///
    #[test]
    fn test_lfo_phaseincrement() {
        // Test case 1
        let samples = 44100;
        let rate = 0.5;
        let expected = 0.000090702947;
        let result = lfo_phaseincrement(samples, rate);
        assert!((result - expected).abs() < 0.000000001);

        // Test case 2
        let samples = 48000;
        let rate = 0.8;
        let expected = 0.000031370135;
        let result = lfo_phaseincrement(samples, rate);
        assert!((result - expected).abs() < 0.000000001);

        // Test case 3
        let samples = 96000;
        let rate = 0.2;
        let expected = 0.000234640556;
        let result = lfo_phaseincrement(samples, rate);
        assert!((result - expected).abs() < 0.000000001);
    }
}
