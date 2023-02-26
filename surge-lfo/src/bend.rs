/*!
  | The `lfo_bend*` functions are used to apply
  | a "bend" effect to an input value `x` based on
  | a deformation factor `deform`.
  |
  | The degree of bending is controlled by the
  | `deform` parameter, with higher values
  | resulting in more pronounced bending.
  |
  | Here's a brief description of the shape of the
  | bending for each function:
  |
  | - `lfo_bend1`: This function adds a sine wave
  | to the input `x`, with the amplitude of the
  | sine wave determined by the `deform`
  | parameter.
  |
  |   The sine wave is centered around `x + 0.25`
  |   and has a period of 1.0. The result is then
  |   shifted back down by 0.25 to restore the
  |   original range of `x`.
  |
  | - `lfo_bend2`: This function also adds a sine
  | wave to the input `x`, but without the initial
  | shift and restore of `x` as in `lfo_bend1`.
  |
  |   The amplitude of the sine wave is again
  |   determined by the `deform` parameter.
  |
  | - `lfo_bend3`: This function applies
  | a polynomial distortion to the input `x`,
  | again based on the `deform` parameter.
  |
  |   The polynomial is a quadratic function of the
  |   form `a*x^2 - a*x + x`, where `a` is
  |   determined by the `deform` parameter.
  |
  |   The polynomial is applied twice to `x` to
  |   increase the amount of distortion.
  |
  | In general, the `deform` parameter controls
  | the amount of bending or distortion applied to
  | the input `x`.
  |
  | For example, a larger value of `deform` will
  | result in a more pronounced distortion in the
  | case of `lfo_bend3`.
  |
  */

crate::ix!();

/// This function first adds 0.25 to the input
/// value `x`, then applies a sine function to it
/// with a frequency of `x * 2.0 * PI_32`. 
///
/// The sine function is multiplied by a scaling
/// factor `a` and divided by `2.0 * PI_32` to
/// keep the output within the range of -1 to 1. 
///
/// Finally, 0.25 is subtracted from the output
/// value and returned.
///
#[inline] pub fn lfo_bend1(mut x: f32, deform: f32) -> f32 {

    let a: f32 = 0.5 * deform;

    x += 0.25;

    x += a * (x * 2.0 * PI_32).sin() / ((2.0 * PI_32) as f32);

    x -= 0.25;

    x
}

/// This function is similar to `lfo_bend1`, but
/// without the offset of 0.25 added and
/// subtracted from the input and output values,
/// respectively.
///
#[inline] pub fn lfo_bend2(mut x: f32, deform: f32) -> f32 {

    let a: f32 = 0.5 * deform;

    x += a * (x * 2.0 * PI_32).sin() / ((2.0 * PI_32) as f32);

    x
}

/// This function applies a more complex bending
/// effect by using a quadratic equation. 
///
/// The input value `x` is first multiplied by
/// a scaling factor `a`, then squared and
/// multiplied by -1. 
///
/// This result is added to `x` and the scaling
/// factor `a`. 
///
/// The process is repeated once more with the new
/// output value, resulting in a curved waveform.
///
#[inline] pub fn lfo_bend3(mut x: f32, deform: f32) -> f32 {

    let a: f32 = 0.5 * deform;

    x = x - a * x * x + a;

    x = x - a * x * x + a; 

    x
}

/// The `bend1`, `bend2`, and `bend3` methods in
/// the `Lfo` struct simply call their respective
/// `lfo_bend*` functions with the `deform`
/// parameter retrieved from the struct's `params`
/// field.
///
impl Lfo {

    pub fn deform(&self) -> f32 {
        pvali![self.params[LfoParam::Deform]] as f32
    }

    pub fn bend1(&mut self, x: f32) -> f32 {

        let deform = self.deform();

        lfo_bend1(x, deform)
    }

    pub fn bend2(&mut self, x: f32) -> f32 {

        let deform = self.deform();

        lfo_bend2(x, deform)
    }

    pub fn bend3(&mut self, x: f32) -> f32 {

        let deform = self.deform();

        lfo_bend3(x, deform)
    }
}
