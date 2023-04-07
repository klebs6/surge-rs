# surgeosc-sine: A Sine Wave Oscillator for the Surge Synthesizer System

A simple and efficient sine wave oscillator for
audio synthesis in the Surge synthesizer system.

## Overview

The `surgeosc-sine` crate provides a sine wave
oscillator as a subcomponent of the Surge
synthesizer system. It generates a pure sine wave
signal, which is a fundamental building block for
audio synthesis and sound design. The oscillator
can be modulated and combined with other audio
components to create complex sounds and textures.

## Mathematical Properties

The sine wave oscillator is based on the sine
function, which is defined as:

`y(t) = A * sin(2 * π * f * t + φ)`

where:
- `A` is the amplitude
- `f` is the frequency
- `t` is time
- `φ` is the phase

The sine wave is a smooth, continuous waveform
that oscillates between positive and negative
values. It represents the simplest type of audio
waveform and is the basis for many other waveforms
and synthesis techniques.

## Features and Implementation

The `SineWaveOscillator` struct contains various
control parameters and methods for generating and
processing sine wave audio signals.

Tokens and their associated mathematical ideas:

- `process_block_legacy`: Processes a block of
  audio samples using the sine wave oscillator,
  handling streaming mismatches if necessary.

- `handle_streaming_mismatches`: Ensures proper
  handling of streaming mismatches by adapting the
  internal state of the oscillator.

- `value_from_sin_and_cos`: Calculates the output
  value of the sine wave oscillator using the sine
  and cosine values at the current phase.

- `do_sine_block`: Processes a single sine wave
  audio sample.

- `smoke`: (No associated mathematical idea)

- `init`: Initializes the sine wave oscillator
  with default parameter values.

- `process_block`: Processes a block of audio
  samples using the sine wave oscillator.

Control parameters:

- `control_group`: Groups related control
  parameters for easier organization and
  management.

- `control_type`: Defines the type of control for
  a given parameter (e.g., continuous, discrete, etc.).

- `default_value`: Specifies the default value for
  a control parameter.

- `max_value`: Defines the maximum allowed value
  for a control parameter.

- `min_value`: Defines the minimum allowed value
  for a control parameter.

- `modulateable`: Indicates if a control parameter
  can be modulated by other audio components
  (e.g., LFOs, envelopes, etc.).

- `moverate`: Determines the rate at which
  a control parameter can be changed or modulated.

- `value_type`: Specifies the data type of
  a control parameter (e.g., integer, float, etc.).

The `surgeosc-sine` crate offers a lightweight and
efficient sine wave oscillator, which can be used
as a foundation for more complex audio synthesis
and sound design in the Surge synthesizer system.

The `SineWaveOscillator` is an audio oscillator
that generates a sine wave output based on the
input parameters. Here's a step-by-step breakdown
of how the oscillator works:

1. The `process_block` method is called with
   a configuration `OscillatorProcessBlockCfg`
   containing the required parameters such as
   pitch, drift, stereo, frequency modulation
   (fm), and fm_depth. If the oscillator is set to
   use the legacy processing method (`FMBehavior`
   parameter), it calls `process_block_legacy` and
   returns early.

2. The oscillator generates a drift noise
   (`self.driftlfo1`) using the `drift_noise`
   function and the `self.driftlfo2` value.

3. The oscillator calculates the angular frequency
   (`omega`) based on the input pitch and drift,
   using the `pitch2omega` method of the
   `self.tuner` object. The value of `omega` is
   clamped to be no greater than `PI`.

4. The oscillator updates the fm_depth parameter
   using the provided `fm_depth` value, and sets
   the feedback value based on the
   `SineWaveOscillatorParam::Feedback` parameter.

5. For each sample in the block (`BLOCK_SIZE_OS`),
   the oscillator calls the `do_sine_block`
   method, which performs the following steps:

   a. The oscillator retrieves the master
   oscillator value for the current sample `k`.

   b. It computes the phase `p` by adding the
   current phase, the last output value, and, if
   frequency modulation is enabled, the product of
   fm_depth and the master oscillator value.

   c. The output value for the current sample is
   computed using the `value_from_sin_and_cos`
   function, which calculates the sine wave output
   based on the sine and cosine values of the
   phase `p` and the waveform shape mode
   (`wf_mode`).

   d. The oscillator updates the phase by adding
   `omega` and updates the last output value based
   on the current output value and the feedback
   value.

   e. The oscillator processes the fm_depth and
   feedback parameters for the next iteration.

