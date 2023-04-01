# surge-tuning

Surge-Tuning: Advanced Tuning and Mapping for
Surge Synthesizer

Surge-Tuning is a Rust crate designed to provide
advanced tuning and mapping functionality for the
Surge Synthesizer system. It offers a variety of
tuning and keyboard mapping capabilities, allowing
for a wide range of musical scales and
temperaments.

Tokens and Mathematical Ideas

  - Tone: Represents a musical tone with
    a specific frequency. The frequency can be
    calculated using the formula 
    f = 440 * 2^((n - 69)/12), where f is the
    frequency in Hz, and n is the MIDI note
    number.

  - TunerHandle and MaybeOwningTunerHandle:
    Handles for managing access to a SurgeTuner
    instance.

  - KeyboardMapping and Scale: These structures
    define custom keyboard mappings and musical
    scales for the synthesizer, allowing for
    alternative tuning systems and microtonal
    music.

  - n2p and n2pinv: Functions to convert between
    MIDI note numbers and pitch in a custom tuning
    system. The conversion between note numbers
    and pitch can be represented by a mapping
    function M(n), where n is the MIDI note number
    and M(n) is the pitch in a custom tuning
    system.

  - note_to_omega and pitch2omega: Functions for
    converting between pitch and angular frequency
    (omega). The angular frequency can be
    calculated as ω = 2 * π * f, where ω is the
    angular frequency and f is the frequency in
    Hz.

  - retune_to_scale and
    retune_to_standard_tuning: Functions for
    retuning the synthesizer to a custom scale or
    the standard 12-tone equal temperament.

  - remap_to_keyboard and
    remap_to_standard_keyboard: Functions for
    remapping the keyboard layout according to
    a custom keyboard mapping or the standard
    keyboard layout.

  - SurgeTuner: The main struct for managing
    tuning and keyboard mapping within the Surge
    Synthesizer system.

  - TuningTables: A struct that contains
    precomputed tables for tuning conversions.

 - SurgeTuning: A high-level wrapper around the
   SurgeTuner, providing a simplified interface
   for managing tuning within the Surge
   Synthesizer system.

By implementing the Surge-Tuning crate, the Surge
Synthesizer system gains a powerful and flexible
set of tools for working with custom tuning
systems and keyboard mappings, enabling a wide
range of musical possibilities beyond the standard
12-tone equal temperament.

User:

could you teach me more about custom tunings in
the context of electronic music?

Assistant:

Certainly! Custom tunings, also known as
alternative tunings or microtonal tunings, are
tuning systems that differ from the standard
12-tone equal temperament (12-TET) commonly used
in Western music. In electronic music, custom
tunings can be used to explore a variety of
musical expressions, including exotic scales,
unique harmonic relationships, and alternative
musical structures.

Electronic music provides a fertile ground for
experimenting with custom tunings because
synthesizers and digital audio workstations (DAWs)
offer a high degree of control over pitch and
tuning. Here are some key concepts related to
custom tunings in electronic music:

 - Microtonal scales: Microtonal scales are
   scales that use intervals smaller than
   a semitone, which is the smallest interval in
   12-TET. These scales can feature any number of
   notes per octave and can be used to create
   unique harmonic and melodic
   structures. Examples include the 19-tone equal
   temperament, 31-tone equal temperament, and the
   Bohlen-Pierce scale.

 - Just intonation: Just intonation is a tuning
   system based on whole number ratios, which
   creates pure-sounding intervals and
   harmonies. It is often used in electronic music
   to create lush and consonant chords or explore
   alternative harmonic relationships. Just
   intonation can be achieved by tuning the
   intervals between notes using simple frequency
   ratios, such as 3:2 for a perfect fifth or 5:4
   for a major third.

 - Non-octave scales: Most Western music is
   based on the octave, where pitches are repeated
   at double or half the frequency. However,
   custom tunings can also explore non-octave
   scales, where the interval between the lowest
   and highest notes is not an octave. Examples
   include the Bohlen-Pierce scale, which uses
   a 3:1 frequency ratio instead of the 2:1 octave
   ratio.

 - Dynamic tuning: In electronic music, custom
   tunings can be applied dynamically, allowing
   for real-time modulation of the tuning
   system. This can be used for expressive
   effects, such as gliding between different
   tuning systems, or creating evolving harmonic
   landscapes.

