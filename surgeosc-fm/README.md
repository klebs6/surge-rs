# FM Oscillator for the Surge synthesizer system

The `surgeosc-fm` crate provides an implementation
of a Frequency Modulation (FM) Oscillator for the
Surge synthesizer system. This oscillator allows
the creation of complex and rich sounds by
modulating the frequency of one waveform (carrier)
with the amplitude of another waveform
(modulator). This FM synthesis technique is widely
used in electronic music production, offering
a vast range of sonic possibilities.

In the context of FM synthesis, the core
mathematical concept is frequency modulation,
represented by the following equation:

`output(t) = A * sin(2 * π * (f_c + (B * f_m) * sin(2 * π * f_m * t)) * t)`

where:
- `output(t)` is the oscillator output at time `t`
- `A` is the amplitude
- `f_c` is the carrier frequency
- `B` is the modulation index
- `f_m` is the modulator frequency

This crate provides the following tokens:

- `control_group`, `control_type`,
  `default_value`, `max_value`, `min_value`,
  `modulateable`, `moverate`, and `value_type`:
  These tokens are used to define and manage
  oscillator parameters, such as carrier
  frequency, modulation index, and waveform types.

- `FMOscillator`: The struct representing the FM
  oscillator, which holds the oscillator state and
  configuration.

- `do_fm_block`: A helper function to perform the
  core FM synthesis algorithm on a block of
  samples.

- `init`: A method to initialize the FM oscillator
  with the appropriate settings and parameter
  values.

- `process_block`: A method that processes a block
  of samples by applying the FM synthesis
  algorithm, using the `do_fm_block` function.

To use this crate, you'll need to integrate it
into your Surge synthesizer system or a compatible
application, ensuring proper initialization and
parameter management. By leveraging the power of
FM synthesis, you'll be able to create an
extensive range of sounds, from classic FM
electric pianos and bells to evolving textures and
complex harmonic structures.

9> /file surge-rs/surgeosc-fm/src/process.rs

The `FMOscillator` implementation provided here is
a frequency modulation (FM) oscillator that
processes audio in blocks. The algorithm uses
three modulators with different amounts and
frequency ratios or absolute frequencies, as well
as feedback to create a wide variety of
sounds. The implementation is optimized for the
Surge synthesizer system and processes audio
samples in chunks defined by `BLOCK_SIZE_OS`.

Let's break down the `process_block` function:

1. The method starts by retrieving the current
   parameter values for modulator amounts,
   frequency ratios, and absolute frequencies, as
   well as the feedback amount.

2. The oscillator pitch is updated with drift,
   which introduces small random variations to the
   pitch over time to simulate analog oscillator
   behavior.

3. The rates for each modulator are calculated
   based on the drifted pitch, frequency ratios,
   and absolute frequencies.

4. The modulation depths for each modulator are updated.

5. The FM depth is updated if FM is enabled.

6. The feedback depth is updated.

7. The `do_fm_block` function is called for each
   sample in the block to apply the FM algorithm.

8. If stereo processing is enabled, the left
   channel is duplicated to the right channel.

10> /file surge-rs/surgeosc-fm/src/block.rs

The `do_fm_block` function performs the core FM
synthesis operation on a single audio sample. It
takes the current sample index `k`, the carrier
waveform frequency `omega`, and a boolean flag
`fm` that indicates whether FM synthesis is
enabled or not.

Let's walk through the function step by step:

1. The function starts by retrieving the master
   oscillator's value at the current sample index
   `k`.

2. The three modulator oscillators (`rm1`, `rm2`,
   and `am`) are processed.

3. The output for the left channel is calculated
   by adding the current phase, the contributions
   of the three modulators, and the last output
   scaled by the feedback depth.

4. If FM synthesis is enabled, the contribution
   from the master oscillator is added, scaled by
   the FM depth.

5. The sine function is applied to the output
   sample to generate the final waveform.

6. The last output value is updated with the
   current output value, scaled by the feedback
   depth.

7. The phase of the oscillator is incremented by
   `omega`.

8. The modulation depths and feedback depth are
   updated by processing their respective
   envelopes.

This `do_fm_block` function is called for each
sample in a block by the `process_block`
method. By combining multiple modulators,
feedback, and optional FM synthesis, this
implementation of `FMOscillator` generates a wide
variety of complex and rich sounds.

11> cool thanks! could you teach me even more
about FM synthesis? what are its mathematical
properties?

Frequency Modulation (FM) synthesis is a technique
for generating complex sounds by modulating the
frequency of one waveform, called the carrier,
with another waveform, called the modulator. The
carrier waveform is usually a simple waveform like
a sine wave, and the modulator is often another
sine wave or a more complex waveform. The
resulting sound can have a rich harmonic content,
making FM synthesis suitable for creating a wide
variety of timbres.

The basic mathematical equation for FM synthesis
is as follows:

y(t) = A * sin(2 * π * (f_c + I * f_m * sin(2 * π * f_m * t)) * t)

Where:

- y(t) is the output signal at time t

- A is the amplitude

- f_c is the carrier frequency

- f_m is the modulator frequency

- I is the modulation index, determining the
  amplitude of the sidebands in the output signal

The modulation index (I) is crucial in determining
the harmonic content of the resulting
sound. A higher modulation index results in a more
complex sound with a richer harmonic structure,
while a lower modulation index produces a simpler
sound closer to the original carrier waveform.

One of the key features of FM synthesis is the
ability to create a wide range of harmonic
structures by varying the modulation index and the
ratio between the carrier and modulator
frequencies. When the frequency ratio is an
integer or a simple fraction, the resulting sound
tends to have a harmonic structure, making it
suitable for musical applications. In contrast,
when the frequency ratio is an irrational number,
the sound becomes more inharmonic and noisy.

In practice, FM synthesis can involve multiple
modulators, each with its own frequency and
modulation index. This allows for even more
complex and evolving sounds, as demonstrated in
the `FMOscillator` implementation you've provided
earlier. Furthermore, feedback can be introduced
by feeding the output of the carrier waveform back
into the modulator, resulting in even richer
harmonic structures.

Overall, FM synthesis is a powerful technique for
sound synthesis, offering a wide range of
possibilities for creating diverse and intricate
sounds with relatively simple mathematical
principles.
