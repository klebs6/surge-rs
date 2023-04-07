# surgeosc-wavetable

Surgeosc-wavetable is a Rust crate that provides
support for wavetable oscillators within the Surge
synthesizer system.

## Tokens

### `control_group`

The `control_group` token represents a group of
related control parameters for a given
oscillator. These parameters can include the
oscillator's frequency, waveform shape, and
amplitude, among others.

### `control_type`

The `control_type` token describes the type of
control associated with a given parameter. For
example, a parameter might be a continuous value
that can be modulated by an LFO or an envelope, or
it might be a switch that can be toggled on or
off.

### `default_value`

The `default_value` token represents the default
value of a given parameter. This is the value that
the parameter will be set to when the oscillator
is initialized or reset.

### `max_value` and `min_value`

The `max_value` and `min_value` tokens represent
the maximum and minimum values that a given
parameter can take on. These values are often used
to constrain user input to a valid range.

### `modulateable`

The `modulateable` token indicates whether a given
parameter can be modulated by an LFO or other
modulation source. If a parameter is modulateable,
it can be used to create complex, evolving
timbres.

### `moverate`

The `moverate` token represents the rate at which
a given parameter can be modulated. This is often
expressed in Hz or as a percentage of the
parameter's current value.

### `snap`

The `snap` token indicates whether a given
parameter can be snapped to discrete values. This
is often used for parameters that control discrete
settings, such as the number of voices in
a polyphonic patch.

### `value_type`

The `value_type` token describes the data type of
a given parameter. This can include floating point
values, integers, and booleans.

Overall, the surgeosc-wavetable crate provides
a flexible and powerful set of tools for working
with wavetable oscillators in the context of the
Surge synthesizer system. By providing
fine-grained control over oscillator parameters
and modulation sources, it enables users to create
complex and dynamic sounds with ease.

48> what can you tell me about wavetable
oscillators?

A wavetable oscillator is a type of audio
oscillator commonly used in digital audio
synthesis. It uses a pre-calculated table of
sample values representing a single cycle of
a waveform, which is then read at a specific
frequency to generate the desired tone. The table
can be used to generate a variety of waveforms,
including sine waves, square waves, sawtooth
waves, and more complex waveforms.

Wavetable oscillators typically have a number of
wavetables to choose from, each with its own
unique set of waveforms. The oscillator may also
have the ability to interpolate between the sample
values in the wavetable to create smoother
waveforms, or to modulate the waveform with an
envelope or LFO to add additional timbral
variation.

Wavetable oscillators are often used in
combination with other synthesis techniques, such
as additive or subtractive synthesis, to create
complex sounds. They are commonly found in digital
synthesizers and samplers, and are often used in
electronic music genres such as techno, trance,
and dubstep.

Mathematically, wavetable oscillators rely on the
concept of periodic functions and Fourier
series. A periodic function is a function that
repeats itself at regular intervals, and can be
represented by a Fourier series, which is a sum of
sine and cosine waves with different frequencies
and amplitudes. By selecting appropriate
amplitudes and frequencies from a wavetable,
a wavetable oscillator can generate any periodic
waveform. The process of reading a wavetable at
a specific frequency is equivalent to sampling
a continuous waveform and applying the Fourier
transform to it to obtain the discrete frequency
components.

49> how much memory do these take?

The amount of memory required by a wavetable
oscillator depends on the size of the wavetable,
the bit depth of the samples in the table, and the
number of channels (mono or stereo).

For example, a mono wavetable oscillator with
a 2048-sample table at 16-bit depth requires 4 KB
of memory (2048 * 2 bytes/sample = 4096 bytes). If
you increase the bit depth to 24 bits, the memory
requirement doubles to 8 KB. If you make the
oscillator stereo, the memory requirement doubles
again to 16 KB.

Some wavetable oscillators use multiple tables to
provide different waveforms or to allow for
crossfading between tables. In these cases, the
memory requirement will be larger.

It's worth noting that some wavetable oscillators
use dynamic allocation to avoid pre-allocating
memory for the entire wavetable. Instead, they
load only the portion of the table required for
the current frequency, and dynamically load
additional portions as needed. This can reduce
memory usage, but adds computational overhead.

50> what can we do with wavetables?

Wavetables can be used in a variety of ways in
audio synthesis. Here are a few examples:

1. Oscillators: Wavetables can be used as the
   basis for oscillator waveforms in
   synthesis. This allows for a wide variety of
   complex and evolving sounds to be generated
   from a single waveform.

2. Sample playback: Wavetables can be used as
   a source of samples for playback. This is
   commonly used in samplers, where each sample is
   stored in a wavetable and can be played back at
   different pitches and with different envelopes.