To work with custom tunings in electronic music,
you'll need synthesizers or software that support
alternative tuning systems. Some popular
synthesizers and DAWs that support custom tunings
include Surge, Serum, Kontakt, Ableton Live, and
Logic Pro. These tools typically allow you to
import or create custom tuning files, such as
Scala files (.scl), to define your desired tuning
system. You can then assign these tunings to your
instruments and explore the creative possibilities
of alternative tuning systems in your music.

User:

thanks! could you teach me some more about the
various tunings we know about? how many of them
are there? what are their use cases? how are they
structured mathematically?

Assistant:

There are countless tuning systems, as they can be
created and customized according to different
criteria, mathematical relationships, or cultural
contexts. Here, I'll describe some of the more
well-known tunings, their use cases, and their
mathematical structure.

 - 12-tone equal temperament (12-TET): This is
   the most common tuning system in Western
   music. It divides the octave into 12 equal
   parts, with each note's frequency being
   a multiple of the twelfth root of 2 (2^(1/12))
   times the frequency of the previous note. It
   allows for easy transposition and modulation
   between keys, but some intervals are not
   perfectly consonant due to the approximations
   required for equal division.

 - Just intonation: As mentioned earlier, just
   intonation is based on whole number ratios
   between frequencies, such as 3:2 (perfect
   fifth) and 5:4 (major third). The use of simple
   ratios results in pure-sounding intervals and
   harmonies. It is often used in early music,
   Indian classical music, and contemporary
   compositions that explore alternative harmonic
   relationships.

 - Meantone temperament: This tuning system is
   a compromise between just intonation and equal
   temperament. It aims to provide more accurate
   approximations of pure intervals, particularly
   the major third (5:4 ratio). Meantone
   temperament is used in various historical
   keyboard instruments like harpsichords and
   organs, and can be found in music from the
   Renaissance and Baroque periods.

 - Pythagorean tuning: Based on the work of the
   ancient Greek mathematician Pythagoras, this
   tuning system uses the 3:2 ratio (perfect
   fifth) to generate all other pitches. While it
   results in pure-sounding perfect fifths, it
   creates less consonant thirds, making it more
   suitable for certain types of music like
   monophonic chants and modal melodies.

 - 19-tone equal temperament (19-TET): This system
   divides the octave into 19 equal parts,
   resulting in smaller intervals than 12-TET. It
   provides a closer approximation to just
   intonation intervals and allows for more
   complex and exotic scales. The 19-TET has been
   used in some experimental and contemporary
   music compositions.

 - 31-tone equal temperament (31-TET): Similar to
   19-TET, this system divides the octave into 31
   equal parts. It provides even better
   approximations to just intonation intervals and
   allows for a greater variety of microtonal
   scales. Some composers, like Adriaan Fokker,
   have explored the possibilities of 31-TET in
   their work.

 - Bohlen-Pierce scale: As mentioned earlier, this
   is a non-octave scale based on a 3:1 frequency
   ratio instead of the 2:1 octave ratio. It
   divides the "tritave" (3:1 ratio) into 13 equal
   parts and has been used in some experimental
   and electronic music compositions.

These are just a few examples of the numerous
tuning systems that exist. The use cases and
mathematical structures vary, depending on the
goals and aesthetics of the composers or musicians
using them. The exploration of alternative tunings
can open up new sonic possibilities and enrich the
musical experience for both creators and
listeners.

User:

how do these tunings change the possible scales
within each tuning? am i correct that a tuning is
a higher level of abstraction than a scale?

Assistant:

Yes, you are correct that a tuning is a higher
level of abstraction than a scale. A tuning system
defines the relationships between the pitches
within an octave or other interval, while a scale
is a specific selection of pitches from that
tuning system, usually ordered by ascending or
descending pitch.

Different tuning systems can lead to different
possible scales within each tuning. For instance,
in the 12-tone equal temperament (12-TET) system,
there are many different scales, such as the major
scale, natural minor scale, and various modes like
Dorian, Mixolydian, and Phrygian. These scales all
share the same underlying tuning system but have
different patterns of whole and half steps.

When you switch to a different tuning system, like
just intonation, the scales within that tuning
will have different interval structures and
relationships between pitches compared to those in
the 12-TET system. This can lead to new scales
with unique sonic qualities.

For example, in just intonation, you might create
a scale based on the harmonic series, where the
frequencies of the pitches are integer multiples
of a fundamental frequency.

This would result in different interval sizes than
those found in the 12-TET system, leading to
a distinct harmonic and melodic character.

In microtonal tuning systems, such as 19-TET or
31-TET, the scales can be even more diverse, as
the smaller intervals allow for a greater variety
of pitch combinations. This can lead to new and
exotic scales that aren't possible within the
12-TET system.

In summary, the tuning system sets the foundation
for the possible scales within each
tuning. Changing the tuning system will change the
possible scales and their properties, opening up
new musical possibilities and textures.

2> could you describe for me 5 different scales
for each tuning?

Certainly! Here's a brief description of five
different scales for each of the following tuning
systems: 12-TET (12-tone equal temperament), just
intonation, 19-TET, and 31-TET. Please note that
some of these scales might not be as common or
well-known as others, especially in microtonal
tuning systems.

**12-TET:**

1. Major Scale: The most common scale in Western
   music, consisting of whole, whole, half, whole,
   whole, whole, half steps (e.g., C, D, E, F, G,
   A, B, C).

2. Natural Minor Scale: A scale that has a minor
   quality, consisting of whole, half, whole,
   whole, half, whole, whole steps (e.g., A, B, C,
   D, E, F, G, A).

3. Dorian Mode: A minor scale with a raised sixth
   degree, consisting of whole, half, whole,
   whole, whole, half, whole steps (e.g., D, E, F,
   G, A, B, C, D).

4. Mixolydian Mode: A major scale with a lowered
   seventh degree, consisting of whole, whole,
   half, whole, whole, half, whole steps (e.g., G,
   A, B, C, D, E, F, G).

5. Pentatonic Scale: A five-note scale, often used
   in various musical traditions around the world,
   consisting of the major scale without the
   fourth and seventh degrees (e.g., C, D, E, G,
   A, C).

**Just Intonation:**

1. Pythagorean Diatonic Scale: A scale based on
   the Pythagorean tuning system, which uses a 3:2
   perfect fifth as the basis for generating other
   pitches (e.g., C, D, E, F, G, A, B, C).

2. Ptolemy's Intense Diatonic Scale: A just
   intonation scale that uses the 5-limit tuning,
   creating pure major thirds (e.g., C, D, E, F,
   G, A, B, C).

3. Harmonic Series Scale: A scale based on the
   natural harmonic series, with pitches as
   integer multiples of the fundamental frequency.

4. 5-Limit Just Major Pentatonic: A five-note
   scale in just intonation, similar to the 12-TET
   major pentatonic scale, but with pure intervals
   (e.g., C, D, E, G, A, C).

5. Partch's 11-Limit Tonality Diamond: A scale
   based on the tuning system created by Harry
   Partch, which uses 11-limit just intonation to
   generate pitches.

**19-TET:**

1. 19-TET Major Scale: A major scale in 19-TET
   tuning, with slightly different interval sizes
   than the 12-TET major scale (e.g., C, D, E, F,
   G, A, B, C).