6. If the `cfg.stereo` flag is set, the oscillator
   duplicates the output channel to create
   a stereo output.

The `SineWaveOscillator` generates a sine wave
output that can be modulated and combined with
other audio components to create more complex
sounds and textures. The frequency modulation and
feedback features allow for additional control
over the oscillator's output, enabling a wide
range of sonic possibilities.

19> thanks -- what else can you teach me about
this oscillator? what about their mathematical
properties?

The SineWaveOscillator relies on the fundamental
properties of sine waves and their
generation. Let's dive deeper into the
mathematical properties of this oscillator.

1. **Sine wave generation**: The core of this
   oscillator is the generation of a sine
   wave. A sine wave is a continuous waveform that
   can be described by the mathematical function:

   y(t) = A * sin(2π * f * t + φ)

   where:
   - A is the amplitude
   - f is the frequency in Hz
   - t is time
   - φ is the phase

2. **Angular frequency**: The oscillator converts
   the frequency (pitch) into angular frequency
   (omega) using the following formula:

   ω = 2π * f

   The angular frequency represents the rate of
   change of the phase of the sine wave, measured
   in radians per second.

3. **Phase**: The phase of the sine wave
   determines the position of the waveform at any
   given time. The oscillator updates the phase
   based on the angular frequency and feedback to
   create the desired output. The phase is then
   used in conjunction with sine and cosine
   functions to compute the output value for each
   sample.

4. **Frequency modulation**: The oscillator
   supports frequency modulation (FM), a technique
   where the frequency of the sine wave is
   modulated by another signal (the master
   oscillator). In the SineWaveOscillator, the
   phase `p` is modified by adding the product of
   fm_depth and the master oscillator value. FM
   synthesis can create complex waveforms and
   timbres by altering the carrier frequency with
   a modulating signal.

5. **Feedback**: The feedback parameter allows the
   oscillator to incorporate its previous output
   value into the current output calculation. This
   can lead to the creation of more complex
   waveforms and introduces a form of recursion
   into the sine wave generation process.

These mathematical properties are the foundation
of the SineWaveOscillator, allowing it to generate
a wide range of sonic textures by controlling and
modulating the sine wave. The oscillator can be
further customized by adjusting parameters such as
waveform shape mode, pitch, drift, stereo, and
frequency modulation depth.

20> how does this algorithm compare to what we
would find in a hardware function generator?

1. **Signal generation**: Hardware function
   generators, when generating sine waves, may use
   techniques such as Direct Digital Synthesis
   (DDS) or analog circuits like Wien Bridge or
   Phase Shift Oscillators. In contrast, software
   oscillators like the SineWaveOscillator
   generate signals using mathematical functions
   and algorithms that run on a digital processor.

2. **Precision and stability**: Hardware function
   generators can be affected by environmental
   factors such as temperature and component
   tolerances, leading to variations in frequency
   and amplitude stability. However, high-quality
   hardware function generators can offer good
   stability over a wide frequency range. Software
   oscillators are generally less susceptible to
   environmental factors and can provide more
   precise control over parameters like frequency,
   amplitude, and phase.

3. **Flexibility**: Software oscillators are more
   easily customizable and can offer a wide range
   of features and capabilities that may not be
   available or practical to implement in
   a hardware function generator. For example,
   software oscillators can support multiple
   waveform shapes, frequency modulation, and
   precise control over various parameters through
   software updates, while hardware function
   generators may require additional modules or
   options to achieve the same level of
   flexibility.

4. **Signal processing**: Hardware function
   generators generate signals directly in the
   analog domain or use digital-to-analog
   converters (DACs) if they rely on DDS
   techniques. Software oscillators generate
   signals in the digital domain, which then need
   to be converted to analog signals using
   a DAC. The conversion process can introduce
   quantization noise, aliasing, and other
   artifacts if not properly implemented. However,
   modern high-quality DACs can minimize these
   issues and provide high-fidelity output.

