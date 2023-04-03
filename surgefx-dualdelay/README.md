Rust crate for a dual delay effect in the Surge synthesizer system
---
The `surgefx-dualdelay` crate provides a dual
delay effect, which is one of the subcomponents of
the Surge synthesizer system. This effect is
designed to create stereo delay lines with
independent control over various parameters, such
as delay time, feedback, crossfeed, filtering, and
panning. The crate contains the necessary
functions and structures to initialize, process,
and control the dual delay effect.

The mathematical foundation of the dual delay
effect involves the manipulation of discrete-time
audio signals. The effect uses two separate delay
lines to process the left and right channels
independently. Given an input signal 𝑥(𝑛), the
output signal 𝑦(𝑛) for each channel is computed
using a combination of delay time 𝑇, feedback 𝐹,
crossfeed 𝐶, filtering 𝐻(𝑧), and panning 𝑃:

𝑦(𝑛) = 𝑃(𝑥(𝑛) + 𝐶(𝑥(𝑛 - 𝑇) + 𝐹𝑦(𝑛 - 𝑇))) * 𝐻(𝑧)

Tokens and their mathematical associations:

- `control_group`, `control_type`,
  `default_value`, `max_value`, `min_value`,
  `modulateable`, `moverate`, `value_type`: These
  tokens are used to define and control the
  parameters of the dual delay effect, such as
  delay time, feedback, crossfeed, filtering, and
  panning.

- `update`: This function updates the internal
  state of the dual delay effect, recalculating
  delay times, filter coefficients, and other
  parameters based on the current control values.

- `do_dualdelay_block`: This function processes
  the input audio signal using the dual delay
  effect, computing the output signal according to
  the current delay times, feedback, crossfeed,
  filtering, and panning settings.

- `process_time`: This function processes the
  delay times for each channel, ensuring that they
  are within the allowed range and taking any
  modulation or tempo-syncing into account.

- `dual_delay_smoke`, `clear_buffers`, `init`,
  `instantize_all`, `clear_scratch_buffers`: These
  functions are used to initialize and reset the
  internal state of the dual delay effect,
  including clearing audio buffers, setting
  initial parameter values, and instantiating
  filters.

- `do_crossfeed`, `do_feedback`, `do_filters`,
  `do_mid_side`, `do_mix`, `do_pan`,
  `do_softclip`: These functions are responsible
  for implementing the various processing stages
  of the dual delay effect, such as crossfeeding
  between channels, applying feedback, filtering
  the delayed signal, and adjusting the stereo
  image.

- `maybe_instantize_all`, `process`,
  `process_only_control`, `update_wpos`: These
  functions handle various aspects of processing
  the audio signal, such as updating the write
  position in the delay buffer and applying
  parameter changes in real-time.

- `DualDelay`, `dualdelay_set_buffer`: These
  structures and functions define the main dual
  delay effect object and its associated methods,
  including setting the audio buffer to be used
  for processing.

22> thanks! could you show me some in depth block
diagrams about how the subcomponents of this
effect fit together?

Certainly! Here's an in-depth block diagram
illustrating the signal flow and subcomponents of
a dual delay effect:

```
Input Signal (L, R) ─┐
                     ├─┬──[Panner]───┐
                     │ ├─[Crossfeed]─┼─┐
                     │ └─[Feedback]──┘ │
                     │                 ├─[Delay Line L]─┬─[Filter L]─┬─[Softclip L]─┬─[Output Gain L]──┐
                     ├─────────────────┤                 │            │             │                  ├─ Output Signal (L, R)
                     │                 ├─[Delay Line R]─┼─[Filter R]─┼─[Softclip R]─┼─[Output Gain R]──┘
                     ├─┬──[Panner]───┐ │                 │            │             │
                     │ ├─[Crossfeed]─┼─┘
                     └─┴─[Feedback]──┘
```

1. **Input Signal (L, R)**: The left and right
   channels of the input signal enter the dual
   delay effect.

2. **Panner**: The panner adjusts the stereo
   balance of the input signal before it is sent
   to the delay lines.

3. **Crossfeed**: The crossfeed subcomponent takes
   a portion of the input signal from one channel
   and adds it to the other channel, creating
   a more complex stereo image. The amount of
   crossfeed can be controlled by the user.

4. **Feedback**: The feedback subcomponent takes
   a portion of the delayed signal from the output
   of each delay line and feeds it back into the
   input of the corresponding delay line, creating
   a repeating echo effect. The amount of feedback
   can be controlled by the user.

