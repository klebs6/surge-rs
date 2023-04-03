# surgefx-phaser

The `surgefx-phaser` crate provides an
implementation of the Phaser effect as
a subcomponent for the Surge synthesizer system.

Phaser is a popular audio effect that creates
a sweeping, oscillating filter effect by combining
the original audio signal with a phase-shifted
version of itself. The phase shift is modulated by
a Low-Frequency Oscillator (LFO) to create
a dynamic, time-varying effect.

## Mathematical Concepts

The Phaser effect relies on the following
mathematical concepts:

- Phase shifting: The original audio signal is
  combined with a phase-shifted version, creating
  constructive and destructive interference at
  different frequencies. This interference results
  in a series of peaks and notches in the
  frequency spectrum, creating the characteristic
  Phaser sound.

  ```
  y(t) = x(t) + x(t - T) * e^(i * φ(t))
  ```
  where `y(t)` is the output signal, `x(t)` is the
  input signal, `T` is the time delay, and `φ(t)`
  is the time-varying phase shift.

- Low-Frequency Oscillation (LFO): The phase shift
  is modulated by an LFO, which determines the
  rate and depth of the effect. The LFO waveform
  can be sine, triangle, or other shapes,
  affecting the character of the Phaser effect.

  ```
  φ(t) = A * sin(2π * f * t + θ)
  ```
  where `A` is the amplitude, `f` is the
  frequency, `t` is time, and `θ` is the phase
  offset.

## Tokens

- `control_group`, `control_type`,
  `default_value`, `max_value`, `min_value`,
  `modulateable`, `moverate`, `value_type`: These
  tokens define parameters and their ranges,
  types, and other properties for configuring the
  Phaser effect.

- `clear_blocks`, `suspend_all_biquads`: Functions
  for initializing and suspending the internal
  processing blocks and biquad filters used in the
  Phaser effect.

- `update`, `do_phaser_block`: Functions for
  updating the Phaser effect's internal state and
  processing the audio signal.

- `phaser_smoke`, `init`, `process`,
  `process_only_control`: Functions for
  initializing the Phaser effect, processing the
  audio signal, and handling control signals.

- `Phaser`: The main struct representing the
  Phaser effect, containing the necessary data and
  methods for creating and manipulating the
  effect.

21> thanks! could you teach me how a phaser works?
what is its block diagram? how do the
subcomponents fit together and interact? are there
opportunities to customize the block diagram and
experiment with different architectures?

A phaser is an audio effect that creates
a sweeping, oscillating filter effect by combining
an input audio signal with a phase-shifted version
of itself. The phase shift is modulated by
a Low-Frequency Oscillator (LFO), which results in
a dynamic, time-varying effect.

Here is a simplified block diagram of a typical
phaser:

1. Input audio signal

2. All-pass filters (in series or parallel) for
   phase-shifting

3. LFO to modulate the all-pass filters' center
   frequencies

4. Summing the original and phase-shifted signals

5. Output audio signal

The subcomponents interact in the following way:

- The input audio signal is fed into a series of
  all-pass filters, which create a phase-shifted
  version of the signal.

- The all-pass filters' center frequencies are
  modulated by an LFO, causing the phase shift to
  vary over time.

- The original audio signal and the phase-shifted
  signal are then summed together, creating the
  phaser effect due to constructive and
  destructive interference at different
  frequencies.

- The resulting signal is then sent to the output.

There are opportunities to customize the block
diagram and experiment with different
architectures:

1. **All-pass filter configuration:** You can
   experiment with different all-pass filter
   configurations, such as altering the number of
   filters or using parallel instead of series
   connections. This can change the depth and
   character of the phaser effect.

2. **LFO waveform:** By using different LFO
   waveforms (e.g., sine, triangle, square), you
   can change the character of the phaser
   effect. Different waveforms will produce
   different modulation patterns, resulting in
   a unique sound.

3. **Feedback path:** Introducing a feedback path
   from the output back to the input can create
   a more intense phaser effect with pronounced
   peaks and notches in the frequency spectrum.

4. **Multistage phasing:** Instead of using
   a single phaser, you can cascade multiple
   phasers with different LFO frequencies and
   filter configurations. This can create a more
   complex and evolving sound.

5. **Stereo phasing:** Applying different phaser
   settings to the left and right channels of
   a stereo signal can create a spacious,
   immersive effect.

Experimenting with these variations can result in
unique phaser effects that suit different types of
audio material and creative purposes.

22> why do we use allpass filters in the context
of phasing? how do several allpass filters
interact mathematically?

