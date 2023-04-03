**Crate: surgefx-chorus**

_SurgeFX Chorus: A versatile chorus effect for the
Surge Synthesizer system._

The `surgefx-chorus` crate provides a flexible and
efficient implementation of the chorus effect for
the Surge Synthesizer system. It handles various
control parameters, modulation, and processing
capabilities to create rich and immersive
soundscapes.

## Crate Components

### control_group

The `control_group` represents a collection of
related control parameters for the chorus
effect. Each group contains parameters such as
`control_type`, `default_value`, `max_value`,
`min_value`, `modulateable`, and `moverate`.

### control_type

The `control_type` specifies the type of control
parameter, e.g., depth, rate, or mix. These
parameters dictate how the chorus effect behaves.

### default_value, max_value, min_value

These parameters define the default, maximum, and
minimum values for a control parameter. For
example, the depth of the chorus effect can have
a default value of 0.5, a minimum value of 0, and
a maximum value of 1.

### modulateable

The `modulateable` parameter is a boolean value
that determines if a control parameter can be
modulated by external sources, such as LFOs or
envelopes.

### moverate

The `moverate` represents the rate at which
a control parameter value changes when
modulated. For example, a `moverate` of 0.01 would
mean a slow change, while a `moverate` of 1 would
be a fast change.

### value_type

The `value_type` specifies the type of value the
control parameter holds. Examples include
integers, floating-point numbers, or boolean
values.

### initial_update, update

The `initial_update` function initializes the
chorus effect with the default control parameter
values, while the `update` function modifies the
chorus effect parameters based on user input or
modulation.

### do_chorus_block

The `do_chorus_block` function processes a block
of audio samples, applying the chorus effect. It
uses mathematical functions to create the chorus
effect, such as time-varying delays and phase
modulation.

### Chorus

The `Chorus` struct represents the main chorus
effect, containing control parameters, buffers,
and processing functions.

### chorus_smoke

The `chorus_smoke` function is a test function
used for testing and debugging purposes.

### init, process

The `init` function initializes the `Chorus`
struct, setting up buffers and other internal data
structures. The `process` function takes an input
audio buffer, applies the chorus effect, and
returns the processed audio buffer.

### chorus_set_buffer

The `chorus_set_buffer` function sets the internal
buffer of the `Chorus` struct, which is used for
processing the input audio.

## Mathematical Concepts

The primary mathematical idea behind the chorus
effect is time-varying delays. The input audio
signal is delayed by different amounts, creating
multiple "voices" that are then mixed back into
the original signal. This creates a thicker and
more spacious sound.

The delay time is modulated by a Low-Frequency
Oscillator (LFO) using the following equation:

```
delay_time = base_delay + depth * sin(2 * π * rate * t)
```

Where `base_delay` is the base delay time, `depth`
and `rate` are control parameters, and `t` is the
time variable.

In addition to time-varying delays, phase
modulation can be applied to create more complex
and rich-sounding effects. The phase modulation
equation is:

```
Φ(t) = Φ₀ + A * sin(2 * π * f * t + ϕ)
```

2> thanks, could you teach me more about the
chorus effect? can we visualize it with a block
diagram?

Of course! The chorus effect is an audio
processing technique that simulates the natural
phenomenon of multiple instruments or voices
playing the same part but with slight variations
in pitch and timing. The result is a fuller,
richer, and more immersive sound.

Here is a simple block diagram of a chorus effect:

```
+--------------+     +--------------+     +-------------------+
| Input Signal |---->| Delay Buffer |---->| Time-varying Delay|
+--------------+     +--------------+     +-------------------+
                                          |
                                          v
                                +-----------------+
                                | LFO (sinusoidal)|
                                +-----------------+
                                          |
                                          v
                          +----------------------+     +------------------+
                          | Modulated Delay Line |---->| Wet/Dry Mix & Sum |
                          +----------------------+     +------------------+
                                          ^
                                          |
+--------------+     +--------------+     +-------------------+
| Input Signal |---->| Delay Buffer |---->| Time-varying Delay|
+--------------+     +--------------+     +-------------------+
                                          |
                                          v
                                +-----------------+
                                | LFO (sinusoidal)|
                                +-----------------+
                                          |
                                          v
                          +----------------------+     +------------------+
                          | Modulated Delay Line |---->| Wet/Dry Mix & Sum |
                          +----------------------+     +------------------+
                                                               |
                                                               v
                                                      +-------------+
                                                      | Output Signal|
                                                      +-------------+
```

1. **Input Signal**: The audio signal that needs
   to be processed.

2. **Delay Buffer**: Stores a copy of the input
   signal that will be used for creating the
   chorus effect.

