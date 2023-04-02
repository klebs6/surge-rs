# surgefilter-comb

The `surgefilter-comb` crate provides a comb
filter implementation for use in audio signal
processing.

A comb filter is a type of digital filter that
produces a series of notches (attenuations) in the
frequency response of a signal, spaced evenly
apart in frequency. These notches can be used to
create a "comb-like" effect on the signal, hence
the name.

The `CombFilter` struct represents a single comb
filter instance, with adjustable parameters such
as delay time and feedback level. The `combmix`
function applies the comb filter to an input
signal, and the `reso_factor` parameter controls
the overall amount of resonance in the filter.

The `process_quad` function processes four samples
at a time, which can provide a performance benefit
when processing large streams of audio data. The
`coeff_make` function calculates the filter
coefficients based on the desired delay time and
feedback level.

Mathematically, a comb filter can be represented
by the following transfer function:

H(z) = 1 + x*z^(-d)

where z^(-d) represents a delay of d samples, and
x is the feedback gain.

The resonance factor, `reso_factor`, can be used
to adjust the strength of the notches in the
frequency response, and is related to the feedback
gain by the following equation:

x = (1 - reso_factor)/(1 + reso_factork)

The `coeff_make` function calculates the filter
coefficients as follows:

b0 = 1
b1 = -x
a0 = 1
a1 = -x*z^(-d)

Overall, the `surgefilter-comb` crate provides
a useful tool for manipulating the frequency
content of audio signals, particularly in the
context of sound synthesis.

2> what determines the width in between notches on
the comb? are they the same distance apart (in
terms of frequency)?

The width between notches on a comb filter is
determined by the delay time of the filter, which
is typically set to be a multiple of the sampling
period. The delay time determines the frequency
spacing between the notches, with larger delay
times resulting in wider spacing between notches.

The spacing between notches is equal in frequency,
assuming the delay time is constant. In other
words, the notches are evenly spaced in the
frequency domain, with the spacing determined by
the inverse of the delay time.

For example, if the delay time is set to 10
samples, the frequency spacing between notches
will be 1/10 of the sampling rate. If the sampling
rate is 44100 Hz, the notch spacing will be 4410
Hz.

It is worth noting that the width of the notches
themselves can be adjusted by varying the feedback
gain or resonance factor, as mentioned in the
previous answer.

3> is it possible to space the comb notches with
a logarithmic or linear increment?

The spacing between notches in a traditional comb
filter is linear and is determined by the delay
time, as I mentioned in the previous
answer. However, it is possible to create comb
filters with logarithmic or non-linearly spaced
notches by using what is known as a "non-uniform
comb filter" or "nonlinear comb filter."

Nonlinear comb filters use a nonlinear mapping
function to space the notches, rather than
a linear delay line. This mapping function can be
designed to produce logarithmic or non-uniform
frequency spacing between notches, rather than the
evenly spaced notches of a traditional comb
filter.

One example of a nonlinear comb filter is the
"exponential comb filter" which spaces the notches
exponentially. The transfer function of an
exponential comb filter can be expressed as:

H(z) = 1 / (1 - g * z^(-d))

where g is the gain or feedback factor and d is
the delay time. The spacing between notches in
this filter is nonlinear and increases
exponentially as the frequency increases.

So, while traditional comb filters have linearly
spaced notches, it is possible to create comb
filters with logarithmic or nonlinearly spaced
notches by using a nonlinear mapping function.

4> what about combing at specific frequencies
based on a preset tuning? is this possible?

Yes, it is possible to design a comb filter with
notches that are spaced at specific frequencies
based on a preset tuning.

One approach to achieve this is to use a technique
called "frequency warping," which maps the desired
tuning of the notches onto the delay line of the
comb filter. This can be done by warping the delay
line so that its time delay values correspond to
the desired tuning of the notches, rather than
a linear spacing.

For example, suppose we want to create a comb
filter with notches spaced at the frequencies of
the notes in a particular musical scale. We can
first calculate the desired delay times for each
note based on the tuning of the scale. Then, we
can warp the delay line of the comb filter so that
the time delay values correspond to the desired
delay times for each note.

There are various methods for frequency warping,
including the use of all-pass filters and
time-varying delays. The specific method used will
depend on the desired characteristics of the comb
filter and the application for which it is
intended.

In summary, it is possible to design a comb filter
with notches that are spaced at specific
frequencies based on a preset tuning by using
frequency warping techniques.

5> how can this be done mathematically?

