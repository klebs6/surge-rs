# surgefx-ringmod

SurgeFX-RingMod is a Rust crate that provides
a ring modulator component for the Surge
Synthesizer system. The ring modulator is
a crucial part of many synthesis techniques and
can produce interesting harmonic spectra by
multiplying two input signals.

## Ring Modulation

Ring modulation is a non-linear modulation
technique that multiplies two input signals,
resulting in a new signal that contains sum and
difference frequencies of the original
signals. The formula for ring modulation is:

`y(t) = (x1(t) * x2(t))`

where `x1(t)` and `x2(t)` are the input signals,
and `y(t)` is the output signal.

## Crate Tokens

- `control_group`: A grouping of control
  parameters that are related to each other.

- `control_type`: The type of the control
  parameter, such as a knob or a button.

- `default_value`: The default value of the
  control parameter.

- `max_value`: The maximum value of the control
  parameter.

- `min_value`: The minimum value of the control
  parameter.

- `modulateable`: A flag that indicates whether
  the control parameter can be modulated.

- `moverate`: The rate at which the modulated
  parameter changes.

- `value_type`: The type of value for the control
  parameter, such as integer or float.

- `diode_sim`: A flag that indicates whether the
  ring modulator should simulate a diode.

- `update_unison_settings`: A function that
  updates the unison settings of the ring
  modulator.

- `RingModulator`: The main struct of the ring
  modulator component.

- `ringmod_smoke`: A function that generates
  a smoke effect when the ring modulator is in
  use.

- `init`: A function that initializes the ring
  modulator component.

- `process`: A function that processes the input
  signals and produces the output signal.

The `RingModulator` struct contains control
parameters for adjusting the input signals and the
output signal of the ring modulator. The `process`
function multiplies the input signals and applies
the control parameters to the output signal. The
`ringmod_smoke` function provides a visual effect
that simulates the ring modulator generating
smoke.

## Conclusion

SurgeFX-RingMod provides a versatile ring
modulator component for the Surge Synthesizer
system, which can generate interesting harmonic
spectra by multiplying two input signals. The
crate provides a range of control parameters that
can be adjusted to fine-tune the output signal.

2> could you teach me about ring modulation in
more depth?

Certainly! Ring modulation is a non-linear
modulation technique that multiplies two input
signals together to produce a new output
signal. The resulting signal contains sum and
difference frequencies of the original signals.

Let's look at the formula again:

`y(t) = (x1(t) * x2(t))`

where `x1(t)` and `x2(t)` are the input signals,
and `y(t)` is the output signal.

When the two input signals are the same frequency,
the output signal is simply a new version of the
original signal with a different
amplitude. However, when the input signals have
different frequencies, the output signal contains
additional frequencies that are not present in
either input signal.

For example, let's say `x1(t)` is a sine wave with
a frequency of 440 Hz (A4), and `x2(t)` is a sine
wave with a frequency of 660 Hz (E5). When we
apply ring modulation to these two signals, the
resulting output signal will contain frequencies
at 220 Hz (A3), 330 Hz (E4), 1100 Hz (A5), and
1320 Hz (E6), in addition to the original
frequencies of 440 Hz and 660 Hz.

This effect can be useful for creating interesting
harmonic content in sound synthesis. In fact, ring
modulation is often used in combination with other
synthesis techniques, such as frequency modulation
(FM) or subtractive synthesis, to create complex
and unique sounds.

It's worth noting that ring modulation is
a non-linear operation, which means that it can
introduce distortion into the
signal. Additionally, the output of a ring
modulator is often more complex and dissonant than
the original signals, so it can require some
careful tweaking to get the desired effect.

3> how can we understand ring modulation from the
perspective of spectral analysis?

Ring modulation can be understood from the
perspective of spectral analysis by examining the
resulting frequency spectrum of the output
signal. As I mentioned earlier, the output signal
of a ring modulator contains sum and difference
frequencies of the original signals.

To see this more clearly, let's take a look at the
frequency domain equation for ring modulation:

`Y(f) = 0.5 * (X1(f) * X2(f - Fc) + X1(f - Fc) * X2(f))`

where `X1(f)` and `X2(f)` are the Fourier
transforms of the input signals, `Y(f)` is the
Fourier transform of the output signal, and `Fc`
is the frequency of a carrier signal that is used
to modulate the input signals.

The term `X1(f) * X2(f - Fc)` represents the
frequency components that result from multiplying
the two input signals together, and the term `X1(f
- Fc) * X2(f)` represents the frequency components
that result from multiplying the two input signals
with a phase shift of `Fc`.

When we add these two terms together and take the
magnitude of the result, we get the frequency
spectrum of the output signal. This spectrum will
contain the original frequencies of the input
signals, as well as new frequencies that are the
sum and difference of the original frequencies.

For example, let's say we have two input signals
with frequencies of 440 Hz and 660 Hz, and we
modulate them with a carrier signal of 1000
Hz. The resulting output signal will have
frequency components at 340 Hz, 400 Hz, 740 Hz,
1100 Hz, and 1320 Hz, in addition to the original
frequencies of 440 Hz and 660 Hz.

Spectral analysis can be a useful tool for
understanding the frequency content of a signal
and how different processing techniques, such as
ring modulation, can affect that content.

4> how does it relate to the eigenvalue problem?

Ring modulation is not directly related to the
eigenvalue problem. The eigenvalue problem is
a concept from linear algebra and matrix theory
that involves finding the eigenvalues and
eigenvectors of a matrix.

However, there is a connection between ring
modulation and frequency modulation (FM), which is
a type of synthesis that can be modeled using the
eigenvalue problem. In FM synthesis, a carrier
signal is modulated by a modulator signal, which
results in a new frequency spectrum that is rich
in harmonics. This effect is similar to ring
modulation, but with a continuously varying
modulation index that changes the amount of
frequency modulation over time.

The FM synthesis algorithm can be represented as
a matrix multiplication problem, where the input
signal (the modulator) is multiplied by a matrix
that depends on the carrier frequency and
modulation index. The resulting output signal (the
carrier) is then the eigenvalue of the matrix
multiplied by the input signal (the eigenvector).

So, while ring modulation itself does not directly
involve the eigenvalue problem, it is related to
FM synthesis, which can be modeled using the
eigenvalue problem.

5> isn't ring modulation essentially an individual
instance of FM modulation? or do i have that mixed
up

You are correct that ring modulation can be seen
as a specific instance of frequency modulation
(FM) synthesis, where the modulation index is
fixed and not varied over time.

In ring modulation, two input signals are
multiplied together to create a new signal that
contains sum and difference frequencies of the
original signals. This is similar to FM synthesis,
where a carrier signal is modulated by
a modulating signal to create a new signal with
a frequency spectrum that contains harmonics of
the carrier frequency.

In both cases, the resulting signal can have
complex spectral content that is not present in
the original signals. However, in FM synthesis,
the modulation index is varied over time, which
can create dynamic changes in the frequency
spectrum of the output signal.

So, while ring modulation and FM synthesis are
related, they are not exactly the same thing. Ring
modulation can be seen as a simpler version of FM
synthesis with a fixed modulation index.

6> what do you mean when you say "the modulation
index is varied over time"? does this mean the
frequency of the modulator?

The modulation index in FM synthesis refers to the
amount of frequency modulation applied to the
carrier signal by the modulating signal. It
determines how much the frequency of the carrier
signal is modulated by the modulating signal.

In FM synthesis, the modulation index can be
varied over time, which means that the amount of
frequency modulation applied to the carrier signal
changes over time. This can result in a dynamic
and evolving sound with a complex frequency
spectrum.

The frequency of the modulating signal itself does
not necessarily need to vary over time for the
modulation index to change. Instead, the
modulation index is typically controlled by
a separate parameter that can be adjusted in
real-time or automated using an envelope or other
modulation source.