3. **Time-varying Delay**: The delay times are
   modulated using a Low-Frequency Oscillator
   (LFO) to create variations in pitch and
   timing. This is the core part of the chorus
   effect.

4. **LFO (sinusoidal)**: The LFO generates a sine
   wave that modulates the delay times. The LFO's
   frequency and amplitude control the rate and
   depth of the modulation, respectively.

5. **Modulated Delay Line**: The modulated delay
   line applies the time-varying delay to the
   input signal, creating multiple "voices" with
   slightly different pitches and timings.

6. **Wet/Dry Mix & Sum**: The wet (processed) and
   dry (original) signals are mixed according to
   the user-defined mix level. The mixed signal is
   then summed with the original input signal to
   create the final output.

7. **Output Signal**: The processed audio signal
   with the chorus effect applied.

The block diagram above shows a two-voice chorus
effect. In practice, chorus effects can have
multiple voices to create a more complex and rich
sound. Additionally, other parameters such as
feedback and phase can be used to further shape
the sound.

3> could you teach me more about the input signal?
what does the chorus effect need on this front?

The input signal for a chorus effect is an audio
signal, typically a monophonic or polyphonic
source that you want to enhance with the chorus
effect. It cank be a single instrument, a voice,
or even a mix of multiple instruments. The input
signal is the starting point for applying the
chorus effect.

In the context of a synthesizer, the input signal
is usually generated by an oscillator, a sample
playback unit, or a combination of these. In
a digital audio workstation (DAW) or an effects
processor, the input signal can be a recorded
audio track or a live audio feed.

There are no specific requirements for the input
signal when it comes to the chorus
effect. However, certain characteristics of the
input signal may influence how the chorus effect
is perceived:

1. **Frequency content**: The chorus effect is
   more noticeable and impactful on sounds with
   a richer harmonic content, such as strings,
   pads, or electric pianos. Simple waveforms like
   sine waves may not benefit as much from the
   chorus effect since they have minimal harmonic
   content.

2. **Mono or stereo**: Although the chorus effect
   can be applied to both mono and stereo signals,
   it tends to produce a more spacious and
   immersive sound when applied to a mono
   input. This is because the chorus effect
   generates additional "voices" that are spread
   across the stereo field, creating the illusion
   of a wider and more enveloping sound.

3. **Amplitude envelope**: Sounds with longer
   sustain and release times (e.g., pads, strings)
   tend to benefit more from the chorus
   effect. This is because the overlapping
   "voices" created by the chorus effect have more
   time to interact and create a richer
   texture. Sounds with short durations or sharp
   transients may not benefit as much from the
   chorus effect.

In summary, the chorus effect can be applied to
a wide range of input signals, but the resulting
sound will vary depending on the characteristics
of the input. It is often more effective on
signals with rich harmonic content, sustained
sounds, and mono sources.

4> what about the delay buffer?

The delay buffer is a crucial component in the
implementation of a chorus effect. It is a memory
buffer that temporarily stores a copy of the input
signal. The primary purpose of the delay buffer is
to provide the means for creating time-varying
delays that are essential to the chorus effect.

In a digital implementation of a chorus effect,
the delay buffer is usually implemented as
a circular buffer (also known as a ring buffer)
due to its efficient memory management and ease of
use in real-time audio processing.

Here's how the delay buffer works in the context
of the chorus effect:

1. The input signal is continuously fed into the
   delay buffer.

2. The buffer has a fixed size, which determines
   the maximum delay time that can be
   achieved. For example, if the buffer size is
   1000 samples and the sample rate is 44,100 Hz,
   the maximum delay time would be 1000/44100 ≈ 22.7 ms.

3. A read pointer reads the delayed samples from
   the buffer, while a write pointer writes new
   samples into the buffer. The read pointer's
   position is modulated by the Low-Frequency
   Oscillator (LFO) to create time-varying delays.

4. When the read pointer reaches the end of the
   buffer, it wraps around to the beginning,
   creating a continuous loop of the input signal.

The delayed signal read from the buffer is then
mixed with the original input signal to create the
chorus effect. By modulating the position of the
read pointer, the chorus effect can generate
multiple "voices" with slight variations in pitch
and timing, simulating the natural phenomenon of
multiple instruments or voices playing the same
part.

In summary, the delay buffer is an essential
component of the chorus effect, providing the
means to create time-varying delays that are mixed
with the original input signal to produce a richer
and more immersive sound.

5> what about the time varying delay?

The time-varying delay is a key component of the
chorus effect, responsible for creating the
variations in pitch and timing that give the
chorus its distinctive sound. The time-varying
delay works by modulating the delay time applied
to the input signal using a Low-Frequency
Oscillator (LFO).