5. **Delay Line L & R**: These are the main delay
   lines for the left and right channels. They
   store the audio signal and provide
   a time-varying delay, controlled by the delay
   time parameter.

6. **Filter L & R**: After the delay lines, the
   signal passes through filters for each
   channel. These filters can be used to shape the
   frequency content of the delayed signal, such
   as low-pass, high-pass, or band-pass
   filters. The filter settings can be controlled
   by the user.

7. **Softclip L & R**: The softclip subcomponents
   apply a soft-clipping non-linear function to
   the delayed and filtered signal, adding
   harmonic distortion and preventing excessive
   peaks in the output signal.

8. **Output Gain L & R**: The output gain stages
   adjust the overall level of the delayed signal
   before it is combined with the dry signal and
   sent to the output.

9. **Output Signal (L, R)**: The processed left
   and right channels of the delayed signal are
   combined with the original input signal,
   creating the final output signal with the
   desired dual delay effect.

The diagram above shows the main subcomponents of
a dual delay effect and how they are connected to
create the desired stereo delay processing. The
specific implementations of these subcomponents
may vary depending on the desired sound and
functionality, but the general signal flow remains
the same.

23> thanks -- how are these modules parametrized?
which handles can the user adjust at runtime?

In a dual delay effect, there are several
parameters that users can adjust to control the
sound and behavior of the effect. Here is a list
of common parameters that can be adjusted at
runtime:

1. **Delay Time (L & R)**: The delay time
   determines the length of the delay lines for
   each channel. Users can set different delay
   times for the left and right channels to create
   a more complex stereo image. In some
   implementations, users can also synchronize the
   delay times to the tempo of the music.

2. **Feedback (L & R)**: The feedback parameter
   controls the amount of signal fed back from the
   output of each delay line to its input. By
   adjusting the feedback, users can create more
   or fewer repeats of the delayed signal.

3. **Crossfeed**: The crossfeed parameter controls
   the amount of signal from one channel that is
   added to the other channel before entering the
   delay lines. This can be used to create a more
   complex and spacious stereo image.

4. **Filter (L & R)**: The filter parameters
   control the characteristics of the filters
   applied to the output of each delay line. Users
   can adjust the filter type (low-pass,
   high-pass, band-pass, etc.), the filter cutoff
   frequency, and the filter resonance.

5. **Soft Clipping (L & R)**: The soft clipping
   parameters control the amount of distortion
   applied to the delayed signal after
   filtering. Users can adjust the level of soft
   clipping to add warmth or saturation to the
   sound.

6. **Output Gain (L & R)**: The output gain
   parameters control the level of the processed
   delayed signal before it is combined with the
   dry signal and sent to the output. This can be
   used to balance the levels of the dry and wet
   signals.

7. **Dry/Wet Mix**: The dry/wet mix parameter
   controls the balance between the unprocessed
   input signal (dry) and the processed delayed
   signal (wet). Users can adjust this parameter
   to achieve the desired blend of the original
   and delayed signals.

8. **Panning (L & R)**: The panning parameters
   control the stereo positioning of the input
   signal before it enters the delay lines. Users
   can adjust the panning to create a wider or
   narrower stereo image.

These parameters allow users to shape the sound
and behavior of the dual delay effect to suit
their needs. In a well-designed effect
implementation, these parameters can be adjusted
in real-time, providing users with a high degree
of control and flexibility.

24> could you describe a dozen different
descriptions of effects produced by this processor
at different parameter settings?

Certainly! Here are twelve different descriptions
of effects produced by the dual delay processor
with different parameter settings:

1. **Basic Stereo Delay**: Set equal delay times
   for both left and right channels, moderate
   feedback, and a balanced dry/wet mix. This
   results in a simple stereo echo effect, with
   the delay repeats equally spaced in both
   channels.

2. **Ping Pong Delay**: Set different delay times
   for the left and right channels and high
   crossfeed. The delayed signal will bounce back
   and forth between the left and right speakers,
   creating a ping pong effect.

3. **Slapback Echo**: Set short delay times for
   both channels, low feedback, and a moderate
   dry/wet mix. This creates a slapback echo,
   often used in rockabilly and early rock 'n'
   roll music.

