crate::ix!();

/// `ProcessSample` trait is used to define
/// a process for a single audio sample. 
///
pub trait ProcessSample {

    /// The `process_sample` method takes an input
    /// `f32` sample and returns a processed
    /// output `f32` sample.
    ///
    fn process_sample(
        &mut self, 
        input: f32) -> f32;
}

/// `ProcessSampleStereo` trait is used to define
/// a process for a pair of stereo audio samples. 
///
pub trait ProcessSampleStereo {

    /// The `process_sample_stereo` method takes
    /// two input `f32` samples, one for the left
    /// channel and the other for the right
    /// channel, and outputs two processed output
    /// `f32` samples, one for the left channel
    /// and the other for the right channel.
    ///
    fn process_sample_stereo(
        &mut self, 
        l: f32, 
        r: f32, 
        l_out: &mut f32, 
        r_out: &mut f32);
}

/// `ProcessSampleNolag` trait is used to define
/// a process that operates directly on the audio
/// samples, without any delay or lag. 
///
pub trait ProcessSampleNolag {

    /// The `process_sample_nolag` method takes two
    /// mutable input `f32` samples, one for the left
    /// channel and the other for the right channel,
    /// and processes them in place.
    ///
    fn process_sample_nolag(
        &mut self, 
        l: &mut f32, 
        r: &mut f32);
}

/// `ProcessSampleStereoNolag` trait is similar to
/// `ProcessSampleNolag`, but it processes stereo
/// audio samples instead of individual audio
/// samples. 
///
pub trait ProcessSampleStereoNolag {

    /// The `process_sample_stereo_nolag` method takes
    /// two mutable input `f32` samples, one for the
    /// left channel and the other for the right
    /// channel, and two mutable output `f32` samples,
    /// one for the left channel and the other for the
    /// right channel, and processes them in place.
    ///
    fn process_sample_stereo_nolag(
        &mut self, 
        l: &mut f32, 
        r: &mut f32, 
        l_out: &mut f32, 
        r_out: &mut f32);
}

/// `ProcessSampleNolagNoinput` trait is similar
/// to `ProcessSampleNolag`, but it doesn't take
/// any input sample. 
///
pub trait ProcessSampleNolagNoinput {

    /// The `process_sample_nolag_noinput` method
    /// takes two mutable output `f32` samples,
    /// one for the left channel and the other for
    /// the right channel, and processes them
    /// directly.
    ///
    fn process_sample_nolag_noinput(
        &mut self, 
        l_out: &mut f32, 
        r_out: &mut f32);
}