So, while the frequency of the modulating signal
can affect the spectral content of the output
signal, it is not directly related to the
modulation index or the dynamic changes in
frequency modulation that can be achieved through
varying the modulation index over time.

7> so, it is like "depth" -- is that correct?

Yes, the modulation index in FM synthesis can be
thought of as the "depth" of the frequency
modulation applied to the carrier signal.

When the modulation index is high, the carrier
signal will be more significantly modulated by the
modulating signal, resulting in a more complex and
harmonically rich output signal. When the
modulation index is low, the carrier signal will
be less affected by the modulating signal,
resulting in a simpler and more straightforward
output signal.

So, increasing the modulation index can be thought
of as increasing the "depth" of the frequency
modulation applied to the carrier signal, and
decreasing the modulation index can be thought of
as decreasing the depth of modulation.

8> interesting -- which other ideas from
mathematics apply to ring modulation?

Ring modulation is a technique that involves
multiplying two signals together, which makes it
a non-linear operation. As a result, there are
several mathematical concepts and techniques that
can be applied to understand the behavior of ring
modulation.

One such concept is the Fourier transform, which
can be used to analyze the frequency content of
the input and output signals in the frequency
domain. The Fourier transform can reveal the
spectral content of the original signals, as well
as the additional frequencies that are generated
by the ring modulation process.

Another mathematical concept that is relevant to
ring modulation is the concept of harmonics. When
two signals are multiplied together, the resulting
signal contains frequencies that are the sum and
difference of the original frequencies. This can
result in the creation of additional harmonics,
which are integer multiples of the original
frequencies.

Ring modulation can also be thought of as a type
of amplitude modulation (AM), which involves
varying the amplitude of a carrier signal with
a modulating signal. AM synthesis can be modeled
using trigonometric functions, such as sine waves,
which can be combined to create complex waveforms.

Lastly, the concept of distortion is relevant to
ring modulation, as the non-linear multiplication
process can introduce additional harmonic content
and distortion into the signal. This can be
modeled mathematically using techniques such as
wave shaping or waveshaping, which involve shaping
the amplitude of a signal to create distortion and
other complex harmonic content.

9> how does ring modulation alter the harmonics of
the output signal?

Ring modulation alters the harmonics of the output
signal by adding new frequencies to the frequency
spectrum of the original signals. When two signals
are multiplied together in ring modulation, the
resulting output signal contains frequencies that
are the sum and difference of the original
frequencies.

For example, let's say we have two input signals
with frequencies of 440 Hz and 660 Hz. When we
apply ring modulation to these two signals, the
resulting output signal will contain frequency
components at 220 Hz, 330 Hz, 1100 Hz, and 1320
Hz, in addition to the original frequencies of 440
Hz and 660 Hz.

The addition of these new frequencies can create
complex harmonic content in the output signal that
was not present in the original signals. This can
result in a unique and interesting sound,
especially when combined with other synthesis
techniques like filtering, modulation, and effects
processing.

In general, the harmonics of the output signal
from ring modulation depend on the frequencies and
amplitudes of the input signals, as well as the
modulation depth and frequency. Varying these
parameters can create different harmonics and
result in a wide range of possible sounds.

10> could you show me a dozen examples of
techniques from electronic music production that
rely on ring modulation to achieve the result?
what does each of these techniques sound like and
how is the ring modulator parametrized?

Sure! Here are a dozen examples of techniques from
electronic music production that rely on ring
modulation to achieve their sound:

1. Bell and gong-like sounds: Ring modulation can
   be used to create metallic bell-like sounds,
   such as those heard in percussive or pitched
   instruments like bells or gongs. This is
   achieved by using two input signals with
   frequencies that are slightly detuned from each
   other, and by adjusting the modulation depth to
   produce a rich and complex sound with
   overtones.

2. Harmonic distortion: Ring modulation can be
   used to create harmonic distortion, which adds
   new harmonics to a sound and can give it a more
   aggressive or edgy character. This is achieved
   by using a high modulation depth and adjusting
   the frequencies and amplitudes of the input
   signals to create a specific harmonic
   structure.

