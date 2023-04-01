# Surge-lag

## Description

The `surge-lag` crate is a Rust implementation of
a lag processor. The purpose of the lag processor
is to smoothly transition between different
parameter values in a signal processing chain.

The `Lag` struct represents the lag processor, and
it has methods to instantiate it, set the rate of
change, set the starting value, and process input
values. The `VLag` struct is a variant of `Lag`
that uses `VDouble` values.

The `Lag` processor can be used to create smooth
transitions between different parameter values in
a synthesizer, such as a filter cutoff frequency
or oscillator frequency. It can also be used to
create gradual fades in and out of signals.

The implementation is based on the following
equation:

```
y[n] = y[n-1] + rate * (x[n] - y[n-1])
```

where `x` is the input signal, `y` is the output
signal, and `rate` is the rate of change.

## Tokens

- `Lag`: The struct representing the lag
  processor.

- `VLag`: The struct representing the lag
  processor using `VDouble` values.

- `default`: A method to create a default `Lag`
  instance.

- `instantize`: A method to initialize a `Lag`
  instance with a specific starting value.

- `set_rate`: A method to set the rate of change
  for a `Lag` instance.

- `start_value`: A method to set the starting
  value for a `Lag` instance.

- `process`: A method to process an input value
  through a `Lag` instance.

- `VDouble`: A struct representing a double value
  using SIMD operations.

- `init_x87`: A method to initialize a `VLag`
  instance with a specific starting value using
  x87 instructions.

- `test_vlag_init_x87`: A test to verify the
  initialization of a `VLag` instance using x87
  instructions.

- `test_vlag_instantize`: A test to verify the
  initialization of a `VLag` instance with
  a specific starting value.

- `test_vlag_process`: A test to verify the
  processing of an input value through a `VLag`
  instance.

- `test_vlag_start_value`: A test to verify the
  setting of the starting value for a `VLag`
  instance.

- `smoke`: A test to verify the basic
  functionality of a `Lag` instance.

- `test_default`: A test to verify the creation of
  a default `Lag` instance.

- `test_instantize`: A test to verify the
  initialization of a `Lag` instance with
  a specific starting value.

- `test_process`: A test to verify the processing
  of an input value through a `Lag` instance.

- `test_set_rate`: A test to verify the setting of
  the rate of change for a `Lag` instance.

- `test_start_value`: A test to verify the setting
  of the starting value for a `Lag` instance.

107> so this is like a sort of linear
interpolator, with a variable rate.  why do we use
sines and cosines for its implementation?

The use of sines and cosines in the implementation
of the `VLag` struct is related to the concept of
phase. In this implementation, the phase of a sine
and cosine wave is used to determine the
fractional part of the delay time. The delay time
is then calculated using linear interpolation
between two consecutive samples based on the
fractional part of the delay time.

The use of sines and cosines in this way is
actually a common technique in digital signal
processing, known as "phase accumulation". By
accumulating phase values over time, we can create
a variable time delay that is continuously
changing in a smooth and continuous way. This can
be useful in a variety of audio processing
applications, such as reverberation, chorus, and
flanging effects.

108> could you describe phase accumulation in more
detail? could you show me how to derive the delay
time computation?

Sure! Phase accumulation is a technique used in
many digital signal processing applications to
generate or process signals with specific
frequencies. The basic idea is to keep track of
a phase variable that increases over time, and
then use this phase variable to compute the
amplitude of the output signal at each sample.

In the context of the surge-lag crate, phase
accumulation is used to implement a variable delay
line. The phase variable is directly proportional
to the delay time, so by controlling the rate of
phase accumulation, we can control the delay time.

To derive the delay time computation, let's start
with the definition of the phase variable phi:

```
phi(n) = 2 * pi * f * n * T
```

where `n` is the sample index, `T` is the sample
period, and `f` is the frequency we want to
generate. To get the delay time, we need to solve
for `n` in terms of the delay time `d`:

```
n = d / T
```

Substituting this into the expression for
`phi(n)`, we get:

```
phi(d) = 2 * pi * f * d
```

This is the formula for the phase at delay time
`d`, in radians. To compute the amplitude of the
output signal, we need to take the sine or cosine
of this phase value:

```
y = A * sin(phi(d))
```

or

```
y = A * cos(phi(d))
```

where `A` is the amplitude of the output
signal. By changing the rate of phase
accumulation, we can change the delay time, and
thus the frequency of the output signal.

I hope that helps! Let me know if you have any
more questions.

109> thanks -- how do these techniques apply to
creating digital reverberation, chorus, and
flanging effects? are there other signal
processors which can benefit from these
functionalities?