In a digital implementation of a chorus effect,
the time-varying delay is usually achieved by
modulating the read pointer's position in the
delay buffer. The position of the read pointer
determines how much the input signal is delayed
before being mixed with the original signal.

The delay time modulation is typically controlled
by an LFO, which generates a periodic waveform
(often a sine wave) at a low frequency (usually
below 20 Hz). The LFO modulates the read pointer's
position, causing the delay time to continuously
change, resulting in variations in pitch and
timing.

Here's a simple equation to describe the
time-varying delay:

```
delay_time(t) = base_delay + depth * sin(2 * π * rate * t)
```

Where:
- `delay_time(t)` is the delay time at time `t`

- `base_delay` is the base delay time (usually in
  milliseconds)

- `depth` is the modulation depth, which controls
  the intensity of the pitch and timing variations

- `rate` is the frequency of the LFO (in Hz),
  which controls the speed of the pitch and timing
  variations

- `t` is the time variable

The time-varying delay creates multiple "voices"
with slightly different pitches and timings by
continuously changing the delay time applied to
the input signal. When these delayed signals are
mixed with the original input signal, they produce
the rich and immersive sound characteristic of the
chorus effect.

In summary, the time-varying delay is an essential
component of the chorus effect, responsible for
creating the pitch and timing variations that give
the chorus its distinctive sound. It is typically
achieved by modulating the read pointer's position
in the delay buffer using an LFO, resulting in
a continuously changing delay time.

6> what about the lfo?

The Low-Frequency Oscillator (LFO) is an essential
component in the chorus effect, responsible for
modulating the time-varying delay, which in turn
creates the pitch and timing variations
characteristic of the chorus effect. The LFO
generates a periodic waveform, typically at
frequencies below 20 Hz, which is too slow to be
perceived as a pitch.

In the context of the chorus effect, the LFO's
primary role is to modulate the delay time applied
to the input signal. This modulation causes the
delay time to change continuously, resulting in
variations in pitch and timing that create the
illusion of multiple "voices" playing the same
part.

The most common waveform used for LFO in chorus
effect is a sine wave, but other waveforms, such
as triangle, square, or sawtooth, can also be used
to create different modulation
characteristics. The choice of waveform will
influence the overall sound and texture of the
chorus effect.

The LFO typically has two main parameters:

1. **Rate**: The rate of the LFO controls the
   frequency of the modulation waveform, which in
   turn determines the speed of the pitch and
   timing variations. Higher rates will produce
   more rapid fluctuations, while lower rates will
   result in slower, more subtle changes.

2. **Depth**: The depth of the LFO controls the
   amplitude of the modulation waveform, which
   determines the intensity of the pitch and
   timing variations. Higher depth values will
   produce more pronounced changes in pitch and
   timing, while lower depth values will yield
   more subtle effects.

The LFO modulates the delay time applied to the
input signal using an equation like this:

```
delay_time(t) = basew_delay + depth * sin(2 * π * rate * t)
```

Where:
- `delay_time(t)` is the delay time at time `t`

- `base_delay` is the base delay time (usually in
  milliseconds)

- `depth` is the LFO's modulation depth

- `rate` is the LFO's frequency (in Hz)

- `t` is the time variable

In summary, the LFO is a key component in the
chorus effect, responsible for modulating the
time-varying delay to create the characteristic
pitch and timing variations. By adjusting the
LFO's rate and depth, you can control the speed
and intensity of the chorus effect, shaping the
overall sound and texture.

7> what about the Modulated Delay Line?

The Modulated Delay Line is an essential component
of the chorus effect, responsible for applying the
time-varying delay to the input signal to create
the multiple "voices" with slightly different
pitches and timings that give the chorus effect
its distinctive sound.

In the context of a digital implementation of
a chorus effect, the Modulated Delay Line uses the
delay buffer and the time-varying delay, which is
controlled by an LFO, to create a continuously
changing delay time for the input signal.

Here's how the Modulated Delay Line works:

1. The input signal is fed into the delay buffer.

2. The LFO generates a periodic waveform
   (typically a sine wave) that modulates the
   delay time by controlling the read pointer's
   position in the delay buffer.

3. The read pointer retrieves the delayed samples
   from the delay buffer, while the write pointer
   writes new samples into the buffer.

4. The delayed signal, now with a continuously
   changing delay time, is extracted from the
   delay buffer using the read pointer.

5. The modulated delayed signal is then mixed with
   the original input signal to create the chorus
   effect.

