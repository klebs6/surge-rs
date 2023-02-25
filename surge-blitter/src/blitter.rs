crate::ix!();

/// A blitter, short for "block transfer engine",
/// is a type of hardware accelerator used for
/// performing high-speed memory operations. It
/// was originally developed for early graphics
/// systems, but has since been used for a wide
/// range of applications including audio
/// processing.
/// 
/// In graphics, a blitter is used to move blocks
/// of pixels between different areas of the
/// screen memory. This can be used for tasks like
/// scrolling, animation, and sprite
/// rendering. The blitter typically operates
/// independently of the CPU, allowing it to
/// perform these operations in parallel with
/// other tasks.
/// 
/// In audio, a blitter can be used for tasks like
/// sample rate conversion, filtering, and
/// mixing. It works by loading blocks of audio
/// data into a buffer, performing the necessary
/// processing, and then transferring the results
/// back to main memory. This can significantly
/// reduce the CPU overhead required for audio
/// processing, allowing for more complex audio
/// effects and synthesis to be performed in
/// real-time.
///
/// This struct contains various fields that
/// describe the blitter's internal state,
/// including buffers for audio output, oscillator
/// coefficients, pitch multiplier,
/// synchronization state, and LFO drift. 
///
#[derive(Debug)]
pub struct AbstractBlitter {

    /// A buffer for the left channel's oscillator
    /// output.
    ///
    pub oscbuffer_l:         Align16<A1d::<f32>>,

    /// A buffer for the right channel's
    /// oscillator output.
    ///
    pub oscbuffer_r:         Align16<A1d::<f32>>,

    /// A buffer for the DC output.
    ///
    pub dcbuffer:            Align16<A1d::<f32>>,

    /// The current output of the left channel's
    /// first oscillator.
    ///
    pub osc_out_l:           __m128,

    /// The current output of the left channel's
    /// second oscillator
    ///
    pub osc_out_2l:          __m128,

    /// The current output of the right channel's
    /// first oscillator.
    ///
    pub osc_out_r:           __m128,

    /// The current output of the right channel's
    /// second oscillator
    ///
    pub osc_out_2r:          __m128,

    /// The high-pass filter's integrator
    /// coefficient.
    ///
    pub integrator_hpf:      f32,

    /// The pitch multiplier for the oscillator.
    ///
    pub pitchmult:           f32,

    /// The inverse of the pitch multiplier for
    /// the oscillator.
    ///
    pub pitchmult_inv:       f32,

    /// The number of unison voices.
    ///
    pub n_unison:            i32,

    /// The buffer position.
    ///
    pub bufpos:              i32,

    /// The output attenuation.
    ///
    pub out_attenuation:     f32,

    /// The inverse of the output attenuation.
    ///
    pub out_attenuation_inv: f32,

    /// The detune bias.
    ///
    pub detune_bias:         f32,

    /// The detune offset.
    ///
    pub detune_offset:       f32,

    /// The state of the oscillator.
    ///
    pub oscstate:            A1d::<f32>,

    /// The state of the synchronization.
    ///
    pub syncstate:           A1d::<f32>,

    /// The rate of the oscillator.
    ///
    pub rate:                A1d::<f32>,

    /// The drift LFO.
    ///
    pub driftlfo:            A1d::<f32>,

    /// The second drift LFO.
    ///
    pub driftlfo2:           A1d::<f32>,

    /// The left channel's pan.
    ///
    pub pan_l:               A1d::<f32>,

    /// The right channel's pan.
    ///
    pub pan_r:               A1d::<f32>,

    /// The state of the oscillator.
    ///
    pub state:               A1d::<i32>,
}
