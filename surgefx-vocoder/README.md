## surgefx-vocoder

The `surgefx-vocoder` crate provides a vocoder
effect for the Surge synthesizer system.

The vocoder effect is a type of audio processing
that uses a bank of bandpass filters and an
envelope follower to analyze the spectral
characteristics of a modulator signal (such as
a voice) and apply those characteristics to
a carrier signal (such as a synthesizer). The
result is a "talking synthesizer" effect that can
be used to create a wide range of robotic, sci-fi,
or otherworldly sounds.

The vocoder effect implemented in
`surgefx-vocoder` is based on a bank of bandpass
filters and an envelope follower, which are used
to analyze the spectral characteristics of the
modulator signal. The resulting spectral envelope
is then used to modulate the amplitude of the
carrier signal, creating the characteristic
"talking synthesizer" effect. The implementation
includes support for adjusting the frequency range
of the bandpass filters, as well as the gain of
the effect.

Mathematically, the vocoder effect can be thought
of as a type of filtering and modulation. The
bandpass filters used in the vocoder can be
modeled as a set of transfer functions that
selectively attenuate or pass certain frequency
components of the modulator signal. The envelope
follower can be thought of as a detector that
tracks the amplitude of the modulator signal over
time. The resulting spectral envelope can be used
to modulate the amplitude of the carrier signal,
creating the characteristic "talking synthesizer"
effect.

The `surgefx-vocoder` crate includes several
tokens for controlling the behavior of the vocoder
effect. These include `control_group`,
`control_type`, `default_value`, `max_value`,
`min_value`, `modulateable`, `moverate`, and
`value_type`, which are used to define the
behavior of the effect's control parameters. The
crate also includes tokens for setting the default
high and low frequency range of the bandpass
filters (`vocoder_default_freq_high` and
`vocoder_default_freq_low`, respectively), as well
as tokens for handling streaming mismatches and
updating the effect's state
(`handle_streaming_mismatches` and `update`,
respectively).

The `surgefx-vocoder` crate includes a `Vocoder`
struct that encapsulates the state of the vocoder
effect, as well as a `VocoderBlockCfg` struct that
defines the configuration of the vocoder
processing block. The `do_vocoder_block` function
is used to apply the vocoder effect to a block of
audio samples, and the `gain` and `set_gain`
functions are used to adjust the gain of the
effect.

Overall, the `surgefx-vocoder` crate provides
a powerful and versatile vocoder effect that can
be used to create a wide range of unique and
expressive sounds.

37> what does a vocoder block diagram look like?

A vocoder block diagram typically consists of
several processing stages that work together to
analyze and synthesize the modulator and carrier
signals. Here is a high-level block diagram of
a typical vocoder:

```
  +-------------+
  | Modulator   |
  |             |
  +--+--+--+---+
     |  |  |
     |  |  +-------------------------+
     |  |                            |
     |  |  +-------------------+    |
  +--v--+--v--+--+---+    +-----v----v--+
  | Bandpass    |      |    | Envelope  |
  | Filters     |      |    | Follower  |
  +-------------+      |    +-----------+
                       |
  +-------------+      |
  | Carrier     |      |
  |             +------+
  +-------------+
```

In this diagram, the modulator signal (typically
a voice or other audio input) is first passed
through a bank of bandpass filters that are tuned
to different frequency bands. Each bandpass filter
selects a particular frequency range of the
modulator signal and attenuates all other
frequencies. The resulting filtered signals are
then rectified and smoothed using an envelope
follower, which tracks the amplitude of each
frequency band over time.

The resulting set of envelopes (one for each
frequency band) are then used to modulate the
amplitude of a carrier signal (typically
a synthesizer or other sound source). The carrier
signal is usually passed through a bank of
bandpass filters that are tuned to the same
frequency bands as the modulator filters. The
amplitude of each band of the carrier signal is
then multiplied by the corresponding envelope from
the modulator signal, resulting in a set of
amplitude-modulated carrier signals.

Finally, the amplitude-modulated carrier signals
are summed together to produce the output
signal. The result is a sound that is shaped by
the spectral characteristics of the modulator
signal, creating the characteristic "talking
synthesizer" effect.

It's worth noting that the specific implementation
of a vocoder can vary depending on the particular
application and desired sound. Some vocoders may
include additional processing stages, such as
harmonic scaling or formant shifting, to further
shape the output sound.