The Modulated Delay Line is what generates the
characteristic pitch and timing variations of the
chorus effect. By applying a time-varying delay to
the input signal, it creates multiple "voices"
that are slightly out of sync in terms of pitch
and timing. When these delayed signals are mixed
with the original input signal, they produce the
rich and immersive sound characteristic of the
chorus effect.

In summary, the Modulated Delay Line is an
essential component of the chorus effect,
responsible for applying the time-varying delay
controlled by the LFO to the input signal. It
generates the multiple "voices" with slightly
different pitches and timings that give the chorus
effect its distinctive sound.

8> what about the Wet/Dry Mix & Sum?

The Wet/Dry Mix & Sum is an important component in
the chorus effect, responsible for blending the
processed (wet) signal with the original (dry)
input signal to create the final output. This
stage allows you to control the balance between
the unprocessed input signal and the
chorus-processed signal, shaping the overall sound
and intensity of the chorus effect.

Here's how the Wet/Dry Mix & Sum works:

1. The processed signal, which has the
   time-varying delay applied by the Modulated
   Delay Line, is referred to as the "wet" signal.

2. The original, unprocessed input signal is
   referred to as the "dry" signal.

3. The wet and dry signals are mixed together
   according to a user-defined mix level, which
   controls the balance between the wet and dry
   signals.

4. The mixed signal is then summed with the
   original input signal to create the final
   output.

The Wet/Dry Mix parameter typically ranges from 0%
to 100%, where 0% corresponds to only the dry
signal being present in the output, and 100%
corresponds to only the wet signal being
present. Values in between allow you to control
the balance between the original signal and the
chorus effect, adjusting the overall intensity of
the effect.

In many chorus effects, the Wet/Dry Mix parameter
is sometimes referred to simply as the "Mix" or
"Blend" control. Adjusting this parameter allows
you to find the right balance between the
unprocessed input signal and the chorus-processed
signal, creating a range of subtle to more
pronounced chorus effects.

In summary, the Wet/Dry Mix & Sum is an important
component in the chorus effect, responsible for
blending the processed (wet) signal with the
original (dry) input signal. By adjusting the mix
level, you can control the balance between the
unprocessed input signal and the chorus-processed
signal, shaping the overall sound and intensity of
the chorus effect.

9> what about the Output Signal?

The Output Signal is the final result of the
chorus effect after processing the input signal
through the various components: time-varying
delay, modulated delay line, and the wet/dry mix
& sum stage. It contains a blend of the original
input signal and the chorus-processed signal, with
the balance between them controlled by the wet/dry
mix parameter.

The output signal is typically a stereo signal,
even if the input signal is mono, as the chorus
effect often creates a spatially enhanced sound by
spreading the multiple "voices" across the stereo
field. The output signal can be further processed
by other effects or sent directly to the main mix
or an audio output device, depending on the
specific implementation and use case.

In the context of a synthesizer, the output signal
of the chorus effect would be mixed with other
audio signals generated by the synthesizer, such
as other oscillators, filters, and effects. In
a digital audio workstation (DAW) or an effects
processor, the output signal can be routed to
additional audio tracks or effects, or it can be
mixed with other audio signals in the final mix.

In summary, the Output Signal is the final result
of the chorus effect, containing a blend of the
original input signal and the chorus-processed
signal. The output signal is typically stereo and
can be further processed by other effects or sent
directly to the main mix or an audio output
device. The output signal represents the
culmination of the various processing stages of
the chorus effect, delivering the rich and
immersive sound characteristic of the effect.

10> could you show me an implementation in
verilog-a?

Implementing a chorus effect in Verilog-A is not
straightforward, as Verilog-A is primarily used
for analog circuit simulation and not for digital
signal processing (DSP) tasks like audio
effects. However, I can describe the general
concept of how a chorus effect could be
implemented in the context of an analog circuit,
which you could then potentially simulate using
Verilog-A.

An analog chorus effect can be implemented using
a bucket-brigade device (BBD), which is
a discrete-time analog delay line. The BBD,
combined with an LFO to modulate the delay time
and a mixer to combine the wet and dry signals,
can create a chorus effect.

Here's a high-level description of the components
required for an analog chorus effect:

1. **Input signal**: The audio input signal that
   you want to process with the chorus effect.

2. **LFO**: An analog oscillator that generates
   a low-frequency sine or triangle waveform to
   modulate the delay time.

3. **Voltage-controlled delay line**: A BBD-based
   circuit that uses the LFO's output to control
   the delay time. The BBD shifts the input signal
   through a series of capacitors, creating
   a time-delayed version of the signal.