4. **Ambient Delay**: Set long delay times, high
   feedback, low-pass filters with a low cutoff
   frequency, and a high wet mix. This creates
   a spacious, atmospheric effect, with the delays
   blending into the background.

5. **Dub-style Delay**: Set medium to long delay
   times, high feedback, low-pass filters with
   a moderate cutoff frequency, and a high wet
   mix. This results in a warm, analog-style delay
   effect commonly used in dub and reggae music.

6. **Shimmer Delay**: Set long delay times, high
   feedback, and apply a pitch-shifting effect
   (e.g., one octave up) to the feedback
   loop. This creates a shimmering, ethereal
   sound, often used in ambient and experimental
   music.

7. **Filtered Rhythmic Delay**: Set different
   delay times for the left and right channels,
   with the delay times synced to the tempo of the
   music. Use band-pass filters with a narrow
   bandwidth and modulate the filter cutoff
   frequency in time with the music. This creates
   a rhythmic, pulsating delay effect.

8. **Distorted Delay**: Set medium delay times,
   moderate feedback, and high soft clipping
   levels. This creates a delay effect with
   a gritty, distorted character.

9. **Spatial Widener**: Set very short delay times
   (e.g., 10-30 ms) for both channels, low
   feedback, and a moderate dry/wet mix. This
   creates a subtle widening effect, enhancing the
   stereo image of the input signal.

10. **Reverse Delay**: Reverse the delayed signal
    before feeding it back into the input. This
    creates a reverse delay effect, often used for
    experimental or psychedelic sounds.

11. **Chorus-like Delay**: Set very short delay
    times (e.g., 20-50 ms), low feedback, and
    modulate the delay time with a slow LFO. This
    creates a chorus-like effect, adding depth and
    movement to the sound.

12. **Freeze Delay**: Set very long delay times,
    maximum feedback, and a high wet mix. By
    momentarily sending a signal into the delay
    lines, you can create a "frozen" sound that
    sustains indefinitely, creating a drone or
    pad-like effect.

These are just a few examples of the wide range of
effects that can be achieved using a dual delay
processor. By adjusting the parameters and
combining different settings, you can create an
almost limitless variety of delay-based
soundscapes.

25> thanks, i like these descriptions -- could you
do a dozen more?

Sure! Here are twelve more descriptions of effects
produced by the dual delay processor with
different parameter settings:

1. **Glitchy Delay**: Set short, unsynchronized
   delay times for both channels, high feedback,
   and apply a bitcrusher or sample rate reducer
   to the feedback loop. This creates a glitchy,
   lo-fi delay effect.

2. **Multi-Tap Delay**: Configure multiple delay
   lines with different delay times and feedback
   levels for each channel, and mix them
   together. This creates a complex, multi-tap
   delay effect that can add texture and depth to
   the sound.

3. **Gated Delay**: Apply a noise gate or expander
   to the delayed signal, with a threshold set to
   only allow louder delay repeats to pass
   through. This creates a dynamic delay effect
   where the echoes are only audible when the
   input signal is loud enough.

4. **Swirling Delay**: Modulate the delay time,
   filter cutoff frequency, and/or panning of the
   delay lines with different LFOs or envelope
   followers. This creates a swirling, evolving
   delay effect with a lot of movement.

5. **Granular Delay**: Set very short delay times
   (e.g., 1-10 ms), high feedback, and use
   a granular synthesis engine to process the
   delay lines. This creates a granular delay
   effect, where the input signal is broken into
   small grains and rearranged in time and space.

6. **Dynamic Delay**: Use an envelope follower or
   sidechain compression to control the delay
   level or feedback based on the input signal's
   amplitude. This creates a dynamic delay effect,
   where the echoes become more prominent when the
   input signal is quieter.

7. **Pitch-Shifting Delay**: Apply different
   pitch-shifting intervals to the left and right
   delay lines, such as a perfect fifth or an
   octave. This creates a harmonically rich delay
   effect, adding melodic or harmonic interest to
   the sound.

8. **Resonant Delay**: Set high feedback levels
   and use resonant low-pass or band-pass filters
   in the feedback loop. This creates a resonant
   delay effect, where the echoes emphasize
   specific frequencies and produce a ringing or
   resonant sound.

9. **Flanger-like Delay**: Set very short delay
   times (e.g., 5-20 ms), low feedback, and
   modulate the delay time with a slow LFO. This
   creates a flanger-like effect, with
   a characteristic "whooshing" or "jet plane"
   sound.

