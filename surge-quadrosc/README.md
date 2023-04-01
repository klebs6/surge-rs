# `surge-quadrosc`

The `surge-quadrosc` crate provides an
implementation of a quadrature oscillator, a key
component of digital audio synthesis. A quadrature
oscillator generatess two audio waveforms that are
phase-shifted by 90 degrees, which can be used to
produce a wide range of timbres when combined.

The `QuadrOsc` struct provides a simple and
efficient implementation of a quadrature
oscillator. It maintains the current state of the
oscillator, and provides methods for setting the
frequency and phase of the generated waveforms, as
well as updating the state to produce the next
samples.

Mathematically, the quadrature oscillator can be
represented as a complex exponential:

`z(t) = r * exp(i * w * t)`

where `r` is the magnitude of the waveform, `w` is
the angular frequency, `t` is time, and `i` is the
imaginary unit. The real part of `z(t)`
corresponds to the "in-phase" waveform, while the
imaginary part corresponds to the "quadrature"
waveform.

The `set_rate` method sets the angular frequency
of the oscillator, which is decomposed into its
real and imaginary parts using `cos` and `sin`
functions. The magnitude of the oscillator is then
normalized to ensure it has unit magnitude.

The `set_phase` method sets the initial phase of
the oscillator by setting the real and imaginary
parts of the waveform.

The `process` method updates the state of the
oscillator to produce the next sample, using the
current state and the real and imaginary parts of
the frequency to update the state with a complex
multiplication.

Overall, the `QuadrOsc` struct provides a simple
and flexible way to generate quadrature waveforms
for use in digital audio synthesis.

62> thanks! could you teach me about quadrature
oscillators?

Sure, I'd be happy to!

A quadrature oscillator is a type of oscillator
that generates two output signals that are 90
degrees out of phase with each other. These
signals are also known as the "in-phase" and
"quadrature" signals.

The in-phase signal and quadrature signal can be
used to create various types of modulated signals,
such as amplitude modulation, phase modulation,
and frequency modulation. Quadrature oscillators
are often used in communication systems and
digital signal processing.

The math behind quadrature oscillators involves
complex numbers and phasors. A phasor is a complex
number that represents a sinusoidal signal, and
can be written in polar form as 
`A * exp(j*theta)`, where `A` is the amplitude of
the signal and `theta` is the phase angle.

In a quadrature oscillator, the in-phase signal
and quadrature signal are represented by the real
and imaginary parts of a complex phasor,
respectively. The oscillator generates these
signals by multiplying a complex phasor by another
complex phasor that is 90 degrees out of
phase. This operation is equivalent to rotating
the phasor by 90 degrees in the complex plane.

The code you provided implements a basic
quadrature oscillator that generates a sine wave
and a cosine wave, which are 90 degrees out of
phase with each other. The oscillator uses the
`dr` and `di` variables to store the sine and
cosine values of the current phase angle, and
updates these values during each iteration of the
`process` function.

I hope that helps! Let me know if you have any
further questions.

63> what should we know when using this?

When using a quadrature oscillator like
`QuadrOsc`, it's important to remember that it
generates two signals - the real part (`r`) and
the imaginary part (`i`) - which are 90 degrees
out of phase with each other. This makes it useful
for generating stereo or spatial audio effects, as
well as for frequency modulation synthesis.

To use the `QuadrOsc`, you typically set its
frequency using the `set_rate` method, which takes
a frequency in radians per sample. You can also
set the initial phase using the `set_phase`
method, which takes an angle in radians.

After setting the frequency and phase, you can
call the `process` method to generate the next
sample of the oscillator's output. You can then
use the `r` and `i` fields to get the oscillator's
real and imaginary parts, respectively.

Note that the `QuadrOsc` struct is mutable, and
each call to `process` modifies the oscillator's
state. If you want to generate a sequence of
samples at a fixed frequency and phase, you should
call `process` repeatedly without modifying the
oscillator's state in between calls.

64> how are they used in communications systems?

In communications systems, quadrature oscillators
are often used as a local oscillator (LO) for
quadrature downconversion.

