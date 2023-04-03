A 3-band equalizer effect for the Surge synthesizer system
---

The `surgefx-eq3band` crate provides a 3-band
equalizer effect as a subcomponent of the Surge
synthesizer system. This effect allows users to
boost or cut the amplitude of three distinct
frequency bands in an audio signal, providing
tonal shaping and sound design capabilities.

The 3-band equalizer is implemented using a series
of band-pass filters, each with adjustable gain
(in dB), frequency, and Q factor. The mathematical
representation of a single band-pass filter is
given by:

H(ω) = G / (1 + jQ * (ω/ω0 - ω0/ω))

where H(ω) is the frequency response, G is the
gain, Q is the quality factor, ω is the angular
frequency, ω0 is the center frequency, and j is
the imaginary unit.

In the `surgefx-eq3band` crate, the following
tokens are used to control the equalizer's
parameters:

- `control_group`: Defines the group of control
  elements for the equalizer.

- `control_type`: Represents the type of control,
  such as sliders or knobs, used to adjust the
  equalizer's parameters.

- `default_value`: Specifies the default value for
  each control.

- `max_value`: Sets the maximum value for each
  control.

- `min_value`: Sets the minimum value for each
  control.

- `modulateable`: Indicates whether a parameter
  can be modulated by another source, such as an
  LFO or envelope.

- `moverate`: Represents the rate at which
  a parameter changes.

The `Eq3Band` struct is the main component of the
equalizer, containing methods for initialization,
processing, and updating the equalizer's
parameters. Key methods include `init`, `process`,
and `update`, which handle the initialization,
processing, and updating of the equalizer's
parameters, respectively.

During the processing stage, the equalizer applies
the gain to each frequency band using the
`apply_gain` method. The equalizer's output is
a combination of the processed bands, providing
a versatile tool for sound design and tonal
shaping.

44> how does an equalizer work from a mathematical
perspective? what about from a physical
perspective? what about from the perspective of
engineering?

From a mathematical perspective, an equalizer
works by adjusting the amplitude of specific
frequency bands within an audio signal. This is
typically done using filters, which are designed
to have a specific frequency response. The
frequency response H(ω) of a filter determines how
it will affect the amplitude and phase of each
frequency component in the input signal.

In the case of a simple graphic equalizer, the
filters used are often bandpass filters with fixed
center frequencies and bandwidths. The user can
adjust the gain of each filter to boost or cut the
amplitude in the corresponding frequency
band. A parametric equalizer provides more
control, allowing the user to adjust the center
frequency and bandwidth (or Q factor) of each
bandpass filter, in addition to the gain.

From a physical perspective, an equalizer can be
thought of as a series of filters that selectively
amplify or attenuate specific frequency bands in
an audio signal. This can be accomplished using
analog electronic components, such as resistors,
capacitors, and inductors, or digital algorithms
that approximate the behavior of analog filters.

In analog equalizers, the filters are usually
implemented using passive or active electronic
circuits. Passive equalizers rely on resistors,
capacitors, and inductors to create the desired
frequency response, while active equalizers use
operational amplifiers (op-amps) to control the
response and provide additional gain. Digital
equalizers, on the other hand, use digital signal
processing (DSP) algorithms to manipulate the
audio signal in the digital domain.

From an engineering perspective, the design of an
equalizer involves choosing the appropriate filter
types, determining the desired frequency bands,
and selecting the appropriate control mechanisms
(e.g., sliders, knobs, or digital interfaces). The
performance of the equalizer can be evaluated
using various metrics, such as signal-to-noise
ratio, total harmonic distortion, and frequency
response accuracy. Additionally, engineers must
also consider factors such as power consumption,
size, and cost when designing an equalizer for
a specific application or product.

45> could you show me a block diagram? how is this
processor parametrized?

Here's a simple block diagram for a 3-band
equalizer:

```
    ┌─────────────┐
    │ Input Signal│
    └──────┬──────┘
           │
  ┌────────▼─────────┐
  │   Splitter       │
  └───────┬─┬────────┘
          │ │
┌─────────▼─▼────────┐
│Low-band│Mid-band│High-band│
│ Filter │ Filter │ Filter  │
└─────────┬─┬───────┬─┬──────┘
          │ │       │ │
   ┌──────┴─▼───────┴─▼─────┐
   │       Summer          │
   └─────────┬─────────────┘
             │
    ┌────────▼───────┐
    │ Output Signal  │
    └───────────────┘
```

In this diagram, the input signal is first sent to
a splitter, which divides the signal into multiple
frequency bands. Each frequency band is then
processed through a separate filter (low-pass,
band-pass, or high-pass), which can have
adjustable gain to boost or cut the amplitude of
the corresponding frequency range. After
filtering, the signals are summed together to
produce the output signal.

The parameters that can be adjusted in an
equalizer typically include:

1. **Center frequency**: The center frequency of
   each bandpass filter. In a graphic equalizer,
   these frequencies are usually fixed, while in
   a parametric equalizer, they can be adjusted by
   the user.

2. **Bandwidth or Q factor**: The width of the
   frequency band that the filter
   affects. A narrower bandwidth (or higher
   Q factor) allows for more precise control over
   a specific frequency range. This parameter is
   adjustable in a parametric equalizer.

3. **Gain**: The amount by which the amplitude of
   each frequency band is boosted or cut. This
   parameter can be adjusted in both graphic and
   parametric equalizers.

By adjusting these parameters, users can tailor
the frequency response of the equalizer to suit
their needs, whether it's enhancing specific
frequency ranges, reducing unwanted frequencies,
or shaping the overall tonal balance of an audio
signal.

46> could you show me a dozen ways this processor
can be used to achieve a variety of audio effects?
how is each parametrized? how would you describe
the sound it produces? how computationally
expensive is it?

Here are a dozen different ways to use a 3-band
equalizer to achieve various audio effects. Each
scenario describes the parameter settings and the
resulting sound. The computational expense depends
on the specific algorithm and filter design used
in the equalizer, but generally, a 3-band
equalizer is not very computationally expensive.

1. **Subtle enhancement**: Slightly boost the low
   and high bands to add warmth and presence to
   a mix. Parameters: Low-band gain +1-2 dB,
   High-band gain +1-2 dB. Sound: Warm and
   slightly brighter.

2. **Mid scoop**: Cut the mid-band to create
   a scooped sound, common in heavy metal guitar
   tones. Parameters: Mid-band gain -4 to -6
   dB. Sound: Scooped, emphasizing low and high
   frequencies.

3. **Presence boost**: Boost the high-band to
   enhance the presence and clarity of vocals or
   instruments. Parameters: High-band gain +3 to
   +6 dB. Sound: Clearer, more detailed high
   frequencies.

4. **Bass boost**: Increase the low-band gain to
   emphasize the bass frequencies in
   a mix. Parameters: Low-band gain +4 to +6
   dB. Sound: Fuller, more powerful bass.

5. **Bass cut**: Reduce the low-band gain to
   decrease the bass frequencies and create
   a thinner sound. Parameters: Low-band gain -4
   to -6 dB. Sound: Thinner, less bass-heavy.

6. **High-frequency roll-off**: Decrease the
   high-band gain to create a warmer, more vintage
   sound. Parameters: High-band gain -3 to -6
   dB. Sound: Warmer, less bright.

7. **Mid boost**: Increase the mid-band gain to
   emphasize the midrange frequencies, which can
   help a vocal or instrument cut through
   a mix. Parameters: Mid-band gain +3 to +6
   dB. Sound: Richer midrange, more focused.

8. **Narrow notch filtering**: Use a narrow
   bandwidth and cut a specific frequency to
   reduce resonances or feedback. Parameters:
   Mid-band gain -12 to -18 dB, narrow
   bandwidth/Q. Sound: Reduced problematic
   frequencies.

9. **Radio effect**: Boost the mid-band while
   cutting the low and high bands to simulate the
   sound of a small radio speaker. Parameters:
   Low-band gain -6 dB, Mid-band gain +6 dB,
   High-band gain -6 dB. Sound: Tinny, narrow
   frequency range.

10. **Drum room enhancement**: Boost the low-band
    to emphasize the kick drum and the high-band
    to enhance the cymbals'
    brightness. Parameters: Low-band gain +3 dB,
    High-band gain +3 dB. Sound: Punchier kick,
    brighter cymbals.

11. **Vocal de-essing**: Reduce a narrow band
    around 5-8 kHz to decrease the harshness of
    sibilant sounds in vocals. Parameters:
    Mid-band gain -3 to -6 dB, narrow bandwidth/Q,
    center frequency 5-8 kHz. Sound: Smoother,
    less harsh sibilance.

12. **Lo-fi effect**: Cut the low and high bands
    to create a narrow, telephone-like
    sound. Parameters: Low-band gain -12 dB,
    High-band gain -12 dB. Sound: Narrow, focused
    midrange, reduced bass and treble.

Keep in mind that the exact resulting sound may
vary depending on the specific equalizer
implementation, the input audio, and any
additional processing in the signal chain.

47> why are some equalizers better than others?
could you describe how to implement the
mathematical differences which make some
equalizers shine? I'm considering units like the
SSL, for example, which are renowned for their
quality. what are some other highly renowned
equalizers and how do they work?

The quality of an equalizer depends on various
factors, such as its filter design, the type of
equalization it provides, its ease of use, and its
sonic characteristics. Here are some reasons why
some equalizers are considered better than others:

1. **Filter design**: High-quality equalizers
   often use more advanced filter designs that
   result in a more desirable frequency response
   or phase characteristics. For example, they may
   have minimal phase distortion or a more precise
   control over the shape of the equalization
   curve.