10. **Ducking Delay**: Use a sidechain compressor
    or envelope follower to reduce the delay level
    or mix when the input signal is playing. This
    creates a ducking delay effect, where the
    echoes are pushed back in the mix when the
    input signal is present, helping to maintain
    clarity and prevent clutter.

11. **Diffused Delay**: Use multiple delay lines
    with slightly different delay times and
    feedback levels, and process the outputs with
    a diffusion network or reverb algorithm. This
    creates a diffused delay effect, where the
    echoes are smeared and blended together,
    producing a more diffuse and spacious sound.

12. **Comb Filter Delay**: Set very short delay
    times (e.g., 1-10 ms), high feedback, and no
    filtering. This creates a comb filter delay
    effect, where the input signal is combined
    with a slightly delayed copy, creating
    a series of notches in the frequency spectrum
    and adding a metallic or "tuned" quality to
    the sound.

These additional examples showcase the versatility
of a dual delay processor. By experimenting with
different parameter settings and combinations, you
can create a vast array of unique and interesting
delay effects.

26> how do we prevent unwanted artifacts when
engineering this signal processor?

To prevent unwanted artifacts when engineering
a dual delay signal processor, there are several
aspects you need to consider and address:

1. **Aliasing**: When working with digital signal
   processing, aliasing can occur due to the
   discrete nature of the process. To minimize
   aliasing, use oversampling (processing the
   signal at a higher sample rate) and apply
   anti-aliasing filters before downsampling back
   to the original sample rate.

2. **Noise**: Ensure that the input signal is
   clean and free from noise, as any noise present
   will be amplified by the delay process,
   especially with high feedback settings. You can
   use noise reduction techniques or low-pass
   filters to remove unwanted noise from the input
   signal.

3. **Feedback instability**: When using high
   feedback levels, the delay can quickly become
   unstable and produce unwanted artifacts or
   oscillations. To prevent this, apply a limiter
   or soft-clipper in the feedback loop to control
   the amplitude and ensure stability.

4. **Phase issues**: When using multiple delay
   lines, phase issues can arise, causing
   cancellation or reinforcement of certain
   frequencies. To minimize phase issues, use
   different delay times for each channel or apply
   all-pass filters to adjust the phase response
   without affecting the amplitude.

5. **Clicks and pops**: Abrupt changes in delay
   times or parameter settings can cause clicks
   and pops in the output signal. To prevent this,
   use interpolation, crossfading, or smoothing
   techniques to ensure smooth transitions between
   parameter changes.

6. **Denormalization**: In digital signal
   processing, very small floating-point numbers
   can cause performance issues and create
   artifacts. To prevent this, use denormalization
   techniques, such as adding a small offset to
   the signal or using specialized functions that
   handle denormal numbers efficiently.

7. **Latency**: When designing a real-time audio
   processor, it's crucial to minimize latency to
   ensure a responsive and natural-sounding
   effect. Optimize your code for performance and,
   if necessary, use parallel processing or
   look-ahead techniques to reduce latency.

8. **Parameter modulation**: When modulating
   parameters, such as delay time or filter cutoff
   frequency, use smooth modulation sources like
   low-frequency oscillators (LFOs) or
   envelopes. Abrupt changes in modulation can
   cause artifacts and produce unnatural-sounding
   results.

9. **Filter design**: When using filters in the
   delay processor, ensure that they are designed
   to minimize artifacts like ringing, aliasing,
   or phase distortion. Choose appropriate filter
   types (e.g., Butterworth, Chebyshev, or
   elliptic filters) based on the desired
   characteristics and performance requirements.

10. **Signal headroom**: Ensure that there is
    enough headroom in the processing chain to
    prevent clipping and distortion. Use gain
    staging techniques to maintain an appropriate
    signal level throughout the processor.

By addressing these potential issues during the
design and engineering of a dual delay signal
processor, you can minimize unwanted artifacts and
ensure a clean, high-quality output.

27> what else can you tell me about dual delay?
what can you show me?

A dual delay is a versatile audio effect that
processes an input signal by independently
delaying the left and right channels. This creates
a stereo effect that can add depth, width, and
space to a sound. Here are some additional
insights and visualizations related to dual delay:

**Configurations and variations:**

Dual delay can be used in various configurations,
such as parallel or serial processing. In
a parallel configuration, the left and right
channels are processed independently, while in
a serial configuration, one channel's output is
fed into the other channel's input, creating
a more complex, cascading delay effect.