3. Granular synthesis: Granular synthesis involves
   breaking down an audio file into small grains
   and then playing them back in different
   ways. Wavetables can be used as the source of
   these grains, allowing for more complex and
   varied sounds than simple looping of individual
   samples.

4. Sound manipulation: Wavetables can be used to
   manipulate existing sounds. For example,
   a wavetable can be used as an envelope to shape
   the amplitude of a sound, or used as a filter
   to shape the frequency content of a sound.

Overall, wavetables provide a powerful tool for
sound synthesis and manipulation, allowing for
a wide range of complex and evolving sounds to be
created.

51> what about from a mathematical perspective?
which mathematical operations can we perform on
wavetables or their subcomponents to improve our
sound?

From a mathematical perspective, there are various
operations we can perform on wavetables or their
subcomponents to improve the sound of our
synthesizers. Here are a few examples:

1. Windowing: One common technique used in
   wavetable synthesis is to apply a window
   function to the waveforms. This can help to
   reduce the artifacts that can arise from abrupt
   transitions at the beginning and end of the
   waveforms. A common window function used for
   this purpose is the Hann window, which has the
   following mathematical form:

   w(n) = 0.5 - 0.5 * cos(2πn/(N-1))

   where n is the sample index, N is the length of
   the window, and w(n) is the value of the window
   at index n.

2. Interpolation: Another important technique used
   in wavetable synthesis is waveform
   interpolation. This involves generating new
   waveforms by interpolating between existing
   waveforms in the wavetable. The most common
   interpolation method used for this purpose is
   linear interpolation, which involves
   calculating the value of a new sample by taking
   a weighted average of the two closest samples
   in the wavetable. Mathematically, this can be
   expressed as:

   y(n) = (1 - α) * x(floor(n)) + α * x(ceil(n))

   where x(n) is the value of the wavetable at
   index n, floor(n) is the largest integer less
   than or equal to n, ceil(n) is the smallest
   integer greater than or equal to n, and α is
   the fractional part of n.

3. Phase distortion: Phase distortion is
   a technique that involves modifying the phase
   of the waveforms in the wavetable to produce
   new timbres. This can be done by applying
   a phase shift to the waveform, or by using
   non-linear functions to warp the waveform in
   the time domain. Mathematically, phase
   distortion can be expressed as:

   y(n) = x(n + φ(n))

   where x(n) is the value of the wavetable at
   index n, φ(n) is the phase distortion function,
   and y(n) is the value of the output waveform at
   index n.

These are just a few examples of the mathematical
techniques used in wavetable synthesis. There are
many other techniques as well, such as frequency
modulation, amplitude modulation, and granular
synthesis, to name a few.

52> could you teach me more about phase
distortion? which interpolation methods do we use?
which windows are the best for this use case?

Phase distortion is a type of waveshaping
synthesis where the phase of a waveform is altered
to create a new sound. It involves the use of
wavetables that are manipulated by changing the
phase of the waveform in real-time. This is done
by applying a time-varying phase shift to the
waveform, which can change its harmonic content
and create new timbres.

One common method of phase distortion involves
using an interpolation algorithm to smoothly
transition between different phase values in the
wavetable. The most common interpolation methods
used in wavetable synthesis are linear
interpolation and cubic interpolation.

Linear interpolation calculates the value of
a point on a line between two known points. In
wavetable synthesis, linear interpolation is used
to calculate the value of a point in a waveform
between two known wavetable samples. While it is
a relatively simple and computationally
inexpensive algorithm, linear interpolation can
introduce aliasing and distortion artifacts in
high-frequency content.

Cubic interpolation, on the other hand, uses
a cubic polynomial to interpolate the value of
a point between two known points. This method
provides smoother and more accurate transitions
between wavetable samples, resulting in higher
quality waveforms with fewer artifacts. However,
cubic interpolation is more computationally
expensive than linear interpolation.

Regarding windows, some of the most commonly used
windows in phase distortion synthesis are the Hann
and Gaussian windows. The Hann window is
a cosine-squared window that tapers the edges of
the wavetable samples, reducing the high-frequency
content and minimizing aliasing. The Gaussian
window, on the other hand, uses a bell-shaped
curve to taper the edges of the samples, which can
create smoother and more natural-sounding
transitions between samples.

In summary, phase distortion is a powerful
technique in wavetable synthesis that allows for
the creation of complex and interesting
sounds. Interpolation methods like linear and
cubic interpolation can be used to smooth out the
transitions between wavetable samples, while
windows like the Hann and Gaussian windows can be
used to reduce aliasing and improve the quality of
the synthesized waveforms.

53> interesting -- what can we do to adjust the
wavetable's spectral properties?