2. 19-TET Harmonic Minor Scale: A harmonic minor
   scale in 19-TET tuning, featuring a raised
   seventh degree (e.g., A, B, C, D, E, F, G#, A).

3. 19-TET Double Harmonic Scale: A scale in 19-TET
   tuning that features augmented seconds and
   a raised seventh degree (e.g., C, Db, E, F, G,
   Ab, B, C).

4. 19-TET Diminished Scale: A symmetrical scale in
   19-TET tuning that alternates whole and half
   steps (e.g., C, D, Eb, F, Gb, Ab, A, B, C).

3> thanks! which tunings are considered the most
useful for electronic music?

In electronic music, the most commonly used tuning
is the 12-tone equal temperament (12-TET) system,
as it is the standard tuning system for most
modern synthesizers, MIDI controllers, and digital
audio workstations (DAWs). However, electronic
musicians who want to explore alternative tuning
systems or incorporate microtonality in their work
may find the following tunings particularly
useful:

1. Just Intonation: This tuning system is based on
   simple whole number ratios and provides purer,
   more consonant intervals. It can be used to
   create more harmonically rich electronic music,
   especially when working with drones or ambient
   textures.

2. 19-TET (19-tone equal temperament): This tuning
   system divides the octave into 19 equal steps,
   offering a closer approximation of just
   intonation than 12-TET while still being
   relatively accessible. It can be used to create
   more exotic and unique harmonic progressions
   and melodic lines.

3. 31-TET (31-tone equal temperament): With 31
   equal divisions of the octave, this tuning
   system provides an even better approximation of
   just intonation and offers a wide range of
   microtonal intervals. It can be useful for
   electronic musicians who want to delve deeper
   into microtonal music.

4. 24-TET (quarter-tone system): This tuning
   system divides the octave into 24 equal parts,
   effectively halving the step size of 12-TET. It
   offers a range of quarter-tone intervals, which
   can be used to create unusual and innovative
   harmonies and melodies.

5. Bohlen–Pierce tuning: This tuning system is
   based on a 3:1 frequency ratio instead of the
   traditional 2:1 octave. It can be used to
   create electronic music with a distinct and
   unconventional harmonic structure.

These tuning systems are by no means exhaustive,
and there are many more options to explore. The
choice of tuning system ultimately depends on the
musician's artistic goals and the specific sound
they are trying to achieve.

4> could you show me as many others as you can?

There are numerous tuning systems and scales that
have been developed throughout history and across
cultures. Here are some additional examples:

1. Pythagorean tuning: This ancient Greek tuning
   system is based on the pure 3:2 ratio (perfect
   fifth) and 2:1 ratio (octave). It can create
   interesting harmonies and melodic structures,
   especially for electronic music with a focus on
   drones or modal melodies.

2. Meantone temperament: This historical tuning
   system attempts to achieve pure 5:4 major
   thirds. It was used in Western music from the
   Renaissance to the Baroque period and offers
   a distinctive and expressive harmonic
   character.

3. 22-TET (22-tone equal temperament): This tuning
   system divides the octave into 22 equal steps,
   offering a unique set of intervals and
   harmonies. It is used in some Indian classical
   music traditions and can be employed in
   electronic music to create exotic sounds.

4. 17-TET (17-tone equal temperament): With 17
   equal divisions of the octave, this tuning
   system provides an interesting set of
   microtonal intervals and harmonies. It can be
   used for electronic music that seeks to explore
   unusual melodic and harmonic possibilities.

5. 53-TET (53-tone equal temperament): This highly
   precise tuning system closely approximates just
   intonation and offers a large number of
   microtonal intervals. It can be used in
   electronic music that aims for extreme
   precision and a wide variety of tonal colors.

6. Wendy Carlos' Alpha, Beta, and Gamma scales:
   These scales are based on non-octave-repeating
   scales with unique interval structures. They
   offer unconventional harmonic possibilities and
   can be used for electronic music that seeks to
   break away from traditional tonal systems.

7. Hexany: A six-note scale derived from the
   harmonic series, based on the work of American
   composer and music theorist Harry Partch. It
   can be used for electronic music that
   incorporates just intonation and alternative
   harmonies.

8. Pelog and Slendro: Traditional Indonesian
   gamelan scales that offer unique and
   non-Western interval structures. They can be
   used for electronic music that incorporates
   world music elements and non-Western harmony.

These are just a few examples of the many tuning
systems and scales available to electronic
musicians. The choice of tuning system and scale
ultimately depends on the desired musical effect
and the specific sounds the composer or producer
wishes to explore.

5> thanks! from an engineering perspective, how do
we design a tool which can handle the wide variety
of tunings we have available to us?

To design a tool that can handle a wide variety of
tunings, you'll want to make sure it's flexible,
modular, and user-friendly. Here are some key
considerations for building such a tool:

1. **Flexible data structures**: Use data
   structures that can store and manipulate
   various tunings and scales. For example,
   a tuning could be represented as an array or
   list of frequency ratios or cent values, and
   a scale could be a list of indices or steps
   within a tuning.

2. **Tuning and scale import/export**: Provide
   import and export functionality for common
   tuning and scale file formats like Scala
   (.scl), TUN (.tun), and MIDI Tuning Standard
   (.mts). This will enable users to work with
   a wide variety of pre-existing tunings and
   scales, as well as share their custom tunings
   with others.

3. **User interface**: Create a user interface
   that allows users to easily select, modify, and
   create tunings and scales. This may include
   options for selecting from a list of presets,
   entering frequency ratios or cent values
   manually, or importing and exporting custom
   tunings and scales.

4. **Real-time pitch adjustment**: Implement
   real-time pitch adjustment algorithms that can
   retune the output of your tool according to the
   selected tuning and scale. This can be done
   using various techniques, such as
   pitch-shifting, granular synthesis, or
   wavetable synthesis.

5. **MIDI integration**: Provide MIDI integration
   so that the tool can be used with a wide range
   of hardware and software synthesizers,
   samplers, and sequencers. This may involve
   sending MIDI messages for pitch bend or
   implementing the MIDI Tuning Standard.

6. **Modularity**: Design the tool in a modular
   way, so that it can be easily integrated with
   other audio processing tools, DAWs, or
   synthesizers. This can be achieved by
   developing the tool as a plugin (VST, AU, AAX,
   etc.) or using a modular programming
   environment like Max/MSP, Pure Data, or
   Reaktor.

7. **Documentation and tutorials**: Provide clear
   documentation and tutorials for users to
   understand the capabilities of the tool and how
   to use it effectively. This may include
   information on the available tunings and
   scales, how to create custom tunings, and how
   to integrate the tool with other software and
   hardware.

By following these guidelines, you can develop
a tool that supports a wide variety of tunings and
scales, opening up new possibilities for
electronic musicians and composers to explore
alternative tuning systems and create unique sonic
landscapes.

6> what about for an individual tuning? how do we
design it from a mathematical/programming
perspective?

Designing an individual tuning from a mathematical
and programming perspective involves defining the
frequency ratios or cent values that determine the
relationships between the pitches in the tuning
system. Here are some steps to guide you through
this process:

1. **Choose the number of notes per octave**:
   Decide how many notes you want to have in your
   tuning system within an octave. For example,
   you might choose 12 notes for an equal
   temperament system, or a different number for
   alternative tuning systems like 19-tone equal
   temperament or just intonation-based systems.

2. **Define the frequency ratios or cent values**:
   Determine the frequency ratios or cent values
   that define the relationships between the
   pitches in your tuning system. For equal
   temperament systems, you can calculate the
   frequency ratios using the following formula:

   - Frequency ratio = 2^(n/N), where 'n' is the
     note index (0 to N-1) and 'N' is the number
     of notes per octave.

   Alternatively, you can define the relationships
   in cents, where 1 cent represents 1/1200 of an
   octave. In this case, the cent value for each
   note can be calculated as:

   - Cent value = 1200 * (n/N), where 'n' is the
     note index (0 to N-1) and 'N' is the number
     of notes per octave.

   For just intonation or other non-equal
   temperament systems, you'll need to define the
   frequency ratios or cent values based on the
   specific harmonic relationships you want to
   achieve.

3. **Create a data structure to store the
   tuning**: Store the frequency ratios or cent
   values in a data structure, such as an array or
   list. This will allow you to easily access and
   manipulate the tuning data in your program.

4. **Implement pitch conversion functions**:
   Develop functions that can convert between note
   numbers (e.g., MIDI note numbers) and
   frequencies, based on the tuning system you've
   defined. These functions should take into
   account the reference frequency (e.g., A4 = 440
   Hz) and the frequency ratios or cent values
   you've defined for your tuning system. For
   example:

   - Note number to frequency: Given a note
     number, calculate its frequency using the
     reference frequency and the tuning data
     (frequency ratios or cent values).

   - Frequency to note number: Given a frequency,
     calculate the closest note number using the
     reference frequency and the tuning data.

5. **Handle octave transpositions**: Implement
   functions that can handle octave
   transpositions, making sure to preserve the
   relationships between pitches as defined by
   your tuning system. This can be done by
   multiplying or dividing the frequency ratios by
   2, or adding or subtracting 1200 cents for each
   octave transposed.

By following these steps, you can design and
implement an individual tuning system from
a mathematical and programming perspective,
providing a solid foundation for creating music
that uses alternative tuning systems.

7> how do we make sure our tuning corresponds to
the audio samples we have selected for our song?

To ensure that your tuning corresponds to the
audio samples you've selected for your song,
you'll need to retune the samples according to the
tuning system you've chosen. Here are the steps to
follow:

1. **Determine the original pitch of each
   sample**: Identify the original pitch or note
   of each audio sample. You may find this
   information in the sample's metadata, or you
   might need to analyze the sample using a pitch
   detection algorithm.

2. **Implement pitch-shifting**: Develop or
   integrate a pitch-shifting algorithm into your
   program that allows you to change the pitch of
   the audio samples without affecting their
   duration. There are several pitch-shifting
   techniques available, such as granular
   synthesis, phase vocoder, or the SOLA
   (Synchronous Overlap and Add) method.

3. **Calculate the target pitch**: For each audio
   sample, determine the target pitch based on the
   original pitch and the desired tuning
   system. Use the pitch conversion functions
   you've implemented for the tuning system (e.g.,
   note number to frequency) to calculate the
   target frequency.

4. **Retune the samples**: Apply the
   pitch-shifting algorithm to each audio sample,
   adjusting its pitch to match the target
   frequency determined in the previous step. This
   will ensure that the sample's pitch aligns with
   the tuning system you've chosen for your song.

5. **Verify the tuning**: Check that the retuned
   samples correspond to the desired tuning by
   analyzing their pitches after the
   pitch-shifting process. This can be done using
   a pitch detection algorithm or by comparing the
   retuned samples' frequencies to the target
   frequencies.

6. **Integrate the retuned samples**: Use the
   retuned audio samples in your song or audio
   project, making sure to preserve the
   relationships between pitches as defined by
   your tuning system. When playing back the
   samples, the playback engine should take into
   account the tuning system and any additional
   pitch adjustments (e.g., MIDI note events) to
   ensure the correct pitches are played.

By following these steps, you can make sure your
chosen tuning system corresponds to the audio
samples you've selected for your song, creating
a coherent and harmonically consistent musical
piece.

8> could you show me how to do each of these steps
in detail? is there a way to do the whole process
automatically?
