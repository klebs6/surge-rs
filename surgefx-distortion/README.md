Distortion effect for the Surge synthesizer
---

The `surgefx-distortion` crate provides
a distortion effect as a subcomponent of the Surge
synthesizer system. Distortion is a popular audio
effect that alters the sound by introducing
non-linearities and harmonics to the input signal,
often resulting in a warmer, thicker, or more
aggressive tone.

The main struct provided by this crate is
`Distortion`, which is responsible for processing
and managing the distortion effect. The
`Distortion` struct has several methods and fields
related to its operation, including:

- control_group
- control_type
- default_value
- max_value
- min_value
- modulateable
- moverate
- value_type
- handle_streaming_mismatches
- coeff_instantize_all_lp
- maybe_update
- update
- update_all_bands
- update_all_lp
- update_band1
- update_band2
- update_drive
- update_outgain
- update_post_hc
- update_pre_hc
- zero_drives
- get_waveshape_idx
- denormalizer
- do_denormalize_channels
- do_distortion_block
- do_feedback_channels
- do_waveshape_channels
- set_wetblock
- distortion_smoke
- init
- process
- process_band1
- process_band2
- process_blocks
- process_drive
- process_halfrate
- process_outgain

The distortion effect is typically achieved by
applying a non-linear transfer function to the
input signal, which introduces harmonic distortion
and changes the signal's waveform. Common types of
distortion include:

1. **Clipping**: Hard clipping or soft clipping
   introduces a sharp or smooth cut-off for the
   input signal's amplitude, resulting in
   a squared or rounded waveform. The transfer
   function for hard clipping can be represented
   as `y = min(max(x, -A), A)`, where `x` is the
   input signal, `y` is the output signal, and `A`
   is the clipping threshold.

2. **Overdrive**: Overdrive emulates the
   non-linear behavior of analog amplifiers and
   produces a smooth, gradual distortion. The
   transfer function for overdrive can be
   represented using polynomial approximations,
   sigmoid functions, or other smooth functions
   that exhibit non-linear behavior.

3. **Wave Shaping**: Wave shaping involves
   applying a custom transfer function to the
   input signal, allowing for more creative and
   diverse distortion effects. The transfer
   function can be defined as a mathematical
   equation or a piecewise function using lookup
   tables.

The `Distortion` struct provides methods for
configuring and processing the distortion effect,
such as adjusting the drive level, equalizing the
input signal using pre and post high-cut filters,
and applying various waveshaping techniques. By
combining these features, the `surgefx-distortion`
crate offers a flexible and powerful distortion
effect for the Surge synthesizer system.

Please note that this description assumes
a typical distortion effect based on the provided
tokens. The actual functionality of your specific
implementation might vary, so adjust the
description as needed to accurately reflect your
implementation.

16> could you show me a few different variants of
a distortion pipeline? what are the block
diagrams?

Certainly! Here are a few different variants of
a distortion pipeline, each with its own block
diagram:

1. **Basic Distortion Pipeline**:

```
   Input Signal → [Pre-Gain] → [Transfer Function] → [Output Gain] → Output Signal
```

   In this simple pipeline, the input signal is
   first amplified or attenuated using a pre-gain
   stage. Then, the signal is processed through
   a non-linear transfer function to apply the
   desired distortion effect. Finally, an output
   gain stage adjusts the overall level of the
   distorted signal before sending it to the
   output.

2. **Distortion with Pre and Post Equalization**:

```
   Input Signal → [Pre-EQ] → [Pre-Gain] → [Transfer Function] → [Output Gain] → [Post-EQ] → Output Signal
```

   This pipeline adds pre and post equalization
   stages to the basic distortion pipeline. The
   pre-EQ stage allows you to shape the frequency
   response of the input signal before applying
   distortion, while the post-EQ stage adjusts the
   frequency response of the distorted
   signal. These equalization stages can be
   implemented using various filter types (e.g.,
   low-pass, high-pass, band-pass, etc.) to create
   a more tailored distortion effect.

3. **Multiband Distortion**:

```
   Input Signal → [Crossover] → [Distortion (Low)] → [Sum] → Output Signal
                             ↳ [Distortion (Mid)] →
                             ↳ [Distortion (High)] →
```

   In a multiband distortion pipeline, the input
   signal is first split into multiple frequency
   bands using a crossover network. Each frequency
   band is then processed through its own
   distortion pipeline, consisting of pre-gain,
   transfer function, and output gain
   stages. Finally, the distorted signals from
   each band are summed together to create the
   output signal. This approach allows for
   independent control of the distortion
   characteristics for each frequency band,
   resulting in a more versatile and precise
   effect.

4. **Distortion with Feedback**:

```
   ┌─────────────────────────────┐
   │                             ↓
   Input Signal → [Pre-Gain] → [Transfer Function] → [Output Gain] → Output Signal
                             ↑
                             └───────────────────────┘
```

   This pipeline adds a feedback loop to the basic
   distortion pipeline. The output of the transfer
   function is fed back into the input of the
   pre-gain stage, creating a more complex and
   dynamic distortion effect. The amount of
   feedback can be controlled using a feedback
   gain or mix control, allowing you to adjust the
   intensity of the feedback effect.