2. **Algorithm quality**: The algorithms used in
   digital equalizers can impact the sound
   quality. More advanced algorithms can lead to
   better sound quality by minimizing artifacts,
   such as aliasing and quantization noise.

3. **Ease of use**: Some equalizers are more
   intuitive and easier to use, allowing users to
   achieve the desired sound more quickly and
   efficiently. This can be attributed to
   well-designed user interfaces or specific
   features that simplify the equalization
   process.

4. **Sonic characteristics**: Some equalizers are
   known for their specific sonic characteristics,
   which can be attributed to the unique circuitry
   or algorithms used in their design. For
   example, analog equalizers can introduce
   harmonic distortion, saturation, or noise that
   adds a particular character to the sound, which
   can be desirable in certain contexts.

Some renowned equalizers and their
characteristics:

1. **SSL G-Series EQ**: This is a famous equalizer
   found in Solid State Logic's G-Series analog
   consoles. It's known for its smooth and musical
   sound, which is partly due to the specific
   circuitry used in the design, including the use
   of VCAs (Voltage Controlled Amplifiers) and
   a unique bell curve shape.

2. **API 550A**: This is a classic 3-band
   parametric equalizer known for its punchy and
   warm sound. It uses API's proprietary
   "Proportional Q" design, which automatically
   adjusts the bandwidth of the EQ curve based on
   the amount of boost or cut applied. This allows
   for a more natural and musical equalization.

3. **Neve 1073**: This is a legendary equalizer
   and preamp module that has been used in
   countless recordings. It's known for its warm
   and rich sound, which is due in part to the
   discrete Class A circuitry and the use of
   transformers for both the input and output
   stages.

4. **Pultec EQP-1A**: This is a passive tube
   equalizer that has been used in numerous
   classic recordings. It's known for its ability
   to add a smooth and warm character to the
   sound, largely due to the use of tubes and
   transformers in the design.

5. **Massenburg DesignWorks MDWEQ**: This is
   a digital equalizer designed by renowned
   engineer George Massenburg. It's known for its
   transparent and precise equalization, which is
   a result of the advanced algorithms and filter
   designs used in the software.

Each of these equalizers has its unique design,
and it's essential to understand their specific
features and characteristics to implement their
mathematical differences effectively. While it's
possible to emulate some aspects of these designs
in a digital equalizer, some characteristics, such
as the interaction of analog components and the
impact of transformers, may be more challenging to
replicate accurately.

48> how would you implement a 3 band equalizer in
verilog-a?

Implementing a 3-band equalizer in Verilog-A
involves designing filters that can alter the
amplitude of certain frequency components in the
input signal. The following steps can be taken to
implement a basic 3-band equalizer in Verilog-A:

1. Define the filter types: The three bands in the
   equalizer are usually defined by low-pass,
   band-pass, and high-pass filters. The filter
   types can be implemented using the
   corresponding transfer functions or circuit
   models.

2. Implement the filter transfer functions:
   Transfer functions can be implemented using
   standard mathematical functions such as
   logarithms, exponentials, and trigonometric
   functions. For example, the transfer function
   of a simple first-order low-pass filter can be
   defined as:

     HLP(s) = 1/(1 + s*TLP)

   where TLP is the time constant of the filter.

3. Connect the filters: The three filters can be
   connected in series, each filtering out certain
   frequency ranges. The output of one filter is
   connected to the input of the next filter.

4. Implement the gain control: Each filter can be
   adjusted to have a different gain in order to
   achieve the desired equalization curve. The
   gain can be controlled by a set of
   voltage-controlled amplifiers (VCAs).

Here is an example Verilog-A code that implements
a simple 3-band equalizer:

```
module three_band_eq(in, out, clk, rst);

  // Input and output signals
  electrical in, out;
  electrical clk, rst;

  // Define filter parameters
  real R1 = 1e3;
  real R2 = 2e3;
  real C1 = 1e-9;
  real C2 = 2e-9;

  // Define filter transfer functions
  real s = 2*pi*freq;
  real Hlp = 1/(1 + s*R1*C1);
  real Hbp = s*R2*C2/(1 + s*R2*C2);
  real Hhp = s*R1*C1/(1 + s*R1*C1);

  // Define gain control parameters
  real G1 = 0.5;
  real G2 = 0.7;
  real G3 = 1.0;

  // Implement the filters
  real v1 = in*v(G1*Hlp);
  real v2 = v1*v(G2*Hbp);
  real v3 = v2*v(G3*Hhp);

  // Output the filtered signal
  out <+ v3;

endmodule
```

This code defines three filter transfer functions
for low-pass, band-pass, and high-pass filters,
and then applies gain control to each filter to
achieve the desired equalization curve. The
filters are then connected in series to produce
the final output signal.