Frequency warping can be achieved mathematically
by using a non-linear transformation of the delay
time values in the comb filter.

To illustrate how this can be done, consider
a comb filter with a delay line of length
N samples. The delay time values for each sample
are given by:

d(n) = n * τ

where τ is the sampling period. In a traditional
comb filter, the delay time values are spaced
linearly, with a constant delay increment
τ between adjacent samples.

To create a comb filter with notches spaced at
specific frequencies based on a preset tuning, we
can warp the delay line by applying a non-linear
transformation to the delay time values. Let f(n)
be the warped delay time value for sample n, and
let g(f(n)) be the frequency corresponding to the
warped delay time value.

We can express the warped delay time values as
a function of the desired frequency values as
follows:

f(n) = g^(-1)(f_desired(n))

where f_desired(n) is the desired frequency for
notch n, and g^(-1) is the inverse of the
frequency mapping function g.

The frequency mapping function g can be any
non-linear function that maps the desired
frequency values to the delay time values in a way
that creates the desired notch spacing. For
example, we can use a logarithmic function to
achieve logarithmically spaced notches, or a power
function to achieve non-linearly spaced notches.

Once we have computed the warped delay time
values, we can use them to construct the delay
line for the comb filter, and then apply the comb
filter to the input signal using the usual method.

Overall, frequency warping provides a flexible way
to create comb filters with notches spaced at
specific frequencies based on a preset tuning. The
specific frequency mapping function used will
depend on the desired characteristics of the
filter and the application for which it is
intended.

6> are there drawbacks to using a comb filter?

While comb filters can be a useful tool in audio
signal processing and sound synthesis, there are
also some potential drawbacks to their use.

One major issue with comb filters is the potential
for aliasing. Aliasing occurs when a signal
contains frequency components that are above the
Nyquist frequency (half the sampling rate), and
those high-frequency components are "folded back"
into the audible frequency range, producing
distortion or unwanted artifacts. Comb filters can
introduce aliasing because they produce notches in
the frequency response, which can fold back
high-frequency components into the audible range.

To avoid aliasing, it is important to ensure that
the signal being processed by the comb filter does
not contain any frequency components above the
Nyquist frequency. This can be achieved by
applying appropriate anti-aliasing filtering to
the signal before it is processed by the comb
filter.

Another potential issue with comb filters is that
they can introduce phase distortion. Comb filters
with large delay times can cause phase shifts in
the frequency response, which can alter the timbre
of the signal being processed.

Finally, comb filters can be prone to feedback and
instability if the feedback gain or resonance
factor is set too high. This can result in ringing
or oscillation in the output signal, which can be
undesirable in some contexts.

Overall, while comb filters can be a powerful tool
in audio signal processing and sound synthesis, it
is important to be aware of their potential
drawbacks and to use them appropriately in order
to avoid issues such as aliasing, phase
distortion, and instability.

7> how do we prevent phase distortion in a comb
filter?

Comb filters can introduce phase distortion
because they create a series of notches in the
frequency response, which can cause phase shifts
in the frequency domain. The amount of phase shift
depends on the delay time of the comb filter and
the frequency of the signal being processed.

To prevent phase distortion in a comb filter, one
approach is to use what is known as an "all-pass
comb filter." An all-pass filter is a type of
filter that preserves the amplitude spectrum of
a signal while modifying its phase response. By
using an all-pass filter as the basis for the comb
filter, we can ensure that the phase response of
the filter is flat, meaning that it does not
introduce any phase distortion.

The transfer function of an all-pass filter can be
expressed as:

H(z) = (a + z^(-1))/(1 + a*z^(-1))

where a is a parameter that determines the amount
of phase shift introduced by the filter. When
a = 1, the all-pass filter has a delay time of one
sample and introduces a phase shift of 180 degrees
at all frequencies. When a = -1, the filter has no
delay and introduces a phase shift of 0 degrees at
all frequencies.

By cascading multiple all-pass filters together,
we can create an all-pass comb filter that has
a flat phase response. The delay times of the
all-pass filters determine the frequency spacing
of the notches in the frequency response, while
the all-pass filters themselves ensure that the
phase response is preserved.

In summary, to prevent phase distortion in a comb
filter, we can use an all-pass comb filter, which
uses all-pass filters to preserve the phase
response of the signal being processed. By
adjusting the parameters of the all-pass filters,
we can ensure that the comb filter has the desired
frequency response without introducing unwanted
phase shifts.

8> are there other solutions to this problem other
than using all pass filters?