In this application, the LO generates two
sinusoidal signals that are 90 degrees out of
phase with each other. These signals are mixed
with the incoming signal to produce in-phase (I)
and quadrature (Q) components. The I and
Q components are then demodulated to extract the
original signal.

Quadrature oscillators can also be used for
frequency translation and modulation in
communication systems. For example, a quadrature
oscillator can be used to modulate a carrier
signal with a baseband signal in a quadrature
amplitude modulation (QAM) scheme.

65> what are local oscillators? what is quadrature
downconversion? how do these two ideas apply to
communications systems?

Local oscillators (LOs) are signal generators used
in many communication systems to mix with an
incoming radio-frequency (RF) signal to produce
a downconverted signal at an intermediate
frequency (IF). The purpose of downconversion is
to enable further signal processing, including
filtering, amplification, and demodulation, at
a lower and more manageable frequency range. The
LO is used to generate a sinusoidal signal at
a frequency that is very close to the carrier
frequency of the incoming RF signal, creating
a beat signal at the IF when the two signals are
mixed together.

Quadrature downconversion is a specific type of
downconversion technique that uses two LO signals
that are 90 degrees out of phase with each other,
referred to as in-phase (I) and quadrature (Q)
signals. The I and Q signals are mixed with the RF
signal separately to produce two downconverted
signals at the same IF, but with a 90-degree phase
difference between them. The two downconverted
signals are then used in subsequent signal
processing, such as phase demodulation and digital
signal processing.

Quadrature downconversion is particularly useful
for demodulating signals that use amplitude and
phase modulation schemes, such as quadrature
amplitude modulation (QAM). These modulation
schemes encode both amplitude and phase
information into a single signal, which can be
extracted using I/Q demodulation
techniques. Additionally, I/Q signals are often
used in other signal processing applications, such
as digital signal processing and software-defined
radio (SDR), due to their easy manipulation in the
complex plane.

66> what are the mathematical properties of
a quadrature oscillator?

A quadrature oscillator is a type of oscillator
that generates two sinusoidal signals with
a 90-degree phase difference between
them. Mathematically, the signals can be
represented as:

x(t) = A * cos(ωt)
y(t) = A * sin(ωt)

where A is the amplitude, ω is the angular
frequency, and t is time.

The key mathematical property of a quadrature
oscillator is that the two signals have a fixed
phase relationship, meaning that the phase
difference between them remains constant over
time. This is important in many applications, such
as in communication systems, where the signals
need to be properly aligned in order to be
accurately detected and decoded.

Another important property of a quadrature
oscillator is that it can be used to generate
a complex signal, which is a signal that has both
amplitude and phase components. This is done by
combining the two quadrature signals x(t) and y(t)
into a single signal z(t) as:

z(t) = x(t) + j * y(t)

where j is the imaginary unit. This complex signal
can be used in many applications, such as in
digital signal processing and communications,
where it allows for more efficient and flexible
processing of signals.

67> could we understand these from the perspective
of complex analysis?

Yes, complex analysis can provide a useful
framework for understanding the mathematical
properties of a quadrature oscillator.

A quadrature oscillator can be represented as
a complex exponential signal of the form:

$$ z(t) = e^{j\omega t} $$

where $\omega$ is the frequency of the oscillator
in radians per second, and $j$ is the imaginary
unit. We can also express $z(t)$ in terms of its
real and imaginary components:

$$ z(t) = x(t) + jy(t) $$

where

$$ x(t) = \cos(\omega t) \quad \text{and} \quad y(t) = \sin(\omega t) $$

From the perspective of complex analysis, the
complex exponential function $e^{j\omega t}$ is
a complex-valued function of the real variable
$t$. We can think of this function as tracing out
a path in the complex plane as $t$ varies, with
the magnitude of the function staying constant at
every point on the path and the argument of the
function increasing linearly with $t$.

The quadrature oscillator produces two such paths
in the complex plane, corresponding to the real
and imaginary components $x(t)$ and $y(t)$. These
paths are orthogonal to each other, meaning they
are at right angles to each other in the complex
plane. This is why the term "quadrature" is used:
the two paths are "in quadrature" with each other.