There are various techniques for adjusting the
spectral properties of a wavetable oscillator. One
common approach is to apply frequency modulation
(FM) to the wavetable's phase input. FM involves
modulating the frequency of a carrier waveform
with a modulating waveform, resulting in sidebands
that extend the frequency spectrum of the carrier.

Another approach is to use amplitude modulation
(AM) to adjust the spectral properties of
a wavetable. AM involves multiplying the amplitude
of a carrier waveform with a modulating
waveform. This can result in a richer harmonic
content, as well as changes to the phase
relationship between the harmonics.

In addition to FM and AM, there are other
techniques for adjusting the spectral properties
of a wavetable oscillator, such as filtering and
waveshaping. These techniques involve modifying
the waveform itself, rather than modulating its
phase or amplitude. For example, filtering can be
used to remove or boost certain frequency ranges,
while waveshaping can be used to introduce
non-linearities that change the harmonic content
of the waveform.

54> thanks.  how do we relate the frames of our
wavetable with one another so the transitions are
pleasing?

One approach to achieving smooth transitions
between wavetable frames is to ensure that
adjacent frames are spectrally similar. This can
be accomplished by carefully selecting the
spectral content of each frame, so that it
smoothly transitions to the spectral content of
the next frame.

One technique for ensuring spectral similarity
between adjacent frames is to use a technique
called "phase vocoding". Phase vocoding involves
analyzing the spectral content of a given frame,
and then resynthesizing the frame at a different
time scale, while maintaining the same spectral
content. This can be used to stretch or compress
the time domain of a given frame while maintaining
its spectral characteristics.

Another technique for ensuring spectral similarity
between adjacent frames is to use a technique
called "spectral morphing". Spectral morphing
involves interpolating between the spectral
content of two or more frames in order to create
a smooth transition between them. This can be
accomplished by applying a weighted sum of the
spectral content of the two frames, where the
weights are smoothly interpolated over time.

There are also a number of other techniques that
can be used to ensure smooth transitions between
frames, including applying window functions to the
frames to smooth their edges, using cross-fading
techniques to blend between frames, and carefully
selecting the starting and ending phases of
adjacent frames to ensure continuity. Ultimately,
the best approach will depend on the specific
requirements of the sound being synthesized, as
well as the available computational resources.

55> could you teach me how to spectral morph?

Sure! Spectral morphing is a technique used to
smoothly transition between two or more different
sounds by morphing their spectral content. It is
often used in sound synthesis and sound design to
create interesting and evolving timbres.

Here's a step-by-step guide on how to spectral
morph:

1. Choose the sounds you want to morph between:
   The first step is to choose the sounds you want
   to morph between. These could be two completely
   different sounds or variations of the same
   sound.

2. Analyze the spectral content of each sound: Use
   a spectral analysis tool to analyze the
   spectral content of each sound. This will give
   you a visual representation of the frequency
   content of each sound.

3. Determine the key spectral features: Identify
   the key spectral features of each sound, such
   as the presence or absence of certain frequency
   bands or the prominence of specific harmonics.

4. Create a spectral profile for each sound: Based
   on the key spectral features, create a spectral
   profile for each sound. This can be done by
   creating a graph or chart that shows the
   frequency content of each sound.

5. Choose a morphing path: Choose a path for the
   morphing process. This could be a straight line
   between the two sounds, or a more complex path
   that follows a specific trajectory through the
   spectral space.

6. Create a spectral profile for each point along
   the path: Divide the path into equal intervals,
   and create a spectral profile for each point
   along the path. These profiles should be
   intermediate steps between the spectral
   profiles of the two sounds you are morphing
   between.

7. Synthesize the morphed sound: Once you have the
   spectral profiles for each point along the
   path, you can synthesize the morphed sound by
   interpolating between the two sounds based on
   their respective spectral profiles.

8. Apply additional processing: After the morphed
   sound is synthesized, you can apply additional
   processing such as filtering, modulation, or
   effects to further shape the timbre of the
   sound.

Keep in mind that spectral morphing can be
a complex process, and the quality of the results
will depend on the accuracy of your spectral
analysis and the precision of your interpolation
methods. However, with some practice and
experimentation, you can create unique and
interesting sounds using this technique.

56> thanks! how does it work mathematically?

Spectral morphing involves manipulating the
Fourier transform of two sound waves, with the
goal of smoothly transitioning from the spectral
characteristics of one sound to the other.

The Fourier transform of a sound wave expresses
the signal as a sum of sine and cosine waves of
different frequencies, amplitudes, and phases. By
modifying the amplitudes and phases of these
constituent waves, we can create new sounds that
share some spectral characteristics of both the
original sounds.