These are just a few examples of distortion
pipeline variants. You can combine and modify
these stages to create your own custom distortion
pipelines tailored to your specific needs and
desired sound characteristics.

17> how do we know the distortion is "good
distortion" and not degradation?

Determining whether distortion is "good" or "bad"
depends on the context and the subjective
preferences of the listener. In general, good
distortion is often perceived as adding warmth,
character, or musicality to a sound, while bad
distortion is typically associated with unwanted
artifacts or degradation of the audio signal.

Here are some factors to consider when evaluating
the quality of a distortion effect:

1. **Harmonic Content**: Good distortion often
   generates harmonics that are musically related
   to the input signal, such as integer multiples
   of the fundamental frequency (e.g., even or odd
   harmonics). These harmonics can enhance the
   tonal richness of the sound. In contrast, bad
   distortion might produce inharmonic or
   dissonant harmonics that can make the sound
   harsh or unpleasant.

2. **Dynamics**: Good distortion can add sustain
   or compression to a sound, which can make it
   more consistent and balanced. However,
   excessive compression or limiting can lead to
   a loss of dynamic range and a "squashed" sound
   that might be perceived as undesirable.

3. **Frequency Response**: Good distortion might
   emphasize certain frequency ranges, such as
   boosting the low-mids for warmth or the
   high-mids for presence. Bad distortion might
   cause an uneven or unpredictable frequency
   response, which can result in a loss of clarity
   or a muddy sound.

4. **Transparency**: Good distortion can be
   transparent and natural-sounding, meaning it
   does not introduce noticeable artifacts or
   coloration when applied subtly. On the other
   hand, bad distortion might create audible
   artifacts or a "processed" sound, even at low
   levels.

5. **Versatility**: A high-quality distortion
   effect should offer a range of tonal
   characteristics and distortion types to cater
   to different musical styles and
   applications. A limited or inflexible
   distortion effect might not be suitable for
   a wide variety of sounds or genres.

It's important to note that the perception of good
or bad distortion is subjective and can vary
depending on the listener, the musical context,
and the specific sound being processed. In some
cases, a distortion effect that is considered
undesirable in one context might be desirable in
another. For example, a heavily distorted electric
guitar sound might be perfect for a rock or metal
track, but it might be considered bad distortion
in a jazz or classical piece.

Ultimately, the quality of a distortion effect
should be evaluated based on how well it meets the
needs and preferences of the user in a given
context. By offering a wide range of adjustable
parameters, a versatile distortion effect can
cater to various tastes and applications,
increasing the likelihood of producing "good"
distortion for a broader range of listeners and
situations.

18> where is the fronteir of the distortion field?

The frontier of the distortion field lies in the
ongoing research and development of new
algorithms, hardware, and techniques that push the
boundaries of traditional distortion effects. Some
areas of interest in this field include:

1. **Machine Learning and AI**: The application of
   machine learning and artificial intelligence
   techniques to model and create new types of
   distortion effects. This can involve training
   models on various distortion characteristics to
   produce novel and unique sounds or using AI to
   optimize parameters for specific musical
   contexts.

2. **Emulation of Vintage Hardware**: As
   technology advances, there is a growing
   interest in accurately emulating the
   characteristics of vintage hardware, such as
   analog amplifiers, tape machines, and tube
   circuits. This involves creating digital models
   that capture the non-linearities, harmonic
   content, and other nuances of these classic
   devices.

3. **Modular and Flexible Effects**: The
   development of modular and flexible distortion
   effects that allow users to combine and
   customize various stages of the distortion
   pipeline, such as pre-gain, transfer function,
   equalization, and feedback. This provides users
   with more control and creative possibilities
   when designing their distortion sounds.

4. **Physical Modeling**: Advances in physical
   modeling techniques enable the creation of more
   realistic and accurate digital representations
   of analog distortion devices, such as guitar
   amplifiers, stompboxes, and tube circuits. This
   involves simulating the complex interactions
   between various components (e.g., tubes,
   transformers, speakers) to reproduce the
   behavior of these devices in the digital
   domain.

5. **Real-Time Performance and Lower Latency**:
   The ongoing improvement of hardware and
   software capabilities enables the development
   of more efficient and lower-latency distortion
   effects. This allows musicians and producers to
   use more complex distortion algorithms in
   real-time, both in live performances and studio
   environments.

6. **Immersive Audio and Spatial Effects**: As
   immersive audio formats like surround sound and
   spatial audio become more popular, there is an
   increasing interest in developing distortion
   effects that can be applied to these
   formats. This involves creating distortion
   algorithms that can process and manipulate
   audio signals in three-dimensional space,
   adding depth and movement to the effect.

These are just a few examples of the frontier
areas in the distortion field. As technology
continues to evolve and new techniques and ideas
emerge, we can expect to see even more innovative
and creative distortion effects in the future.

19> how do we emulate vintage distortion hardware
with artificial intelligence?

Emulating vintage distortion hardware with
artificial intelligence involves training machine
learning models on the characteristics and
behavior of vintage hardware to create digital
models that can reproduce the nuances of these
classic devices. The process generally consists of
several steps:

1. **Data Collection**: Record the input and
   output signals of the vintage hardware at
   various settings, capturing the device's
   response to different types of audio signals
   and a range of parameter configurations. This
   dataset should include clean and distorted
   signals, covering the full range of the
   hardware's capabilities.

2. **Feature Extraction**: Analyze the input and
   output signals to extract relevant features
   that represent the behavior of the vintage
   hardware. Features may include time-domain
   characteristics (e.g., amplitude, waveform
   shape), frequency-domain characteristics (e.g.,
   harmonic content, spectral shape), and
   non-linear behavior (e.g., saturation, dynamic
   response).

3. **Model Selection**: Choose an appropriate
   machine learning model to represent the
   relationship between the input and output
   signals and the extracted features. Popular
   choices for audio processing tasks include deep
   learning models like convolutional neural
   networks (CNNs) and recurrent neural networks
   (RNNs), as well as other machine learning
   techniques such as support vector machines
   (SVMs) and decision trees.

4. **Training**: Train the selected machine
   learning model on the collected dataset,
   adjusting the model's parameters to minimize
   the difference between its output and the
   actual output of the vintage hardware. This may
   involve splitting the dataset into training,
   validation, and test sets to evaluate the
   model's performance and prevent overfitting.

5. **Evaluation**: Test the trained model on
   a separate set of input signals and compare its
   output to the actual output of the vintage
   hardware. Measure the model's performance using
   appropriate metrics such as mean squared error
   (MSE) or perceptual evaluation of audio quality
   (PEAQ). Iterate the training and evaluation
   process until the model achieves satisfactory
   performance.

6. **Implementation**: Implement the trained model
   in a digital audio processing environment, such
   as a plugin or standalone application, and
   optimize its performance for real-time
   use. This may involve converting the model to
   a more efficient representation, such as
   a reduced-precision neural network or a lookup
   table, and implementing it using
   high-performance programming languages and
   libraries.

By following these steps, artificial intelligence
can be used to create accurate digital emulations
of vintage distortion hardware that capture the
unique characteristics and behavior of these
classic devices. The resulting models can be used
in digital audio workstations (DAWs) and other
audio processing environments to achieve the
sought-after sound of vintage distortion effects
without the need for the original hardware.

20> how do the subcomponents of the distortion
module interact from a physical perspective to
produce the desired effect?

From a physical perspective, the subcomponents of
a distortion module interact by processing the
input audio signal through a series of stages,
which can include gain stages, non-linear transfer
functions, filters, and feedback loops. Each stage
affects the signal in a specific way to create the
desired distortion effect.

Here's a step-by-step breakdown of how the
subcomponents of a typical analog distortion
module interact physically:

1. **Input Stage**: The input stage receives the
   incoming audio signal, which can be a voltage
   waveform from a guitar pickup, a line-level
   signal from a synthesizer, or any other audio
   source. This stage may include impedance
   matching or buffering to ensure the signal is
   properly interfaced with the rest of the
   distortion module.

2. **Pre-Gain Stage**: The pre-gain stage
   amplifies or attenuates the input signal to
   adjust its level before being processed by the
   transfer function. This stage can be
   implemented using operational amplifiers
   (op-amps), transistors, or vacuum tubes. The
   gain is usually adjustable by the user through
   a potentiometer or other control.

3. **Transfer Function**: This stage applies the
   non-linear distortion effect to the signal. It
   can be implemented using various electronic
   components, such as diodes, transistors, or
   vacuum tubes, which introduce non-linear
   behavior as they operate near their saturation
   or cutoff regions. The transfer function can be
   designed to produce different types of
   distortion effects, such as soft clipping, hard
   clipping, or wave shaping.

4. **Tone Control / Equalization**: After the
   non-linear stage, the signal may pass through
   a tone control or equalization stage. This
   stage adjusts the frequency content of the
   distorted signal, emphasizing or attenuating
   specific frequency bands. It can be implemented
   using passive or active filters, such as
   low-pass, high-pass, or band-pass filters.

5. **Output Gain Stage**: The output gain stage
   adjusts the overall level of the distorted
   signal before it is sent to the output. This
   stage can also be implemented using op-amps,
   transistors, or vacuum tubes. The output gain
   is usually adjustable by the user through
   a potentiometer or other control.

6. **Feedback Loop (optional)**: Some distortion
   modules may include a feedback loop, which
   takes a portion of the output signal and feeds
   it back to the input of the module or an
   earlier stage in the signal path. This creates
   a more complex and dynamic distortion
   effect. The amount of feedback can be
   controlled using a feedback gain or mix
   control, allowing the user to adjust the
   intensity of the feedback effect.

From a physical perspective, the subcomponents of
the distortion module interact by affecting the
voltage waveform of the input signal in various
ways. This can include amplifying or attenuating
the signal, introducing non-linear behavior,
adjusting the frequency content, and creating
feedback loops. The combined effect of these
interactions results in the desired distortion
effect, which can add warmth, character, or
aggression to the input signal, depending on the
specific design and settings of the distortion
module.