The techniques used in the `surge-lag` crate,
particularly phase accumulation and linear
interpolation, are commonly used in digital signal
processing algorithms for creating various types
of time-based effects, including digital
reverberation, chorus, and flanging.

In digital reverberation, for example, multiple
reflections of a sound wave are created to
simulate the effect of sound bouncing off walls
and other surfaces in a physical space. Each
reflection is delayed by a certain amount, and the
delayed reflections are then summed together with
the original sound to create a reverberant
sound. The delays are typically implemented using
some form of a delay line, such as the `Lag`
structure in the `surge-lag` crate.

In chorus and flanging effects, multiple, slightly
detuned copies of the original sound are created
and mixed together with the original signal to
create a thick, swirling sound. The detuning is
typically implemented by introducing a small delay
and slightly altering the phase of the delayed
copy. Again, this can be implemented using a delay
line with a variable delay time.

Other signal processors that can benefit from
these functionalities include time-based effects
such as echo, delay, and phaser, as well as
oscillators and filters that require phase
coherence.

110> let's describe some of these in more detail:
echo, delay, and phaser, as well as oscillators
and filters that require phase coherence

Sure, I'd be happy to describe these in more
detail:

1. **Echo**: An echo effect creates a delayed copy
   of the input signal that is repeated at regular
   intervals. The delayed copy is typically
   attenuated to simulate the effect of sound
   reflecting off surfaces and returning to the
   listener. This effect can be created using
   a delay line, which is a buffer that stores
   a portion of the input signal and outputs it
   after a specified delay time. The delayed
   signal can then be combined with the original
   signal to create the echo effect.

2. **Delay**: A delay effect is similar to an echo
   effect, but with a shorter delay time and fewer
   repeats. Delay effects are commonly used to
   create a sense of space and depth in
   a mix. Delay can be implemented using a delay
   line, just like an echo effect, but with
   a shorter delay time and fewer repeats.

3. **Phaser**: A phaser is a type of signal
   processor that creates a sweeping filter effect
   by combining two delayed copies of the input
   signal. The first copy is passed through
   a variable all-pass filter, which shifts the
   phase of certain frequency components, while
   the second copy is passed through a fixed
   all-pass filter with a different phase
   shift. The two signals are then combined to
   create a series of peaks and dips in the
   frequency response of the processed signal,
   which creates the characteristic phaser effect.

4. **Oscillators**: Oscillators are electronic
   circuits that generate periodic waveforms, such
   as sine waves, square waves, and sawtooth
   waves. These waveforms can be used to create
   a variety of sounds, from simple tones to
   complex timbres. Phase coherence is important
   when using multiple oscillators to create
   polyphonic sounds, such as chords or multiple
   voices. If the phase of the oscillators is not
   synchronized, it can create unwanted phase
   cancellations and frequency interference.

5. **Filters**: Filters are signal processing
   circuits that selectively attenuate or boost
   certain frequency components of
   a signal. Filters are commonly used in audio
   processing to shape the frequency response of
   a signal, either to correct for unwanted
   resonances or to create specific
   timbres. Filters that require phase coherence,
   such as comb filters or phasers, must maintain
   the phase relationship between the input and
   output signals to avoid unwanted phase
   cancellations or frequency interference. This
   is often achieved by using all-pass filters,
   which shift the phase of the signal without
   affecting its magnitude, or by using
   phase-locked loops, which synchronize the phase
   of the input and output signals.

111> thanks -- what is the mathematical idea of
phase coherence? how do we know when a system will
exhibit problems with phase coherence?

Phase coherence refers to the property of a signal
processing system where the relative phases of its
output signals are preserved with respect to its
input signals. In other words, if a system is
phase coherent, then the phase relationships
between different components of the input signal
are preserved in the output signal.

A system can exhibit problems with phase coherence
when there are differences in phase shifts among
different frequency components of a signal. This
can occur in systems that use different processing
paths for different frequency bands or in systems
that use feedback loops, among others. If the
phase shifts are not properly compensated for, the
output signal can become distorted and lose its
coherence with respect to the input signal. This
can result in a variety of undesirable effects,
including phase cancellation, comb filtering, and
other artifacts that can degrade the quality of
the signal.

112> thanks -- could you provide 10 examples of
projects which may benefit from using this crate?

Sure, here are 10 examples of projects that may
benefit from using `surge-lag`:

1. Audio plugins and digital audio workstations
   (DAWs) that require time-based effects like
   echo, delay, phaser, or flanger.

2. Synthesizer software that requires phase
   coherence for its oscillators and filters.

