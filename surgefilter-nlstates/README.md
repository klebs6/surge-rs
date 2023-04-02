# surgefilter-nlstates

<NonlinearStatesFilter>

NonlinearStatesFilter is a Rust module
implementing a nonlinear feedback filter, which
can be used in various audio signal processing
applications. The module provides functionality
for calculating filter coefficients using the
Audio EQ Cookbook method, as well as different
types of saturators for the filter stages. The
filter stages themselves are implemented using the
Direct Form I topology, which is a common
structure for digital filter implementations.

The filter can be tuned to different cutoff
frequencies using the `clamped_frequency`
function, which ensures that the frequency remains
within a certain range to prevent unstable
behavior.

Overall, NonlinearStatesFilter provides a flexible
and customizable way to implement nonlinear
feedback filters in Rust, with support for
different types of filters and saturators.

<coeff_make>

The `coeff_make` function in NonlinearStatesFilter
calculates the filter coefficients using the Audio
EQ Cookbook method. This involves determining the
resonance (`reso`) and frequency (`freq`) of the
filter, then calculating the different
coefficients using the following formulas:

q = (reso ^ 3) * 18.0 + 0.1

w0 = 2 * π * freq / sr

sin(w0) = wsin

cos(w0) = wcos

alpha = wsin / (2 * q)

a0 = 1 / (1 + alpha)

a1 = -2 * wcos * a0

a2 = (1 - alpha) * a0

b0 = (1 - cos(w0)) * a0 / 2

b1 = (1 - cos(w0)) * a0

b2 = (1 - cos(w0)) * a0 / 2

The resulting coefficients are used in the Direct
Form I topology to implement the filter.

<clamped_frequency>

The `clamped_frequency` function in
NonlinearStatesFilter ensures that the cutoff
frequency remains within a certain range to
prevent unstable behavior. It takes a pitch value
and uses a TunerHandle to convert it to
a frequency value. The frequency is then clamped
to a range between 5 Hz and 30% of the sample
rate, which is returned as the output. The
function uses the following formula to calculate
the frequency from the pitch:

freq = n2p(pitch + 69) * MIDI_0_FREQ

<process_quad>

The `process_quad` function in
NonlinearStatesFilter implements the Direct Form
I topology to process a single sample of input
data through the nonlinear feedback filter. The
function takes a QuadFilterUnitState and an input
value, and returns the output of the filter for
that input value. The function applies different
types of saturators and calculates the filter
coefficients for each stage, using the
coefficients calculated by the `coeff_make`
function. The Direct Form I topology is
implemented using a set of z-1 state registers and
coefficient registers, which are updated for each
stage of the filter.

<alias of process_quad to
test_process_quad_allpass>

The `test_process_quad_allpass` function is an
alias of `process_quad`, with the difference that
it is specialized for the Allpass filter
type. This allows for a more efficient
implementation of the Allpass filter, which has
a simpler coefficient calculation than the other
filter types.

<alias of process_quad to test_process_quad_allpass_complex>

The `test_process_quad_allpass_complex` function
is an alias of `process_quad`, with the difference
that it is specialized for the Complex Allpass
filter type. This allows for a more efficient
implementation of the Complex Allpass filter,
which uses complex coefficients.

<alias of process_quad to test_process_quad_bandpass>

The `test_process_quad_bandpass` function is an
alias of `process_quad`, with the difference that
it is specialized for the Band

52> what is the Direct Form I topology?

The Direct Form I is a common topology used to
implement digital filters. It is a type of
transposed direct form filter structure, which
means that it is derived from the canonical direct
form filter structure.

The Direct Form I topology represents a recursive
filter using a series of additions and
multiplications. It consists of a set of delay
elements, or "z^-1" blocks, which represent the
delay of the input signal, and a set of "b" and
"a" coefficient blocks that implement the filter
transfer function.

The transfer function of a digital filter in
Direct Form I can be represented mathematically
as:

```
Y(z) = b0*X(z) + b1*X(z^-1) + b2*X(z^-2) - a1*Y(z^-1) - a2*Y(z^-2)
```

where X(z) and Y(z) represent the input and output
signals, respectively, and b0, b1, b2, a1, and a2
are the filter coefficients.

The Direct Form I topology is a relatively simple
and efficient way to implement recursive filters,
but it can be sensitive to coefficient
quantization errors, and it is not well-suited for
implementing high-order filters due to its
numerical stability issues.

53> are there other related topologies? are there
drawbacks to this one?

