
/// This constant defines a multiplier that is used to convert time values into phase values for an
/// envelope generator. 
///
/// Envelope generators are used to shape the volume or other aspects of a sound over time. 
///
/// By multiplying time values by this constant, we get a phase value that can be used to look up
/// the envelope value at a particular point in time.
///
pub const ENV_PHASEMULTI: f32 = 1000.0 / 44100.0;

/// This constant defines the range of values that an LFO (low-frequency oscillator) can output. 
///
/// An LFO is a type of oscillator that generates a waveform with a frequency typically below the
/// audible range. 
///
/// The output of an LFO is often used to modulate other parameters of a sound, such as its volume
/// or pitch. 
///
/// In this case, the range of the LFO is set to 1000, meaning that its output can vary between
/// -1000 and +1000.
///
pub const LFO_RANGE:      f32 = 1000.0;