**Synchronization:**

You can synchronize the delay times to the tempo
of a song, using musical note values (e.g.,
quarter notes, eighth notes) to define the delay
times. This helps create rhythmic coherence
between the delay effect and the rest of the
music. You can also use unsynchronized delay times
to create more experimental or unpredictable
effects.

**Modulation:**

Modulating the delay time, filter cutoff
frequency, feedback, or other parameters with LFOs
or envelope followers can create dynamic and
evolving delay effects. For example, you can
create a chorus or flanger effect by modulating
the delay time with a slow LFO.

**Visualization:**

Here is a simple block diagram of a dual delay
processor:

```
   Input Signal
        |
        V
  ┌─────┴─────┐
  │  Left Delay ├──────┐
  └────────────┘      |
                       V
                  Wet/Dry Mix
                       |
                       V
  ┌─────┴─────┐      Output
  │ Right Delay ├──────┘
  └────────────┘
```

This diagram shows the basic structure of a dual
delay, with separate delay lines for the left and
right channels. The wet/dry mix control adjusts
the balance between the original (dry) signal and
the delayed (wet) signal before output.

**Ping-pong delay:**

A common variation of the dual delay is the
ping-pong delay, where the output of one channel
is fed into the input of the other channel,
creating a bouncing effect between the left and
right speakers. This can add even more stereo
width and depth to the sound.

**Creative uses:**

Dual delay can be used creatively to enhance
a variety of sounds, such as adding space and
depth to vocals, creating rhythmic patterns for
guitars or synths, or adding a sense of movement
and interest to pads and ambient textures. By
experimenting with different delay times, feedback
levels, modulation, and filtering, you can create
a wide range of unique and interesting audio
effects.

Overall, dual delay is a powerful and versatile
audio effect that can be used in a variety of ways
to enhance and enrich your sound. By understanding
the underlying concepts and exploring different
configurations and parameter settings, you can
unlock its full potential and create unique,
captivating audio experiences.

28> how do we prevent the delayed audio from
phasing with the original source and causing
unwanted interference?

To prevent the delayed audio from phasing with the
original source and causing unwanted interference,
there are several techniques you can apply:

1. **Use different delay times for left and right
   channels**: By using different delay times for
   each channel, you can minimize the chance of
   phase cancellation between the original and
   delayed signals. This will create a more
   spacious stereo image and can help reduce
   phase-related issues.

2. **Apply filters to the delayed signal**: By
   using filters, such as low-pass or high-pass
   filters, you can remove certain frequency
   components from the delayed signal. This can
   help reduce the chances of phase interference
   with the original signal. Additionally, you can
   use band-pass or band-reject filters to target
   specific frequency ranges.

3. **Modulate delay times**: By slightly
   modulating the delay times with a low-frequency
   oscillator (LFO), you can create a constantly
   shifting phase relationship between the
   original and delayed signals. This can help
   prevent static phase cancellation and add
   a sense of motion and depth to the sound.

4. **Utilize Haas effect**: The Haas effect, also
   known as the precedence effect, states that
   when two similar sounds arrive at the
   listener's ears within approximately 40
   milliseconds of each other, the listener
   perceives them as a single sound coming from
   the direction of the first-arriving sound. By
   setting the delay times shorter than 40
   milliseconds, you can create a sense of spatial
   localization without causing significant phase
   interference.

5. **Adjust the feedback**: Reducing the amount of
   feedback in the delay can help minimize the
   buildup of phase interference over time. Lower
   feedback settings will result in fewer delay
   repeats, which can reduce the chance of phase
   issues.

6. **Experiment with delay algorithms**: Different
   delay algorithms, such as tape, digital, or
   analog delay, can have varying effects on phase
   relationships. Experiment with different
   algorithms to find the one that best suits your
   needs and minimizes phase-related issues.

7. **Use mid/side processing**: By processing the
   mid (mono) and side (stereo) components of the
   signal separately, you can control the phase
   relationship between the original and delayed
   signals more effectively. For example, you can
   apply the delay only to the side signal to
   preserve the phase coherence of the original
   mono signal.

By using these techniques, you can minimize the
chance of phase interference between the original
and delayed signals, resulting in a cleaner and
more spacious sound. Experimenting with different
settings and processing techniques will help you
find the best balance between preserving the
original signal's integrity and achieving the
desired delay effect.
