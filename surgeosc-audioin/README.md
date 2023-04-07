# surgeosc-audioin

Audio input oscillator component for the Surge
synthesizer system.

## Description

`surgeosc-audioin` is a Rust crate providing an
audio input oscillator component as a part of the
Surge synthesizer system. This crate enables
real-time audio input processing with various
control parameters and modulation options, making
it an essential building block for any Surge-based
synthesizer project.

### Control Parameters

The following tokens represent the different
control parameters available in this crate:

- `control_group`: Represents a group of related
  controls that can be manipulated together.

- `control_type`: Describes the type of control
  (e.g. knob, slider, etc.).

- `default_value`: The initial value of a control
  parameter when instantiated.

- `max_value`: The maximum allowed value for
  a control parameter.

- `min_value`: The minimum allowed value for
  a control parameter.

- `modulateable`: A boolean indicating whether
  a control parameter can be modulated by external
  sources (e.g., LFOs, envelopes, etc.).

- `moverate`: The rate at which a control
  parameter can change over time when being
  modulated.

- `value_type`: The data type of the control
  parameter value (e.g., float, integer, etc.).

### Processing Functions

The crate provides different processing functions
that can be used depending on the desired output:

- `process_block`: Processes an input block of
  audio samples.

- `process_block_mono`: Processes an input block
  of mono audio samples.

- `process_block_stereo`: Processes an input block
  of stereo audio samples.

### AudioInputOscillator

`AudioInputOscillator` is the main struct
representing the audio input oscillator component
in this crate. It contains all the necessary
control parameters and processing functions
required to manipulate and process audio input in
real-time.

## Mathematical Analysis

The audio input oscillator processes the input
signal using various mathematical operations, such
as multiplication, addition, and scaling. These
operations can be represented using the following
mathematical equations in Unicode:

1. Modulation: 𝑦(𝑡) = 𝑥(𝑡) × 𝑚(𝑡), where 𝑥(𝑡) is
   the input signal, 𝑚(𝑡) is the modulation
   signal, and 𝑦(𝑡) is the output signal.

2. Scaling: 𝑦(𝑡) = 𝑎 × 𝑥(𝑡), where 𝑎 is a scaling
   factor and 𝑥(𝑡) is the input signal.

3. Clipping: 𝑦(𝑡) = min(max(𝑥(𝑡), 𝑎), 𝑏), where
   𝑎 is the lower clipping threshold, 𝑏 is the
   upper clipping threshold, and 𝑥(𝑡) is the input
   signal.

These mathematical operations allow for a wide
range of audio processing possibilities, enabling
users to create unique and expressive sounds using
the `surgeosc-audioin` crate.

2> okay, thanks -- please walk me through the
process of processing audio input in real-time.
how do we do it? what are our engineering
constraints?

Processing audio input in real-time is a complex
task that requires efficient algorithms and
careful consideration of various engineering
constraints. Here is a high-level overview of the
process and the main constraints to consider:

1. **Capture audio input**: Real-time audio
   processing begins with capturing audio input
   from a source like a microphone, an instrument,
   or an audio file. This is typically done using
   an audio interface or a sound card that
   converts analog signals into digital samples.

2. **Buffering**: Audio data is usually processed
   in small chunks called buffers. The buffer size
   determines the granularity of the processing
   and directly affects the latency of the
   system. Smaller buffer sizes result in lower
   latency but require more CPU resources to
   process, while larger buffer sizes reduce the
   CPU load but increase latency.

3. **Sampling rate**: Audio signals are continuous
   in nature, but digital audio processing deals
   with discrete samples. The sampling rate is the
   number of samples taken per second, and it
   determines the audio quality and frequency
   range that can be represented. Higher sampling
   rates result in better audio quality but
   require more processing power and memory.

4. **Audio processing**: Once the audio data is
   captured and buffered, it can be processed
   using various audio processing algorithms like
   filtering, modulation, and effects
   processing. The `surgeosc-audioin` crate you
   are working on provides an audio input
   oscillator component that can be used to
   process audio input in real-time. The
   processing functions like `process_block`,
   `process_block_mono`, and
   `process_block_stereo` implement the required
   algorithms and manipulations.