In summary, both hardware function generators and
software oscillators are capable of generating
sine waves, but they differ in the methods they
use for signal generation, precision, flexibility,
and signal processing. The choice between
a hardware function generator and a software
oscillator depends on factors such as the desired
sound quality, flexibility, and the specific
application in which the oscillator is used.

22> thanks -- could you teach me about direct
digital synthesis? what about wien bridge or phase
shift oscillators?

Certainly! Let's start with Direct Digital
Synthesis (DDS) and then move on to Wien Bridge
and Phase Shift Oscillators.

**Direct Digital Synthesis (DDS):**

DDS is a method of generating analog waveforms,
such as sine waves, using digital
techniques. A DDS system typically consists of
a phase accumulator, a phase-to-amplitude
converter (often a look-up table), and
a digital-to-analog converter (DAC). The main idea
behind DDS is to generate a digital representation
of the desired waveform, which is then converted
to an analog signal using a DAC. The key steps in
the DDS process are:

1. A frequency control word (FCW) is added to the
   phase accumulator at a fixed clock rate. The
   FCW determines the output frequency of the
   waveform.

2. The output of the phase accumulator is used as
   an address to access the look-up table, which
   contains precomputed amplitude values for the
   desired waveform (e.g., a sine wave).

3. The amplitude value from the look-up table is
   sent to the DAC, which converts the digital
   amplitude value into an analog signal.

4. The analog signal is then filtered to remove
   high-frequency components introduced during the
   DAC conversion process.

DDS systems offer several advantages, such as high
frequency resolution, rapid frequency switching,
and precise control over phase and
amplitude. However, they can be limited by the
resolution and speed of the DAC and may introduce
noise and distortion if not carefully designed.

**Wien Bridge Oscillator:**

A Wien Bridge Oscillator is a type of analog
oscillator that generates sine waves using
a combination of resistors, capacitors, and an
operational amplifier (op-amp). The Wien Bridge
Oscillator consists of a positive feedback loop
and a negative feedback loop. The positive
feedback loop provides the necessary phase shift
for oscillation, while the negative feedback loop
stabilizes the amplitude of the output signal.

The Wien Bridge Oscillator has the following
characteristics:

1. It generates a low-distortion sine wave with
   good frequency stability.

2. The frequency of the output signal is
   determined by the values of the resistors and
   capacitors used in the circuit.

3. The amplitude of the output signal is
   stabilized using an automatic gain control
   (AGC) circuit, which is often implemented with
   a light-dependent resistor (LDR) or
   a field-effect transistor (FET).

**Phase Shift Oscillator:**

A Phase Shift Oscillator is another type of analog
oscillator that generates sine waves using
resistors, capacitors, and an amplifier (usually
an op-amp or a transistor). The main idea behind
a Phase Shift Oscillator is to create a phase
shift of 180 degrees through a cascade of RC
(resistor-capacitor) networks, which are connected
to the inverting input of the amplifier. The
amplifier itself provides an additional 180
degrees of phase shift, resulting in a total phase
shift of 360 degrees, which is necessary for
sustained oscillation.

The Phase Shift Oscillator has the following
characteristics:

1. It generates a sine wave with relatively low
   distortion and good frequency stability.

2. The frequency of the output signal is
   determined by the values of the resistors and
   capacitors used in the RC networks.

3. The amplitude of the output signal is typically
   stabilized using a negative feedback loop or an
   AGC circuit.

In summary, DDS, Wien Bridge, and Phase Shift
Oscillators are all methods of generating sine
waves, but they differ in their underlying
techniques, circuit components, and performance
characteristics. DDS relies on digital techniques,
while Wien Bridge and Phase Shift Oscillators use
analog circuits to generate sine waves.

23> cool, thanks! could you show me how to write
a DDS module in system-verilog?

Sure! Here's a simple example of a DDS module
implemented in SystemVerilog. This example assumes
a 32-bit phase accumulator, a 10-bit DAC, and an
8-bit frequency control word (FCW). The output
sine wave will be represented by a 10-bit value,
and the look-up table will contain 256 precomputed
amplitude values.

