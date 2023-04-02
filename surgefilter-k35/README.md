## `surgefilter-k35`

The `surgefilter-k35` crate provides a set of
filters based on the K35 algorithm, which is
a variant of the classic Korg 35 filter used in
analog synthesizers. The K35 algorithm provides
a flexible and powerful way to implement lowpass
and highpass filters with adjustable cutoff
frequencies and resonance.

The crate provides two main filter types: `LPF`
(lowpass filter) and `HPF` (highpass filter), each
with its own set of coefficients and processing
functions. The filters can be configured using
a clamped frequency parameter, which determines
the cutoff frequency of the filter as a fraction
of the sampling rate.

The filters are implemented using a cascade of
second-order IIR filters, or biquads, which
provide a flexible and efficient way to implement
complex frequency response curves. The biquad
processing functions use the Direct Form II
Transposed structure, which is a canonical way to
implement IIR filters in a computationally
efficient way.

The `coeff_make` function is used to compute the
filter coefficients based on the desired filter
parameters. The function takes as input the filter
type (lowpass or highpass), the sampling rate, the
clamped frequency, and the resonance (Q factor),
and returns a set of filter coefficients in
a `FilterCoeffs` struct.

The `process_lp` and `process_hp` functions are
used to process input signals with the lowpass and
highpass filters, respectively. The functions take
as input the filter coefficients, the input
signal, and a mutable output buffer, and apply the
filter to the input signal in place.

The `do_lpf` and `do_hpf` functions are
convenience functions that wrap the `coeff_make`
and `process_lp`/`process_hp` functions to provide
a simpler API for processing signals with the
lowpass and highpass filters.

The `process_quad` function is a lower-level
function that implements the biquad processing
logic directly. It takes as input the biquad
coefficients, the input signal, and a mutable
state buffer, and applies the biquad filter to the
input signal in place.

Overall, the `surgefilter-k35` crate provides
a flexible and efficient way to implement K35
filters in Rust, with a simple and intuitive API
that is well-suited for real-time audio processing
and other signal processing applications.

11> thanks! could you show me how to implement the
korg 35 filter in verilog-a?

Sure! Here's an example implementation of the Korg
35 filter in Verilog-A:

```verilog

// verilog-a, (using verilog highlighting)

module korg_35_filter (
  input real x,
  output real y,
  parameter real fc = 1000, // cutoff frequency
  parameter real q = 0.5 // resonance
);

  real fs = $abstime_step(); // sampling rate
  real w0 = 2 * $pi * fc / fs; // normalized cutoff frequency
  real alpha = sin(w0) / (2 * q); // filter resonance

  real a0 = 1 + alpha;
  real a1 = -2 * cos(w0);
  real a2 = 1 - alpha;
  real b0 = (1 - cos(w0)) / 2;
  real b1 = 1 - cos(w0);
  real b2 = (1 - cos(w0)) / 2;

  real x1, x2, y1, y2;

  initial begin
    x1 = 0;
    x2 = 0;
    y1 = 0;
    y2 = 0;
  end

  analog begin
    real v0 = x - a1 * x1 - a2 * x2;
    real v1 = b0 * v0 + b1 * x1 + b2 * x2;
    real v2 = a0 * y1 - a1 * y2 + v1;

    y = v2;

    x2 = x1;
    x1 = x;
    y2 = y1;
    y1 = y;
  end
endmodule
```

This implementation uses the normalized cutoff
frequency `w0` and resonance `alpha` to compute
the filter coefficients `a0`, `a1`, `a2`, `b0`,
`b1`, and `b2`. The filter is implemented using
a Direct Form II Transposed structure, which is
a common way to implement IIR filters in
Verilog-A.

The `x1`, `x2`, `y1`, and `y2` variables are used
to store the previous input and output samples for
each stage of the filter, and are updated on each
simulation time step using Verilog-A's `analog`
block.

To use this filter in your Verilog-A simulation,
you can instantiate the `korg_35_filter` module
and connect its `x` and `y` ports to your input
and output signals, respectively. You can also
adjust the `fc` and `q` parameters to configure
the filter cutoff frequency and resonance to meet
your specific requirements.