3. Frequency doubling: Ring modulation can be used
   to create an effect known as frequency
   doubling, which creates new frequencies that
   are twice the original frequencies. This is
   achieved by using a carrier signal with
   a frequency that is double the frequency of the
   input signal, and by adjusting the modulation
   depth to produce a clear doubling effect.

4. Metallic bass sounds: Ring modulation can be
   used to create metallic bass sounds, which have
   a rich and complex character with
   overtones. This is achieved by using
   a low-frequency carrier signal and a modulator
   signal with a harmonic structure that is in the
   bass range. The modulation depth can be
   adjusted to create different levels of
   complexity in the sound.

5. Sci-fi sounds: Ring modulation can be used to
   create sci-fi or futuristic sounds, which have
   a robotic or metallic quality. This is achieved
   by using a carrier signal with a high frequency
   and a modulator signal with a complex waveform
   that emphasizes high frequencies. The
   modulation depth can be adjusted to create
   different levels of complexity and metallic
   character in the sound.

6. Vocal sounds: Ring modulation can be used to
   create vocal sounds, such as robot or
   vocoder-like effects. This is achieved by using
   a carrier signal with a high frequency and
   a modulator signal with a waveform that
   emphasizes the formant frequencies of a human
   voice. The modulation depth can be adjusted to
   create different levels of robotic or synthetic
   character in the sound.

7. Alien sounds: Ring modulation can be used to
   create alien or otherworldly sounds, which have
   a unique and surreal character. This is
   achieved by using two input signals with
   frequencies that are not harmonically related,
   and by adjusting the modulation depth to create
   a complex and dissonant sound with overtones.

8. Percussive sounds: Ring modulation can be used
   to create percussive sounds, such as metallic
   or bell-like hits. This is achieved by using
   two input signals with frequencies that are
   detuned from each other, and by adjusting the
   modulation depth to create a complex and bright
   sound with overtones.

9. Ambient sounds: Ring modulation can be used to
   create ambient or atmospheric sounds, which
   have a complex and evolving character. This is
   achieved by using a carrier signal with a low
   frequency and a modulator signal with a complex
   waveform that emphasizes high frequencies. The
   modulation depth can be adjusted to create
   different levels of complexity and metallic
   character in the sound.

10. Rhythmic sounds: Ring modulation can be used
    to create rhythmic or percussive sounds, such
    as glitchy or stuttering effects. This is
    achieved by using a carrier signal with a high
    frequency and a modulator signal with
    a rhythmic pattern or waveform that emphasizes
    high frequencies. The modulation depth can be
    adjusted to create different levels of
    rhythmic complexity in the sound.

11. Distorted guitar sounds: Ring modulation can
    be used to create distorted guitar sounds,
    which have a unique and aggressive
    character. This is achieved by using two input
    signals with frequencies that are harmonically
    related, and by adjusting the modulation depth
    to create a rich and

11> cool thanks -- please finish! you were on 11

Sorry about that, here is the last example:

12. FM synthesis: As I mentioned earlier, ring
    modulation can be seen as a specific instance
    of frequency modulation (FM) synthesis, where
    the modulation index is fixed and not varied
    over time. FM synthesis can be used to create
    a wide range of sounds, including bell-like
    tones, metallic sounds, and complex harmonic
    textures. The parameters of FM synthesis can
    be adjusted to create different harmonic
    structures, including the depth of frequency
    modulation, the ratio between the carrier and
    modulating frequencies, and the waveform of
    the modulating signal.

In terms of how the ring modulator is
parametrized, this can vary depending on the
specific application and desired
effect. Typically, the ring modulator will have
controls for the input signals, including their
frequencies, amplitudes, and waveforms. There will
also be controls for the carrier signal and the
modulation depth, which determines how much the
input signals are modulated by the
carrier. Finally, there may be additional controls
for shaping and filtering the output signal, such
as EQ, distortion, and effects processing. The
specific parameters and controls will depend on
the implementation of the ring modulator and the
application in which it is used.

