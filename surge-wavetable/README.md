# `surge-wavetable`

`surge-wavetable` is a Rust crate that provides
functionality for creating, manipulating, and
working with wavetables in the context of the
Surge synthesizer system.

## Wavetable Synthesis

Wavetable synthesis is a technique used in audio
synthesis to create sounds by interpolating
between pre-recorded waveforms, called
wavetables. The sound produced by a wavetable
synthesizer can be modified by changing the
frequency at which the wavetable is read, as well
as by applying various modulation techniques to
the waveform.

## Wavetable Building

The `surge-wavetable` crate provides functionality
for building and working with wavetables. The
`WaveTable` struct represents a wavetable, and
includes metadata such as the number of samples in
each table, the number of tables in the wavetable,
and the number of mipmaps (pre-generated lower
resolution versions of the wavetable) that should
be generated.

The `populate_mipmaps` function can be used to
populate the mipmaps for a wavetable, while the
`populate_base_mipmap_level` function can be used
to populate the base mipmap level for a wavetable.

## Reading Wavetables

The `wavetable_read` function can be used to read
a wavetable at a specified frequency and
phase. The resulting waveform can be modified
using various modulation techniques, such as phase
modulation or frequency modulation.

## Other Functionality

The `surge-wavetable` crate also includes
functionality for loading and working with patch
formats and extensions, as well as utility
functions for converting between various data
types.

## Mathematical Ideas

Wavetable synthesis involves mathematical
techniques such as interpolation and modulation to
generate complex waveforms from pre-recorded
waveforms. The `surge-wavetable` crate uses
mathematical techniques such as linear
interpolation to read wavetables at non-integer
frequencies, as well as modulation techniques such
as phase modulation and frequency modulation to
modify the waveform. The `populate_mipmaps`
function uses techniques such as downsampling and
low-pass filtering to generate lower resolution
versions of the wavetable.

Overall, the `surge-wavetable` crate provides
powerful functionality for working with wavetables
in the context of the Surge synthesizer system.

9> could you teach me about wavetables in
mathematical detail?

Sure! Wavetable synthesis involves using
pre-recorded waveforms, called wavetables, as
building blocks to create complex sounds. The
basic idea behind wavetable synthesis is to read
the waveform from the wavetable at a specific
frequency and phase, and then use various
modulation techniques to modify the waveform.

Mathematically, a wavetable can be represented as
a sequence of samples, where each sample
represents the amplitude of the waveform at
a specific point in time. The sequence of samples
can be thought of as a function of time, where the
value of the function represents the amplitude of
the waveform at that point in time.

When a wavetable is read at a specific frequency
and phase, the waveform is effectively stretched
or compressed in time to fit the desired
frequency. This can be achieved by reading the
wavetable at non-integer sample indices, which
effectively stretches or compresses the waveform.

For example, if a 1024-sample wavetable is being
read at a frequency of 440 Hz (A4), the index into
the wavetable can be calculated as follows:

```
index = time * sample_rate * (1/frequency)
```

where `time` is the current time in seconds,
`sample_rate` is the sampling rate of the
wavetable in samples per second, and `frequency`
is the desired frequency in Hertz. If `index` is
not an integer, linear interpolation can be used
to interpolate between the nearest samples in the
wavetable to get the desired amplitude.

Once the waveform has been read from the
wavetable, various modulation techniques can be
applied to modify the waveform. For example,
frequency modulation (FM) involves modulating the
frequency of the waveform using another waveform,
while amplitude modulation (AM) involves
modulating the amplitude of the waveform using
another waveform.

In summary, wavetable synthesis involves using
pre-recorded waveforms, called wavetables, as
building blocks to create complex sounds. The
waveform is read from the wavetable at a specific
frequency and phase, and then modified using
various modulation techniques. Mathematically, the
wavetable can be represented as a sequence of
samples, and linear interpolation can be used to
read the waveform at non-integer sample indices.

10> thanks -- what are the related mathematical
operations we can use with them to make them
better sounding and more flexible?

There are several related mathematical operations
that can be used with wavetables to make them more
flexible and better sounding. Here are a few
examples:

1. **Windowing:** Windowing is a technique that
   involves multiplying the wavetable samples by
   a window function before performing any other
   operations. Windowing can be used to shape the
   spectrum of the waveform, which can improve the
   sound quality and reduce unwanted
   noise. Examples of window functions include the
   Hamming window, Hann window, and Blackman
   window.

2. **Phase Distortion:** Phase distortion involves
   modifying the phase of the waveform to create
   new harmonic content. Phase distortion can be
   used to create complex, evolving sounds, and
   can be especially effective when used with
   wavetables that have simple harmonic
   content. Phase distortion can be implemented
   using functions such as arctan, cubic, or sine
   modulation.

3. **Additive Synthesis:** Additive synthesis
   involves adding together multiple waveforms to
   create complex sounds. Additive synthesis can
   be used with wavetables by combining multiple
   wavetables together, each with a different
   harmonic content. The harmonic content of the
   resulting waveform can be controlled by
   adjusting the amplitude of each wavetable.

4. **Granular Synthesis:** Granular synthesis
   involves breaking up a waveform into small
   grains and reassembling them in different ways
   to create new sounds. Granular synthesis can be
   used with wavetables by breaking the waveform
   into short segments, and then rearranging the
   segments to create new sounds. The duration and
   position of each segment can be controlled
   using various modulation techniques.

5. **Frequency Modulation:** Frequency modulation
   (FM) involves modulating the frequency of the
   waveform using another waveform. FM can be used
   with wavetables by applying a modulating
   waveform to the frequency index used to read
   the wavetable. This can create complex,
   evolving sounds with a rich harmonic content.