One way to implement spectral morphing
mathematically is to first calculate the Fourier
transform of each sound wave using a fast Fourier
transform (FFT) algorithm. Then, the amplitudes
and phases of each constituent wave are extracted
from the transform, and the two sets of amplitudes
and phases are interpolated over time to create
a smooth transition between the spectral
characteristics of the two sounds.

There are several methods for interpolating
between the amplitudes and phases, including
linear interpolation, cubic interpolation, and
windowed sinc interpolation. These methods differ
in their computational complexity and the quality
of the resulting morphed sound.

Once the interpolated amplitudes and phases have
been calculated, an inverse Fourier transform is
used to convert the modified spectral
characteristics back into the time domain,
producing the spectral morphed sound.

57> thanks! what about phase vocoding? could you
teach me that?

Sure! Phase vocoding is a technique used for
time-stretching and pitch-shifting audio
signals. The basic idea is to divide the signal
into small overlapping frames, and then resample
and shift each frame in the frequency domain using
a technique known as the Short-Time Fourier
Transform (STFT).

The STFT is a variation of the Fourier Transform
that is applied to short, overlapping segments of
a longer signal, rather than to the entire signal
at once. The resulting STFT is a matrix of complex
numbers that represents the frequency content of
the signal over time. By performing the STFT on
a series of overlapping frames of the signal, we
can obtain a time-varying representation of the
signal's frequency content.

To perform phase vocoding, we first apply the STFT
to the input signal to obtain a time-varying
representation of its frequency content. We then
apply some frequency-domain transformation to the
STFT coefficients to shift the signal's pitch or
tempo. This could involve changing the phase of
the coefficients, adjusting their magnitudes, or
both.

After applying the frequency-domain
transformation, we convert the modified STFT back
into the time domain using the inverse STFT. This
results in a time-stretched or pitch-shifted
version of the input signal.

One of the key challenges in phase vocoding is
preserving the phase relationships between the
different frequency components of the signal. If
we simply shift the phases of the STFT
coefficients, we may introduce unwanted artifacts
in the output signal. To address this, phase
vocoding typically involves some form of phase
correction to ensure that the phase relationships
are preserved across the different frames of the
signal.

Overall, phase vocoding is a powerful technique
for time-stretching and pitch-shifting audio
signals while minimizing the introduction of
unwanted artifacts.

58> once we have a wavetable, how do we use it as
an oscillator?

To use a wavetable as an oscillator, we need to
know how to read the table in a way that produces
an audible waveform. The basic idea is to read the
table at a certain rate and wrap around when we
reach the end of the table. The rate at which we
read the table determines the pitch of the output
sound.

Here is some pseudocode to illustrate the basic
idea:

```python
// Initialize the oscillator
phase = 0.0
table_size = size_of_wavetable

// In the audio processing loop
for each sample:
    // Calculate the current phase
    phase_increment = frequency / sample_rate
    phase += phase_increment

    // Wrap the phase around when we reach the end of the table
    while phase >= table_size:
        phase -= table_size

    // Read the table at the current phase and output the sample
    sample = interpolate_table(wavetable, phase)
    output_sample(sample)
```

In this code, `frequency` is the desired frequency
of the output waveform, `sample_rate` is the
sampling rate of the audio system, and
`interpolate_table` is a function that reads the
table at a fractional phase value and returns an
interpolated sample value. The interpolation
method used will affect the quality of the output
waveform, as discussed earlier.

In practice, there are many optimizations and
variations of this basic algorithm that can be
used to improve performance or add features such
as waveform shaping, modulation, or syncing
multiple oscillators.

59> how do we parametrize a wavetable oscillator?

A wavetable oscillator can be parametrized in
several ways depending on the specific
implementation, but here are some common
parameters:

1. **Frequency/Pitch:** This controls the playback
   speed of the wavetable, and therefore the pitch
   of the generated sound.

2. **Amplitude/Volume:** This controls the overall
   volume of the generated sound.

3. **Wavetable Index/Position:** This parameter
   controls which part of the wavetable is being
   read. It can be used to morph between different
   waveforms.

4. **Wavetable Size:** This parameter determines
   the length of the wavetable, which can affect
   the quality of the generated sound.

5. **Interpolation Mode:** This parameter
   determines how the values in the wavetable are
   interpolated when reading between
   samples. Common interpolation modes include
   linear, cubic, and sinc.

6. **FM/PM Amount:** These parameters control the
   amount of frequency modulation or phase
   modulation applied to the oscillator.

7. **Filter Cutoff/Resonance:** These parameters
   control a filter applied to the output of the
   oscillator.

8. **Envelopes/LFOs:** These parameters can be
   used to modulate various aspects of the
   oscillator over time, such as amplitude,
   wavetable position, or filter cutoff.