12> could you show me a block diagram?

Certainly! Here is a block diagram showing the
basic signal flow of a ring modulator:

```
                 +----> Input Signal 1 ---+
                 |                        |
                 |       +---> Multiply ---+---> Output
Carrier Signal --+-------|                |
                 |       +---> Multiply ---+
                 |                        |
                 +----> Input Signal 2 ---+
```

In this diagram, the carrier signal is modulating
the two input signals. Each input signal is
multiplied with the carrier signal, resulting in
a signal that contains sum and difference
frequencies of the original signals. The output
signal is the sum of the two modulated signals.

The input signals and carrier signal can be
adjusted to control the frequency content of the
output signal, and the modulation depth can be
adjusted to control the amount of modulation
applied to the input signals.

In practice, there may be additional processing
blocks before and after the ring modulator, such
as filtering, distortion, and effects
processing. However, this basic block diagram
represents the fundamental signal flow of a ring
modulator.

13> any caveats? do we get any negative artifacts
when doing this?

There are a few potential caveats to consider when
using ring modulation in electronic music
production:

1. Signal level: Ring modulation can result in
   a significant increase in the level of the
   output signal, which may cause distortion or
   clipping if the signal is not properly
   attenuated. It is important to adjust the
   signal levels appropriately to avoid unwanted
   distortion or clipping.

2. Frequency content: Ring modulation can add new
   frequencies to the signal that were not present
   in the original input signals. Depending on the
   specific application, this may be desirable or
   undesirable. Careful consideration should be
   given to the frequency content of the output
   signal to ensure that it is appropriate for the
   intended use.

3. Noise and artifacts: Ring modulation is
   a non-linear process that can introduce noise
   and other artifacts into the output
   signal. This can be minimized by using
   high-quality input signals and by adjusting the
   parameters of the ring modulator appropriately.

4. Modulation depth: The modulation depth, which
   controls the amount of frequency modulation
   applied to the input signals, can significantly
   affect the character of the output signal. It
   is important to carefully adjust the modulation
   depth to achieve the desired effect and to
   avoid unwanted distortion or clipping.

Overall, ring modulation is a powerful tool for
creating unique and interesting sounds in
electronic music production. However, it is
important to use it judiciously and to be aware of
the potential caveats and limitations associated
with the technique. With careful consideration and
experimentation, it is possible to achieve a wide
range of creative and innovative sounds using ring
modulation.

14> are there any other related mathematics we
should keep in mind?

Yes, there are several other related mathematical
concepts that can be useful when working with ring
modulation:

1. Complex numbers: Ring modulation involves
   multiplying two signals together, which can be
   represented mathematically as a multiplication
   of complex numbers. This can be useful for
   analyzing the frequency content and phase
   relationships of the input and output signals.

2. Fourier analysis: As I mentioned earlier, the
   Fourier transform can be used to analyze the
   frequency content of the input and output
   signals in the frequency domain. This can
   reveal the harmonic content and frequency
   relationships of the signals, and can help to
   identify and isolate specific frequency
   components.

3. Amplitude and phase modulation: Ring modulation
   can be seen as a type of amplitude modulation
   (AM), which involves varying the amplitude of
   a carrier signal with a modulating
   signal. Similarly, phase modulation (PM)
   involves varying the phase of a carrier signal
   with a modulating signal. Understanding these
   related modulation techniques can help to
   inform the design and implementation of ring
   modulators.

4. Nonlinear dynamics: Ring modulation is
   a non-linear process that can exhibit complex
   and chaotic behavior under certain
   conditions. Understanding the principles of
   nonlinear dynamics can be useful for predicting
   and controlling the behavior of ring
   modulators, and for creating more complex and
   interesting sounds using this technique.

