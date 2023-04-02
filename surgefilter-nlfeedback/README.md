## Note

 This contains an adaptation of the filter found at:
 https://ccrma.stanford.edu/~jatin/ComplexNonlinearities/NLFeedback.html

 with coefficient calculation from:
 https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html

## `NonlinearFeedbackFilter`

The `NonlinearFeedbackFilter` is an IIR filter
that uses nonlinear feedback to create complex,
interesting distortion effects. The filter is
designed to be used in audio processing
applications such as music synthesis, audio
effects processing, and sound design.

The filter is implemented using a feedback loop
that includes a nonlinear distortion element,
which introduces harmonic distortion and other
nonlinear effects into the audio signal. The exact
nature of the distortion element can be controlled
by adjusting the filter coefficients and other
parameters.

The `NonlinearFeedbackFilter` is implemented as
a Rust struct that exposes a set of methods for
configuring and processing audio signals. The
filter can be used in real-time audio processing
applications or in offline audio processing
workflows.

### Mathematical Analysis

The `NonlinearFeedbackFilter` is an IIR filter,
which means that it uses feedback to create
a resonant response to certain frequencies in the
audio signal. The filter is implemented as a set
of difference equations that describe the
relationship between the input signal, the output
signal, and the internal state of the filter. The
equations can be expressed mathematically as:

y[n] = a0 * x[n] + a1 * x[n-1] + a2 * x[n-2] - b1 * y[n-1] - b2 * y[n-2]

where `x[n]` and `y[n]` represent the input and
output signals at time `n`, and `a0`, `a1`, `a2`,
`b1`, and `b2` represent the filter coefficients.

The specific values of the filter coefficients
determine the frequency response and distortion
characteristics of the filter. The `coeff_make`
method can be used to generate filter coefficients
for a given cutoff frequency and resonance value.

### Token Descriptions

- `C`: This token represents the value of the
  feedback capacitance in the filter circuit. The
  value of `C` affects the resonance and frequency
  response of the filter.

- `R`: This token represents the value of the
  feedback resistor in the filter circuit. The
  value of `R` affects the resonance and frequency
  response of the filter.

- `clamped_frequency`: This token represents
  a frequency value that has been clamped to
  a specific range. Clamping the frequency value
  ensures that it remains within a valid range for
  the filter.

- `fastsin_ps`: This token represents a fast
  implementation of the sine function that uses
  a polynomial approximation. The `fastsin_ps`
  function can be used to generate nonlinear
  distortion effects in the filter.

- `ojd_waveshaper`: This token represents
  a waveshaping function that can be used to
  generate distortion effects in the filter.

- `ojd_waveshaper_ps`: This token represents
  a fast implementation of the `ojd_waveshaper`
  function that uses a polynomial approximation.

- `coeff_make`: This token represents a method
  that generates filter coefficients for a given
  cutoff frequency and resonance value. The
  `coeff_make` method takes these values as input
  parameters and returns a set of filter
  coefficients.

- `process_quad`: This token represents a method
  that processes a quad of samples using the
  filter. The `process_quad` method takes four
  input samples and returns four output samples.

## END

44> thanks! what is a nonlinear feedback filter?

A nonlinear feedback filter is an IIR (infinite
impulse response) filter that uses a nonlinear
feedback loop to introduce nonlinear distortion
and other interesting effects into an audio
signal. Unlike traditional linear feedback
filters, which rely solely on the principle of
resonance to shape the frequency response of the
signal, nonlinear feedback filters also use
nonlinear effects to create complex harmonic and
subharmonic content.

The exact nature of the nonlinear distortion
introduced by the feedback loop can be controlled
by adjusting the filter coefficients and other
parameters. In some cases, the feedback loop may
include a distortion element such as a waveshaper
or a clipping circuit, which further modifies the
signal.

Nonlinear feedback filters are often used in audio
processing applications such as music synthesis,
audio effects processing, and sound design. They
can be used to create a wide range of interesting
and unique sounds, from subtle harmonic
enhancement to extreme distortion and saturation.