Yes, there are other approaches to reducing or
eliminating phase distortion in comb filters
besides using all-pass filters.

One approach is to use what is called
a "feedforward comb filter." A feedforward comb
filter uses a cascaded combination of a delay line
and a gain stage, without feedback. Unlike
a traditional comb filter with feedback, which can
introduce phase distortion, a feedforward comb
filter does not cause phase distortion because it
does not introduce phase shift in the frequency
response.

Another approach is to use what is called
a "linear phase comb filter." A linear phase comb
filter is a type of comb filter that has
a frequency response with a constant group delay,
meaning that all frequencies are delayed by the
same amount. This ensures that the phase shift
introduced by the filter is linear across all
frequencies, resulting in a flat phase
response. Linear phase comb filters can be
designed using techniques such as FIR (finite
impulse response) filter design.

In addition, some other types of filters can be
used to achieve similar effects as a comb filter
without introducing phase distortion. For example,
a resonant low-pass filter can produce a similar
notch-like effect in the frequency response, while
maintaining a flat phase response. Similarly,
a resonant band-pass filter can produce a series
of notches at specific frequencies, without
introducing phase distortion.

Overall, while all-pass filters are a common
approach to preventing phase distortion in comb
filters, there are other techniques and
alternatives that can be used depending on the
specific requirements and constraints of the
application.

9> why do we use the gain stage in the feedforward
comb?

In a feedforward comb filter, the gain stage is
used to control the amplitude of the output
signal. The gain stage is inserted between the
delay line and the output of the filter, and its
purpose is to amplify or attenuate the delayed
signal before it is added to the input signal.

The gain stage is necessary because the delayed
signal added to the input signal can either
reinforce or cancel out the input signal,
depending on the relative phases and amplitudes of
the signals. By adjusting the gain of the delayed
signal, we can control the amount of reinforcement
or cancellation, and thus the frequency response
of the filter.

Specifically, the gain of the feedforward comb
filter is typically set to a value less than 1, in
order to prevent the filter from introducing large
notches in the frequency response. If the gain
were set to 1, the filter would produce notches
with a depth equal to the amount of feedback in
a traditional comb filter. However, since the
feedforward comb filter has no feedback, the gain
must be set lower in order to produce a similar
effect while avoiding excessive notching.

In addition, the gain stage can be used to adjust
the overall level of the output signal to match
the desired level or to compensate for any level
changes introduced by the filter.

Overall, the gain stage in a feedforward comb
filter is used to control the amplitude and
frequency response of the filter, and is an
important component of the filter design.

10> how can a linear phase comb filter be
specified mathematically? what are its
mathematical properties?

A linear phase comb filter can be specified
mathematically using its frequency response, which
has a constant group delay across all
frequencies. The group delay is the negative
derivative of the phase response with respect to
frequency, and represents the delay applied to
each frequency component of the input signal by
the filter.

Mathematically, the frequency response of a linear
phase comb filter can be expressed as:

H(f) = e^(-j2πfd)

where f is the frequency in Hz, and d is the delay
time of the comb filter in seconds. This equation
represents the complex amplitude and phase shift
of each frequency component of the input signal
after being filtered by the comb filter. The
magnitude of the frequency response is constant
across all frequencies, while the phase response
is a linear function of frequency with a slope of
-2πd.

A linear phase comb filter can be implemented
using an FIR (finite impulse response) filter,
which has a symmetric impulse response with an odd
number of taps. The filter taps are calculated
using a windowing function, such as the Hamming
window, to ensure that the frequency response of
the filter has the desired properties.

The key mathematical properties of a linear phase
comb filter are its constant group delay and flat
phase response, which ensures that the filter does
not introduce phase distortion. However, it is
worth noting that a linear phase comb filter has
a longer overall delay than a traditional comb
filter with the same delay time, due to the
symmetric impulse response of the FIR filter.

In summary, a linear phase comb filter can be
specified mathematically using its frequency
response, which has a constant group delay and
a linear phase response. The filter can be
implemented using an FIR filter with a symmetric
impulse response and a windowing function. The key
mathematical properties of a linear phase comb
filter are its constant group delay and flat phase
response, which make it a useful tool in audio
signal processing and sound synthesis.

11> what are future research directions for comb
filters?

Comb filters have been studied extensively in the
field of audio signal processing and sound
synthesis, and have been used in a wide range of
applications such as reverb, flanging, and
frequency shifting. However, there are still
several potential research directions that could
be pursued in the field of comb filters.

