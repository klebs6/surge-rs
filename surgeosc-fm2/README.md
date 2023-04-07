# FM2 Oscillator: Advanced frequency modulation synthesis for Surge

An advanced frequency modulation oscillator
implementation for the Surge synthesizer system,
providing more complex modulation options and
increased flexibility in sound design.

The `surgeosc-fm2` crate builds upon the
principles of frequency modulation (FM) synthesis,
adding new capabilities and functionality to
create a more powerful and versatile
oscillator. This oscillator is designed for
advanced sound synthesis in electronic music
production and allows users to create a wide range
of intricate and evolving sounds.

The main mathematical concept behind this
oscillator is still FM synthesis, which is based
on the following equation:

y(t) = A * sin(2 * π * (f_c + I * f_m * sin(2 * π * f_m * t)) * t)

However, the `FM2Oscillator` extends the basic FM
synthesis by incorporating additional modulation
sources, multiple carriers, and other enhancements
to create more complex and dynamic sounds. The
specific mathematical properties and relationships
between the tokens will depend on the
implementation details of the crate.

To provide a detailed mathematical analysis and
description of the associated tokens, it is
essential to have a deeper understanding of the
implementation and functionality of the
oscillator. With more information about the
specific features and enhancements provided by the
`FM2Oscillator`, a more accurate and informative
description can be created.

14> thanks, first though: how can we enhance an fm
synthesizer beyond what we have already seen? what
are some techniques we can use to add more depth?

Enhancing an FM synthesizer can be done by
incorporating additional features and techniques
to provide more depth and flexibility in sound
design. Some of these enhancements include:

1. **Multiple carriers and modulators**:
   Introducing additional carriers and modulators
   allows for the creation of more complex
   sounds. This can lead to intricate timbral
   variations and richer harmonic content.

2. **Operator feedback**: Implementing feedback
   loops within the modulation chain can create
   more diverse sounds and increase the harmonic
   complexity of the output. This allows for
   a broader range of timbres and textures.

3. **Dynamic modulation routing**: Allowing for
   dynamic changes in modulation routing enables
   users to create evolving sounds with
   time-varying timbral characteristics. This can
   be achieved by incorporating user-controllable
   modulation matrices or flexible routing
   schemes.

4. **Waveshaping**: Adding waveshaping
   capabilities to the modulators or carriers can
   create additional harmonic content and provide
   unique spectral characteristics to the
   synthesized sound. This can include applying
   non-linear functions, wavefolding, or other
   transformations to the waveform.

5. **Modulation of modulation indices**:
   Modulating the modulation indices with
   additional envelopes or LFOs can create
   time-varying depth and complexity in the
   synthesized sound. This allows for more
   expressive and evolving sounds.

6. **Velocity-sensitive modulation**:
   Incorporating velocity-sensitive modulation in
   the FM synthesis process can add expressiveness
   to the sound, as the modulation depth and other
   parameters can be controlled by the velocity of
   the incoming MIDI notes.

7. **Multiple oscillator modes**: Implementing
   different oscillator modes, such as phase
   modulation, amplitude modulation, or ring
   modulation, can offer a broader range of sonic
   possibilities and textures.

8. **Polyphony and voice management**:
   Incorporating polyphony and advanced voice
   management features can lead to more complex
   soundscapes and enable users to create lush,
   layered sounds.

9. **Integration with other synthesis
   techniques**: Combining FM synthesis with other
   synthesis techniques, such as additive,
   subtractive, or wavetable synthesis, can result
   in unique and intricate sounds that offer
   a wide variety of timbral possibilities.

10. **Built-in effects**: Adding built-in effects,
    such as reverb, delay, or distortion,/ can
    enhance the sound and provide additional
    creative possibilities for the user.

By incorporating these enhancements and
techniques, an FM synthesizer can become more
versatile and capable of generating a broader
range of sounds, offering more depth and
flexibility to the user.

15> /file surge-rs/surgeosc-fm2/src/process.rs

The FM2Oscillator differs from the original
FM1Oscillator in several key aspects, which are as
follows:

1. **Shift parameter**: The FM2Oscillator
   introduces a new parameter called `shift`. This
   parameter is applied to the modulator
   frequencies (rm1 and rm2). It adds a frequency
   shift to the modulators, allowing for more
   complex interactions between the carrier and
   modulators, resulting in a wider range of
   sounds.

2. **Integer ratios**: In the FM2Oscillator, the
   ratios for the modulators are defined as
   integers (`i_ratio1` and `i_ratio2`), which
   leads to simpler relationships between the
   carrier and modulators. This can result in
   a more harmonically pleasing sound, as the
   frequency relationships are more likely to
   produce harmonic content that is musically
   related to the carrier frequency.

3. **Start phase parameter**: The FM2Oscillator
   introduces a new parameter called `startphase`
   that allows you to control the starting phase
   of the output waveform. This gives you more
   control over the phase relationship between the
   carrier and modulators, which can be useful for
   creating unique sounds and can influence the
   way the timbre evolves over time.

4. **Phase offset**: The FM2Oscillator adds
   a phase offset to the output waveform. This is
   controlled by the `startphase` parameter and is
   implemented using the `phase_offset`
   SmoothedValue.

These differences contribute to the
FM2Oscillator's ability to produce a broader range
of sounds compared to the original
FM1Oscillator. The new parameters and
modifications enable more control over the
synthesis process and offer additional flexibility
for creating unique and interesting sounds.

16> interestin -- how can we improve it?

To improve the FM2Oscillator, you can consider
implementing the following enhancements:

1. **Add more modulators**: Increase the number of
   modulators to create even more complex
   frequency modulation patterns. This can result
   in a richer sound spectrum and more interesting
   timbres.

2. **Variable modulator ratios**: Allow for
   non-integer and variable modulator ratios to
   further expand the available timbral
   palette. This can lead to more inharmonic and
   experimental sounds, increasing the versatility
   of the oscillator.

3. **Flexible modulation routing**: Implement
   a more flexible modulation routing system that
   allows users to define custom modulation paths
   between the carrier and multiple
   modulators. This can lead to a wider range of
   possible sounds and more intricate modulation
   patterns.

4. **Phase modulation**: Add phase modulation as
   an option in addition to frequency
   modulation. Phase modulation can produce
   similar sounds to frequency modulation but with
   different mathematical properties, providing an
   alternative synthesis method within the same
   oscillator.

5. **Waveform selection**: Allow users to choose
   different waveforms for the carrier and
   modulators. This can result in a broader range
   of sounds and provide more options for sound
   designers to experiment with.

6. **Additional modulation sources**: Integrate
   additional modulation sources such as LFOs,
   envelopes, or sequencers to create evolving and
   dynamic sounds.

7. **Built-in effects**: Implement built-in
   effects such as chorus, flanger, or distortion
   to further shape and process the oscillator's
   output.

8. **Polyphony and unison options**: Add support
   for polyphony and unison to create thicker,
   more complex sounds by stacking multiple
   instances of the oscillator with varying pitch,
   phase, and other parameters.

9. **Morphing between parameters**: Implement
   morphing between different parameter settings
   to create smooth transitions between different
   sounds.

10. **Randomization**: Add a randomization feature
    to generate new, unique sounds by randomly
    changing the oscillator's parameters.

These improvements can contribute to a more
versatile and powerful FM2Oscillator, providing an
expanded range of sound possibilities and a more
engaging user experience for sound designers and
musicians.
