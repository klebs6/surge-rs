# surgefilter-snh

The `surgefilter-snh` crate is a Rust library
containing a filter based on the Sample and Hold
Noise (SNH) technique.

## SNH Filter

The SNH filter generates a noise signal that is
updated at a fixed frequency and held constant
between updates. This noise signal is then
multiplied with the input signal to produce the
filtered output. The effect of the SNH filter can
be varied by changing the frequency of the noise
signal.

## Mathematical Analysis

The SNH filter can be represented by the following
equation:

```
output(t) = input(t) * snh(t)
```

where `snh(t)` is the noise signal at time
`t`. The SNH filter can be implemented using the
direct form II topology, and the coefficients for
the filter can be generated using the `coeff_make`
function provided in the `surgefilter-snh` crate.

## Tokens

- `SnhFilter`: The main struct representing the
  SNH filter.

- `coeff_make`: A function for generating the
  filter coefficients.

- `process_quad`: A function for processing
  a block of audio using the SNH filter.

## Usage

To use the SNH filter in your Rust project, simply
add `surgefilter-snh` as a dependency in your
`Cargo.toml` file:

```toml
[dependencies]
surgefilter-snh = "0.1"
```

Then, import the crate and use the `SnhFilter`
struct to create a new SNH filter. You can
generate the filter coefficients using the
`coeff_make` function and then use the
`process_quad` function to process audio data in
blocks.

```rust
use surgefilter_snh::{SnhFilter, coeff_make, process_quad};

fn main() {
    // Create a new SNH filter with a noise frequency of 10 Hz
    let noise_frequency = 10.0;
    let mut snh_filter = SnhFilter::new(coeff_make(noise_frequency));

    // Process audio data in blocks
    let block_size = 512;
    let mut input_buffer = vec![0.0; block_size];
    let mut output_buffer = vec![0.0; block_size];
    loop {
        // Fill the input buffer with audio data
        // ...

        // Process the input buffer using the SNH filter
        process_quad(&mut input_buffer, &mut output_buffer, &mut snh_filter);

        // Do something with the output buffer
        // ...
    }
}
```

101> thanks -- could you please teach me the
sample and hold noise function?

Sure, I can explain sample and hold noise function
(S&H).

In analog circuits, a sample and hold (S&H)
circuit is used to sample an analog signal at
a specific point in time and then hold that sample
for a certain period. The S&H function is often
used to convert continuous analog signals into
discrete digital signals.

In a typical S&H circuit, a switch is used to
connect a capacitor to the input signal. The
capacitor charges up to the input voltage level
and is then disconnected from the input signal by
the switch. The capacitor then holds the sampled
voltage level until the switch is closed again to
take a new sample.

The noise in the S&H circuit can come from various
sources, such as the switch itself or the input
signal itself. To model this noise in a digital
system, a random number generator can be used to
simulate the noise.

In the SnhFilter crate, the S&H noise function is
implemented using a uniform distribution random
number generator. The output signal is held for
a certain amount of time, determined by the sample
rate and the sample and hold time, before being
updated with a new sample.

The S&H noise function is useful for generating
random noise signals or for modulating other
signals in a random way.

102> could show block diagram?

Sure, here's a block diagram of a basic Sample and
Hold circuit:

```
         +---------+
Vin ---->|         |
         | Sample  |----> Vout
Clock--->| and Hold|
         |         |
Gnd ---->|         |
         +---------+
```

The input voltage `Vin` is sampled and held at the
output voltage `Vout` for a certain period of
time, determined by the clock signal. The clock
signal triggers the Sample and Hold circuit to
sample the input voltage at a specific moment, and
then holds that voltage level until the next clock
pulse. The holding capacitor is charged to the
sampled voltage and discharges slowly over time.

In a Sample and Hold Noise circuit, the voltage
held by the capacitor can be corrupted by
noise. The SNH filter is used to reduce this
noise.

103> could show subcomponents in block diagram?

Sure, here is a block diagram of the SNH filter,
with subcomponents labeled:

```
               +------------------------+
               |                        |
      +--------|+                      |
      |        |  |                     |
      |        |  |  +--------+         |
      |        |  |  |        |         |
      |        |  |  |  Hold  |         |
      |        |  |  |        |         |
      |        |  |  +--------+         |
      |        |  |       |             |
      |        |  |    +-----+          |
      |        |  |    |     |          |
      |        |  |  +-|+   +|-+        |
      |        |  |  | |     | |        |
      |        |  |  | +-----+ |        |
      |        |  |  |    D    |        |
      |        |  |  |        |        |
      |        |  |  +--------+        |
      |        |  |                     |
      |        |  |      v(t)           |
      |        |  |      +---------+   |
      |        |  |      |         |   |
      |        |  +------|+        |   |
      |        |         | Sample  |   |
      |        |         |         |   |
      |        |         +--------|+  |
      |        |                  ||  |
      |        |              +---||--+|
      |        |              | +---+   |
      |        |              |         |
      |        |              | Quant   |
      |        |              |         |
      |        |              +---------+
      |        |
      |        |               v(t+T)
      |        |         +----------------+
      |        |         |                |
      |        +---------|+              |
      |                  |  Delay Line   |
      |                  |              |
      |                  +--------------+
      |                          |
      |                          |
      |                          |
      |                          |
      |                          |
      |                          |
      +--------------------------+
```