Note that Verilog-A is a behavioral modeling
language, and its performance and accuracy may
vary depending on the simulation environment and
settings. For high-performance or high-precision
simulations, you may want to consider using a more
specialized hardware description language such as
Verilog or VHDL, or a specialized filter design
tool such as MATLAB or Python's scipy.signal
library.

12> thanks! what are the mathematical properties
of this algorithm?

The Korg 35 filter is a type of second-order
Infinite Impulse Response (IIR) filter that is
commonly used in analog synthesizers. Here are
some of the mathematical properties of the Korg 35
filter algorithm:

1. Linearity: The Korg 35 filter is a linear
   system, which means that its input/output
   relationship is linear. In other words, the
   output of the filter is a linear combination of
   its input and previous output samples.

2. Time-invariance: The Korg 35 filter is
   a time-invariant system, which means that its
   behavior does not change over time. In other
   words, the filter's response to an input signal
   is the same at any point in time.

3. Memory: The Korg 35 filter is a recursive
   system, which means that it has memory. The
   filter output depends not only on the current
   input sample, but also on past input and output
   samples.

4. Frequency response: The frequency response of
   the Korg 35 filter depends on the location of
   its poles and zeros in the complex plane. The
   filter's magnitude and phase response can be
   analyzed using tools such as the Fourier
   transform and the Z-transform.

5. Stability: The stability of the Korg 35 filter
   depends on the location of its poles in the
   complex plane. If all of the poles are inside
   the unit circle, the filter is stable and its
   output will not grow without bound.

6. Nonlinear phase: The Korg 35 filter has
   a nonlinear phase response, which means that it
   can introduce phase distortion in the filtered
   signal. This can be a desirable or undesirable
   effect depending on the application.

7. Resonance: The Korg 35 filter has a resonance
   parameter, which controls the peak gain of the
   filter at its cutoff frequency. This can be
   used to create a resonant peak or notch in the
   frequency response of the filter.

These mathematical properties of the Korg 35
filter algorithm are important for understanding
its behavior and performance in various
applications. By manipulating the location of the
poles and zeros, adjusting the resonance
parameter, and selecting the appropriate
implementation method, you can design Korg 35
filters with a wide range of characteristics to
meet your specific requirements.

13> what distinguishes it from other IIR filters?

The Korg 35 filter is a type of Infinite Impulse
Response (IIR) filter that is widely used in
analog synthesizers. What distinguishes the Korg
35 filter from other IIR filters are its specific
design characteristics and resulting frequency
response. Here are some of the key features that
distinguish the Korg 35 filter from other IIR
filters:

1. Resonance: The Korg 35 filter is known for its
   strong resonance characteristic, which allows
   it to create a peak or notch in the frequency
   response at the cutoff frequency. This makes it
   well-suited for use in audio processing
   applications such as equalizers, tone controls,
   and other filters that require precise
   frequency shaping.

2. Nonlinear phase response: The Korg 35 filter
   has a nonlinear phase response, which means
   that it can introduce phase distortion in the
   filtered signal. This can be a desirable or
   undesirable effect depending on the
   application, but it makes the Korg 35 filter
   well-suited for use in audio processing
   applications where the phase response is an
   important consideration.

3. Second-order implementation: The Korg 35 filter
   is implemented as a cascade of second-order IIR
   filters, or biquads. This provides a flexible
   and efficient way to implement complex
   frequency response curves, and allows for
   a greater degree of control over the filter
   response.

4. Adjustable cutoff frequency: The Korg 35 filter
   has an adjustable cutoff frequency, which can
   be set to any frequency within the passband of
   the filter. This makes it well-suited for use
   in applications where precise control over the
   frequency response is required.

5. Analog heritage: The Korg 35 filter was
   originally designed for use in analog
   synthesizers, and its implementation reflects
   this heritage. While it can be implemented
   digitally, it has a distinct character and
   warmth that is often associated with analog
   filters.