In communications systems, quadrature oscillators
are often used in conjunction with a mixer to
perform quadrature downconversion, as I mentioned
earlier. This involves multiplying an incoming
signal by a complex exponential of the form
$e^{j\omega_c t}$, where $\omega_c$ is the
frequency of the local oscillator used for
downconversion. The result of this multiplication
is a complex signal that can be separated into its
in-phase and quadrature components using a pair of
filters, allowing the original signal to be
recovered.

In summary, complex analysis provides a useful
framework for understanding the mathematical
properties of quadrature oscillators, which are
widely used in communications systems for
quadrature downconversion and other applications.

68> what about from the perspective of functional
analysis?

The theory of functional analysis is a branch of
mathematics that studies vector spaces of
functions and their properties. While it is
possible to apply functional analysis to the study
of quadrature oscillators, it is not the most
common perspective for analyzing these types of
systems.

However, one area where functional analysis can be
applied to quadrature oscillators is in the study
of the spectral properties of the signals they
generate. By representing the oscillator's output
signal as a function in a suitable function space,
one can use techniques from functional analysis to
study its Fourier transform, which can provide
insights into the oscillator's frequency content
and other properties.

Another area where functional analysis is commonly
used in signal processing is in the study of
linear time-invariant (LTI) systems, which can be
used to model a wide range of physical systems,
including electrical circuits, mechanical systems,
and acoustic systems. Quadrature oscillators can
be modeled as LTI systems, and techniques from
functional analysis can be used to study their
frequency response and other properties.

69> yes, that's what i was interested in -- the
spectral properties

Functional analysis provides a powerful framework
for studying the spectral properties of quadrature
oscillators. In particular, one can use the theory
of Banach algebras to analyze the algebraic and
spectral properties of the space of continuous
functions on the unit circle, which is the natural
domain for quadrature oscillator signals.

The space of continuous functions on the unit
circle, denoted by $C(\mathbb{T})$, can be
equipped with the convolution product defined by
$(f*g)(\theta) = \frac{1}{2\pi}\int_{-\pi}^\pi
f(\theta - \phi)g(\phi) d\phi$. This product turns
$C(\mathbb{T})$ into a commutative Banach algebra
with unit element $1(\theta) = 1$ for all $\theta
\in \mathbb{T}$.

One can then define the Fourier transform on
$C(\mathbb{T})$ by $\hat{f}(n)
= \frac{1}{2\pi}\int_{-\pi}^\pi f(\theta)
e^{-in\theta} d\theta$, where $n \in \mathbb{Z}$,
and its inverse by $f(\theta)
= \sum_{n=-\infty}^\infty \hat{f}(n)
e^{in\theta}$. The Fourier transform is a linear
bijection from $C(\mathbb{T})$ onto the space
$\ell^1(\mathbb{Z})$ of absolutely convergent
complex sequences, and it satisfies the Parseval
identity $\|f\|_2 = \|\hat{f}\|_2$ for all $f \in
C(\mathbb{T})$, where $\|\cdot\|_2$ denotes the
$\ell^2$-norm.

From this perspective, a quadrature oscillator can
be seen as a signal in $C(\mathbb{T})$ that has
a well-defined frequency content. Specifically,
let $f(\theta) = r(\theta) + i\cdot q(\theta)$ be
a quadrature oscillator signal, where $r(\theta)$
and $q(\theta)$ are real-valued functions on
$\mathbb{T}$ that represent the real and imaginary
parts of the signal, respectively. Then the
Fourier transform of $f$ is given by $\hat{f}(n)
= \hat{r}(n) + i\cdot\hat{q}(n)$, where
$\hat{r}(n)$ and $\hat{q}(n)$ are the Fourier
coefficients of $r$ and $q$, respectively.

The magnitude of the Fourier transform
$|\hat{f}(n)|$ represents the spectral content of
the signal at frequency $n$, and the phase of
$\hat{f}(n)$ represents the phase relationship
between the real and imaginary components of the
signal at that frequency. Thus, a quadrature
oscillator can be designed to have a desired
frequency and phase relationship by carefully
choosing the functions $r$ and $q$.

In summary, the theory of Banach algebras and the
Fourier transform provide a powerful mathematical
framework for analyzing the spectral properties of
quadrature oscillators, and for designing signals
with desired frequency and phase characteristics.