The subcomponents of the SNH filter are:

- **Sample & Hold**: This module samples the input
  signal at periodic intervals and holds the value
  until the next sample. The output signal is
  a staircase waveform that approximates the input
  signal.

- **Quantizer**: This module discretizes the
  staircase waveform output by the sample and hold
  module into a finite number of levels. This is
  done by comparing the input signal to a set of
  thresholds, which divide the input signal into
  equal-sized intervals.

- **Delay Line**: This module delays the quantized
  output of the quantizer by a specified time
  period. The delayed signal is then subtracted
  from the quantized output to produce the SNH
  output signal.

The SNH filter is a type of non-linear filter, as
the quantization step introduces non-linearities
into the output signal. The SNH filter is commonly
used in synthesizers to generate random or
pseudo-random signals for modulation or other
purposes.

104> could show equations more?

Certainly! Here are the equations for the SNH
filter:

Let x(t) be the input signal to the SNH filter,
and y(t) be the output signal. Let T be the sample
and hold time, and let f_s be the sampling
frequency.

The SNH filter can be implemented as follows:

y(t) = x(kT), for kT ≤ t < (k+1)T
y(t) = y(kT), for (k+1)T ≤ t < (k+2)T

This equation describes a step function that
samples the input signal at multiples of the
sample and hold time. The output signal is held
constant between sample times.

This can be implemented in discrete time as
follows:

y[n] = x[k], for k ≤ n < k + M
y[n] = y[k], for k + M ≤ n < k + 2M

where M = T * f_s is the number of samples in one
sample and hold period.

To implement this in code, we can use a modulo
operator to check if the current sample index is
within the sample and hold window, and then either
sample the input signal or hold the previous
output signal:

```rust
let mut y = vec![0.0; x.len()];
let mut prev_y = 0.0;
let mut index = 0;

for n in 0..x.len() {
    if n % M == 0 {
        prev_y = x[n];
        index = n;
    }
    if index <= n && n < index + M {
        y[n] = prev_y;
    } else if n >= index + M {
        prev_y = y[n - M];
        y[n] = prev_y;
        index = n;
    }
}
```

This code loops through each sample of the input
signal x, and checks if the current index is
within the sample and hold window. If it is, the
previous output signal prev_y is held as the
current output signal y. If it is not, the
previous output signal is updated to the output
signal from M samples ago, and the current output
signal is set to that value.

I hope that helps!

105> any caveats?

There are a few caveats to keep in mind when using
the SNH filter:

1. The SNH filter introduces noise into the
   signal. While this can be a desirable effect in
   some cases, it may not always be appropriate
   for certain applications.

2. The amplitude of the noise introduced by the
   SNH filter is proportional to the rate at which
   the signal changes. This means that if the
   input signal changes rapidly, the noise
   introduced by the filter will be larger,
   potentially leading to undesirable artifacts in
   the output signal.

3. The SNH filter can be sensitive to the sampling
   rate of the input signal. In some cases, this
   can result in aliasing or other artifacts in
   the output signal.

4. The SNH filter may not be suitable for use in
   certain applications where high precision or
   accuracy is required, as the noise introduced
   by the filter can be unpredictable and
   difficult to control.

5. Finally, it is important to note that while the
   SNH filter can be a powerful tool for creating
   unique and interesting sounds, it should always
   be used in a thoughtful and intentional manner
   to avoid unwanted noise or other artifacts in
   the output signal.

106> any usage techniques?

Sure! Here are some usage techniques for the
`SnhFilter`:

1. Use it to create sample-and-hold effects: The
   `SnhFilter` is great for creating
   sample-and-hold effects in your audio
   signal. You can use it to sample the input
   signal at regular intervals, and then hold the
   sampled value until the next sample is
   taken. This can create interesting rhythmic
   effects or can be used to add a bit of
   randomness to your signal.

2. Experiment with different sampling rates: The
   sampling rate of the `SnhFilter` determines how
   often the input signal is sampled. Experiment
   with different sampling rates to achieve
   different effects. Higher sampling rates will
   result in more detail in the sampled signal,
   while lower sampling rates will produce a more
   coarse signal.

3. Use it in combination with other filters: The
   `SnhFilter` can be used in combination with
   other filters to achieve more complex
   effects. Try chaining the `SnhFilter` with
   other filters, such as a low-pass or high-pass
   filter, to create more interesting sounds.

4. Use it as a modulation source: The `SnhFilter`
   can also be used as a modulation source. You
   can modulate the sampling rate or the amplitude
   of the sampled signal to create dynamic changes
   in your sound.

5. Be aware of aliasing: When using the
   `SnhFilter`, be aware of the potential for
   aliasing. Aliasing can occur when the sampling
   rate is too low and results in unwanted
   artifacts in the output signal. To avoid
   aliasing, make sure the sampling rate is high
   enough to capture the detail in the input
   signal.