5. **Control parameters**: The behavior of the
   audio processing can be controlled using
   various parameters like gain, modulation depth,
   and filter cutoff. These parameters can be
   adjusted in real-time, either by user
   interaction or through automation and
   modulation sources (e.g., LFOs, envelopes). The
   crate you are working on provides tokens to
   represent these control parameters, like
   `control_group`, `control_type`,
   `default_value`, `max_value`, `min_value`,
   `modulateable`, `moverate`, and `value_type`.

6. **Output**: The processed audio data is then
   sent to an audio output device (e.g., speakers,
   headphones) or further processing (e.g.,
   mixing, effects). This may involve converting
   the digital samples back to analog signals.

### Engineering Constraints

There are several engineering constraints to
consider when implementing real-time audio
processing:

- **Latency**: Low latency is crucial for
  real-time audio processing, as high latency can
  result in noticeable delays between user input
  and audio output. To minimize latency, you need
  to optimize buffer sizes and processing
  algorithms for efficiency.

- **CPU usage**: Real-time audio processing can be
  computationally intensive, particularly when
  using complex algorithms and high sampling
  rates. You must optimize your code to minimize
  CPU usage and avoid performance bottlenecks that
  could cause audio artifacts like clicks, pops,
  or dropouts.

- **Memory usage**: Audio processing can require
  significant amounts of memory, particularly when
  dealing with large sample buffers and multiple
  processing stages. Be mindful of memory usage
  and optimize memory allocation and data
  structures as needed.

- **Precision and numerical stability**: Audio
  processing algorithms often involve
  floating-point arithmetic, which can be prone to
  rounding errors and instability. Ensuring
  numerical stability and adequate precision is
  essential to avoid artifacts and maintain audio
  quality.

- **Concurrency and thread safety**: Real-time
  audio processing often involves multiple
  threads, such as a separate thread for audio
  input/output and another for user interface
  updates. Ensuring your code is thread-safe and
  free from race conditions is crucial for
  stability and performance.

- **Platform compatibility**: Different platforms
  (e.g., Windows, macOS, Linux) and hardware
  configurations may have different requirements,
  performance characteristics, and
  limitations. Ensure that your crate is
  compatible with the target platforms and can
  handle varying hardware capabilities.

By carefully

3> /file surge-rs/surgeosc-audioin/src/process.rs

The provided code defines the implementation of
the process functions for the
`AudioInputOscillator` struct. There are three
main process functions: `process_block`,
`process_block_mono`, and
`process_block_stereo`. Let's examine each
function and understand its purpose.

1. `process_block`: This function is implemented
   for the `OscillatorProcess` trait and serves as
   a dispatcher. It takes a `cfg:
   OscillatorProcessBlockCfg` parameter, which
   provides information about the processing
   context, such as whether the input is stereo or
   mono. Depending on the `cfg.stereo` value, it
   calls either `process_block_stereo` or
   `process_block_mono`.

2. `process_block_mono`: This function processes
   a mono input signal. It computes the gain and
   input parameters from the oscillator's
   parameter values using `pvalf`. Then, it
   calculates the coefficients `a` and `b` based
   on the gain and input values. Finally, it
   iterates over each sample in the input buffer
   (with a size of `BLOCK_SIZE_OS`), combines the
   left and right input channels with the
   coefficients, and writes the result to the left
   output channel.

3. `process_block_stereo`: This function processes
   a stereo input signal, similar to the
   `process_block_mono` function. It calculates
   the gain, input, and coefficients `a` and `b`
   in the same way. The main difference is that it
   keeps the left and right channels separate when
   processing the input signal. It writes the
   result to the left and right output channels
   separately, applying the coefficients `a` and
   `b` to the respective input channels.

Some interesting points about these functions:

- The use of `match` in the `process_block`
  function is a simple and elegant way to branch
  between the stereo and mono processing functions
  based on the `cfg.stereo` value.

- The use of the `limit_range` function in both
  `process_block_mono` and `process_block_stereo`
  ensures that the input value is clamped between
  -1.0 and 1.0, preventing potential issues caused
  by out-of-range values.

- The gain and input parameters are converted from
  decibels to linear values using the
  `db_to_linear` function from the `tables` field
  in the `AudioInputOscillator` struct. This is
  a common operation in audio processing, as it
  allows for more intuitive control over gain
  values.

In summary, the process functions in the provided
code handle real-time audio processing for both
mono and stereo input signals. They control the
balance between input channels and apply gain
adjustments based on the oscillator's parameter
values.