45> /file surge-rs/surgefilter-nlfeedback/src/process.rs

Certainly! This code implements the
`process_quad()` function for the
`NonlinearFeedbackFilter` struct. The purpose of
this function is to apply the filter to an input
signal, producing an output signal that has been
shaped by the nonlinear feedback loop of the
filter.

The function begins by initializing the input
signal and extracting some parameters from the
filter state. The `stages` variable determines the
number of filter stages to apply, while the `sat`
variable selects the type of saturator to use in
the feedback loop.

The function then iterates through each filter
stage, applying the filter coefficients and
feedback loop at each stage. For each stage, the
function calculates the filter output using the
current filter state and input signal, and then
applies the nonlinear feedback loop using the
selected saturator function.

After applying the nonlinear feedback, the
function updates the filter state for the current
stage, storing the calculated values for the next
iteration. The output signal is then scaled by
a `makeup` coefficient, which is applied to each
filter stage.

Finally, the function applies any changes to the
filter coefficients that have been made since the
last call to `process_quad()`, and returns the
output signal.

Overall, the `process_quad()` function implements
the core signal processing functionality of the
`NonlinearFeedbackFilter` struct, applying
a configurable number of filter stages and
nonlinear feedback loops to the input signal to
produce a shaped output signal.

46> /file surge-rs/surgefilter-nlfeedback/src/coeff.rs

Sure, let me explain the `coeff_make` function in
more detail. This function takes in the frequency
and resonance values and returns an array of
coefficients used by the filter's `process_quad`
function.

First, the resonance value is limited to the range
of 0 to 1. Then, the function calculates the
number of stages and selects the appropriate
saturator function based on the comb write
position.

Next, it calculates the quality factor (Q) based
on the resonance value. Then, it calculates the
normalized frequency and the cutoff frequency. It
also calculates the sine and cosine of the cutoff
frequency, which are used later in the function.

The function then calculates the alpha value,
which is used to calculate the feedback
coefficients. The reciprocal of a0 is calculated,
as it is used to normalize the other coefficients.

Depending on the type of filter, the function
calculates different coefficients. For example,
for a low-pass filter, it calculates the b0, b1,
and b2 coefficients based on the normalized
frequency and cosine of the cutoff frequency. It
also calculates a makeup gain based on the
resonance value, the normalized frequency, and
a normalization factor.

Once all the coefficients are calculated, they are
stored in an array and returned. These
coefficients are used in the `process_quad`
function to process the audio signal.

I hope that helps!

47> /file surge-rs/surgefilter-nlfeedback/src/nlff.rs

This is the definition of the
`NonlinearFeedbackFilter` struct.

It is a struct that contains an adaptation of the
filter found at
https://ccrma.stanford.edu/~jatin/ComplexNonlinearities/NLFeedback.html
with coefficient calculation from
https://webaudio.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html.

The struct has the following fields:

- `tuner`: a handle to a `Tuner` object used to
  calculate the frequency of the filter

- `srunit`: a handle to a `SampleRate` object used
  to calculate the sample rate

- `ty`: an `NLFFType` object representing the type
  of the filter (low-pass, high-pass, band-pass,
  notch, or all-pass)

- `subty`: a `FilterSubType` object representing
  the subtype of the filter (used to select the
  saturator)

The struct also has the following methods:

- `clamped_frequency`: a method that takes a pitch
  value and returns the clamped frequency of the
  filter within a certain range

- `fastsin_ps`: a method that applies a fast sine
  function to each element of a `__m128` vector by
  breaking it into floats and reassembling

- `ojd_waveshaper`: a method that applies a custom
  waveshaper function to a single float value

- `ojd_waveshaper_ps`: a method that applies the
  custom waveshaper function to each element of
  a `__m128` vector by breaking it into floats and
  reassembling