107> how to break?

I'm sorry, but I'm not sure what you're
asking. Could you please clarify your question?

108> possible to break snh filter?

It is possible to break any filter if the input
signal is beyond its operating range or if the
filter parameters are set in such a way that they
cause numerical instability or overflow. However,
if the input signal is within the normal operating
range and the filter parameters are set correctly,
the SNH filter should function properly without
breaking.

109> any edge cases?

In general, edge cases for the SNH filter arise
when the input signal to the filter changes
rapidly or unexpectedly. When the input signal
changes too quickly or jumps erratically, the
output of the filter can be unpredictable and may
result in unwanted noise or distortion. It is also
possible for the filter to generate unwanted noise
or distortion if the sample rate of the incoming
signal is too low, as the filter may not be able
to accurately track the changes in the input
signal. It is important to consider the dynamic
range of the input signal and adjust the filter
parameters accordingly to ensure the best possible
performance.

110> what about fast scrubbing?

When the user rapidly changes the control value
being sampled, it can cause issues with the SNH
filter because it does not have enough time to
sample enough values to produce an accurate
output. This can result in undesirable artifacts
such as stepping and quantization noise. To
mitigate this issue, one approach is to use a slew
limiter to smooth out the changes in the control
signal and limit the rate of change. Another
approach is to use a different type of filter that
is better suited for rapidly changing signals,
such as a one-pole low-pass filter or a comb
filter.

111> what about scrubbing near poles?

Scrubbing near poles can cause instability in the
SNH filter, resulting in unpredictable output
behavior. To avoid this, it is recommended to
avoid scrubbing near poles, or to implement
pole-zero cancellation techniques in the design of
the filter.

112> what about scrubbing near edges?

Scrubbing near the edges of the sample and hold
noise function may lead to aliasing, where
high-frequency components are incorrectly
represented as lower-frequency components. This
can result in unwanted distortion and noise in the
output signal. It is generally advisable to avoid
scrubbing near the edges of the function and to
use appropriate filtering techniques to reduce
aliasing.

113> what applications?

The SNH filter can be used in a variety of
applications in digital signal processing, such as
audio effects processing, data smoothing and
filtering, and control systems. Specifically, it
is often used to introduce noise or randomness
into a signal, which can be useful for creating
unique sounds or adding character to
a recording. It can also be used for smoothing out
data, such as in sensor data processing, or for
filtering out unwanted noise in a control system.

114> please show?

Unfortunately, I cannot provide an example of an
application without further context about what you
are looking for. Could you please provide more
information or clarify your question?

115> sample and hold technique applications

The sample and hold technique has various
applications, some of which are:

1. Analog-to-digital conversion: Sample and hold
   circuits are often used in analog-to-digital
   converters to sample the analog signal and hold
   it long enough to be converted into a digital
   signal.

2. Pulse width modulation: Sample and hold
   circuits can be used in pulse width modulation
   (PWM) to maintain the pulse width for a certain
   period of time.

3. Signal processing: Sample and hold circuits can
   be used in signal processing to freeze the
   value of a signal at a particular instant for
   further processing.

4. Synthesizer and audio processing: Sample and
   hold circuits can be used in synthesizers and
   audio processing to generate a stepped waveform
   or to introduce randomness into a signal.

5. Image processing: Sample and hold circuits can
   be used in image processing to capture and
   store the image data for further processing.

6. Communication systems: Sample and hold circuits
   can be used in communication systems to capture
   and hold the signal before processing it.

These are just a few examples of the applications
of sample and hold circuits. The technique is
widely used in various fields of engineering and
technology.

116> more please!

Sure, here are some additional applications of
sample and hold techniques:

1. Analog-to-Digital Conversion: Sample and hold
   circuits are commonly used in analog-to-digital
   converters (ADCs) to hold a voltage steady
   during the conversion process. The input
   voltage is sampled and held for a brief period
   of time, allowing it to be accurately measured
   by the ADC.

2. Communication Systems: Sample and hold circuits
   are used in communication systems to capture
   analog signals and convert them into digital
   signals. This is commonly used in digital
   communication systems like Wi-Fi, Bluetooth,
   and cellular networks.

3. Audio Processing: Sample and hold circuits are
   used in audio processing applications to create
   effects like portamento and glissando, which
   involve smoothly changing the pitch of an audio
   signal.

4. Image Processing: Sample and hold circuits are
   used in image processing applications to
   capture the brightness or color of individual
   pixels in an image.

5. Control Systems: Sample and hold circuits are
   used in control systems to take samples of
   a system's output and use them to adjust the
   input signals. This is commonly used in
   feedback control systems, where the output of
   the system is compared to a desired value and
   adjustments are made to the input signals to
   bring the output closer to the desired value.

6. Sensor Applications: Sample and hold circuits
   are used in sensor applications to capture
   analog signals from sensors like temperature
   sensors, pressure sensors, and strain
   gauges. The sampled signals can be processed to
   extract useful information about the system
   being monitored.

117> wahoo! thank you :)

You're welcome! If you have any more questions,
feel free to ask.