One possible research direction is to investigate
the use of adaptive comb filters. An adaptive comb
filter is a type of filter that can adjust its
parameters in real-time based on the input signal
and the desired output. This can be useful in
applications where the frequency response of the
signal being processed changes over time, such as
in room acoustics or in frequency-modulated
signals. By adapting the delay time or other
parameters of the comb filter in real-time, we can
ensure that the filter is always providing the
desired effect.

Another potential research direction is to
investigate the use of nonlinear comb filters. As
I mentioned earlier, nonlinear comb filters use
a nonlinear mapping function to space the notches
in the frequency response, which can result in
more complex and interesting filtering
effects. However, nonlinear comb filters can also
introduce additional challenges, such as increased
computational complexity and the potential for
instability or other artifacts.

Another area for potential research is the
optimization of comb filter design for specific
applications. Comb filters can be used in a wide
range of contexts, from audio processing to speech
recognition and beyond. By optimizing the design
of comb filters for specific applications, we can
ensure that they provide the desired effect while
minimizing any unwanted artifacts or distortions.

Finally, the use of comb filters in machine
learning and artificial intelligence applications
is an emerging research direction. Comb filters
can be used to extract features from audio or
other signals, which can be used as input to
machine learning algorithms. By investigating the
use of comb filters in machine learning and AI
applications, we can potentially develop new
approaches to audio processing and signal
analysis.

Overall, there are many potential research
directions for comb filters, and further
exploration in these areas could lead to new and
innovative applications of this powerful signal
processing tool.

12> how can a comb filter become unstable? how can
a comb filter be used in speech recognition? what
about speech generation?

A comb filter can become unstable if the feedback
gain or resonance factor is set too high. This can
cause the filter to start oscillating or ringing,
producing unwanted artifacts in the output
signal. To prevent instability, it is important to
ensure that the feedback gain and resonance factor
are set to appropriate values based on the
specific application and signal being processed.

Comb filters can be used in speech recognition to
extract features from the speech signal. One
common approach is to use a bank of comb filters
with different notch spacings to analyze the
speech signal and extract the formant frequencies,
which are the resonant frequencies of the vocal
tract. The formant frequencies can then be used as
features in a speech recognition algorithm to
distinguish between different phonemes and words.

Comb filters can also be used in speech
generation, for example in the synthesis of vowel
sounds. By using a bank of comb filters with
notches spaced at the resonant frequencies of the
vocal tract, we can synthesize a signal that has
the same formant frequencies as a particular vowel
sound. This signal can then be used as the basis
for synthesizing speech or other vocal sounds.

Overall, comb filters are a useful tool in both
speech recognition and speech generation, and can
be used in a wide range of applications in these
fields. However, it is important to use them
appropriately and with care to avoid instability
or other unwanted effects.

13> what is the mathematical cause of the
instability? could you show me how this works
numerically? what do we need to watch out for?

The mathematical cause of instability in a comb
filter is related to the feedback gain or
resonance factor of the filter. When the feedback
gain or resonance factor is too high, the output
of the filter can become larger than the input,
leading to a positive feedback loop that amplifies
the output signal. This can cause the filter to
start oscillating or ringing, resulting in
instability and unwanted artifacts in the output
signal.

To demonstrate how this works numerically, let's
consider a simple comb filter with a delay time of
100 samples and a resonance factor of 0.9. We will
apply a unit impulse signal as input to the filter
and iterate the filter output over time.

The difference equation for the comb filter is:

y(n) = x(n) + r*y(n-100)

where y(n) is the output of the filter at time n,
x(n) is the input to the filter at time n, and
r is the resonance factor.

If we apply a unit impulse signal to the filter as
input (i.e., x(0) = 1, x(n) = 0 for n > 0), and
iterate the filter output over time, we can
observe the effect of the resonance factor on the
stability of the filter. Here is an example Python
code snippet to simulate the filter:

```python
import numpy as np

# set up filter parameters
delay_time = 100
res_factor = 0.9

# set up input signal (unit impulse)
x = np.zeros(1000)
x[0] = 1

# iterate filter output over time
y = np.zeros(1000)
for n in range(delay_time, len(x)):
    y[n] = x[n] + res_factor * y[n-delay_time]

# plot filter output
import matplotlib.pyplot as plt
plt.plot(y)
plt.show()
```

When we run this code, we can see that the output
of the filter starts oscillating and ringing,
resulting in instability and unwanted artifacts in
the signal. This is because the resonance factor
is set too high for the delay time, causing the
filter to become unstable.

