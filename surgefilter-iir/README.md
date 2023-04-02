## surgefilter-iir

Rust crate for IIR (Infinite Impulse Response)
digital filter design and processing, used as
a subcomponent in the Surge synthesizer system.

### IIRFilter

The `IIRFilter` struct represents a second-order
IIR filter. It is implemented as a direct form II
transposed structure, where the input signal is
fed into the first stage and the output of each
stage is recursively fed back into the subsequent
stage. The `process_quad` method applies the
filter to a block of input samples.

### FilterCoeffs

The `FilterCoeffs` struct represents the
coefficients of a second-order IIR filter. The
coefficients can be computed using the
`coeff_make` function, which takes the filter type
(`lowpass`, `highpass`, `bandpass`, `bandreject`),
the filter frequency, and the Q factor as input
parameters. The Q factor determines the width of
the frequency band for bandpass and bandreject
filters.

### to_normalized_lattice

The `to_normalized_lattice` function converts
a set of filter coefficients to a normalized
lattice form, which can be useful for analyzing
the filter's frequency response.

### iir_12_a_quad, iir_12_b_quad, iir_12_cfl_quad, iir_12_cfc_quad, iir_12_wdf_quad

These are various implementations of second-order
IIR filters in different forms, including direct
form I, coupled form, and wave digital filter
form. They differ in the way the filter
coefficients are used to compute the filter
output.

### iir_24_b_quad, iir_24_cfl_quad, iir_24_cfc_quad

These are similar to the above, but represent
fourth-order IIR filters.

### BandRejectFilter

Thek `BandRejectFilter` struct represents
a fourth-order band-reject filter, which can be
constructed using the `construct_iir_filter`
function. The `resoscale_4pole` function can be
used to scale the Q factor of each stage to
achieve the desired frequency response.

### map_2pole_resonance, map_2pole_resonance_noboost, map_4pole_resonance

These functions map the resonance frequency and
Q factor of a filter to the corresponding filter
coefficients.

### a0inv, alpha, q2inv, gain, resoscale

These are mathematical constants and scaling
factors used in the computation of filter
coefficients and frequency response.

Overall, `surgefilter-iir` provides a range of
tools for designing and processing IIR filters in
Rust, allowing for precise control over the filter
characteristics and frequency response.

2> thanks -- could you teach how an infinite
impulse response filter works?

Sure, I'd be happy to explain how an infinite
impulse response (IIR) filter works!

An IIR filter is a type of digital filter that
operates on a discrete-time signal. It is called
"infinite impulse response" because the filter's
output depends not only on the current input
sample, but also on past input and output samples.

The filter works by recursively applying a set of
coefficients to the input samples and the previous
output samples. These coefficients determine how
the filter modifies the input signal to produce
the output signal. The coefficients are typically
computed based on the desired frequency response
of the filter.

The most common implementation of an IIR filter is
the second-order IIR filter, which consists of two
poles and two zeros. Each pole or zero corresponds
to a complex conjugate pair of values in the
frequency domain. The poles determine the filter's
frequency response, while the zeros cancel out
certain frequencies.

The poles of the filter are located in the complex
plane, and their location determines the filter's
frequency response. If the poles are close to the
unit circle (i.e., have a magnitude close to 1),
the filter will have a narrow bandwidth and
a sharp cutoff at the corner frequency. If the
poles are further from the unit circle, the filter
will have a wider bandwidth and a more gradual
transition.

The frequency response of an IIR filter is
characterized by its magnitude response (the
amount of attenuation or amplification at each
frequency) and its phase response (the delay
introduced by the filter at each frequency). The
magnitude and phase responses are determined by
the location of the poles and zeros in the complex
plane.

One of the advantages of IIR filters is that they
can achieve a more selective frequency response
than finite impulse response (FIR) filters with
the same order. However, IIR filters are more
sensitive to numerical precision errors and can
become unstable if the poles are too close to the
unit circle.

I hope this helps! Let me know if you have any
more questions.

3> thanks -- could you show me how to analyze them
from the perspective of poles/zeros?

Certainly! Analyzing an IIR filter in terms of its
poles and zeros is a powerful technique that
allows you to understand its frequency response
and stability properties.

Let's start with the transfer function of
a second-order IIR filter:

```
H(z) = (b0 + b1*z^-1 + b2*z^-2) / (1 + a1*z^-1 + a2*z^-2)
```