All-pass filters are used in the context of
phasing because they can introduce phase shifts to
the input signal without affecting the magnitude
of its frequency components. This makes them ideal
for creating the characteristic sweeping effect of
a phaser while preserving the overall tonal
balance of the input signal.

An all-pass filter can be defined by the following
transfer function:

```
H(z) = (k - z^(-n)) / (1 - kz^(-n))
```

where `H(z)` is the transfer function, `k` is
a coefficient that controls the amount of phase
shift, `n` is the filter order, and `z` is the
complex frequency variable.

When several all-pass filters are combined (in
series or parallel), their individual phase shifts
add up, creating a more pronounced phase shift
across the frequency spectrum.

Mathematically, when all-pass filters are
connected in series, their transfer functions are
multiplied:

```
H_total(z) = H_1(z) * H_2(z) * ... * H_n(z)
```

When connected in parallel, the transfer functions
are combined in the following manner:

```
H_total(z) = (H_1(z) + H_2(z) + ... + H_n(z)) / n
```

The interaction of multiple all-pass filters can
create a complex pattern of phase shifts across
the frequency spectrum, resulting in the
characteristic peaks and notches in the frequency
response of a phaser effect. By modulating the
center frequencies of the all-pass filters using
an LFO, the peaks and notches sweep through the
frequency spectrum, producing the dynamic,
time-varying sound of the phaser effect.

23> could you provide 12 examples from electronic
music of production-quality techniques involving
phasers? please describe what the result is, in
terms of sonic characteristics.  also, please
describe how we configure the phaser module in
order to achieve this result

Here are 12 examples of production-quality
techniques involving phasers in electronic music,
along with their sonic characteristics and how to
configure the phaser module to achieve these
results:

1. **Classic psychedelic swirl:** This effect is
   reminiscent of 60s and 70s psychedelic music,
   featuring a pronounced swirling
   effect. Configure the phaser with a moderate
   depth, slow LFO rate, and a sine or triangle
   LFO waveform.

2. **Subtle stereo widening:** For a subtle stereo
   widening effect, use two separate phasers with
   slightly different settings for the left and
   right channels. Set the depth and feedback to
   low values, and use a slow to moderate LFO
   rate.

3. **Rhythmic pulsing:** Create a rhythmic pulsing
   effect by syncing the phaser's LFO rate to the
   tempo of your track, using a square or sawtooth
   LFO waveform. Set the depth and feedback to
   moderate values for a more pronounced pulsing
   effect.

4. **Drum loop enhancement:** To add interest to
   a drum loop, use a phaser with a low depth,
   slow LFO rate, and a sine or triangle LFO
   waveform. This will introduce subtle movement
   and variation to the drum sounds without
   overwhelming the mix.

5. **Vocal shimmer:** To add a shimmering effect
   to vocals, use a phaser with a low to moderate
   depth, slow LFO rate, and a sine or triangle
   LFO waveform. Experiment with the number of
   all-pass filters to fine-tune the vocal
   presence in the mix.

6. **Synth pad movement:** To add movement and
   depth to a synth pad, use a phaser with
   a moderate to high depth, slow LFO rate, and
   a sine or triangle LFO waveform. Adjust the
   feedback level to control the intensity of the
   effect.

7. **Bass enhancement:** To add character to
   basslines, use a phaser with a low depth, slow
   LFO rate, and a sine or triangle LFO
   waveform. This can help to add harmonic
   interest and subtle movement without
   compromising the low-end impact.

8. **Guitar swirl:** Create a rich, swirling
   effect for guitars by configuring the phaser
   with a moderate to high depth, slow to moderate
   LFO rate, and a sine or triangle LFO
   waveform. Adjust the feedback level to control
   the intensity of the effect.

9. **Organic texture:** To create an organic,
   evolving texture, use multiple phasers in
   parallel, each with different LFO rates and
   waveforms. Experiment with different numbers of
   all-pass filters to create a complex, shifting
   sound.

10. **Riser effect:** Create a riser effect by
    automating the LFO rate or depth of a phaser
    over time. Start with a slow LFO rate and low
    depth and increase both parameters to create
    a sense of tension and anticipation.

11. **Harmonic emphasis:** Enhance specific
    frequency bands in a mix by configuring the
    phaser with a high depth, moderate feedback,
    and a narrow frequency range. Automate the LFO
    rate or depth to emphasize different harmonic
    elements in the mix.

12. **Creative sound design:** For experimental
    sound design, use a phaser with extreme
    settings (e.g., high depth, fast LFO rate,
    square LFO waveform, high feedback). This can
    create unconventional and unpredictable
    effects, suitable for soundscapes, special
    effects, or transitions.

