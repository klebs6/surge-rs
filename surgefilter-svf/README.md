# Surge SVF Filters

The `surgefilter-svf` crate provides
implementations of various State-Variable Filters
(SVF) used in audio signal processing. These
filters are commonly used in audio synthesis,
particularly in subtractive synthesis, to shape
the frequency spectrum of a signal.

The SVF is a type of biquadratic filter, meaning
that it uses a transfer function with two poles
and two zeros in the complex plane. The SVF can be
implemented as a set of differential equations or
as a cascaded pair of biquad filters.

The `surgefilter-svf` crate provides several
different types of SVF filters, including:

- `SvfFilter`: a basic implementation of the SVF
  that can be configured as a low-pass, high-pass,
  or band-pass filter with a 12dB/octave or
  24dB/octave slope.

- `svf_lp12_a_quad`: a low-pass filter with
  a 12dB/octave slope implemented using the Direct
  Form I topology and the Audio EQ Cookbook
  algorithm.

- `svf_lp24_a_quad`: a low-pass filter with
  a 24dB/octave slope implemented using the Direct
  Form I topology and the Audio EQ Cookbook
  algorithm.

- `svf_hp12_a_quad`: a high-pass filter with
  a 12dB/octave slope implemented using the Direct
  Form I topology and the Audio EQ Cookbook
  algorithm.

- `svf_hp24_a_quad`: a high-pass filter with
  a 24dB/octave slope implemented using the Direct
  Form I topology and the Audio EQ Cookbook
  algorithm.

- `svf_bp12_a_quad`: a band-pass filter with
  a 12dB/octave slope implemented using the Direct
  Form I topology and the Audio EQ Cookbook
  algorithm.

- `svf_bp24_a_quad`: a band-pass filter with
  a 24dB/octave slope implemented using the Direct
  Form I topology and the Audio EQ Cookbook
  algorithm.

The `SvfFilter` struct uses a set of coefficients
to configure the filter type and cutoff
frequency. The other filter types are implemented
using the Direct Form I topology, which uses two
biquad filters in series to implement the filter
transfer function.

The SVF filter has a number of desirable
properties, including linear phase response,
constant group delay, and a relatively simple
implementation. However, it does have some
limitations, such as sensitivity to coefficient
quantization and a tendency to produce noise at
high frequencies.

Overall, the `surgefilter-svf` crate provides
a set of useful filters for shaping the frequency
content of audio signals in a variety of
applications.

122> could you teach me what a state variable
filter is? how does it work?

Sure! A state variable filter (SVF) is a type of
filter that uses three interconnected stages to
produce its output. These stages are a high-pass
filter, a band-pass filter, and a low-pass filter,
and they work together to create a filter that can
be configured to produce different types of
frequency responses.

At the heart of the SVF is a set of state
variables that are used to keep track of the
signal as it passes through the different
stages. These state variables are updated on each
sample, and they are used to calculate the output
of the filter.

The basic idea behind the SVF is that it uses
feedback from the output of the band-pass filter
to drive the inputs of the high-pass and low-pass
filters. This feedback creates a resonance in the
filter, which can be adjusted to produce different
types of frequency responses.

Here is a block diagram of the basic SVF
structure:

```
                 +-------+             +---------+
      Input --->| High  |-------------|         |
                | Pass  |             |         |
                | Filter|             |         |<--- Output
                +-------+             |         |
                    |                 |         |
                    +----+------------| Band-   |
                         |            | Pass    |
                         |            | Filter  |
                         |            +---------+
                         |
                         +------------| Low     |
                                      | Pass    |
                                      | Filter |
                                      +--------+
```

The input signal is first passed through the
high-pass filter, which removes low frequency
content from the signal. The resulting signal is
then passed through the band-pass filter, which
allows a certain range of frequencies to pass
through. Finally, the output of the band-pass
filter is passed through the low-pass filter,
which removes high frequency content from the
signal.

The output of the SVF is the output of the
low-pass filter. By adjusting the frequency and
resonance of the band-pass filter, different types
of frequency responses can be achieved.

The state variables used in the SVF are the
outputs of the high-pass and band-pass filters, as
well as the difference between the input signal
and the output of the low-pass filter. These state
variables are updated on each sample, and they are
used to calculate the output of the filter.

Overall, the SVF is a versatile and widely used
filter design that can be used in a variety of
audio applications.

123> thanks -- could you teach me the mathematics?