38> how does an envelope follower work?

An envelope follower is a signal processing block
that is commonly used in audio applications to
track the amplitude or "envelope" of an input
signal over time. In the context of a vocoder, an
envelope follower is typically used to extract the
amplitude envelope of the modulator signal for use
in modulating the carrier signal.

The basic operation of an envelope follower can be
described using the following steps:

1. Rectification: The input signal is first passed
   through a rectifier circuit that removes any
   negative values from the signal. This is
   typically done using a diode or similar
   component.

2. Smoothing: The rectified signal is then passed
   through a smoothing filter that removes any
   high-frequency variations in the signal. This
   filter can be a low-pass filter or a simple
   smoothing function such as an exponential
   moving average.

3. Scaling: The smoothed signal is then scaled to
   match the desired amplitude range. This scaling
   is typically done using a gain control or
   similar component.

The resulting signal is an estimate of the
envelope of the input signal, which tracks the
overall amplitude of the signal over time. The
envelope follower can then be used to modulate the
amplitude of another signal, such as a carrier
signal in a vocoder.

Mathematically, the operation of an envelope
follower can be described using the following
equation:

```
e[n] = g * (alpha * e[n-1] + (1 - alpha) * abs(x[n]))
```

where `x[n]` is the input signal, `e[n]` is the
output envelope signal, `g` is a scaling factor,
and `alpha` is a smoothing parameter. The `abs()`
function is used to rectify the input signal, and
the smoothing filter is implemented as
a first-order recursive filter with a time
constant determined by the `alpha` parameter. The
output signal is then scaled by the gain factor
`g`.

Overall, the envelope follower is an important
component of many audio processing applications,
including vocoders, compressors, and filters. It
provides a way to extract the amplitude envelope
of an input signal, which can be used to modulate
the amplitude of other signals in interesting and
expressive ways.

39> how do we get the vocoder to "talk"?

To make a vocoder "talk", you need to provide it
with a modulator signal that contains speech or
other vocal sounds. The modulator signal is
typically a microphone input or a pre-recorded
audio file that contains speech or other vocal
sounds.

Once you have a modulator signal, you can use the
vocoder to analyze the spectral characteristics of
the signal and apply those characteristics to
a carrier signal, such as a synthesizer or other
sound source. The result is a "talking
synthesizer" effect that sounds like the carrier
signal is "talking" in the same way as the
modulator signal.

To achieve the best possible "talking" effect with
a vocoder, it's important to carefully match the
frequency range of the modulator signal to the
frequency range of the carrier signal. This is
typically done by adjusting the frequency range of
the bandpass filters in the vocoder to match the
spectral characteristics of the modulator
signal. For example, if the modulator signal
contains a lot of high-frequency content, you may
need to adjust the high-frequency cutoff of the
bandpass filters to ensure that the carrier signal
is modulated appropriately.

It's also important to experiment with different
settings for the envelope follower and gain
control in the vocoder to achieve the desired
"talking" effect. In general, you want to use
a fast attack time and a slow release time for the
envelope follower, to ensure that the envelope
tracks the fast changes in the modulator signal
while maintaining a smooth and natural-sounding
output. The gain control can be used to adjust the
overall level of the carrier signal, and to adjust
the balance between the modulator and carrier
signals.

Overall, achieving a good "talking" effect with
a vocoder requires careful attention to the
frequency range, envelope follower settings, and
gain control settings. With the right settings,
however, a vocoder can be a powerful tool for
creating unique and expressive vocal effects.

40> what is a spectral envelope?

A spectral envelope is a mathematical
representation of the overall shape of the
frequency spectrum of a signal. It is commonly
used in audio processing applications to analyze
and manipulate the spectral characteristics of
a sound.

To understand spectral envelopes, it's helpful to
first understand the basic idea of a frequency
spectrum. A frequency spectrum is a plot that
shows the amplitude of the different frequency
components of a signal. The spectrum can be
obtained using a Fourier transform, which converts
a time-domain signal into a frequency-domain
representation.

The spectral envelope is a way of summarizing the
frequency spectrum in a more compact and useful
way. It does this by identifying the "peaks" and
"valleys" in the spectrum and representing them as
a series of "formants" or frequency bands. These
frequency bands are typically logarithmically
spaced and may be of different widths.