4> interesting, so does this oscillator do
anything more than properly fill the oscillator
output by reading the input in mono or stereo
mode? (other than the light clamping, and value
conversion) also, could you teach me about
converting from decibels to linear values? why do
we do this on the audio input oscillator?

You're correct; the primary function of this
`AudioInputOscillator` is to read the input signal
(either mono or stereo) and fill the oscillator
output buffer accordingly. It applies gain
adjustments and light clamping on the input
values, but other than that, it does not perform
any complex operations or transformations like
a traditional oscillator that generates
waveforms. This oscillator is designed to act as
an audio input source that can be processed
further down the signal chain.

Regarding converting decibels (dB) to linear
values, let's first understand why we use
decibels. Decibels are a logarithmic scale that
measures the ratio between two values (e.g.,
power, intensity, or amplitude). In audio
processing, decibels are often used to represent
gain, attenuation, and signal levels, as they
provide a more intuitive way of working with the
wide range of values involved in audio
signals. The human perception of loudness is
logarithmic, meaning that we perceive a doubling
of the amplitude as a constant increase in
loudness, rather than a linear increase. By using
a logarithmic scale like decibels, we can better
align with this perceptual characteristic.

However, when performing mathematical operations
or processing audio signals, it is often necessary
to convert these decibel values back into linear
values. This conversion is done using the
following formula:

`linear_value = 10^(decibel_value / 20)`

In the `AudioInputOscillator`, we convert gain
values from decibels to linear values because the
actual audio signal processing (e.g.,
multiplication, addition) requires linear
values. By converting the gain to a linear scale,
the oscillator can apply the appropriate gain
adjustments directly to the input samples.

In the provided code, the `db_to_linear` function
performs this conversion. It is called in both
`process_block_mono` and `process_block_stereo`
functions to convert the gain parameter from
decibels to a linear value before applying it to
the input signal.

5> interesting. where does the audio input come
from?

In the provided code, the audio input comes from
the `synth_in` field of the `AudioInputOscillator`
struct. Specifically, the input is accessed using
the following methods:

- For the left input channel:
  `self.synth_in.audio_in0(k)`

- For the right input channel:
  `self.synth_in.audio_in1(k)`

Here, `k` represents the sample index within the
input buffer.

The `synth_in` field is an instance of a struct
that represents the synthesizer input context,
which includes the audio input buffers, among
other things. This struct is typically provided by
the larger synthesizer system or application that
incorporates the `AudioInputOscillator`.

To give you a general idea of how the audio input
is captured and routed to the
`AudioInputOscillator`, here's a high-level
overview of the process:

1. An audio input source, such as a microphone, an
   instrument, or an audio file, generates an
   analog audio signal.

2. The analog signal is captured by an audio
   interface or a sound card, which converts the
   analog signal into digital samples. This
   process involves analog-to-digital conversion
   (ADC) and sampling at a specific rate, such as
   44.1 kHz or 48 kHz.

3. The digital audio samples are buffered into
   small chunks for efficient processing. These
   buffers are typically managed by the audio
   system or audio library used by the synthesizer
   application.

4. The synthesizer application routes the audio
   input buffers to the relevant processing
   components, such as oscillators, filters, and
   effects. In this case, the audio input buffers
   are passed to the `AudioInputOscillator`
   through the `synth_in` field.

5. The `AudioInputOscillator` processes the audio
   input buffers as described in the previous
   explanations of the `process_block_mono` and
   `process_block_stereo` functions.

It's important to note that the specific
implementation details and routing of the audio
input will depend on the larger synthesizer system
or application that you are integrating the
`AudioInputOscillator` crate into. The provided
code assumes that the audio input context (i.e.,
the `synth_in` field) is available and properly
set up by the surrounding system.

6> cool, thanks! what else should we know when
using this oscillator? any caveats?

When using the `AudioInputOscillator`, there are
a few important points and caveats to keep in
mind:

1. **Properly setting up the audio input
   context**: As mentioned before, the
   `AudioInputOscillator` relies on an external
   audio input context (the `synth_in` field) to
   access the input buffers. Ensure that the
   surrounding synthesizer system or application
   properly sets up and manages this context,
   including capturing the audio input, handling
   buffering, and routing the buffers to the
   oscillator.

2. **Parameter ranges and handling**: The
   oscillator has parameters like Gain and Input,
   which are used to control the audio
   processing. Make sure that the user interface
   or automation system that interacts with these
   parameters correctly handles the allowed
   parameter ranges and updates the oscillator
   state accordingly.

