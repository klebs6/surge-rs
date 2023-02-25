crate::ix!();

/// Overall, the `NonlinearStatesFilter` struct
/// appears to implement a flexible and efficient
/// digital filter with nonlinear saturation,
/// suitable for processing audio signals in
/// real-time applications.
///
/// To do this, it leverages an auxiliary
/// structure called QuadFilterUnitState during
/// its `process_quad` call.
///
/// The QuadFilterUnitState appears to store the
/// internal state of the filter. It contains two
/// registers (`z1` and `z2`) and an array of
/// filter coefficients (`coeff`). 
///
/// The filter coefficients and internal state are
/// updated dynamically during the filtering
/// process.
/// 
/// The `NonlinearStatesFilter` struct appears to
/// use a second-order IIR filter structure with
/// direct form II transposed. 
///
/// The filter coefficients are stored in the
/// `coeff` array and correspond to the standard
/// transfer function coefficients (b0, b1, b2,
/// a1, a2) of the IIR filter. 
///
/// The filter is implemented as a series of
/// `stages`, where each stage corresponds to one
/// set of coefficients.
/// 
/// The filter implementation in `process_quad`
/// uses SIMD instructions (`_mm` functions) to
/// perform efficient vectorized processing of the
/// audio samples. The filtering is performed
/// in-place on the `input` vector, and the output
/// is returned in the same vector.
/// 
/// The original algorithm can be found at the
/// following link:
/// https://ccrma.stanford.edu/~jatin/ComplexNonlinearities/NLBiquad.html.
/// 
/// The implementation in this codebase uses
/// coefficients calculated from the Audio EQ
/// Cookbook
/// (https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html),
///
#[derive(Derivative)] #[derivative(Debug)]
pub struct NonlinearStatesFilter {

    /// The TunerHandle appears to be used to define
    /// the tuning system used by the filter, while
    pub tuner:  TunerHandle,

    /// the SampleRateHandle defines the sampling rate
    /// of the filter. 
    pub srunit: SampleRateHandle,

    /// The filter can be in one of several types,
    /// such as LowPass, HighPass, Notch, etc.
    ///
    pub ty:     NLSFType,
}

impl NonlinearStatesFilter {

    /// Returns the clamped frequency for the given
    /// pitch, based on the filter's tuning system.
    ///
    /// The frequency is calculated using the pitch
    /// and the tuning system defined in the
    /// filter's `tuner` field.
    ///
    /// The resulting frequency is then clamped to
    /// a range between 5.0 and 30% of the filter's
    /// sampling rate, as defined by the `srunit`
    /// field.
    ///
    /// # Arguments
    ///
    /// * `pitch` - The pitch in MIDI note number format.
    ///
    /// # Returns
    ///
    /// The clamped frequency in Hz.
    ///
    pub fn clamped_frequency(&self, pitch: f32) -> f32
    {
        let freq = self.tuner.n2p::<f32,true>(pitch + 69.0) * (MIDI_0_FREQ as f32);

        limit_range(freq, 5.0, self.srunit.samplerate_os() * 0.3)
    }
}