where `b0`, `b1`, and `b2` are the filter
coefficients for the numerator, and `a1` and `a2`
are the filter coefficients for the denominator.

We can factor the numerator and denominator into
their roots (also known as zeros and poles,
respectively) using the quadratic formula:

```
H(z) = (b0 / a0) * (z - z1) * (z - z2) / (z - p1) * (z - p2)
```

where `a0` is the leading coefficient of the
denominator (usually 1), and `z1` and `z2` are the
zeros of the numerator and `p1` and `p2` are the
poles of the denominator.

The roots of the numerator correspond to the zeros
of the filter, while the roots of the denominator
correspond to the poles of the filter.

The location of the zeros and poles in the complex
plane determines the frequency response of the
filter. If a zero or pole is close to the unit
circle (i.e., has a magnitude close to 1), it will
have a significant effect on the frequency
response. If a zero or pole is far from the unit
circle, it will have a negligible effect.

The magnitude response of the filter is given by
the absolute value of the transfer function:

```
|H(z)| = |b0 / a0| * |z - z1| * |z - z2| / |z - p1| * |z - p2|
```

The phase response of the filter is given by the
argument of the transfer function:

```
arg(H(z)) = arg(b0 / a0) + arg(z - z1) + arg(z - z2) - arg(z - p1) - arg(z - p2)
```

The poles of the filter determine its stability
properties. A filter is stable if all of its poles
lie inside the unit circle, which means that the
filter's output will not blow up to infinity in
response to a bounded input signal. If one or more
poles lie outside the unit circle, the filter is
unstable and its output will grow without bound.

In summary, analyzing an IIR filter in terms of
its poles and zeros allows you to understand its
frequency response and stability properties. By
manipulating the location of the poles and zeros,
you can design filters with a wide range of
characteristics.

4> any caveats? what are ten practical design
considerations we should keep in mind while
working with iir filters?

Yes, there are several caveats to keep in mind
when working with IIR filters. Here are ten
practical design considerations to keep in mind:

1. Stability: As mentioned earlier, the location
   of the poles in the complex plane determines
   the stability of the filter. To ensure
   stability, all poles must be inside the unit
   circle in the complex plane. This is a critical
   consideration in filter design.

2. Frequency response: The location of the poles
   and zeros determines the frequency response of
   the filter. By adjusting the location of the
   poles and zeros, you can achieve the desired
   frequency response.

3. Phase response: The phase response of an IIR
   filter is nonlinear, meaning that the phase
   shift introduced by the filter is not constant
   across all frequencies. This can lead to
   distortion and other unwanted effects,
   especially in audio applications.

4. Numerical precision: IIR filters are sensitive
   to numerical precision errors, especially when
   the poles are close to the unit circle. This
   can lead to instability or other unwanted
   effects.

5. Filter order: The order of the filter (i.e.,
   the number of poles and zeros) determines its
   complexity and frequency
   selectivity. Higher-order filters can achieve
   a sharper frequency response, but they are also
   more computationally expensive.

6. Filter type: There are several types of IIR
   filters, including lowpass, highpass, bandpass,
   and bandstop filters. Each type has its own
   characteristics and tradeoffs.

7. Q factor: The Q factor (also known as the
   quality factor) determines the bandwidth of the
   filter. A higher Q factor results in a narrower
   bandwidth and a sharper cutoff, while a lower
   Q factor results in a wider bandwidth and
   a more gradual cutoff.

8. Overflow and saturation: IIR filters can
   produce output values that are larger than the
   input values, which can cause overflow or
   saturation. This can be mitigated by scaling
   the filter coefficients appropriately.

9. Filter design tools: There are several software
   tools available for designing IIR filters,
   including MATLAB, Python's scipy.signal
   library, and online calculators. These tools
   can simplify the filter design process and help
   ensure stability and good performance.

10. Real-time processing: IIR filters can be
    computationally expensive, especially for
    higher-order filters. Real-time processing
    requires careful optimization and efficient
    implementation to ensure that the filter can
    process input signals in real-time without
    introducing significant latency or distortion.

By keeping these practical design considerations
in mind, you can design and implement IIR filters
that meet your specific requirements and achieve
good performance.

5> what is the memory footprint of an iir filter?

