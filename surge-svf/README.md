# surge-svf

Surge-svf is a Rust crate implementing
a vectorized state-variable filter (SVF) used in
the Surge synthesizer system. The SVF is a common
filter design used in audio processing that has
the ability to simultaneously output lowpass,
bandpass, and highpass filtered signals. It can be
used for a variety of tasks such as tone shaping,
equalization, and sound synthesis.

The `VectorizedSvfFilter` struct is the main
component of the crate, representing a single
instance of the filter. It provides methods for
calculating the coefficients of the filter
(`calc_f`, `calc_q`, and `calc_bpf`), resetting
the state of the filter (`reset`), copying
coefficients between filters (`copy_coeff`), and
setting coefficients directly (`set_coeff`).

The math behind the SVF is relatively simple,
involving basic equations for calculating cutoff
frequency (`calc_f`), resonance (`calc_q`), and
the bandpass filter (`calc_bpf`). The
`VectorizedSvfFilter` struct implements these
equations in a vectorized form, allowing for
efficient processing of multiple samples at once.

Overall, surge-svf provides a fast and efficient
implementation of a common filter design used in
audio processing, making it a useful component for
any digital audio workstation or synthesizer
system.

123> could you describe a state variable filter in
more depth?

Sure, a state variable filter is a type of filter
commonly used in digital signal processing. It is
a versatile filter that can be used to implement
high-pass, low-pass, and band-pass filters, as
well as notch filters. It is called a state
variable filter because it has three states,
represented by the values of three capacitors,
which are used to calculate the filter output. The
filter is characterized by three parameters: the
center frequency, the quality factor (Q), and the
gain or damping.

The basic structure of a state variable filter
consists of three interconnected stages:
a low-pass filter, a band-pass filter, and
a high-pass filter. The low-pass filter stage is
used to generate a low-pass output, while the
high-pass filter stage is used to generate
a high-pass output. The band-pass filter stage is
used to generate the band-pass output, which is
the sum of the low-pass and high-pass outputs.

The implementation of a state variable filter can
be done using various techniques, including direct
form, cascade, parallel, and series-parallel. One
of the advantages of a state variable filter is
that it can be easily implemented using the Direct
Form II Transposed structure, which is numerically
stable and has low computational cost.

The equations used to calculate the center
frequency, quality factor, and damping of a state
variable filter depend on the specific
implementation and parameters used, but typically
involve a combination of resistance and
capacitance values. The `calc_bpf`, `calc_f`, and
`calc_q` functions in the `surge-svf` crate are
used to calculate these values for a given set of
filter coefficients. The `set_coeff` function is
used to set the filter coefficients, while the
`reset` function is used to reset the filter to
its initial state.

124> what are their mathematical properties?

State variable filters have some notable
mathematical properties that make them popular in
the design of audio filters.

One important property is that the transfer
function of a state variable filter can be
decomposed into two biquad filters that can be
chained together to produce a higher order
filter. This means that the state variable filter
can be used to implement filters of arbitrary
order.

Another important property is that the state
variable filter can be designed to have a constant
group delay across the frequency spectrum. This is
important for audio filters because it preserves
the phase relationships between different
frequency components of a signal, which is crucial
for maintaining the timbre of the signal.

Finally, state variable filters can be designed to
have a high dynamic range and low noise floor,
which makes them ideal for use in high fidelity
audio applications.

125> what is the  Direct Form II Transposed
structure?

The Direct Form II Transposed (DFII) structure is
a method for implementing digital filters,
particularly infinite impulse response (IIR)
filters, in a computationally efficient way. It is
a type of filter structure that is often used in
audio processing applications.

The DFII structure uses a set of delay elements
and a set of adders to implement the filter, as
well as a set of feedback coefficients and a set
of feedforward coefficients. The feedforward
coefficients are applied to the input signal,
while the feedback coefficients are applied to the
output signal. The delay elements are used to
store the previous values of the input and output
signals.

The DFII structure is known for being numerically
stable, which means that it is less prone to
numerical errors and round-off issues than other
filter structures. It is also efficient in terms
of the number of operations required to implement
the filter, making it well-suited for real-time
applications such as audio processing.

Overall, the DFII structure is a widely used and
well-understood method for implementing digital
filters, particularly IIR filters, in
a computationally efficient and numerically stable
way.

126> thanks -- what are the implications of
preserving a constant group delay across the
frequency spectrum?

Preserving a constant group delay across the
frequency spectrum is desirable for audio
applications because it ensures that different
frequencies are delayed by the same amount of
time, maintaining the relative phase relationships
between them. This helps to maintain the integrity
of the audio signal, and avoids distortion or
artifacts caused by changes in phase
relationships.

In audio applications, it is often desirable to
preserve the relative phase relationships between
different frequency components, particularly for
signals that are being mixed together. Any
distortion or artifacts caused by changes in phase
relationships can cause the mixed signal to sound
"muddy" or "unclear". By ensuring that different
frequencies are delayed by the same amount of
time, a constant group delay helps to preserve the
relative phase relationships between different
frequency components, ensuring that the mixed
signal sounds clear and transparent.

127> can you show me how preserving a constant
group delay achieves this mathematically?

Yes! The group delay of a filter is defined as the
negative derivative of the phase response with
respect to frequency:

$$\tau_g(\omega) = -\frac{d\phi(\omega)}{d\omega}$$