5. Wave shaping: As I mentioned earlier, ring
   modulation can introduce distortion and other
   artifacts into the output signal. Wave shaping
   is a related technique that involves shaping
   the amplitude of a signal to create distortion
   and other complex harmonic
   content. Understanding the principles of wave
   shaping can help to inform the design and
   implementation of ring modulators, and can help
   to create more complex and interesting sounds
   using this technique.

Overall, a solid understanding of these related
mathematical concepts can be very helpful when
working with ring modulation, and can help to
inform and guide the design and implementation of
ring modulators for a wide range of applications
in electronic music production.

15> how can we understand ring modulation from the
context of nonlinear dynamics?

Ring modulation can be understood from the context
of nonlinear dynamics as a process that involves
the multiplication of two signals, which can lead
to the creation of new frequencies and harmonic
content. Nonlinear dynamics is concerned with the
behavior of systems that exhibit nonlinear
relationships between inputs and outputs, and ring
modulation is a classic example of a nonlinear
process.

When two signals are multiplied together in a ring
modulator, the resulting output signal can contain
sum and difference frequencies of the original
signals, as well as additional harmonics that were
not present in the original signals. This
nonlinearity can lead to complex and interesting
behavior, including the creation of harmonics that
were not present in the original signals, and the
potential for chaotic or unpredictable behavior
under certain conditions.

One of the key principles of nonlinear dynamics is
the concept of bifurcation, which describes the
behavior of a system as it undergoes changes in
its parameters or inputs. In the case of ring
modulation, changing the input signals or
modulation depth can lead to changes in the
frequency content and harmonic structure of the
output signal. As the parameters of the system are
varied, the behavior of the system can undergo
transitions or bifurcations, leading to new and
interesting sounds.

Another key principle of nonlinear dynamics is the
concept of chaos, which describes the behavior of
systems that are highly sensitive to initial
conditions and exhibit unpredictable
behavior. Ring modulation can exhibit chaotic
behavior under certain conditions, such as when
the modulation depth is high or when the input
signals are detuned from each other. Understanding
these principles can help to inform the design and
implementation of ring modulators, and can help to
create more complex and interesting sounds using
this technique.

Overall, understanding ring modulation from the
context of nonlinear dynamics can provide a useful
framework for analyzing and predicting the
behavior of this technique, and for creating new
and innovative sounds in electronic music
production.

16> what about from the perspective of complex
numbers? how do the properties of complex numbers
impact the dynamics of a system based on ring
modulation?

From the perspective of complex numbers, ring
modulation involves the multiplication of two
signals, which can be represented mathematically
as the multiplication of two complex numbers. This
allows us to analyze the behavior of the system
using the properties of complex numbers, such as
magnitude, phase, and frequency.

One key property of complex numbers that is
relevant to ring modulation is the concept of
phasors. Phasors are complex numbers that
represent sinusoidal signals in the frequency
domain, and can be used to analyze the frequency
content and phase relationships of the input and
output signals. In the context of ring modulation,
we can use phasors to represent the carrier and
modulating signals, and to analyze the behavior of
the system as a function of frequency and phase.

Another key property of complex numbers that is
relevant to ring modulation is the concept of
complex conjugation. Complex conjugation involves
taking the complex conjugate of a complex number,
which involves reversing the sign of the imaginary
part. In the context of ring modulation, complex
conjugation can be used to reverse the phase of
the carrier signal, which can have important
effects on the frequency content and phase
relationships of the output signal.

Finally, the properties of complex numbers can
also be used to analyze the stability and
bifurcations of the system. For example, the
behavior of the system can be analyzed using the
Nyquist stability criterion, which involves
plotting the frequency response of the system in
the complex plane. By analyzing the behavior of
the system in the complex plane, we can gain
insight into the stability and bifurcations of the
system, and can identify parameter regimes where
the system exhibits interesting and complex
behavior.

Overall, understanding the properties of complex
numbers is an important aspect of analyzing and
predicting the behavior of ring modulation
systems, and can help to inform the design and
implementation of ring modulators for a wide range
of applications in electronic music production.