The memory footprint of an IIR filter depends on
several factors, including the order of the
filter, the number of coefficients used to
implement the filter, and the data type used to
represent the coefficients and input/output
samples.

For a second-order IIR filter with
single-precision floating-point coefficients
(i.e., 32-bit floating-point), the memory
footprint can be estimated as follows:

- The filter coefficients require 5 floating-point
  values (2 poles + 3 zeros), or 20 bytes.

- The previous input and output samples for each
  stage of the filter require 4 floating-point
  values (2 inputs + 2 outputs), or 16 bytes.

- Assuming a block-based processing scheme, an
  additional buffer is needed to store the input
  samples for one block of data. The size of this
  buffer depends on the block size and the data
  type used to represent the samples.

Therefore, the total memory footprint for
a second-order IIR filter with single-precision
floating-point coefficients is approximately 36
bytes per filter stage, plus the size of the input
buffer.

For higher-order filters, the memory footprint
scales linearly with the number of stages. For
example, a fourth-order filter would require
approximately 72 bytes per stage.

Note that the memory footprint can be reduced by
using lower-precision coefficients or by
implementing the filter in a more efficient data
format, such as fixed-point arithmetic. However,
these optimizations may come at the cost of
reduced numerical precision or increased
implementation complexity.

6> could you show me how to implement a second
order IIR filter in system verilog? does it make
more sense to use verilog-a?

Sure! Here's an example implementation of
a second-order IIR filter in SystemVerilog:

```verilog
module iir_filter (
  input clk,
  input reset_n,
  input signed [15:0] x,
  output signed [15:0] y
);

  parameter real a1 = 1.5; // denominator coefficient 1
  parameter real a2 = -0.7; // denominator coefficient 2
  parameter real b0 = 0.2; // numerator coefficient 0
  parameter real b1 = 0.4; // numerator coefficient 1
  parameter real b2 = 0.2; // numerator coefficient 2

  real [15:0] x_reg [0:2]; // input register
  real [15:0] y_reg [0:2]; // output register
  real [15:0] y_temp; // temporary output register

  always @(posedge clk or negedge reset_n) begin
    if (!reset_n) begin
      x_reg[0] <= 0;
      x_reg[1] <= 0;
      x_reg[2] <= 0;
      y_reg[0] <= 0;
      y_reg[1] <= 0;
      y_reg[2] <= 0;
    end
    else begin
      x_reg[0] <= x;
      x_reg[1] <= x_reg[0];
      x_reg[2] <= x_reg[1];

      y_temp = b0 * x_reg[0] + b1 * x_reg[1] + b2 * x_reg[2] - a1 * y_reg[1] - a2 * y_reg[2];

      y_reg[0] <= y_temp;
      y_reg[1] <= y_reg[0];
      y_reg[2] <= y_reg[1];

      y <= y_temp;
    end
  end
endmodule
```

This implementation uses a fixed set of filter
coefficients for demonstration purposes. The input
and output signals are represented as signed
16-bit integers, and the filter registers are
implemented as real numbers to avoid numerical
precision issues.

The filter is implemented using a pipeline of
three registers to delay the input and output
signals, and a set of multiply-add operations to
compute the filter output based on the current and
previous input and output values. The output is
then fed back into the pipeline of output
registers to complete the feedback loop.

As for your question about whether to use
Verilog-A, it depends on your specific needs and
requirements. Verilog-A is a higher-level modeling
language that allows you to specify analog
behavior using differential equations and other
mathematical models. It is often used for
simulating analog circuits and systems, and can be
more efficient and accurate than SystemVerilog for
certain types of simulations.

However, if you are implementing a digital IIR
filter in a hardware description language like
SystemVerilog, it may make more sense to stick
with SystemVerilog for consistency and
simplicity. SystemVerilog is well-suited for
implementing digital signal processing (DSP)
algorithms, and provides a rich set of built-in
operators and functions for implementing filters
and other signal processing operations.

7> what throughput can we get for one of these
implemented in hardware? how can we configure it?

The throughput of an IIR filter implemented in
hardware depends on several factors, including the
clock frequency, the number of pipeline stages,
and the complexity of the filter coefficients.

Assuming a clock frequency of 100 MHz and a 16-bit
input and output, the example SystemVerilog
implementation I provided in my previous answer
could achieve a throughput of up to approximately
25 million samples per second (MSPS), or 50
million multiply-accumulate (MAC) operations per
second. This assumes that the filter is
implemented using single-cycle operations, and
that there are no pipeline hazards or other
implementation issues.