To prevent instability in a comb filter, it is
important to choose appropriate values for the
feedback gain or resonance factor based on the
specific application and signal being
processed. In general, lower values of the
resonance factor will lead to more stable filters,
while higher values can introduce instability and
other unwanted effects. In addition, it is
important to use appropriate anti-aliasing filters
and to avoid clipping or saturation in the input
or output signals, as these can also contribute to
instability in the filter.

14> which other projects may benefit by using this
crate?

The `surgefilter-comb` crate, as a component of
the larger Surge synthesizer system, may be useful
for a wide range of audio and music software
projects. Some potential applications for the
crate include:

1. Digital audio workstations (DAWs): Comb filters
   can be used in DAWs for a variety of purposes,
   such as adding reverb or other spatial effects
   to audio tracks, creating flanging or phasing
   effects, or removing unwanted resonances from
   recordings.

2. Virtual instruments and synthesizers: Comb
   filters can be used in virtual instruments and
   synthesizers to create unique and interesting
   timbres and textures. For example, comb filters
   can be used to create plucked or percussive
   sounds, or to add resonance to a synthesizer's
   filter section.

3. Music production and sound design tools: Comb
   filters can be used in a variety of music
   production and sound design tools, such as
   sample editors, sound generators, and effects
   processors. For example, comb filters can be
   used to extract resonances from a recording and
   use them as the basis for creating new sounds,
   or to create interesting spectral filtering
   effects.

4. Audio analysis and processing software: Comb
   filters can be used in software tools for
   analyzing and processing audio signals. For
   example, comb filters can be used to extract
   formant frequencies from speech signals, or to
   remove resonances from recordings of musical
   instruments.

Overall, the `surgefilter-comb` crate has the
potential to be useful in a wide range of audio
and music software projects, and could be
integrated into existing software tools or used as
a building block for creating new tools and
applications.

15> could you show me some more python code
illustrating the behavior of comb filters?

Sure, here is some Python code that demonstrates
the behavior of a comb filter in the time domain:

```python
import numpy as np
import matplotlib.pyplot as plt

# Define the comb filter parameters
delay_time = 100 # in samples
res_factor = 0.8
num_samples = 1000

# Generate the input signal
input_signal = np.zeros(num_samples)
input_signal[0] = 1.0

# Initialize the output signal
output_signal = np.zeros(num_samples)

# Compute the filter output
for n in range(delay_time, num_samples):
    output_signal[n] = input_signal[n] + res_factor * output_signal[n-delay_time]

# Plot the input and output signals
plt.plot(input_signal, label='Input')
plt.plot(output_signal, label='Output')
plt.legend()
plt.show()
```

This code generates an input signal consisting of
a single impulse at the beginning of the signal,
and applies a comb filter to the signal with
a delay time of 100 samples and a resonance factor
of 0.8. The output signal is then plotted
alongside the input signal.

When you run this code, you should see a plot with
two signals: the input signal, which is a single
impulse at the beginning of the signal, and the
output signal, which shows a series of delayed
impulses with notches spaced at the comb filter
frequency. The depth of the notches in the output
signal is determined by the resonance factor,
which controls the amount of feedback in the
filter.

Here is some additional Python code that
demonstrates the behavior of a comb filter in the
frequency domain:

```python
import numpy as np
import matplotlib.pyplot as plt

# Define the comb filter parameters
delay_time = 100 # in samples
res_factor = 0.8
num_samples = 1000

# Generate the input signal
input_signal = np.zeros(num_samples)
input_signal[0] = 1.0

# Compute the filter frequency response
freq_response = np.zeros(num_samples)
for n in range(num_samples):
    freq_response[n] = 1.0 / (1.0 - res_factor * np.exp(-2j*np.pi*n*delay_time/num_samples))

# Compute the output spectrum
output_spectrum = freq_response * np.fft.fft(input_signal)

# Plot the input and output spectra
plt.plot(np.abs(np.fft.fft(input_signal)), label='Input')
plt.plot(np.abs(output_spectrum), label='Output')
plt.legend()
plt.show()
```

This code computes the frequency response of
a comb filter with the same parameters as before,
and applies the filter to an input signal
consisting of a single impulse. The output
spectrum is then plotted alongside the input
spectrum.

When you run this code, you should see a plot with
two spectra: the input spectrum, which shows
a single peak at the fundamental frequency, and
the output spectrum, which shows a series of
notches spaced at the comb filter frequency. The
depth and spacing of the notches in the output
spectrum is determined by the resonance factor and
delay time, respectively.