Overall, the Korg 35 filter is distinguished from
other IIR filters by its strong resonance
characteristic, nonlinear phase response,
second-order implementation, adjustable cutoff
frequency, and analog heritage. These features
make it a popular choice for audio processing
applications where precise frequency shaping and
a distinct character are desired.

14> thanks -- any caveats? what should we keep in
mind while using it?

While the Korg 35 filter is a powerful and
versatile tool for audio processing and other
signal processing applications, there are several
caveats and considerations that should be kept in
mind when using it:

1. Resonance can lead to instability: The
   resonance characteristic of the Korg 35 filter
   can cause instability if it is set too
   high. This can result in unpredictable and
   erratic behavior, and can damage speakers or
   other audio equipment. It is important to use
   the resonance control with care, and to ensure
   that the filter is properly designed and
   implemented to avoid instability.

2. Nonlinear phase response can introduce
   artifacts: The nonlinear phase response of the
   Korg 35 filter can introduce artifacts in the
   filtered signal, such as phase distortion or
   frequency-dependent delay. This can be
   a desirable or undesirable effect depending on
   the application, but it is important to be
   aware of the potential for artifacts and to
   design the filter appropriately to minimize
   their impact.

3. Second-order implementation requires careful
   design: The Korg 35 filter is implemented as
   a cascade of second-order IIR filters, or
   biquads. While this provides a flexible and
   efficient way to implement complex frequency
   response curves, it also requires careful
   design and tuning to ensure stability and
   accuracy.

4. Analog implementation can introduce noise and
   distortion: While the Korg 35 filter can be
   implemented digitally, it has an analog
   heritage and is often associated with analog
   filters. Analog implementation can introduce
   noise and distortion, which may be desirable or
   undesirable depending on the application.

5. Limitations of digital implementation: Digital
   implementation of the Korg 35 filter can
   introduce quantization noise and other
   artifacts, and can be limited by the available
   processing power and precision. It is important
   to design the filter appropriately for the
   available hardware and software, and to ensure
   that the implementation meets the desired
   performance criteria.

Overall, the Korg 35 filter is a powerful and
versatile tool that requires careful design and
implementation to achieve the desired results. By
understanding the potential caveats and
limitations, and by designing the filter
appropriately, you can take full advantage of the
Korg 35 filter's unique characteristics and
capabilities in your signal processing
applications.

15> what other filters are in its family tree?

The Korg 35 filter is part of a family of filters
that are commonly used in analog synthesizers and
other audio processing applications. Here are some
of the other filters that are related to the Korg
35 filter:

1. Moog ladder filter: The Moog ladder filter is
   a classic analog filter design that was used in
   the original Moog synthesizers. Like the Korg
   35 filter, the Moog ladder filter has a strong
   resonance characteristic and a nonlinear phase
   response.

2. State-variable filter: The state-variable
   filter is a type of filter that can be
   configured as a lowpass, highpass, or bandpass
   filter. It is often used in analog
   synthesizers, and is known for its flexibility
   and versatility.

3. Sallen-Key filter: The Sallen-Key filter is
   a type of active filter that is commonly used
   in audio processing and other signal processing
   applications. It is a second-order filter that
   can be configured as a lowpass, highpass, or
   bandpass filter.

4. Butterworth filter: The Butterworth filter is
   a type of IIR filter that has a maximally flat
   frequency response in the passband. It is often
   used in audio processing and other signal
   processing applications where a flat frequency
   response is desired.

5. Chebyshev filter: The Chebyshev filter is
   a type of IIR filter that has a steeper rolloff
   than the Butterworth filter, but has a ripple
   in the passband. It is often used in audio
   processing and other signal processing
   applications where a steeper rolloff is
   desired.

These filters, along with the Korg 35 filter, are
part of a rich heritage of analog filter designs
that have been used in a wide range of
applications. Each filter has its own unique
characteristics and tradeoffs, and the choice of
filter depends on the specific requirements of the
application.

16> could you please provide 10 examples of
projects which may benefit from using this crate?

Sure, here are ten examples of projects that may
benefit from using the `surgefilter-k35` Rust
crate:

1. Audio synthesizers: The Korg 35 filter is
   a common filter used in audio synthesizers, and
   the `surgefilter-k35` crate could be used to
   implement the filter in software.

2. Audio effects processors: The Korg 35 filter
   can also be used in audio effects processors
   such as equalizers, tone controls, and other
   filters. The `surgefilter-k35` crate could be
   used to implement these filters in software.

3. Music production software: Music production
   software such as digital audio workstations
   (DAWs) and virtual instruments can benefit from
   the Korg 35 filter's unique resonance and
   nonlinear phase characteristics. The
   `surgefilter-k35` crate could be used to
   implement the filter in these applications.

4. Audio processing research: Researchers in audio
   processing may use the `surgefilter-k35` crate
   as a tool for exploring the characteristics of
   the Korg 35 filter and its variants.

5. Audio programming tutorials: The
   `surgefilter-k35` crate could be used in audio
   programming tutorials as an example of a filter
   implementation in Rust.

6. Analog modeling software: The Korg 35 filter is
   often used as a reference filter for analog
   modeling software. The `surgefilter-k35` crate
   could be used to implement the filter in these
   applications.

7. Music visualization software: Music
   visualization software that reacts to audio
   signals in real-time could benefit from the
   Korg 35 filter's unique frequency response and
   resonance characteristics. The
   `surgefilter-k35` crate could be used to
   implement the filter in these applications.

8. Audio art installations: Audio art
   installations that require real-time audio
   processing could benefit from the Korg 35
   filter's unique characteristics. The
   `surgefilter-k35` crate could be used to
   implement the filter in these installations.

9. Audio game development: Game developers that
   require real-time audio processing could use
   the Korg 35 filter in audio effects such as
   filters and synthesizers. The `surgefilter-k35`
   crate could be used to implement these effects
   in game audio.

10. Audio hardware design: Audio hardware
    designers may use the `surgefilter-k35` crate
    as a tool for simulating the behavior of the
    Korg 35 filter in software before implementing
    the filter in hardware.

Overall, the `surgefilter-k35` Rust crate could be
useful in a wide range of audio processing
applications that require real-time processing,
precise frequency shaping, and a distinct
character in the audio output.

17> how could we use this  crate in game
development?

The `surgefilter-k35` Rust crate could be used in
game development to implement audio effects such
as filters and synthesizers that require real-time
processing and a unique character in the audio
output. Here are some examples of how this crate
could be used in game development:

1. Sound design: The Korg 35 filter is known for
   its strong resonance characteristic, which can
   be used to create interesting and unique sounds
   in games. The `surgefilter-k35` crate could be
   used to implement the filter in software and
   create custom sounds for game assets such as
   weapons, vehicles, and environments.

2. Music production: Games that feature original
   music compositions could benefit from the Korg
   35 filter's unique resonance and nonlinear
   phase characteristics. The `surgefilter-k35`
   crate could be used to implement the filter in
   software and process the music in real-time to
   create interesting and dynamic soundscapes.

3. Real-time audio effects: Games that feature
   real-time audio effects such as filters and
   synthesizers could use the `surgefilter-k35`
   crate to implement these effects in
   software. This could be used to create
   immersive sound environments, dynamic
   soundtracks, and unique audio effects.

4. Sound processing for voice-over: Games that
   feature voice-over could use the Korg 35 filter
   to process the voice audio and add interesting
   effects to the audio output. The
   `surgefilter-k35` crate could be used to
   implement the filter in software and add
   effects such as resonance, filtering, and
   distortion to the voice audio.

5. Audio-driven gameplay: Games that use audio
   cues to drive gameplay mechanics could use the
   Korg 35 filter to create unique audio feedback
   for player actions. The `surgefilter-k35` crate
   could be used to implement the filter in
   software and create audio effects that respond
   dynamically to player actions in real-time.

Overall, the `surgefilter-k35` Rust crate could be
a useful tool in game development for creating
unique and interesting audio effects that enhance
the player's experience and add to the overall
immersion of the game.