The throughput of the filter can be increased by
increasing the clock frequency, optimizing the
implementation for pipelining and parallelism, and
using fixed-point arithmetic or other efficient
data formats.

To configure the filter for a specific
application, you can adjust the filter
coefficients to achieve the desired frequency
response. The coefficients can be computed using
various methods, including analog filter design
techniques and digital filter design tools like
MATLAB or Python's scipy.signal library.

You can also adjust the order of the filter to
achieve a more selective or smoother frequency
response, and adjust the Q factor of the filter to
control the bandwidth and peak gain.

Finally, you can adjust the data format and
precision of the input and output signals to match
the requirements of your application, balancing
numerical precision with hardware resources and
power consumption.

8> interesting  -- could you please provide 10
applications of IIR filters in the context of
engineering?

Certainly! Here are ten applications of IIR
filters in engineering:

1. Audio processing: IIR filters are commonly used
   in audio processing applications such as
   equalizers, tone controls, and noise reduction
   filters.

2. Image processing: IIR filters can be used for
   image smoothing, edge detection, and other
   image enhancement techniques.

3. Control systems: IIR filters can be used in
   control systems to filter out noise and other
   unwanted signals from sensor data, or to
   implement feedback loops and other control
   algorithms.

4. Communication systems: IIR filters can be used
   in communication systems for channel
   equalization, digital signal processing, and
   other signal conditioning tasks.

5. Biomedical signal processing: IIR filters can
   be used for filtering and analyzing biomedical
   signals such as electrocardiograms (ECGs) and
   electroencephalograms (EEGs).

6. Instrumentation and measurement: IIR filters
   can be used in instrumentation and measurement
   systems to filter out noise and other unwanted
   signals from sensor data, or to implement
   digital signal processing algorithms for data
   analysis.

7. Radar and sonar systems: IIR filters can be
   used in radar and sonar systems for target
   detection, tracking, and range estimation.

8. Video processing: IIR filters can be used for
   video stabilization, motion detection, and
   other video processing techniques.

9. Power systems: IIR filters can be used in power
   systems to filter out harmonic distortion and
   other unwanted signals from power supply
   outputs.

10. Speech processing: IIR filters can be used in
    speech processing applications such as speech
    recognition, speech synthesis, and voice
    activity detection.

These are just a few examples of the many
applications of IIR filters in engineering. The
flexibility and versatility of IIR filters make
them a powerful tool for processing and analyzing
signals in a wide range of applications.

9> what are the mathematical properties of an IIR
filter?

An IIR filter has several important mathematical
properties that determine its behavior and
performance. Here are some of the key properties:

1. Linearity: An IIR filter is a linear system,
   which means that its input/output relationship
   is linear. In other words, the output of the
   filter is a linear combination of its input and
   previous output samples.

2. Time-invariance: An IIR filter is
   a time-invariant system, which means that its
   behavior does not change over time. In other
   words, the filter's response to an input signal
   is the same at any point in time.

3. Memory: An IIR filter is a recursive system,
   which means that it has memory. The filter
   output depends not only on the current input
   sample, but also on past input and output
   samples.

4. Frequency response: The frequency response of
   an IIR filter depends on the location of its
   poles and zeros in the complex plane. The
   filter's magnitude and phase response can be
   analyzed using tools such as the Fourier
   transform and the Z-transform.

5. Stability: The stability of an IIR filter
   depends on the location of its poles in the
   complex plane. If all of the poles are inside
   the unit circle, the filter is stable and its
   output will not grow without bound.

6. Causality: An IIR filter can be causal or
   non-causal, depending on its
   implementation. A causal filter depends only on
   past input and output samples, while
   a non-causal filter may depend on future
   samples as well.

7. Implementation: There are several ways to
   implement an IIR filter, including direct form,
   cascade form, and lattice form. Each
   implementation has its own advantages and
   tradeoffs in terms of numerical precision,
   stability, and computational complexity.

These mathematical properties of IIR filters are
important for understanding their behavior and
performance in various applications. By
manipulating the location of the poles and zeros,
adjusting the filter order, and selecting the
appropriate implementation method, you can design
IIR filters with a wide range of characteristics
to meet your specific requirements.