3. Music production software that requires precise
   timing of events or synchronization with
   external sources like MIDI or SMPTE timecode.

4. Game audio engines that require accurate timing
   of sound effects and music.

5. Virtual reality and augmented reality software
   that requires accurate spatialization of sound
   sources.

6. Interactive installations and multimedia
   performances that involve real-time audio
   processing and manipulation.

7. Music visualization software that uses audio
   analysis to generate visual effects.

8. Audio hardware devices that require precise
   timing and synchronization with other devices.

9. Research projects in digital signal processing,
   acoustics, and psychoacoustics that require
   precise time-based processing.

10. Audio-related machine learning projects that
    require accurate timing and synchronization of
    audio signals.

113> thanks -- could you provide 10 others?

Sure! Here are 10 more examples of projects that
could benefit from using the surge-lag crate:

1. Audio analysis software that requires
   phase-aligned signals for accurate measurements

2. Audio restoration software that needs to
   preserve the phase coherence of the original
   recordings

3. Digital signal processing libraries that
   implement time-based effects like echo, delay,
   and chorus

4. Music production software that uses digital
   signal processing for virtual instruments and
   effects

5. Audio synthesis software that generates sounds
   with complex timbres and dynamic textures

6. Machine learning algorithms that process audio
   data for speech recognition or music analysis

7. Virtual reality and augmented reality
   applications that require 3D spatialization of
   sound

8. Signal processing hardware for audio
   applications, such as mixers, amplifiers, and
   preamps

9. Educational tools for teaching digital signal
   processing and audio engineering

10. Digital art installations that use sound as an
    integral part of the user experience.

114> okay! some of these overlap, but that is ok

No problem, happy to help!

115> what is the difference between Lag and VLag?

From what I can see in the tokens you provided,
both `Lag` and `VLag` are structs that represent
a delay line used for creating time-based effects
like echoes and reverbs.

However, it seems that `VLag` is a more
specialized version of `Lag`, designed
specifically for linear interpolation of the delay
line, while `Lag` may use different interpolation
methods or not perform any interpolation at all.

Without seeing the code or documentation for these
specific crates, I can't say for sure what the
exact differences are, but based on the tokens you
provided, it appears that `VLag` is a more
specialized version of `Lag` designed specifically
for linear interpolation of the delay line.

116> /file surge-lag/src/vlag.rs

The `VLag` struct is similar to `Lag`, but it uses
double-precision floating-point numbers (`f64`)
instead of single-precision floating-point numbers
(`f32`). It also supports SSE/AVX vectorization,
meaning that the filter's two double-precision
floating-point values (`v` and `target_v`) are
aligned on a 16-byte boundary to facilitate
efficient processing by modern CPUs.

The `VLag` struct represents a variable lag
filter, which is used to smooth out a stream of
data over time, by updating the current value
(`v`) based on the target value (`target_v`) and
a fixed lag coefficient (`VLAG_D_LP`), which
determines how quickly the filter responds to
changes in the target value. The lag coefficient
and the inverse of the lag coefficient
(`VLAG_D_LPINV`) are pre-defined constants.

The `VLag` struct has four public methods
(`new_value`, `instantize`, `start_value`, and
`process`) and several pre-defined constants
(`VLAG_MIN_BW`, `VLAG_D_LP`, and `VLAG_D_LPINV`)
that are used to control the behavior of the
filter.

The `VLag` struct is useful for smoothing out
signals in digital audio processing, especially
for signals with low frequency content. It can be
used in a variety of applications, such as
implementing a tremolo effect on a guitar signal,
or smoothing out the output of a sensor in
a robotics application.

117> /file surge-lag/src/lag.rs

`Lag` is a struct that provides a low-pass filter
that smooths out a stream of data over time. It
achieves this by updating the current value based
on the target value and a fixed low-pass filter
coefficient, which determines how quickly the
filter responds to changes in the target value.

The struct is generic over a type parameter `T`,
which must implement the `Num`, `From<f32>`,
`Clone`, and `AddAssign` traits. This allows `Lag`
to be used with a variety of numeric types.

The `Lag` struct has several methods, including
`new`, `set_rate`, `new_value`, `start_value`,
`instantize`, `process`, and
`get_target_value`. These methods allow the user
to initialize and manipulate the filter, including
setting the low-pass filter coefficient, setting
the target value to be smoothed towards, updating
the current value based on the target value and
filter coefficient, and retrieving the current
target value.

In addition, `Lag` has a constant boolean value
`FIRST_RUN_CHECKS` that determines whether to run
first-time checks, and a struct field `first_run`
that tracks whether the `Lag` struct is being used
for the first time.
