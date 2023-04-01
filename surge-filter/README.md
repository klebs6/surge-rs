# surge-filter

The `surge-filter` crate provides functionality
for filtering audio signals in the Surge
synthesizer system. It includes several filter
types and utilities for filter coefficient
generation and processing.

## Clip Bipolar

The `clip_bipolar` function clips an input signal
to a range of -1 to 1, which is commonly used in
digital audio. This function is implemented using
a simple conditional statement and is highly
optimized for performance.

## Coefficient Generation

The `coeff_make` module provides utilities for
generating filter coefficients. It includes
functions for generating coefficients for several
types of filters, including low-pass, high-pass,
band-pass, and band-reject filters. These
functions are based on standard filter design
techniques, such as the bilinear transform and
windowing methods.

## Lightweight

The `lightweight` module provides a lightweight,
zero-allocation filter implementation for
processing quad channel audio signals. This filter
is designed to be highly efficient and optimized
for performance, making it well-suited for use in
real-time audio processing applications.

## Processing Quad

The `process_quad` function processes a quad
channel audio signal using a given set of filter
coefficients. This function is highly optimized
for performance and is designed to be used with
the `lightweight` filter implementation.

## Shape

The `shape` module provides several functions for
shaping filter response curves. These functions
include the creation of Butterworth and Chebyshev
filter responses, as well as functions for
applying frequency scaling and gain adjustments to
filter coefficients.

## To

The `to` module provides functions for converting
between various filter formats. These functions
include conversions between direct form I and
direct form II filter representations, as well as
conversions between second-order sections and
polynomial coefficients.

Overall, the `surge-filter` crate provides
a comprehensive set of tools for filtering audio
signals in the Surge synthesizer system. Its
efficient implementations and optimized algorithms
make it well-suited for use in real-time audio
processing applications.

-----------------
Based on the source code you provided, here is
a possible crate description for `surge-filter`:

`surge-filter` is a Rust crate that provides
traits and functions for signal processing in the
context of audio synthesis. It includes a trait
`Waveshaper` for implementing signal waveshaping
functions, a function `clip_bipolar` for
performing bipolar signal clipping, and traits
`FilterProcessQuad`, `CoeffMake`, and
`SurgeFilter` for implementing quad filter
processing and filter coefficient
generation. These components can be used together
or independently to create custom audio synthesis
pipelines. It is recommended to use the AVX
instruction set for optimal performance.

75> thanks! could you give me ten examples of how
these traits can be used from the context of other
programs?

Sure, here are ten examples of how these traits
can be used:

1. A software synthesizer could use the
   `Waveshaper` trait to implement various
   waveshaping algorithms for generating different
   kinds of sounds.

2. A digital audio workstation could use the
   `clip_bipolar` function to limit the amplitude
   of an audio signal before further processing.

3. A plugin for a guitar amplifier simulation
   could use the `SurgeFilter` trait to implement
   various filter types and provide adjustable
   frequency and resonance parameters.

4. A software equalizer could use the
   `QuadFilterUnitState` struct and
   `FilterProcessQuad` trait to implement
   a four-band equalizer with adjustable frequency
   and gain controls.

5. A plugin for a mixing console could use the
   `CoeffMake` trait to generate filter
   coefficients for a high-pass or low-pass filter
   based on user-defined cutoff frequency and
   resonance settings.

6. A software effects processor could use the
   `Waveshaper` trait to implement distortion,
   overdrive, or fuzz effects with adjustable gain
   and saturation controls.

7. A granular synthesis engine could use the
   `Waveshaper` trait to generate complex
   waveforms by combining and processing multiple
   grains of sound.

8. An algorithmic composition program could use
   the `SurgeFilter` trait to create dynamic
   filter sweeps and frequency modulation effects
   based on various input parameters.

9. A software delay effect could use the
   `FilterProcessQuad` trait to implement
   a feedback loop with adjustable damping and
   delay time settings.

10. A digital signal processing library could use
    the `SurgeFilter` trait to provide a flexible
    and modular way of processing audio signals
    with different filter types and parameters.

76> cool, thanks! can you see any potential
gotchas in the interface?

Based on the code you provided, I can see a few
potential gotchas in the interface:

1. The use of `unsafe` in `clip_bipolar` may make
   it harder to reason about the safety of the
   code. Users of this function will need to be
   aware of this and take extra care when calling
   it.

2. The use of low-level AVX instructions in
   `clip_bipolar` may make it less portable across
   different hardware architectures.

3. The use of fixed-length arrays in `CoeffMake`
   may limit the flexibility of implementing
   structs. If users need more or fewer
   coefficients, they would need to modify the
   `N_COEFFMAKER_COEFFS` constant and recompile
   the code.

4. The lack of documentation for the
   `QuadFilterUnitState` type in
   `FilterProcessQuad` may make it hard for users
   to know how to initialize and use it correctly.

5. The use of `f32` for the frequency and
   resonance parameters in `CoeffMake` may limit
   the precision of the filter coefficients, which
   could be an issue for some use cases. Using
   `f64` instead may be a better choice in some
   cases.

6. The lack of default implementations for the
   traits may make it more difficult for users to
   implement their own custom structs that use
   these traits. This may make the code less
   approachable for beginners.

Overall, these potential gotchas can be addressed
through careful documentation and design
decisions. For example, providing more information
about the safety requirements of `clip_bipolar`,
or adding default implementations for the traits,
could make the code easier to use and understand.

77> how much can the choice of f32 precision
impact the filter coefficient calculations? how
much do we trade off performance?

The choice of `f32` precision can have
a significant impact on the filter coefficient
calculations, particularly for high-order filters
with high-Q resonances. This is because the `f32`
type has a limited range and precision, which can
cause significant errors when calculating
coefficients for very narrow frequency bands or
high resonance values.

In practice, the choice of `f32` precision is
often a tradeoff between accuracy and
performance. Using higher-precision types, such as
`f64`, can improve the accuracy of the filter
coefficients but at the cost of increased
computation time and memory usage. On the other
hand, using lower-precision types, such as `f16`,
can improve performance but at the cost of reduced
accuracy.

In the case of the `SurgeFilter` trait, the use of
`f32` precision is likely a reasonable choice, as
it strikes a balance between accuracy and
performance for most audio applications. However,
it is important to keep in mind the limitations of
the `f32` type when designing and implementing the
filter coefficient calculations, and to test the
resulting filter performance carefully to ensure
that it meets the desired specifications.