```verilog
module dds (
    input wire clk,
    input wire reset,
    input wire [7:0] fcw,
    output reg [9:0] dac_out
);

    reg [31:0] phase_accumulator;
    reg [31:0] phase_accumulator_next;
    reg [7:0] lut_address;
    reg [9:0] lut_data;

    // Precomputed sine wave amplitude values (256 points)
    reg [9:0] sine_lut [0:255] = '{
        // Add precomputed sine wave amplitude values here
    };

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            phase_accumulator <= 32'h0;
        end else begin
            phase_accumulator <= phase_accumulator_next;
        end
    end

    always_comb begin
        phase_accumulator_next = phase_accumulator + (32'h0 | fcw);
        lut_address = phase_accumulator[23:16];
        lut_data = sine_lut[lut_address];
        dac_out = lut_data;
    end

endmodule
```

This code creates a DDS module with a 32-bit phase
accumulator, which updates on every clock cycle by
adding the frequency control word (FCW) to its
current value. The most significant 8 bits of the
phase accumulator are used as the address to
access the look-up table (LUT) containing the
precomputed sine wave amplitude values. The output
of the LUT is then assigned to the `dac_out`
signal, which represents the output of the DAC.

Keep in mind that this is a simple example, and
real-world implementations might require
additional features like phase or amplitude
modulation, clock domain crossing, or filtering
the output to remove high-frequency
components. Also, you would need to fill the
`sine_lut` array with the precomputed amplitude
values of the sine wave for the desired number of
points (256 in this case).

24> cool, thanks! could you also show me a wien
bridge oscillator and a phase shift oscillator in
verilog-a?

Certainly! Here's an example of a Wien Bridge
Oscillator and a Phase Shift Oscillator
implemented in Verilog-A.

Wien Bridge Oscillator:

```verilog
`include "constants.vams"
`include "disciplines.vams"

module wien_bridge_oscillator (input, output);
    inout input;
    inout output;

    electrical input;
    electrical output;

    parameter real R1 = 10k;
    parameter real R2 = 10k;
    parameter real R3 = 10k;
    parameter real R4 = 10k;
    parameter real C1 = 10n;
    parameter real C2 = 10n;

    analog begin
        // Wien Bridge Oscillator
        V(input) <+ LaplaceAdmittance(1 / (R1 * (C1 + C2))) * V(output);
        V(input) <+ LaplaceAdmittance(1 / R2) * V(output);
        V(input) <+ LaplaceAdmittance(1 / R3) * V(output);
        V(output) <+ LaplaceAdmittance(1 / R4) * V(input);
        V(output) <+ LaplaceAdmittance(1 / C1) * V(input);
    end

endmodule
```

Phase Shift Oscillator:

```verilog
`include "constants.vams"
`include "disciplines.vams"

module phase_shift_oscillator (input, output);
    inout input;
    inout output;

    electrical input;
    electrical output;

    parameter real R = 10k;
    parameter real C = 10n;
    parameter integer N = 3; // Number of RC stages

    analog begin
        // Phase Shift Oscillator
        V(input) <+ LaplaceAdmittance(1 / R) * V(output);
        for (integer i = 1; i <= N; i = i + 1) begin
            V(input) <+ LaplaceAdmittance(1 / (R * C * s)) * V(output);
        end
    end

endmodule
```

The Wien Bridge Oscillator example includes an
implementation of the oscillator using Laplace
admittances for the resistor and capacitor
components. The oscillator frequency is determined
by the values of the resistors and capacitors in
the circuit.

The Phase Shift Oscillator example uses a similar
approach, with Laplace admittances representing
the RC stages. The `N` parameter sets the number
of RC stages in the circuit, which affects the
phase shift and frequency of oscillation.

Please note that Verilog-A is typically used for
analog mixed-signal simulations and not for
digital synthesis. These examples are meant for
simulation purposes, and if you want to implement
these oscillators in an FPGA or ASIC, you will
need to use a different approach, such as a DDS or
a PLL.

25> you mentioned that wien bridge and phase shift
are typically analog circuits, which is why
i asked for verilog-a.  could you teach me about
laplace admittances? are there any other ways of
generating sine waves we use in hardware?