The spectral envelope can be used to modify the
spectral characteristics of a signal, for example
by selectively boosting or attenuating different
frequency bands. In a vocoder, the spectral
envelope of the modulator signal is used to
modulate the amplitude of the carrier signal,
resulting in a "talking synthesizer" effect.

Overall, the spectral envelope is an important
concept in audio processing and provides a way to
analyze and manipulate the spectral
characteristics of a sound in a compact and useful
way.

41> how do we connect a vocoder to a microphone,
so that we can sing into the microphone and then
play the keyboard to modulate the sound?

To connect a microphone to a vocoder, you will
need to use an audio interface or mixer that has
both a microphone input and a line-level
output. The microphone input will typically be an
XLR or TRS jack, and the line-level output will
typically be a 1/4" jack or a set of RCA jacks.

Here is a general outline of the steps involved in
connecting a microphone to a vocoder:

1. Connect the microphone to the microphone input
   of your audio interface or mixer. This will
   typically require an XLR or TRS cable,
   depending on the type of microphone.

2. Connect the line-level output of your audio
   interface or mixer to the input of the
   vocoder. This will typically require a 1/4"
   jack or a set of RCA jacks, depending on the
   input of the vocoder.

3. Connect the output of the vocoder to your
   amplifier or speakers. This will typically
   require a 1/4" jack or a set of RCA jacks,
   depending on the output of the vocoder.

Once you have connected the microphone to the
vocoder, you can start singing into the microphone
to modulate the sound of the carrier signal. To
play the keyboard to modulate the sound, you will
need to connect the output of the keyboard to the
input of the vocoder. This can be done using
a 1/4" jack or a set of RCA jacks, depending on
the output of the keyboard and the input of the
vocoder.

Overall, connecting a microphone to a vocoder is
a relatively simple process that can be done using
an audio interface or mixer. With the right
connections in place, you can start experimenting
with different modulator and carrier signals to
create unique and expressive vocal effects.

42> interesting -- what else should we know about
vocoders? what is their history?

Vocoders have a rich history in both music and
communication technology. The first vocoder was
invented in the early 1930s by Homer Dudley at
Bell Labs, and was originally designed as a tool
for encoding speech signals for transmission over
telephone lines. The vocoder was used extensively
during World War II for secure military
communication, and became an important tool for
speech analysis and synthesis in the following
decades.

In the 1960s, musicians began to experiment with
vocoders as a creative tool for making music. The
first commercially available vocoder for music was
the EMS Vocoder 5000, which was released in 1976
and quickly became popular with artists such as
Kraftwerk and Pink Floyd. The vocoder was also
used extensively in electronic and hip-hop music
throughout the 1980s and 1990s, and has remained
a popular tool for music production to this day.

In recent years, advances in digital signal
processing have made it possible to implement
vocoders on a wide range of platforms, from
hardware synthesizers to software plugins for
digital audio workstations. Today, the vocoder
remains an important tool for creative expression
in music, as well as a powerful tool for speech
analysis and synthesis in fields such as
telecommunications and speech therapy.

Overall, the vocoder has a rich and diverse
history, and has played an important role in both
music and communication technology over the past
century. Its continued popularity and versatility
are a testament to the power and flexibility of
this unique signal processing technique.

43> anything else we should know about their
effective usage?

Here are a few tips for using a vocoder
effectively:

1. Match the frequency ranges of the modulator and
   carrier signals: To achieve the best possible
   "talking" effect with a vocoder, it's important
   to carefully match the frequency range of the
   modulator signal (typically a vocal recording)
   to the frequency range of the carrier signal
   (typically a synthesizer or other sound
   source). This can be done by adjusting the
   frequency range of the bandpass filters in the
   vocoder to match the spectral characteristics
   of the modulator signal.

2. Experiment with different envelope follower
   settings: The envelope follower is an important
   component of the vocoder, as it tracks the
   amplitude of the modulator signal and uses it
   to modulate the amplitude of the carrier
   signal. Different envelope follower settings
   can produce different "talking" effects, so
   it's important to experiment with different
   attack and release times to find the right
   settings for your application.