Overall, the `NonlinearFeedbackFilter` struct
represents a complex filter that combines
nonlinear feedback with various types of
saturators to achieve a desired sound.

48> thanks! could you teach me more about the
mathematical concepts involved in this crate?

Sure! The Nonlinear Feedback Filter crate
implements a nonlinear digital filter with
feedback. Let me explain the key mathematical
concepts involved in this process:

1. Nonlinear filters: Nonlinear filters modify an
   input signal based on nonlinear functions,
   which can be much more complex than simple
   linear filters. Nonlinear filters are often
   used for creative sound synthesis and sound
   design, as they can create complex and
   interesting harmonic structures.

2. Feedback: Feedback is a process in which
   a portion of the output of a system is fed back
   into the input, to modify the system's
   behavior. Feedback can have both positive and
   negative effects, depending on the system and
   the feedback loop design.

3. Coefficient calculation: Coefficient
   calculation is the process of determining the
   filter coefficients, which determine the
   behavior of the filter. In the Nonlinear
   Feedback Filter crate, the coefficients are
   calculated based on the desired frequency and
   resonance of the filter, using the formulas
   from the Audio EQ Cookbook.

4. Saturators: A saturator is a type of nonlinear
   function that can be used to create distortion
   and other interesting effects. In the Nonlinear
   Feedback Filter crate, the filter uses
   different types of saturators to modify the
   feedback signal.

5. SIMD: SIMD (Single Instruction Multiple Data)
   is a technique for processing multiple data
   elements in parallel, using a single
   instruction. The Nonlinear Feedback Filter
   crate uses SIMD instructions to process
   multiple samples simultaneously, improving
   performance and efficiency.

These are just a few of the mathematical concepts
involved in the Nonlinear Feedback Filter
crate. Let me know if you have any further
questions!

49> thanks! what are some others?

Here are some additional mathematical concepts
that may be relevant to the surgefilter-nlfeedback
crate:

- Biquad filters: NonlinearFeedbackFilter
  implements a biquad filter, which is a type of
  digital filter commonly used in audio signal
  processing. A biquad filter consists of two
  poles and two zeros, and can be used to
  implement various types of filters, such as
  low-pass, high-pass, band-pass, and notch
  filters. The coefficients of the biquad filter
  determine the frequency response of the filter,
  and can be calculated using various design
  methods.

- Saturators: NonlinearFeedbackFilter also
  implements several saturators, which are
  nonlinear functions that can be used to distort
  or clip an input signal. Saturators can be used
  to create various types of distortion effects in
  audio processing, such as overdrive, fuzz, and
  distortion. The nonlinear behavior of saturators
  can be described using various mathematical
  models, such as piecewise-linear models,
  exponential models, and polynomial models.

- Frequency response: The frequency response of
  a filter is the relationship between the input
  and output signals at different frequencies. The
  frequency response of a filter can be
  represented using various graphical and
  mathematical representations, such as magnitude
  response, phase response, frequency response
  curves, transfer functions, and pole-zero
  plots. The frequency response of a filter is
  determined by its design and coefficient values.

- Trigonometric functions: NonlinearFeedbackFilter
  uses various trigonometric functions, such as
  sine and cosine, to calculate the coefficients
  of the filter and to implement some of the
  saturators. Trigonometric functions are
  mathematical functions that relate the angles
  and sides of triangles, and are used in various
  areas of mathematics and science, such as
  geometry, calculus, physics, and engineering.

- Digital signal processing:
  NonlinearFeedbackFilter is a digital filter,
  which means that it operates on discrete-time
  signals. Digital signal processing is the branch
  of signal processing that deals with the
  analysis, manipulation, and synthesis of
  discrete-time signals using digital
  computers. Digital signal processing techniques
  are used in various areas of science and
  engineering, such as audio and video processing,
  telecommunications, control systems, and
  biomedical signal processing.

50> ok, great! thanks for your help!

You're welcome! Don't hesitate to ask if you have
any more questions.