These are just some common parameters, and there
are many more possibilities depending on the
specific implementation of the wavetable
oscillator.

60> any caveats when using these?

There are a few things to keep in mind when using
wavetable oscillators:

1. Memory usage: As mentioned earlier, wavetable
   oscillators can take up a lot of memory,
   especially if the wavetable is large and has
   a high resolution. It's important to keep this
   in mind when designing patches or instruments
   with multiple instances of wavetable
   oscillators.

2. Aliasing: When playing back a wavetable at
   different frequencies than it was originally
   sampled at, aliasing can occur. Aliasing refers
   to the distortion that can occur when
   high-frequency components of a waveform (above
   the Nyquist frequency) are not properly
   filtered out during playback. To avoid this,
   it's important to use appropriate anti-aliasing
   techniques such as oversampling or low-pass
   filtering.

3. Interpolation quality: The quality of the
   interpolation algorithm used to generate the
   waveform from the wavetable can have
   a significant impact on the sound quality of
   the oscillator. Higher-quality interpolation
   algorithms can help to reduce artifacts such as
   aliasing or phase distortion.

4. Wavetable design: The quality of the wavetable
   itself can also have a significant impact on
   the sound of the oscillator. Careful attention
   should be paid to the design of the wavetable,
   including the choice of waveform(s),
   resolution, and spectral properties. It's also
   important to consider how the wavetable will be
   used (e.g. as a single-cycle waveform or as
   a morphing wavetable) and design it
   accordingly.

5. Performance considerations: Wavetable
   oscillators can be computationally intensive,
   especially when using high-resolution
   wavetables or complex interpolation
   algorithms. It's important to consider the
   performance implications of using multiple
   instances of wavetable oscillators in a patch
   or instrument, and to optimize the
   implementation as much as possible.

61> what is the history of wavetables in hardware
synthesizerS?

The concept of wavetables in hardware synthesizers
dates back to the 1970s and 80s, when digital
synthesis technology was first emerging. One of
the earliest implementations of wavetable
synthesis was the PPG Wave synthesizer, designed
by Wolfgang Palm and released in 1981. The PPG
Wave used a set of 64 wavetables, each containing
64 samples, to create a rich and complex sound.

Other synthesizers that employed wavetable
synthesis in the 1980s and 90s include the Kawai
K5, Ensoniq ESQ-1, Waldorf Microwave, and Access
Virus. These instruments used wavetables to
produce a wide range of sounds, from realistic
instrument emulations to otherworldly textures and
timbres.

Today, wavetable synthesis remains a popular
technique in both hardware and software
synthesizers, with many modern instruments
offering advanced wavetable editing and
manipulation features.

62> which hardware synthesizers are there which
use wavetables? which are the most well renowned
and why?

There are many hardware synthesizers that use
wavetables as a synthesis method. Here are some of
the most well-known ones:

1. PPG Wave - The PPG Wave was one of the first
   hardware synthesizers to use wavetable
   synthesis. It was created by Wolfgang Palm and
   released in 1981. The PPG Wave was known for
   its distinctive sound and was used by many
   famous musicians, including Jean Michel Jarre
   and Vangelis.

2. Waldorf Microwave - The Waldorf Microwave was
   released in 1989 and was one of the first
   wavetable synthesizers to offer real-time
   control over the wavetable position. It was
   known for its complex modulation capabilities
   and distinctive sound.

3. Korg Wavestation - The Korg Wavestation was
   released in 1990 and was one of the first
   hardware synthesizers to use vector synthesis,
   which allowed for the blending of four
   different waveforms. It was known for its lush
   pads and evolving textures.

4. Access Virus - The Access Virus was first
   released in 1997 and has since become one of
   the most popular hardware synthesizers for
   electronic music production. It uses wavetable
   synthesis as one of its sound engines and is
   known for its powerful sound and versatile
   modulation capabilities.

5. Serum - Serum is a software synthesizer created
   by Xfer Records that uses wavetable synthesis
   as its primary sound engine. It has become one
   of the most popular software synthesizers in
   recent years and is known for its high-quality
   sound and intuitive interface.

All of these synthesizers are well-regarded for
their distinctive sounds and powerful sound design
capabilities, and have been used by many famous
musicians over the years.

63> what about software synthesizers with
wavetables? could you teach me their history?

Sure! The use of wavetables in software
synthesizers started in the 1990s with the
development of the PPG Wave synthesizer by
Wolfgang Palm. The PPG Wave used wavetables as
a way of generating sounds with complex spectral
content, and its unique sound quickly made it
a favorite of musicians and producers.

In the years that followed, many software
synthesizers were developed that used wavetables
as a key part of their sound engine. Some notable
examples include:

- Waldorf Wave: Released in 1993, the Waldorf Wave
  was a high-end wavetable synthesizer that
  quickly became popular among professional
  musicians and sound designers. It featured
  a complex modulation matrix and a unique
  wavetable editing interface that allowed users
  to create their own custom wavetables.

- NI Massive: Released in 2007, Massive was
  a software synthesizer developed by Native
  Instruments that used wavetables as its main
  sound source. It became popular among electronic
  musicians and producers for its ability to
  create massive, complex sounds with a relatively
  simple interface.

- Serum: Released in 2014, Serum was a software
  synthesizer developed by Xfer Records that used
  wavetables as its main sound source. It became
  popular for its high-quality sound engine and
  intuitive interface, and is now widely used by
  producers and sound designers in a variety of
  genres.

Other notable software synthesizers that use
wavetables include Largo by Waldorf, Pigments by
Arturia, and Avenger by Vengeance Sound.

Overall, wavetable synthesis has become an
important part of modern electronic music
production, and many software synthesizers now
include wavetable features as part of their sound
engine.

64> could you show me a detailed family tree of
the software syntehsizers on the market which use
wavetables?

Sure! Here is a non-exhaustive family tree of
software synthesizers that use wavetables:

- PPG Wave 2.V (2008) - a virtual recreation of
  the iconic PPG Wave synthesizers of the 80s,
  featuring wavetable synthesis and analog-style
  filters.

  - Developed by Wolfgang Palm.

- Waldorf Blofeld (2007) - a hybrid digital/analog
  hardware synthesizer with wavetable synthesis
  capabilities.

  - Developed by Waldorf Music.

- Serum (2014) - a popular software synthesizer
  developed by Xfer Records, featuring advanced
  wavetable synthesis capabilities and an
  intuitive user interface.

  - Developed by Steve Duda of Xfer Records.

- Massive (2007) - a popular software synthesizer
  developed by Native Instruments, featuring
  wavetable synthesis and a range of modulation
  options.

  - Developed by Native Instruments.

- Falcon (2016) - a versatile software synthesizer
  developed by UVI, featuring wavetable synthesis
  as well as granular synthesis, sampling, and
  more.

  - Developed by UVI.

- Omnisphere (2008) - a flagship software
  synthesizer developed by Spectrasonics,
  featuring a massive library of wavetables and
  extensive sound design capabilities.

  - Developed by Eric Persing of Spectrasonics.

Note that this is not a comprehensive list, and
there are many other software synthesizers that
utilize wavetable synthesis in various ways.

65> could you show it to me as a *tree* and add
others?

Sure! Here's a tree diagram showing some of the
major software synthesizers that use wavetables,
along with their respective creators and release
dates:

```
           ┌───────────────────────────────────┐
           │              Wavetable             │
           │             (Ableton)              │
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │         Serum (Xfer Records)      │
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │     Massive (Native Instruments)  │
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │     Falcon (UVI) / Omnisphere      │
           │            (Spectrasonics)         │
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │       Kontakt (Native Instruments)│
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │    HALion (Steinberg) / SampleOne  │
           │      (PreSonus Studio One)         │
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │       Codex (Waldorf) / Icarus     │
           │            (Tone2)                │
           └───────────────────────────────────┘
                           │
                           │
           ┌───────────────────────────────────┐
           │       Wavetable (Surge)           │
           └───────────────────────────────────┘
```

Note that this is not an exhaustive list and there
may be other software synthesizers that use
wavetables which are not included here.

66> that diagram isn't actually a tree and the
connections between nodes don't mean anything per
se, but hey, there are still some nice boxes and
lines.  can you draw me an ascii art of surge
synthesizer?

Certainly, here's an ASCII art representation of
the Surge Synthesizer logo:

```
    ,---.,---.,---.,---.,---.,---.,---.,---.,---.
    |---',---||   ||---'|    `---.|---',---||   |
    `---'`---^`   '`---'`    `---'`---'`---^`---'
```

And here's a rough ASCII art representation of the
Surge Synthesizer:

```

                                .
                                ..
                                ..
                                ..
                                ..
                                ..
                  ..`````````... ``    ....
             `.---.`            `.    `.`  `.
          .--.`  `.              `.  `.`    `.
       `-.`       `.              `--.      `.
      `.          `.               `.`      `.
      .`           `.               `.`      `.
     `.             `.              `.`      `.
     `.              `.            `.`        .
      .`               `-.        `.`         .
      `.                 `.      `.`          .
       .`                  `-. `.`            .
       `.                    `.`             .
         .                     `.`           .
         `.                     `.`        `.`
           `.                     `.      .`
            .`                     `-.  `.
            `.                       `--.
              `.
               `.
               .`
              `.