These are just a few examples of the mathematical
operations that can be used with wavetables to
create complex and interesting sounds. By
combining these operations and experimenting with
different wavetables and modulation sources, it is
possible to create an endless variety of sounds
with wavetable synthesis.

11> how much memory do they take? how do we
generate the wavetables?

The amount of memory required for a wavetable
depends on the length and complexity of the
waveform and the number of wavetables used in
a given synth patch. In general, a single
wavetable with a few thousand samples may only
require a few kilobytes of memory, while larger or
more complex wavetables may require several
megabytes of memory.

There are several ways to generate wavetables. One
common method is to record a real instrument or
sound, and then extract a single cycle of the
waveform to use as the basis for the
wavetable. The waveform can then be manipulated
using various techniques, such as additive
synthesis or phase distortion, to create different
harmonics and timbres.

Another method is to generate waveforms using
mathematical functions, such as sine waves or
sawtooth waves. These waveforms can be combined to
create more complex waveforms and used as the
basis for a wavetable.

In addition, there are software tools that can be
used to generate and edit wavetables. These tools
often provide a visual interface for manipulating
the waveform and can simplify the process of
creating complex or custom wavetables.

Once a wavetable has been generated, it can be
used in a synthesizer to create sounds. The
wavetable is typically loaded into memory when the
synthesizer is initialized, and can then be read
and modified in real-time to create a wide range
of sounds.

12> how can we engineer their spectral properties?

There are several techniques that can be used to
engineer the spectral properties of a wavetable,
which can have a significant impact on the sound
quality and timbre of the resulting waveform. Here
are a few examples:

1. **Windowing:** Windowing is a technique that
   involves multiplying the wavetable samples by
   a window function before performing any other
   operations. Windowing can be used to shape the
   spectrum of the waveform, which can improve the
   sound quality and reduce unwanted
   noise. Different window functions can be used
   to achieve different spectral
   characteristics. For example, a Hann window can
   be used to reduce spectral leakage and improve
   the spectral resolution of the waveform.

2. **Additive Synthesis:** Additive synthesis
   involves adding together multiple waveforms to
   create complex sounds. Additive synthesis can
   be used with wavetables by combining multiple
   wavetables together, each with a different
   harmonic content. The harmonic content of the
   resulting waveform can be controlled by
   adjusting the amplitude of each wavetable. This
   can be used to engineer the spectral properties
   of the waveform and create a wide range of
   timbres.

3. **Phase Distortion:** Phase distortion involves
   modifying the phase of the waveform to create
   new harmonic content. Phase distortion can be
   used to create complex, evolving sounds, and
   can be especially effective when used with
   wavetables that have simple harmonic
   content. Phase distortion can be used to
   engineer the spectral properties of the
   waveform and create new timbres.

4. **Frequency Modulation:** Frequency modulation
   (FM) involves modulating the frequency of the
   waveform using another waveform. FM can be used
   with wavetables by applying a modulating
   waveform to the frequency index used to read
   the wavetable. This can create complex,
   evolving sounds with a rich harmonic content,
   and can be used to engineer the spectral
   properties of the waveform.

5. **Granular Synthesis:** Granular synthesis
   involves breaking up a waveform into small
   grains and reassembling them in different ways
   to create new sounds. Granular synthesis can be
   used with wavetables by breaking the waveform
   into short segments, and then rearranging the
   segments to create new sounds. The duration and
   position of each segment can be controlled
   using various modulation techniques, which can
   be used to engineer the spectral properties of
   the waveform.

By combining these techniques and experimenting
with different wavetables and modulation sources,
it is possible to engineer the spectral properties
of a wavetable to create an endless variety of
sounds with different timbres and characteristics.

13> which windows do we use?

There are many different window functions that can
be used to shape the spectrum of a waveform. The
choice of window function depends on the desired
spectral properties of the waveform and the
specific application. Here are a few examples of
common window functions used in wavetable
synthesis:

1. **Rectangular Window:** The rectangular window
   is the simplest window function and is
   essentially a straight line with a constant
   value of 1.0. It does not modify the spectral
   content of the waveform, and is often used when
   a flat frequency response is desired.

2. **Hamming Window:** The Hamming window is
   a popular window function that has a smoother
   taper than the rectangular window. It is
   defined by the following formula:

   `w(n) = 0.54 - 0.46 * cos(2 * pi * n / N)`

   where `n` is the sample index, `N` is the total
   number of samples in the window, and `cos` is
   the cosine function. The Hamming window
   provides a good balance between spectral
   resolution and spectral leakage, and is often
   used in applications where a smooth frequency
   response is desired.

3. **Hann Window:** The Hann window is similar to
   the Hamming window, but has a wider taper that
   extends to zero at both ends of the window. It
   is defined by the following formula:

   `w(n) = 0.5 * (1 - cos(2 * pi * n / N))`

   where `n` is the sample index, `N` is the total
   number of samples in the window, and `cos` is
   the cosine function. The Hann window provides
   a good balance between spectral resolution and
   spectral leakage, and is often used in
   applications where a smooth frequency response
   is desired.

4. **Blackman Window:** The Blackman window is
   a window function with a wider taper than the
   Hamming and Hann windows, which makes it more
   effective at reducing spectral leakage. It is
   defined by the following formula:

   `w(n) = 0.42 - 0.5 * cos(2 * pi * n / N) + 0.08 * cos(4 * pi * n / N)`

   where `n` is the sample index, `N` is the total
   number of samples in the window, and `cos` is
   the cosine function. The Blackman window
   provides a good balance between spectral
   resolution and spectral leakage, and is often
   used in applications where a smooth frequency
   response is desired.

There are many other window functions that can be
used in wavetable synthesis, each with its own
unique properties and trade-offs. The choice of
window function depends on the specific
application and the desired spectral properties of
the waveform.
