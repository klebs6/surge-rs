crate::ix!();

/// Convert an amplitude value to a linear value.
///
/// Input `x` should be non-negative.
///
#[inline] pub fn amp_to_linear(mut x: f32) -> f32 {
    x = x.max(0.0);
    x * x * x
}

/// Convert a linear value to an amplitude value.
///
#[inline] pub fn linear_to_amp(x: f32) -> f32 {
    limit_range(x, 0.0000000001, 1.0).powf(1.0 / 3.0) 
}

/// Convert an amplitude value to a decibel value.
///
/// The resulting decibel value will be clamped between
/// -192.0 and 96.0.
///
#[inline] pub fn amp_to_db(x: f32) -> f32 {
    let val = 6.0 * 3.0 * x.log2();
    limit_range( val, -192.0, 96.0 )
}

/// Convert a decibel value to an amplitude value.
///
/// The resulting amplitude value will be clamped between
/// 0.0 and 1.0.
///
#[inline] pub fn db_to_amp(x: f32) -> f32 {
    let base: f32 = 10.0 / 3.0;
    let exp: f32 = 0.05 * x;
    limit_range( base.powf(exp), 0.0, 1.0 )
}

/// Convert a decibel value to a scalable amplitude value.
///
/// The resulting value will be clamped between 0.0 and 1.0.
///
#[inline] pub fn db_to_scamp(input: f32)  -> f32 // ff rev2
{
   let v: f32 = 10.0_f32.powf( -0.05 * input);
   limit_range(v, 0.0, 1.0)
}

/// Convert a linear value to a decibel value.
///
#[inline] pub fn linear_to_db(input: f64) -> f64 
{
    20.0 * input.log10()
}

/// Convert a decibel value to a linear value.
///
#[inline] pub fn db_to_linear(input: f64) -> f64 
{
    10.0_f64.powf(0.05 * input)
}

/// Convert a timecent value to seconds.
///
#[inline] pub fn timecent_to_seconds(input: f32) -> f32 
{
    2.0_f32.powf(input / 1200.0)
}

/// Convert a seconds value to an envelope time value.
///
/// The resulting value will be clamped between 0.0 and 1.0.
///
#[inline] pub fn seconds_to_envtime(input: f32)  -> f32 // ff rev2
{
    let base: f32 = input / 30.0;
    let v: f32 = base.powf( 1.0 / 3.0 );
    limit_range(v, 0.0, 1.0)
}

/// Convert a timecent value to an envelope time value.
///
#[inline] pub fn timecent_to_envtime(input: f32) -> f32 
{
    //TODO: is this actually doing the right thing?
    // return seconds_to_envtime(timecent_to_seconds(in));
    input / 1200.0
}

/// Convert a 7-bit integer value to a bipolar float value.
///
pub fn int7_to_bipolar_float(x: i32) -> f32
{
    match x {
        _ if x > 64 => ((x - 64) as f32) * (1.0 / 63.0),
        _ if x < 64 => ((x - 64) as f32) * (1.0 / 64.0),
        _ => 0.0,
    }
}