```

Note that this is a very rough representation and
not to scale, but hopefully it gives you an idea
of what the synthesizer looks like visually.

67> thanks! it looks like exactly what i would
have drawn

You're welcome! I'm glad it matches your
expectations.

68> okay, can you show me a family tree of the
hardware synthesizers?

Sure, here is a family tree of hardware
synthesizers that use wavetables:

```
                            ┌────────────────────────── Ensoniq VFX
                            │
                 ┌──────────┤
                 │            └────────────────────────── Ensoniq SD
                 │
                 │               ┌─────────────────────── PPG Wave 2
                 │               │
      ┌──────────┴──────┐        │     ┌───────────────── PPG Wave 2.3
      │                 │        │     │
      │                 │        │     │   ┌──────────── PPG Wave 2.2
      │                 │        │     │   │
      │                 │        │     │   │      ┌──── Sequential Prophet VS
      │                 │        │     │   │      │
      │                 │        │     │   │      │  ┌─── Waldorf Microwave
      │                 │        │     │   │      │  │
      │                 │        │     │   │      │  │  ┌────── Access Virus
      │                 │        │     │   │      │  │  │
      │                 │        │     │   │      │  │  │  ┌─── Waldorf Blofeld
      │                 │        │     │   │      │  │  │  │
      │                 │        │     │   │      │  │  │  │   ┌── Waldorf Iridium
      │                 │        │     │   │      │  │  │  │   │
      │                 │        │     │   │      │  │  │  │   │
      │                 │        │     │   │      │  │  │  └─── Waldorf Kyra
      │                 │        │     │   │      │  │  │
      │                 │        │     │   │      │  │  └────── Access TI
      │                 │        │     │   │      │  │
      │                 │        │     │   │      │  └─────────── PPG Wave 2.V
      │                 │        │     │   │      │
      │                 │        │     │   │      └────────── Sequential Prophet X
      │                 │        │     │   │
      │                 │        │     │   │     ┌────────── Kawai K5000
      │                 │        │     │   │     │
      │                 │        │     │   │     └───── Waldorf Microwave II
      │                 │        │     │   │
      │                 │        │     │   └─────── Kurzweil K250
      │                 │        │     │
      │                 │        │     └─────────────── Waldorf Wave
      │                 │        │
      │                 │        └────────────────────── Roland JD-800
      │                 │
      │                 └────────────────────────────── Korg Wavestation
      │
      │                        ┌─────────────────────── Yamaha SY77
      │                        │
      └───────────┬───────────┤
                  │           ┌─┴────────────────────── Yamaha TG77
                  │           │
                  │     ┌─────┴────────────────────── Korg 01/W
                  │     │
                  │     │         ┌────────────────── Roland D