Keep in mind that the specific phaser settings and
sonic results will depend on the input material
and the phaser module being used. Feel free to
experiment with different configurations to
achieve the desired effect for your production.

24> thanks -- could you show me a block diagram of
each subcomponent?

For a typical phaser, the block diagram of each
subcomponent is as follows:

1. **Input audio signal:** This is the starting
   point where the raw audio signal enters the
   phaser effect.

2. **All-pass filters:** This subcomponent
   consists of one or multiple all-pass filters
   connected in series or parallel, depending on
   the design. Each all-pass filter introduces
   a phase shift in the signal without altering
   its amplitude. The filters can be first or
   second order and are characterized by their
   center frequency and the amount of phase shift
   they introduce.

```
All-pass filter 1 ──> All-pass filter 2 ──> ... ──> All-pass filter N
```

3. **Low-Frequency Oscillator (LFO):** The LFO is
   a periodic waveform generator that modulates
   the center frequency of the all-pass
   filters. It can produce various waveforms, such
   as sine, triangle, square, or sawtooth, and its
   frequency can be set to a specific value or
   synced to the tempo of the track.

```
LFO ──> Modulation signal
```

4. **Modulation:** The modulation signal from the
   LFO is applied to the center frequencies of the
   all-pass filters, causing them to change over
   time. This results in the dynamic, sweeping
   effect that characterizes the phaser sound.

```
All-pass filter 1 (modulated) ──> All-pass filter 2 (modulated) ──> ... ──> All-pass filter N (modulated)
```

5. **Summing:** The original, unprocessed audio
   signal and the phase-shifted signal (output of
   the modulated all-pass filters) are combined to
   create the final phaser effect. Constructive
   and destructive interference at different
   frequencies creates the characteristic peaks
   and notches in the output signal.

```
Input signal + Phase-shifted signal ──> Output signal
```

6. **Feedback (optional):** In some phaser
   designs, a feedback path can be introduced,
   taking the output signal and feeding it back
   into the input. This can increase the intensity
   of the effect and create more pronounced peaks
   and notches.

```
Output signal ──> Feedback ──> Input signal
```

7. **Output audio signal:** The processed signal,
   which has been combined with the original
   input, is sent to the output of the phaser
   effect and can be further processed or mixed
   with other tracks in a project.

These are the main subcomponents of a typical
phaser effect. Each subcomponent can be customized
and experimented with to create unique phaser
sounds and configurations.

25> any caveats while using a phaser?

While using a phaser, there are some potential
caveats to be aware of:

1. **Overuse:** Overusing phaser effects can lead
   to a muddy or overly processed mix. Use the
   effect judiciously and focus on enhancing the
   overall sonic character without making the
   effect too obvious or overpowering.

2. **Phase cancellation:** Excessive phase
   cancellation can occur when the wet and dry
   signals are combined, potentially causing
   unwanted changes in the frequency response or
   volume levels. To mitigate this, carefully
   adjust the depth and feedback settings, as well
   as the position of the effect in the signal
   chain.

3. **Mono compatibility:** Some phaser
   configurations, especially when applied to
   stereo signals, can lead to mono compatibility
   issues. When the mix is collapsed to mono,
   phase cancellation can cause certain elements
   to become inaudible or lose their
   impact. Always check your mix in mono and make
   necessary adjustments to ensure it translates
   well across different playback systems.

4. **Muddiness:** Applying a phaser to
   low-frequency elements, like basslines or kick
   drums, can sometimes cause muddiness in the
   mix. Use a high-pass filter before the phaser
   to remove low-frequency content, or apply the
   phaser selectively to higher-frequency
   elements.

5. **Rhythmic interference:** When a phaser's LFO
   rate is not synced to the track tempo, it can
   create rhythmic interference with the other
   elements of the mix. If this is undesirable,
   sync the LFO rate to the tempo or carefully
   adjust the rate manually to fit the groove of
   the track.

6. **Loss of definition:** Too much phaser effect
   on certain elements, like vocals or lead
   instruments, can cause a loss of definition and
   make them harder to distinguish in the mix. Use
   the effect sparingly on these elements, or
   automate it to be active only during specific
   sections of the song.

7. **Complex signal chains:** When used in
   conjunction with other time-based effects, like
   delay and reverb, phasers can lead to a complex
   and hard-to-control signal chain. Experiment
   with the order of the effects to achieve the
   desired sound and consider using parallel
   processing to maintain clarity and control over
   each effect.

By being aware of these caveats and addressing
them during the mixing and production process, you
can use phasers effectively to enhance the sonic
character of your tracks while avoiding potential
pitfalls.