4. **Wet/dry mix**: An analog mixer circuit that
   combines the delayed signal (wet) with the
   original input signal (dry) according to
   a user-defined mix level. The output of this
   mixer is the final output of the chorus effect.

5. **Output signal**: The audio output signal
   containing the blend of the original input
   signal and the chorus-processed signal.

Although this high-level description does not
provide the specific details required for
a Verilog-A implementation, it gives you an idea
of the analog components needed to create a chorus
effect in the context of an analog
circuit. Simulating such an analog chorus effect
in Verilog-A would involve modeling the behavior
of each component, including the BBD, LFO, and
mixer circuits.

Keep in mind that implementing audio effects like
chorus in Verilog-A is quite unusual and not the
most efficient or practical approach. Digital
signal processing (DSP) techniques, as implemented
in programming languages like C++, Python, or
Rust, are more commonly used for audio effect
development due to their flexibility, precision,
and efficiency.

11> but then you could get a chorus in analog
hardware, right?

Yes, you can create a chorus effect in analog
hardware using components such as a bucket-brigade
device (BBD) for the delay line, an analog
oscillator (LFO) to modulate the delay time, and
an analog mixer to combine the wet and dry
signals. Analog chorus effects were popular in
guitar pedals and synthesizers before digital
signal processing (DSP) became widespread.

However, SystemVerilog, like Verilog-A, is
primarily used for hardware description and
verification, not for digital signal processing
(DSP) tasks like audio effects. SystemVerilog is
mainly used to describe and verify digital
circuits at the register-transfer level (RTL) for
synthesis and simulation purposes, rather than
modeling or implementing audio processing
algorithms like a chorus effect.

Implementing a chorus effect in a hardware
description language (HDL) like SystemVerilog
would require designing a custom digital signal
processing (DSP) chip that performs the required
audio processing operations. This would involve
designing and implementing digital circuits for
components such as digital filters, digital
oscillators (LFO), and digital mixers, as well as
memory elements for delay buffers.

Designing and verifying a custom DSP chip in
SystemVerilog is a complex and time-consuming
task, and it's beyond the scope of a simple
example or explanation here. It's also not the
most practical or efficient approach for creating
audio effects like chorus, as general-purpose
programming languages and digital signal
processing libraries offer a more flexible and
efficient way to implement these effects.

If you're looking to implement a chorus effect,
I would recommend using a general-purpose
programming language like C++, Python, or Rust,
and leveraging existing digital signal processing
libraries or frameworks. These languages and
libraries provide a more accessible and practical
way to develop audio effects like chorus compared
to using hardware description languages like
SystemVerilog.

12> why not just write the whole dsp system in
analog hardware? surely the code isn't
prohibitively complicated

Implementing an entire DSP system in analog
hardware is indeed possible and was the standard
approach before digital signal processing became
widespread. However, there are several reasons why
digital implementations are now favored for most
applications, including audio effects like the
chorus:

1. **Precision**: Analog components are subject to
   manufacturing tolerances, temperature drift,
   and other factors that can introduce
   inaccuracies in the signal processing. Digital
   implementations, on the other hand, can achieve
   a high level of precision and repeatability,
   ensuring consistent performance.

2. **Flexibility**: In an analog hardware system,
   making changes to the design or functionality
   often requires modifying the physical circuit,
   which can be time-consuming and
   expensive. Digital implementations can be
   easily modified through software updates,
   allowing for rapid development and iteration.

3. **Complexity**: As the complexity of signal
   processing algorithms increases, designing and
   implementing them in analog hardware becomes
   more challenging. Digital implementations can
   handle complex algorithms more easily, and
   modern processors have the computational power
   to perform real-time processing of these
   algorithms.

4. **Size and cost**: Implementing a DSP system in
   analog hardware often requires a large number
   of components, increasing the size and cost of
   the system. Digital implementations can be more
   compact and cost-effective, particularly with
   the availability of dedicated DSP chips and
   integrated circuits.

5. **Integration**: Digital systems can be easily
   integrated with other digital components, such
   as microcontrollers, memory, and communication
   interfaces. This integration facilitates the
   development of complex systems that combine
   signal processing with other functionality,
   such as user interfaces or network
   connectivity.

While there are some specific use cases where
analog hardware implementations may still be
preferred, such as in certain guitar pedals or
vintage synthesizers, digital signal processing
has become the standard approach for most audio
effects due to its precision, flexibility, and
ease of integration.

It's worth noting that some designers create
hybrid systems that combine the benefits of both
analog and digital processing, using analog
components for certain tasks (e.g., filtering) and
digital processing for others (e.g., delay lines
or modulation). This approach can achieve the
desired sound characteristics while retaining some
of the advantages of digital processing.