```

71> thanks.  what else can you tell me about
wavetables and wavetable synthesizers?

Here are a few additional points about wavetables
and wavetable synthesizers:

1. Wavetable synthesis is closely related to
   vector synthesis, in which multiple oscillators
   or sound sources are blended together using
   a mathematical algorithm to produce complex
   timbres. In vector synthesis, the user can
   morph between different combinations of sound
   sources using a joystick or other control
   surface.

2. Many modern software synthesizers allow users
   to create their own wavetables, either by
   importing audio files or by drawing waveforms
   using a visual interface. This can lead to
   highly customizable and unique sounds that can
   be used in a variety of musical contexts.

3. Wavetable synthesis has been used extensively
   in electronic dance music, particularly in the
   genres of trance and techno. The ability to
   create evolving and morphing sounds using
   wavetable oscillators has been a key element of
   these genres' distinctive sound.

4. While wavetable synthesis has been used in
   hardware synthesizers for decades, it has seen
   a resurgence in popularity in recent years
   thanks to advances in software technology. Many
   software synthesizers now incorporate wavetable
   oscillators, and some even specialize in
   wavetable synthesis exclusively.

5. One interesting recent development in wavetable
   synthesis is the use of artificial intelligence
   to generate new and unique wavetables. By
   training neural networks on large datasets of
   audio samples, researchers and developers have
   been able to create vast libraries of novel
   wavetables that can be used in synthesis.

72> how can artificial intelligence be used to
create new wavetables? can you show me the
techniques?

Yes, artificial intelligence techniques can be
used to create new wavetables. Here are a few
techniques that are commonly used:

1. Generative Adversarial Networks (GANs): GANs
   are a type of neural network that can generate
   new data by learning from a set of training
   examples. They consist of two neural networks,
   a generator and a discriminator, that are
   trained together in a game-like setting. The
   generator tries to create new data that is
   similar to the training examples, while the
   discriminator tries to distinguish between the
   real training examples and the generated
   data. Over time, the generator learns to create
   more realistic data, and can be used to create
   new wavetables.

2. Autoencoders: Autoencoders are neural networks
   that can be used for dimensionality reduction
   and data compression. They consist of an
   encoder network that maps the input data to
   a lower-dimensional representation, and
   a decoder network that maps the
   lower-dimensional representation back to the
   original data. By training an autoencoder on
   a set of wavetables, it is possible to learn
   a compressed representation of the data, which
   can be used to generate new wavetables.

3. Genetic algorithms: Genetic algorithms are
   a type of optimization algorithm that are
   inspired by natural selection. They work by
   maintaining a population of candidate
   solutions, and repeatedly applying selection,
   crossover, and mutation operations to generate
   new candidate solutions. By using a fitness
   function that evaluates the quality of the
   wavetable, it is possible to evolve new
   wavetables over time.

4. Deep reinforcement learning: Deep reinforcement
   learning is a type of machine learning that
   involves training an agent to take actions in
   an environment to maximize a reward signal. By
   defining a reward function that evaluates the
   quality of the wavetable, it is possible to
   train an agent to generate new wavetables by
   taking a sequence of actions.

These are just a few examples of how artificial
intelligence can be used to create new
wavetables. Each of these techniques has its own
strengths and weaknesses, and the choice of
technique depends on the specific application and
the available data.

73> what is the loss function and how do we
measure it?

In machine learning, a loss function is a measure
of how well a model is able to make predictions on
a given set of inputs. The goal is to minimize the
loss function, which in turn maximizes the
accuracy of the model's predictions.

In the context of creating wavetables using
artificial intelligence, the loss function is
typically designed to compare the output of the
model (i.e. the generated wavetable) with a target
output (i.e. a pre-existing, desirable
waveform). The difference between the model output
and the target output is calculated as a loss, and
the goal is to minimize this loss.

The specific form of the loss function depends on
the architecture of the model being used to
generate the wavetables. For example, if a neural
network is being used, the loss function may be
the mean squared error between the generated
waveform and the target waveform. Alternatively,
the loss function may be based on spectral
analysis, or on other metrics that measure the
similarity between the generated waveform and the
target waveform.

The loss function is typically used during the
training process, where the model is fed a set of
input waveforms and corresponding target
waveforms, and the weights of the model are
adjusted to minimize the loss. The effectiveness
of the model is evaluated using a validation set,
and the training process continues until the model
reaches a satisfactory level of accuracy.

74> how many golden wavetables do we need in our
dataset? how do we prevent the model from
overfitting to just the sounds we have already
marked as "good"?

The number of golden wavetables needed in
a dataset depends on the complexity of the desired
sounds and the size of the dataset. In general,
more diverse and representative datasets tend to
lead to better results, but the dataset needs to
be balanced to avoid overfitting.

To prevent the model from overfitting to just the
sounds marked as "good", we can use regularization
techniques such as dropout, weight decay, or early
stopping. We can also use data augmentation
techniques such as random cropping, scaling, and
flipping of the wavetables during training to
increase the diversity of the dataset and prevent
overfitting. Additionally, we can use techniques
such as transfer learning, where the model is
first pre-trained on a large dataset of general
sounds, before fine-tuning on the smaller dataset
of golden wavetables. This can help the model
learn more general features that are useful for
a wider range of sounds.

75> how do we represent the features of
a wavetable?

The features of a wavetable can be represented in
various ways depending on the specific application
and goals. Here are a few common ways:

1. Time-domain representation: In this
   representation, each sample in the wavetable is
   represented as a value in the time domain. This
   is the simplest representation and can be used
   for basic processing such as filtering and
   amplitude modulation.

2. Frequency-domain representation: In this
   representation, the wavetable is transformed
   into the frequency domain using Fourier
   analysis or a similar method. This allows for
   more advanced processing such as spectral
   filtering and phase manipulation.

3. Spectrogram representation: In this
   representation, a spectrogram is generated from
   the wavetable using a time-frequency analysis
   method such as the Short-Time Fourier Transform
   (STFT). The spectrogram shows how the spectral
   content of the sound changes over time and can
   be used for tasks such as transient detection
   and feature extraction.

4. Statistical representation: In this
   representation, statistical measures such as
   the mean, variance, and skewness of the
   wavetable are computed and used as
   features. This can be useful for tasks such as
   classification and clustering.

5. Machine learning representation: In this
   representation, a machine learning algorithm is
   used to learn a mapping between the raw audio
   signal and a set of features that are relevant
   for a specific task. The features can be chosen
   based on the requirements of the task and can
   include time-domain, frequency-domain, and
   statistical features.