Sure! The state variable filter is based on the
concept of a state variable, which is
a mathematical term that refers to a variable that
describes the current state of a system. In the
context of a filter, a state variable refers to
the internal state of the filter that is used to
calculate the output signal.

The basic idea behind a state variable filter is
to represent the filter as a set of first-order
differential equations, where each equation
corresponds to a state variable. These equations
describe the way in which the input signal is
transformed into the output signal, based on the
values of the state variables.

For example, in the case of a low-pass filter, we
can define three state variables: the output
signal (y), the output signal differentiated with
respect to time (y'), and the internal state of
the filter (x). We can then write the following
set of differential equations:

y' = -α y' - ω0^2 x + α x'
y = x

where α is the damping coefficient, ω0 is the
cutoff frequency, and x' is the derivative of
x with respect to time.

These equations describe the way in which the
input signal is transformed into the output
signal, based on the current state of the
filter. In particular, the first equation
describes how the input signal is filtered by the
internal state of the filter, while the second
equation simply sets the output signal equal to
the internal state.

The state variable filter can be implemented using
various numerical methods, such as the Euler
method or the Runge-Kutta method, which are used
to solve the set of differential equations and
update the state variables at each time step. By
adjusting the values of the coefficients α and ω0,
the state variable filter can be configured to act
as a low-pass, high-pass, or band-pass filter,
depending on the desired frequency response.

Overall, the state variable filter provides
a flexible and efficient way to implement a wide
range of filter types, and is widely used in
digital signal processing applications.

124> what about the physics?

State variable filters are electronic circuits
that can be used to process signals in audio and
other electronic systems. They are based on the
idea of using state variables to represent the
internal state of a system, which can be used to
model the behavior of the system over time.

From a physical perspective, the state variables
represent the energy stored in the various
components of the filter circuit, such as the
capacitors and inductors. The behavior of the
circuit is then determined by the flow of energy
between these components, which can be described
using mathematical equations based on the laws of
physics, such as Kirchhoff's laws and Ohm's law.

For example, in a simple low-pass state variable
filter, there are two capacitors and a resistor in
the circuit. The state variables for this circuit
would be the voltages across the two capacitors,
which represent the energy stored in the
capacitors. The flow of energy between the
components can be described by a set of
differential equations, which can be solved to
determine the output signal of the filter.

Overall, state variable filters provide a powerful
tool for designing and implementing electronic
filters with a wide range of applications in audio
and other fields.

125> what are the applications?

State variable filters have a wide range of
applications in various fields such as audio
processing, control systems, communication
systems, and image processing. In audio
processing, they are commonly used for
equalization, tone shaping, and various sound
effects. In control systems, they are used for
filtering and controlling signals in feedback
loops. In communication systems, they are used for
signal processing and filtering in applications
such as digital signal processing and radar
systems. In image processing, they are used for
filtering and enhancing images. Overall, state
variable filters are widely used in any
application that requires signal filtering or
processing.

126> thanks -- any caveats?

One caveat to keep in mind with state variable
filters is that they are prone to
self-oscillation. This means that if the input
signal has a high enough amplitude and the filter
is set to a certain frequency range, the output
can start to oscillate on its own, without any
input signal. This can be desirable in some cases,
such as when creating a resonant sound, but in
other cases it can be unwanted and lead to
distortion. It is important to carefully choose
the filter parameters and monitor the output to
avoid unwanted self-oscillation.

145> what makes a state variable filter different
than other filter topologies?

State variable filters, also known as biquads or
second-order filters, are a class of filter
topologies that have some unique characteristics
that distinguish them from other filter types.

One of the key features of state variable filters
is that they are capable of producing multiple
filter outputs simultaneously. Specifically, they
can produce low-pass, high-pass, and band-pass
outputs all at the same time, using the same set
of filter coefficients. This is because state
variable filters use a set of three interconnected
basic filter structures, or "states", to create
these multiple outputs. The three states are known
as the low-pass, band-pass, and high-pass states,
and they are combined in different ways to produce
the desired filter response.

Another advantage of state variable filters is
that they can be designed to have a relatively
constant gain response across a wide range of
frequencies. This makes them well-suited for use
in audio processing applications, where a flat
frequency response is often desired.

State variable filters are also known for their
stability and ease of implementation. They are
a popular choice for hardware and software
implementations due to their relatively low
computational requirements and good performance
characteristics.

Overall, the unique combination of multiple filter
outputs, constant gain response, stability, and
ease of implementation make state variable filters
a powerful and versatile tool for a wide range of
signal processing applications.