Yes, there are several other related topologies
for implementing digital filters. Two of the most
commonly used ones are Direct Form II and
Transposed Direct Form II.

Direct Form II is similar to Direct Form I, but
the order of the operations is different. In
Direct Form II, the input is first fed into a set
of feedback delays, and then the feedback
coefficients are applied to these delayed
values. The output of the feedback section is then
fed into a set of feedforward delays, and the
feedforward coefficients are applied to these
delayed values to produce the final output. Direct
Form II is often preferred over Direct Form
I because it can be more numerically stable for
high-order filters.

Transposed Direct Form II is similar to Direct
Form II, but the order of the operations is
reversed. In Transposed Direct Form II, the
feedforward and feedback sections are swapped, so
that the input is first fed into the feedforward
section and then into the feedback section. Like
Direct Form II, Transposed Direct Form II can be
more numerically stable than Direct Form I for
high-order filters.

One potential drawback of Direct Form I is that it
may not be as numerically stable as other
topologies for high-order filters. This is because
the feedback coefficients are applied to the input
signal before it is delayed, which can lead to
large intermediate values that may cause numerical
issues. Additionally, Direct Form I requires twice
as many delays as Direct Form II or Transposed
Direct Form II, which can increase memory
requirements. However, for lower-order filters,
Direct Form I can be a simple and efficient way to
implement a digital filter.

54> could you write a bullet list of pros and cons for
each of these topologies you listed? feel free to
add more rows for more topologies if there are
some other interesting ones

Sure! Here's a bullet list comparing some of the pros
and cons of the Direct Form I, II, and Transposed
Direct Form II topologies:

Direct Form I:
- Pros:
  - Simple and easy to implement
  - Low computational cost
- Cons:
  - Numerical noise amplification

  - Requires more memory compared to other
    structures

  - Can exhibit overflow and limit cycle issues

Direct Form II:
- Pros:
  - Lower numerical noise amplification than
    Direct Form I

  - Can be implemented with fewer memory elements
    than Direct Form I

- Cons:
  - Requires more computational resources than
    Direct Form I

  - Can be less stable than Direct Form I

Direct Form II Transposed:
- Pros:
  - Can have a lower noise floor compared to
    Direct Form I and Direct Form II

  - Uses the least amount of memory of the three
    structures

- Cons:
  - Can be more difficult to implement than Direct
    Form I and Direct Form II

  - Requires more computation than Direct Form I

Cascade Form:
- Pros:
  - Stable and robust
  - Low numerical noise amplification
- Cons:
  - Higher computational cost than Direct Form
    I and Direct Form II

  - Can be more difficult to implement and
    optimize than other structures

Parallel Form:
- Pros:
  - Low computational cost
  - Can be implemented with multiple cascaded
    biquad stages for increased accuracy

- Cons:
  - Numerical noise amplification
  - More complex to implement than Direct Form
    I and Direct Form II

56> thanks -- what else should we keep in mind
while using this crate? how does it differ from
surgefilter-nlfeedback?

Here are some things to keep in mind while using
`surgefilter-nlstates`:

- This crate provides an implementation of
  a non-linear state variable filter (NSLVF) using
  the Direct Form I topology.

- The NSLVF is a type of filter that can create
  non-linear distortion effects by adding feedback
  loops to a basic state-variable filter
  structure.

- The filter coefficients are calculated using the
  same algorithm as `surgefilter-nlfeedback`, but
  the two crates use different filter topologies.

- This crate is optimized for performance on
  modern processors and makes use of SIMD
  instructions to process multiple samples in
  parallel.

- The filter can be used in a variety of different
  applications, including audio processing, image
  processing, and more.

Compared to `surgefilter-nlfeedback`, which
provides a similar filter implementation but uses
a different topology, `surgefilter-nlstates` has
some advantages and disadvantages:

Advantages:

- The Direct Form I topology used in
  `surgefilter-nlstates` is generally more
  efficient than the Direct Form II topology used
  in `surgefilter-nlfeedback`.

- `surgefilter-nlstates` is optimized for
  performance on modern processors and makes use
  of SIMD instructions to process multiple samples
  in parallel.

Disadvantages:

- The Direct Form I topology used in
  `surgefilter-nlstates` can be more prone to
  numerical instability than the Direct Form II
  topology used in `surgefilter-nlfeedback`.

- `surgefilter-nlstates` does not provide the same
  range of filter types as
  `surgefilter-nlfeedback`. For example,
  `surgefilter-nlfeedback` includes a comb filter
  and a state-variable filter, which are not
  available in `surgefilter-nlstates`.