3. **Performance considerations**: Real-time audio
   processing can be computationally intensive,
   especially when dealing with complex
   algorithms, high sampling rates, and low buffer
   sizes. While the `AudioInputOscillator`
   implementation is relatively simple, it's still
   essential to keep an eye on CPU usage and
   optimize the overall system performance if
   needed.

4. **Thread safety and concurrency**: In many
   audio applications, multiple threads are used
   to handle tasks like audio input/output, user
   interface updates, and other tasks. Ensure that
   the `AudioInputOscillator` is used in
   a thread-safe manner, avoiding race conditions
   or other concurrency-related issues that could
   lead to unexpected behavior or crashes.

5. **Platform compatibility**: Different platforms
   (e.g., Windows, macOS, Linux) and hardware
   configurations might have different
   requirements, performance characteristics, and
   limitations. Make sure that your crate, as well
   as the larger synthesizer system, is compatible
   with the target platforms and can handle
   varying hardware capabilities.

6. **Integration with other processing stages**:
   The `AudioInputOscillator` is just one
   component in a larger signal processing chain,
   which might include other oscillators, filters,
   and effects. Ensure that the output of the
   `AudioInputOscillator` is properly routed to
   the subsequent processing stages and that the
   overall signal flow is well managed.

By being mindful of these points and caveats, you
can successfully integrate the
`AudioInputOscillator` into your synthesizer
system or application and achieve the desired
functionality and performance.

## technique

7> what are some techniques we can perform with
this oscillator? could you list me 12 high
production-quality techniques from electronic
music production for using this component?

While the `AudioInputOscillator` itself is
a relatively simple component designed to pass
audio input through with gain adjustments and
channel balancing, it can be a valuable part of
more complex audio processing chains. Here are 12
techniques that you can explore when using the
`AudioInputOscillator` in an electronic music
production context:

1. **Vocoding**: Use the audio input as the
   modulator signal for a vocoder effect, allowing
   you to create synthesized vocal sounds or
   robotic speech effects.

2. **Sidechain compression**: Route the audio
   input to a sidechain input of a compressor
   effect. This technique is popular for creating
   a "pumping" or "ducking" effect, where the
   audio level of one track is reduced based on
   the input level of another track, like a kick
   drum.

3. **Layering with other oscillators**: Combine
   the output of the `AudioInputOscillator` with
   other oscillators generating different
   waveforms or samples. This can create a richer,
   more complex sound and add harmonic content to
   the input signal.

4. **Granular synthesis**: Use the audio input as
   a source for granular synthesis, which involves
   chopping the audio into small pieces (grains)
   and manipulating them to create unique textures
   and soundscapes.

5. **Ring modulation**: Multiply the audio input
   with another oscillator's output to create
   metallic, bell-like, or inharmonic sounds.

6. **Dynamic filtering**: Use the audio input to
   modulate the cutoff frequency or resonance of
   a filter effect. This can create dynamic,
   expressive filtering effects that change based
   on the input signal's amplitude.

7. **Pitch shifting and harmonizing**: Apply pitch
   shifting or harmonizing effects to the audio
   input to create interesting harmonies, chords,
   or textures.

8. **Time stretching**: Manipulate the playback
   speed of the audio input without affecting the
   pitch, creating unique soundscapes and
   textures.

9. **Beat slicing and rearranging**: Process the
   audio input by slicing it into smaller pieces
   based on transients or beats, then rearrange
   the slices to create new rhythms and patterns.

10. **Reverse reverb**: Apply a reverb effect to
    the reversed audio input, and then reverse the
    resulting audio again. This creates a unique
    "swelling" reverb effect that can add
    atmosphere and depth to a sound.

11. **Multiband processing**: Split the audio
    input into multiple frequency bands and apply
    different effects or processing to each band,
    allowing for precise control over the
    frequency content of the input signal.

12. **Feedback loops**: Route the output of the
    `AudioInputOscillator` into a processing chain
    that includes a delay or reverb effect, then
    route the output of that chain back into the
    oscillator's input. This creates a feedback
    loop that can result in evolving,
    self-generating sounds and textures.

These techniques are just a starting point for the
creative possibilities offered by incorporating
the `AudioInputOscillator` into your electronic
music production workflow. By combining this
oscillator with other audio effects and
processors, you can create a wide variety of
unique sounds and textures.