where $\phi(\omega)$ is the phase response of the
filter.

For a linear-phase filter, the phase response can
be expressed as:

$$\phi(\omega) = -\omega\tau_g(0) + \phi_0$$

where $\phi_0$ is a constant and $\tau_g(0)$ is
the group delay at $\omega = 0$. Since
$\phi(\omega)$ is a linear function of $\omega$,
the group delay is constant across the entire
frequency spectrum.

Therefore, if we want to design a filter with
a constant group delay, we need to design a filter
with linear phase. The Direct Form II Transposed
structure is a common way to implement such
a filter.

128> what is linear phase?

In signal processing, a system or filter is said
to have linear phase if the phase shift across all
frequencies is proportional to the frequency, and
therefore, the phase response is a linear function
of frequency. Mathematically, this means that the
phase response of the system can be expressed as:

φ(ω) = -ωτ + φ₀

where φ(ω) is the phase response, ω is the angular
frequency, τ is a constant time delay, and φ₀ is
a constant phase shift.

Linear phase has several important properties,
including constant group delay across the entire
frequency spectrum, which is desirable for audio
applications to avoid distortion and ensure
accurate time alignment of signals. Linear phase
filters are also useful in applications such as
image processing, where preserving the phase of
the image can be important.

129> interesting -- what are the other
mathematical properties of state variable filters?

State variable filters have a number of
mathematical properties, including:

1. Variable cutoff frequency: The cutoff frequency
   of a state variable filter can be adjusted by
   changing the values of the filter's parameters.

2. Low-pass, high-pass, and band-pass filtering:
   By using different combinations of the filter's
   parameters, the state variable filter can be
   configured as a low-pass, high-pass, or
   band-pass filter.

3. Resonance: The state variable filter can
   produce resonance, which is a peak in the
   filter's response at or near the cutoff
   frequency.

4. Stability: A state variable filter is stable if
   the poles of its transfer function are inside
   the unit circle in the complex plane. Stability
   is an important property because an unstable
   filter can produce unstable or oscillatory
   output.

5. Nonlinear distortion: The state variable filter
   can produce nonlinear distortion, which can add
   harmonic content to the filtered
   signal. Nonlinear distortion can be desirable
   in some cases, such as in guitar distortion
   pedals.

6. Efficiency: The state variable filter is
   computationally efficient, requiring only
   a small number of arithmetic operations per
   sample.

Overall, the state variable filter is a versatile
and efficient tool for audio filtering and signal
processing.

130> how do we compare it algorithmically to other
audio filters?

There are many different types of audio filters,
each with their own algorithmic approach and
mathematical properties. Here are a few common
types of audio filters and some of their key
characteristics:

1. Low-pass filter: A low-pass filter allows
   low-frequency signals to pass through while
   attenuating high-frequency signals. One common
   algorithm for a low-pass filter is the
   Butterworth filter, which uses a set of
   coefficients to control the filter's frequency
   response.

2. High-pass filter: A high-pass filter allows
   high-frequency signals to pass through while
   attenuating low-frequency signals. One common
   algorithm for a high-pass filter is the
   Chebyshev filter, which uses a set of
   coefficients to control the filter's frequency
   response.

3. Band-pass filter: A band-pass filter allows
   a specific range of frequencies to pass through
   while attenuating frequencies outside that
   range. One common algorithm for a band-pass
   filter is the biquad filter, which uses two
   poles and two zeros to shape the filter's
   frequency response.

4. Notch filter: A notch filter attenuates
   a narrow range of frequencies, often to remove
   specific harmonics or resonances. One common
   algorithm for a notch filter is the comb
   filter, which uses a set of delay lines to
   create a series of notches in the frequency
   response.

5. State variable filter: A state variable filter
   can be configured as a low-pass, high-pass, or
   band-pass filter, depending on the values of
   its parameters. It uses a set of differential
   equations to calculate the filter's response to
   an input signal.

Each type of filter has its own advantages and
disadvantages, depending on the application. State
variable filters are often used in audio synthesis
and effects processing due to their versatility
and ability to preserve a constant group delay, as
we discussed earlier.

131> what are some caveats we should keep in mind
when using state variable filters?

Here are some caveats to keep in mind when using
state variable filters:

1. High-Q filters can become unstable: High-Q
   filters can produce peaks in the frequency
   response, but this can make them unstable and
   cause the output to ring. Care should be taken
   to avoid using high-Q values in filters with
   steep cutoffs.

2. Nonlinearities: Some state variable filters may
   produce nonlinearities, particularly at high
   resonance values. These nonlinearities can lead
   to distortion and can make the filter sound
   harsh or aggressive.

3. Numerical stability: In some cases, the state
   variable filter may be unstable due to
   numerical errors in the implementation. To
   avoid this, it's important to use appropriate
   numerical techniques and to test the filter
   thoroughly before using it in a production
   setting.

4. Phase distortion: Although state variable
   filters can be designed to have linear phase,
   this is not always the case. Nonlinear phase
   can lead to phase distortion in the output,
   which can be audible in some cases. It's
   important to consider the phase response of the
   filter when designing it for a particular
   application.

5. Memory requirements: State variable filters
   require memory to store the filter coefficients
   and intermediate values. Depending on the
   complexity of the filter and the precision
   required, this can become a significant
   consideration in some applications.
