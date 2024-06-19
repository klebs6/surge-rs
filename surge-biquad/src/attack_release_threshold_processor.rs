crate::ix!();

/// The `AttackReleaseThresholdProcessor` trait is
/// a collection of methods that allows the user
/// to set and get processing parameters for audio
/// signals. 
///
/// This includes parameters such as gain,
/// threshold, attack, and release, which are
/// commonly used in dynamic range compression and
/// other audio processing algorithms.
/// 
/// This trait is designed to be implemented by
/// types that perform audio processing, such as
/// compressors or limiters. 
///
/// By implementing this trait, a type can provide
/// a standard interface for setting and getting
/// these parameters.
///
pub trait AttackReleaseThresholdProcessor: 
SetGain 
+ SetThreshold 
+ SetAttack 
+ SetRelease 
+ GetGain 
+ GetThreshold 
+ GetAttack 
+ GetRelease  { }

/// The `SetGain` trait provides a method for
/// setting the gain of an audio signal. 
///
/// The gain is a multiplier that is applied to
/// the audio signal to increase or decrease its
/// level. The gain value is expressed as
/// a floating-point number, where a value of 1.0
/// represents unity gain (no change in level),
/// values greater than
/// 1.0 represent gain increases, and values less
/// than 1.0 represent gain reductions.
///
pub trait SetGain {

    fn set_gain(&mut self, gain: f32);
}

/// The `SetThreshold` trait provides a method for
/// setting the threshold of an audio signal. 
///
/// The threshold is a level above which gain
/// reduction is applied to the audio signal. 
///
/// When the audio signal exceeds the threshold
/// level, its level is reduced by a certain
/// amount. 
///
/// The threshold value is expressed as
/// a floating-point number, where a higher value
/// represents a higher threshold level.
///
pub trait SetThreshold {

    fn set_threshold(&mut self, threshold: f32);
}

/// The `SetAttack` trait provides a method for
/// setting the attack time of an audio signal. 
///
/// The attack time is the time it takes for the
/// gain reduction to be applied to the audio
/// signal after it exceeds the threshold level. 
///
/// A shorter attack time results in a faster gain
/// reduction, while a longer attack time results
/// in a slower gain reduction. 
///
/// The attack time value is expressed as
/// a floating-point number, where a lower value
/// represents a shorter attack time.
///
pub trait SetAttack {
    fn set_attack(&mut self, attack: f32);
}

/// The `SetRelease` trait provides a method for
/// setting the release time of an audio signal. 
///
/// The release time is the time it takes for the
/// gain reduction to stop being applied to the
/// audio signal after it falls below the
/// threshold level. 
///
/// A shorter release time results in a faster
/// gain recovery, while a longer release time
/// results in a slower gain recovery. 
///
/// The release time value is expressed as
/// a floating-point number, where a lower value
/// represents a shorter release time.
///
pub trait SetRelease {
    fn set_release(&mut self, release: f32);
}

/// The `GetGain` trait provides a method for
/// getting the current gain value of an audio
/// signal.
///
pub trait GetGain {
    fn get_gain(&self) -> f32;
}

/// The `GetThreshold` trait provides a method for
/// getting the current threshold value of an
/// audio signal.
///
pub trait GetThreshold {
    fn get_threshold(&self) -> f32;
}

/// The `GetAttack` trait provides a method for
/// getting the current attack time value of an
/// audio signal.
///
pub trait GetAttack {
    fn get_attack(&self) -> f32;
}

/// The `GetRelease` trait provides a method for
/// getting the current release time value of an
/// audio signal.
///
pub trait GetRelease {
    fn get_release(&self) -> f32;
}
