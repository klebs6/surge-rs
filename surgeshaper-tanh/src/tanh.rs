// hello! could you please show me how the
// following function works? could you would you
// please write me some tests?
// 
// Sure, I'd be happy to explain the `TanhShaper`
// function.
// 
// `TanhShaper` is a struct that implements the
// `Waveshaper` trait, which provides a single
// function `shape` that takes an input `__m128`
// value (a 128-bit SIMD register on x86_64
// processors) and a `drive` `__m128` value and
// returns another `__m128` value.
// 
// The `shape` function first sets up two `__m128`
// constants `m9` and `m27`, which are used in the
// computation of the output value.
// 
// Next, it multiplies the input and drive values
// together using `_mm_mul_ps`, which performs
// a packed multiply of two `__m128` values.
// 
// It then computes `x * x` using `_mm_mul_ps`,
// and multiplies the result by `9.0` using
// `_mm_mul_ps` and `m9`. The result is then added
// to `27.0` using `_mm_add_ps` and `m27` to get
// the denominator of the output value.
// 
// The numerator of the output value is then
// computed by multiplying `x` by `27.0` using
// `_mm_mul_ps`, adding `x * x` using
// `_mm_add_ps`, and multiplying the result by `x`
// using `_mm_mul_ps`.
// 
// The output value is then computed as the
// product of the numerator and the reciprocal of
// the denominator, computed using `_mm_rcp_ps`.
// 
// Finally, the output value is passed through the
// `clip_bipolar` function to ensure it is within
// the range of -1 to 1.
// 
// Here's an example of how you could test the
// `TanhShaper` function:
// 
// ```
// #[test]
// fn test_tanh_shaper() {
//     use std::f32::consts::PI;
// 
//     let shaper = TanhShaper::default();
//     let input: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
//     let drive: [f32; 4] = [0.5, 0.6, 0.7, 0.8];
//     let mut output: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
// 
//     unsafe {
//         shaper.shape(
//             _mm_loadu_ps(input.as_ptr()),
//             _mm_loadu_ps(drive.as_ptr())
//         ).store(output.as_mut_ptr(), 1);
//     }
// 
//     for i in 0..4 {
//         assert!((output[i] - (input[i] * (27.0 + input[i] * input[i] * 9.0) / (27.0 + input[i] * input[i] * 9.0)).tanh()).abs() < 1e-6);
//     }
// }
// ```
// 
// In this test, we first create a `TanhShaper`
// object, and then create arrays of input and
// drive values. We then create an array to hold
// the output values.
// 
// We call the `shape` function using
// `_mm_loadu_ps` to load the input and drive
// values into `__m128` registers, and then call
// `store` on the output `__m128` register to
// write the result back to the output array.
// 
// Finally, we iterate over the output array and
// compare each element to the expected result,
// computed using the formula `(x * (27 + x * x)
// / (27 + 9 * x * x)).tanh()`. We use the
// `assert!` macro to ensure that the difference
// between the expected and actual results is
// within a small tolerance.
// 
// 16> what do you think this struct is designed
// to do?
// 
// Based on the name `TanhShaper` and the fact
// that it implements the `Waveshaper` trait,
// I would assume that this struct is designed to
// shape audio waveforms using a hyperbolic
// tangent (tanh) function. The `shape` method
// takes an input audio signal and a drive level
// as `__m128` vectors (presumably for SIMD
// processing on x86_64 architecture) and applies
// the tanh function to the input signal with
// a specific equation. It then clips the output
// to ensure it stays within the bipolar range of
// [-1, 1].
// 
// 17> do you think it is well designed? is there
// anything you'd do to improve it?
// 
// The `TanhShaper` struct seems to be
// well-designed for its intended purpose of
// applying a hyperbolic tangent waveshaping
// algorithm to an audio signal.
// 
// One possible improvement could be to make the
// `shape` function generic over the input type,
// so that it could accept different types of
// input vectors (such as `__m256` for AVX2 or
// `__m512` for AVX-512), depending on the target
// architecture. This would make the code more
// flexible and future-proof, as it would work on
// a wider range of hardware.
// 
// Another possible improvement could be to
// provide a way to customize the amount of drive
// applied to the input signal. This could be done
// by adding a parameter to the `shape` function
// that controls the amount of drive, allowing the
// user to fine-tune the sound.
// 
// 18> in the surrounding mathematical fields, is
// there anything which can be used to add nuance
// to the design, or improve the numerical
// characteristics of the signal processing?
// 
// In signal processing, there are many techniques
// and methods that can be used to add nuance and
// improve the numerical characteristics of the
// signal processing. For example:
// 
// 1. Filter design: The choice of filter type,
// order, and design method can have a significant
// impact on the numerical characteristics of the
// signal processing. There are many different
// types of filters, such as FIR and IIR filters,
// and many different design methods, such as
// windowing and optimization techniques, that can
// be used to design filters with different
// properties.
// 
// 2. Nonlinear processing: In addition to linear
// processing, such as filtering and equalization,
// nonlinear processing techniques can also be
// used to shape the signal in various ways. For
// example, distortion and saturation can be used
// to add harmonics to a signal, which can be used
// to create different timbres or add warmth to
// a sound.
// 
// 3. Spectral analysis: Spectral analysis
// techniques, such as Fourier analysis and
// wavelet analysis, can be used to analyze the
// spectral content of a signal and identify
// specific frequencies or frequency bands of
// interest. This can be useful for identifying
// and removing unwanted noise or interference, or
// for extracting specific features from a signal,
// such as pitch or formants.
// 
// 4. Statistical signal processing: Statistical
// techniques, such as Bayesian inference and
// maximum likelihood estimation, can be used to
// model the underlying statistical properties of
// a signal and extract useful information from
// noisy or uncertain data.
// 
// Overall, the choice of techniques and methods
// used in signal processing will depend on the
// specific application and the desired
// outcome. It is important to carefully consider
// the trade-offs between complexity,
// computational resources, and performance when
// designing signal processing systems.
// 
// 19> i'm specifically talking about for this
// TanhShaper
// 
// Yes, there are many techniques from numerical
// analysis and signal processing that can be used
// to improve the performance of a waveshaper like
// TanhShaper. Here are a few examples:
// 
// 1. Optimization techniques: The current
// implementation of TanhShaper uses a polynomial
// approximation to the hyperbolic tangent
// function to shape the input signal. While this
// is a reasonable approach, it may not be the
// most efficient or accurate way to achieve the
// desired shaping. Optimization techniques such
// as gradient descent or genetic algorithms could
// be used to find the best set of coefficients
// for the approximation.
// 
// 2. Nonlinear signal processing: Waveshaping is
// a form of nonlinear signal processing, and
// there are many other techniques from this field
// that could be used to further shape the signal
// in interesting ways. For example, one could use
// a soft clipping function instead of a hard
// clipping function to achieve a more subtle
// distortion. Alternatively, one could use
// a wavelet transform to extract different
// frequency bands from the input signal, and then
// apply different shaping functions to each band
// separately.
// 
// 3. Psychoacoustics: Another area that could be
// leveraged to improve the performance of
// TanhShaper is psychoacoustics. This field
// studies the relationship between physical sound
// waves and the subjective experience of sound by
// humans. By taking into account how the human
// ear perceives different frequencies and
// amplitudes, it may be possible to design a more
// effective shaping function that produces a more
// pleasing or musical sound. For example, one
// could apply different shaping functions to
// different frequency bands, based on their
// perceptual importance to the listener.
// 
// 20> could you show me how to implement these,
// 1 thru 3 in rust?
// 
// Sure, here are some possible implementations
// for the requested functions:
// 
// 1. Rectified Linear Unit (ReLU) activation
// function:
// 
// ```rust
// fn relu(x: f32) -> f32 {
//     if x > 0.0 {
//         x
//     } else {
//         0.0
//     }
// }
// ```
// 
// 2. Sigmoid activation function:
// 
// ```rust
// fn sigmoid(x: f32) -> f32 {
//     1.0 / (1.0 + (-x).exp())
// }
// ```
// 
// 3. Softplus activation function:
// 
// ```rust
// fn softplus(x: f32) -> f32 {
//     (1.0 + x.exp()).ln_1p()
// }
// ```
// 
// These are just example implementations, and
// there are many ways to implement these
// functions in Rust. You can also use Rust's
// built-in math functions or third-party
// libraries to implement these functions,
// depending on your specific needs.
// 
// 21> cool! can you apply them to the TanhShaper?
// 
// Sure, here's an example implementation of the
// TanhShaper using double precision floating
// point arithmetic and applying the suggested
// improvements:
// 
// ```rust
// #[derive(Default, Debug)]
// pub struct TanhShaper {
//     // Tunable parameter for shaping curve steepness
//     alpha: f64,
// }
// 
// impl TanhShaper {
//     pub fn new(alpha: f64) -> Self {
//         Self { alpha }
//     }
// }
// 
// impl Waveshaper for TanhShaper {
//     #[cfg(target_arch = "x86_64")]
//     fn shape(&self, input: __m128, drive: __m128) -> __m128 {
//         // Use double precision for computations
//         let x: f64 = input.extract(0) as f64 * self.alpha * drive.extract(0) as f64;
//         let y: f64 = x * (27.0 + x * x) / (27.0 + 9.0 * x * x);
//         // Clamp to range [-1, 1]
//         let y = if y > 1.0 {
//             1.0
//         } else if y < -1.0 {
//             -1.0
//         } else {
//             y
//         };
//         // Convert back to single precision and return as __m128
//         let y: f32 = y as f32;
//         unsafe { _mm_set_ps1(y) }
//     }
// }
// ```
// 
// Here, the `alpha` parameter is used to adjust
// the steepness of the shaping curve. Double
// precision floating point arithmetic is used for
// the computations, which should improve the
// numerical stability of the function. The output
// is clamped to the range [-1, 1] to avoid
// numerical issues with very large or very small
// output values. Finally, the result is converted
// back to single precision and returned as
// a `__m128` vector.
// 
// 22> cool! what can we do here with softplus,
// sigmoid, and relu?
// 
// Softplus, sigmoid, and ReLU are all activation
// functions commonly used in neural networks.
// 
// - Softplus is defined as f(x) = log(1
// + e^x). It is a smooth, monotonically
// increasing function that is similar to the ReLU
// function, but has a smooth transition as
// x approaches zero. Softplus can be useful for
// output layers in neural networks because it
// maps inputs to a positive range.
// 
// - Sigmoid is defined as f(x) = 1 / (1
// + e^-x). It is a smooth, S-shaped function that
// maps inputs to the range [0, 1]. Sigmoid
// functions are commonly used as activation
// functions in the output layer of binary
// classification problems, where the goal is to
// predict a binary outcome.
// 
// - ReLU (Rectified Linear Unit) is defined as
// f(x) = max(0, x). It is a non-linear function
// that maps negative inputs to zero, and positive
// inputs to themselves. ReLU is a commonly used
// activation function in the hidden layers of
// neural networks because it has several
// desirable properties, including being
// computationally efficient and avoiding the
// vanishing gradient problem.
// 
// In the context of the TanhShaper, it may be
// possible to use one of these activation
// functions to shape the input signal in
// a different way than the current
// implementation. However, it would require
// a different approach to the implementation than
// what is currently used in the TanhShaper, as
// each of these activation functions has
// a different mathematical definition than the
// tanh function used in the current
// implementation.
// 
// 23> could you show me how to program this
// approach?
// 
// Sure, here's an example implementation of using
// the softplus, sigmoid, and ReLU functions to
// shape an input signal using Rust:
// 
// ```rust
// use std::f32::consts::E;
// 
// // Define softplus function
// fn softplus(x: f32, threshold: f32, slope: f32) -> f32 {
//     (x - threshold).max(0.0).ln_1p() / slope + threshold
// }
// 
// // Define sigmoid function
// fn sigmoid(x: f32, threshold: f32, slope: f32) -> f32 {
//     1.0 / (1.0 + E.powf(-slope * (x - threshold)))
// }
// 
// // Define ReLU function
// fn relu(x: f32, threshold: f32, slope: f32) -> f32 {
//     (x - threshold).max(0.0) * slope + threshold
// }
// 
// // Define a struct for the shaper
// pub struct ActivationShaper {
//     threshold: f32,
//     slope: f32,
//     activation_fn: fn(f32, f32, f32) -> f32,
// }
// 
// impl ActivationShaper {
//     // Create a new activation shaper
//     pub fn new(threshold: f32, slope: f32, activation_fn: fn(f32, f32, f32) -> f32) -> ActivationShaper {
//         ActivationShaper {
//             threshold,
//             slope,
//             activation_fn,
//         }
//     }
// 
//     // Shape a single input sample
//     pub fn shape_sample(&self, input: f32) -> f32 {
//         (self.activation_fn)(input, self.threshold, self.slope)
//     }
// 
//     // Shape a buffer of input samples in place
//     pub fn shape_buffer_inplace(&self, buffer: &mut [f32]) {
//         for sample in buffer.iter_mut() {
//             *sample = self.shape_sample(*sample);
//         }
//     }
// }
// ```
// 
// With this implementation, you can create an
// `ActivationShaper` instance by specifying the
// threshold and slope values, as well as the
// activation function to use (`softplus`,
// `sigmoid`, or `relu`). You can then use the
// `shape_sample` method to shape a single input
// sample, or the `shape_buffer_inplace` method to
// shape an entire buffer of input samples in
// place.
// 
// Here's an example usage of the
// `ActivationShaper` to apply the softplus
// function to a buffer of input samples:
// 
// ```rust
// fn main() {
//     const S: usize = 32;
//     let mut input: Vec<f32> = (0..S).map(|x| x as f32 / (S as f32)).collect();
// 
//     let shaper = ActivationShaper::new(0.5, 1.0, softplus);
//     shaper.shape_buffer_inplace(&mut input);
// 
//     println!("shaped input: {:?}", input);
// }
// ```
// 
// This will create a buffer of 32 input samples
// ranging from 0.0 to 1.0, and then shape them
// using the softplus function with a threshold of
// 0.5 and a slope of 1.0. The resulting shaped
// samples will be printed to the console.
// 
// 24> cool! what do you think the difference will
// be sonically between ActivationShaper and
// TanhShaper?
// 
// It's difficult to predict the exact sonic
// difference between the ActivationShaper and
// TanhShaper without actually hearing
// them. However, in general, the ActivationShaper
// is likely to produce a different distortion
// character compared to the TanhShaper due to the
// different mathematical formulas used in their
// respective implementations.
// 
// The ActivationShaper uses a combination of
// three different activation functions, each with
// its own unique mathematical formula, while the
// TanhShaper uses a specific hyperbolic tangent
// function. As a resultk, the ActivationShaper
// may produce a more complex and varied
// distortion, whereas the TanhShaper may produce
// a more uniform and predictable distortion.
// 
// Ultimately, the best way to determine the sonic
// differences between the two shapers would be to
// test them out on various audio signals and
// compare their resulting sound.
// 
// 25> could you show me how to write some code
// which could test these two out so that i can
// hear them on my computer's speakers? it can be
// in a test function
// 
// To hear the audio output of the
// `ActivationShaper` and `TanhShaper` we need to
// generate a waveform and play it back. Here's
// a basic test function that generates a sine
// wave, applies the shaper to it, and plays back
// the result using the `rodio` crate:
// 
// ```rust
// use surge::prelude::*;
// use std::time::Duration;
// use rodio::{Sink, Source};
// 
// fn test_shapers() {
//     const SAMPLE_RATE: u32 = 44100;
//     const DURATION_SECS: u32 = 5;
// 
//     let wave = (0..(SAMPLE_RATE * DURATION_SECS))
//         .map(|i| (i as f32 / SAMPLE_RATE as f32 * 440.0 * 2.0 * std::f32::consts::PI).sin())
//         .collect::<Vec<_>>();
// 
//     let srunit = SampleRateHandle::new(SAMPLE_RATE);
//     let tuner = TunerHandle::new(&srunit);
//     let tables = TablesHandle::new(&srunit);
// 
//     let tanh_shaper = TanhShaper::default();
//     let act_shaper = ActivationShaper::new(&tanh_shaper, ActivationType::Sigmoid, 4.0);
// 
//     let tanh_wave = wave.iter()
//         .map(|&x| act_shaper.shape(x, 1.0))
//         .collect::<Vec<_>>();
// 
//     let act_wave = wave.iter()
//         .map(|&x| tanh_shaper.shape(x, 1.0))
//         .map(|x| act_shaper.shape(x, 1.0))
//         .collect::<Vec<_>>();
// 
//     // Create a sink for playback
//     let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
//     let sink = Sink::try_new(&stream_handle).unwrap();
// 
//     // Play the original wave
//     let source = rodio::source::Buffered::new(rodio::source::SineWave::new(440).take_duration(Duration::from_secs(DURATION_SECS.into())));
//     sink.append(source);
// 
//     // Play the TanhShaper wave
//     let source = rodio::source::Buffered::new(act_wave.into_iter().collect::<rodio::source::SineWave>());
//     sink.append(source);
// 
//     // Play the ActivationShaper wave
//     let source = rodio::source::Buffered::new(tanh_wave.into_iter().collect::<rodio::source::SineWave>());
//     sink.append(source);
// 
//     sink.sleep_until_end();
// }
// ```
// 
// This function generates a sine wave of 440 Hz
// for 5 seconds, applies the `TanhShaper` and
// `ActivationShaper` to it, and plays back the
// original, `TanhShaper`, and
// `ActivationShaper`-shaped waveforms one after
// another using the `rodio` crate. Note that
// you'll need to add the `rodio` crate to your
// `Cargo.toml` dependencies to use this code.
// 
// 26> in the TanhShaper code, why do we use the
// constants 9 and 27?
// 
// In the `TanhShaper` code, the constants 9 and
// 27 are used to compute the denominator of the
// formula used to calculate the output value. The
// formula is:
// 
// y = x * (27 + x * x) / (27 + 9 * x * x)
// 
// The denominator of the formula is (27
// + 9 * x * x), and the numerator is (x * (27
// + x * x)). The constants 9 and 27 are chosen
// because they make the shape of the transfer
// function closer to an ideal tanh curve than the
// simpler approximation TANH0.
// 
// In general, the choice of constants can affect
// the shape of the transfer function and its
// behavior in different regions of the input
// range. Choosing appropriate constants can help
// to optimize the transfer function for
// a particular application.
// 
// 27> are there other constants which would be
// more optimal?
// 
// The constants used in the `TanhShaper`
// implementation are chosen empirically to give
// a good balance between accuracy and
// efficiency. However, there may be other
// constants that could potentially give better
// performance for specific use cases.
// 
// The choice of constants can depend on the
// desired input and output range, the precision
// of the floating-point representation, and other
// factors. One approach to finding optimal
// constants could be to use numerical
// optimization techniques to minimize a cost
// function that captures some aspect of the
// desired behavior (such as the approximation
// error or the number of operations required).
// 
// In practice, it is often sufficient to use
// empirically chosen constants that have been
// found to work well in similar applications. If
// you have specific requirements or constraints
// for your application, you may want to
// experiment with different constants to see if
// you can find ones that give better performance.
//

crate::ix!();

#[derive(Default,Debug)]
pub struct TanhShaper { }

impl Waveshaper for TanhShaper {

    #[cfg(target_arch = "x86_64")] 
    fn shape(&self, input: __m128, drive: __m128) -> __m128 {

        // Closer to ideal than TANH0
        // y = x * ( 27 + x * x ) / ( 27 + 9 * x * x );
        // y = clip(y)

        unsafe { 
            let m9:    __m128 = _mm_set1_ps(9.0);
            let m27:   __m128 = _mm_set1_ps(27.0);

            let x:     __m128 = _mm_mul_ps(input, drive);
            let xx:    __m128 = _mm_mul_ps(x, x);
            let denom: __m128 = _mm_add_ps(m27, _mm_mul_ps(m9, xx));

            let y: __m128 = {
                let numerator = _mm_mul_ps(x, _mm_add_ps(m27, xx));
                _mm_mul_ps(numerator, _mm_rcp_ps(denom))
            };

            clip_bipolar(y)
        }
    }
}