3. Use a high-quality microphone: The quality of
   the microphone you use to capture the modulator
   signal can have a big impact on the quality of
   the resulting "talking" effect. A high-quality
   condenser microphone can help capture the
   nuances of the human voice more accurately,
   resulting in a more convincing and expressive
   effect.

4. Adjust the gain control to balance the
   modulator and carrier signals: The gain control
   in the vocoder can be used to adjust the
   overall level of the carrier signal, as well as
   the balance between the modulator and carrier
   signals. This can be used to achieve a more
   natural-sounding "talking" effect, and to
   ensure that the carrier signal is audible above
   the modulator signal.

Overall, using a vocoder effectively requires
careful attention to the spectral characteristics
of the modulator and carrier signals, as well as
the settings of the envelope follower and gain
control. With the right settings and techniques,
however, a vocoder can be a powerful tool for
creating unique and expressive vocal effects.

44> any caveats?

Here are a few caveats to keep in mind when using
a vocoder:

1. Avoid excessive bandpass filtering: While
   bandpass filtering is an essential part of the
   vocoder process, excessive filtering can result
   in a "thin" or "artificial" sound. To avoid
   this, it's important to carefully adjust the
   frequency range and bandwidth of each bandpass
   filter to preserve the natural characteristics
   of the modulator signal.

2. Watch out for "pumping" effects: The envelope
   follower in the vocoder can sometimes produce
   a "pumping" effect, where the amplitude of the
   carrier signal is rapidly modulated up and down
   in response to changes in the modulator
   signal. This effect can be minimized by
   adjusting the attack and release times of the
   envelope follower, as well as the gain control.

3. Use in moderation: While the vocoder can be
   a powerful tool for creating unique and
   expressive vocal effects, it's important to use
   it in moderation and in appropriate musical
   contexts. Overusing the vocoder can result in
   a "gimmicky" or "dated" sound, so it's
   important to use it thoughtfully and
   creatively.

4. Be mindful of copyright issues: The use of
   a vocoder to manipulate copyrighted vocal
   recordings or samples can be a legal gray area,
   so it's important to be mindful of copyright
   laws and licensing requirements when using
   a vocoder in your music production.

Overall, the vocoder is a powerful and versatile
tool for creating unique and expressive vocal
effects, but it should be used thoughtfully and
with care to avoid unwanted artifacts or legal
issues.

45> what are 12 techniques from production-quality
electronic music production which leverage
vocoders?

Here are 12 techniques from festival-quality
electronic music production that leverage
vocoders:

1. Vocal Harmonies: A vocoder can be used to
   create vocal harmonies by modulating
   a synthesizer or other sound source with
   a vocal recording.

2. Robot Voices: The distinctive "robotic" sound
   of a vocoder can be used to create a range of
   futuristic, sci-fi inspired voice effects.

3. Layered Chords: A vocoder can be used to create
   layered chord progressions by modulating
   a chordal synthesizer patch with a vocal
   recording.

4. Drum Synthesis: A vocoder can be used to
   synthesize drum sounds by modulating
   a percussive sound source with a vocal
   recording.

5. Vocal FX: A vocoder can be used to create
   a range of vocal effects, including filtering,
   distortion, and modulation.

6. Instrument FX: A vocoder can be used to create
   a range of instrument effects, including
   filtering, distortion, and modulation.

7. Sound Design: A vocoder can be used as a sound
   design tool to create unique and complex
   timbres by modulating different sound sources
   with a vocal recording.

8. Lead Synths: A vocoder can be used to create
   lead synth sounds by modulating a melodic sound
   source with a vocal recording.

9. Texture Creation: A vocoder can be used to
   create complex, evolving textures by modulating
   multiple sound sources with a vocal recording.

10. Bass Synths: A vocoder can be used to create
    bass synth sounds by modulating
    a low-frequency sound source with a vocal
    recording.

11. Glitch Effects: A vocoder can be used to
    create glitch effects by modulating different
    sound sources with short, staccato vocal
    recordings.

12. Vocal Sampling: A vocoder can be used to
    sample and manipulate vocal recordings in
    real-time, creating unique and expressive
    vocal performances.

Overall, the vocoder is a versatile and powerful
tool for electronic music production, and can be
used in a wide range of creative applications. By
experimenting with different modulator and carrier
signals, as well as different envelope follower
settings and gain controls, producers can create
unique and expressive vocal effects that are sure
to stand out in any mix.
