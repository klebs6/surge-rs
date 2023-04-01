# surge-timeunit

A Rust crate for managing time and tempo
synchronization in the Surge synthesizer system.

`surge-timeunit` is a subcomponent of the Surge
synthesizer system, designed to provide an
efficient and flexible way to handle time and
tempo synchronization for audio processing. It
offers a simple API for managing time-related
variables and updates, making it easy to integrate
into your audio processing pipeline.

## Overview

The crate provides the following main components:

- `TimeUnit`: A struct representing a time unit
  with methods for managing and updating its
  state.

- `TimeUnitHandle`: A handle to a `TimeUnit`
  instance, allowing for easy manipulation and
  access to the time unit's state.

The crate also exposes various functions and
methods related to time management and tempo
synchronization:

- `init`: Initializes the time unit with default
  values.

- `ppq_pos`: Retrieves the current PPQ (Pulses Per
  Quarter note) position.

- `set_songpos`: Sets the song position in the
  time unit.

- `set_temposyncratio`: Sets the tempo
  synchronization ratio.

- `set_temposyncratio_inv`: Sets the inverse tempo
  synchronization ratio.

- `songpos`: Retrieves the current song position.

- `tempo`: Retrieves the current tempo value.

- `temposyncratio`: Retrieves the tempo
  synchronization ratio.

- `temposyncratio_inv`: Retrieves the inverse
  tempo synchronization ratio.

- `update`: Updates the time unit with new values.

## Mathematical Concepts

The crate revolves around the concept of time and
tempo synchronization in audio processing. It
deals with the following mathematical ideas:

- **PPQ (Pulses Per Quarter note)**: A unit used
  to represent musical time in terms of the number
  of pulses (or ticks) per quarter note. It is
  used to precisely control the timing and
  synchronization of audio events.

  Equation: `PPQ = (Time Signature / Tempo) * Resolution`

- **Tempo Synchronization Ratio**: A ratio used to
  synchronize the time unit to an external tempo
  reference. This ratio is the quotient of the
  external tempo and the internal tempo of the
  time unit.

  Equation: `Tempo Sync Ratio = External Tempo / Internal Tempo`

- **Inverse Tempo Synchronization Ratio**: The
  reciprocal of the tempo synchronization
  ratio. It is used to convert time values from
  the external tempo reference to the internal
  tempo of the time unit.

  Equation: `Inverse Tempo Sync Ratio = 1 / Tempo Sync Ratio`

These mathematical concepts are used throughout
the crate to manage and update the time and tempo
synchronization state of the `TimeUnit`. ```

8> what do we need to consider when we work with
`time` in the context of an audio program?

When working with `time` in the context of an
audio program, there are several factors and
concepts to consider to ensure accurate and
efficient processing:

1. **Sampling rate**: The sampling rate defines
   the number of samples per second for digital
   audio. It directly affects the audio quality
   and the maximum frequency that can be
   accurately represented. You need to handle the
   sampling rate properly when working with
   time-dependent audio processes, such as delays
   or time-varying filters.

2. **Buffer size**: Audio programs typically
   process audio in small chunks or buffers. The
   buffer size determines the number of audio
   samples processed at a time, which affects the
   program's latency and efficiency. When working
   with time, it's important to consider the
   buffer size to ensure accurate timing and
   synchronization.

3. **Timing accuracy**: In audio programs, precise
   timing is crucial for maintaining
   synchronization between audio events, such as
   notes or control changes. You should use
   high-resolution timing sources, like sample
   counters or high-resolution timers, to ensure
   accurate timing.

4. **Tempo and time signature**: In music, tempo
   and time signature define the pace and
   structure of the music. When working with time
   in an audio program, it's essential to take
   these factors into account, especially for
   time-based effects like sequencers or
   arpeggiators.

5. **Time-based effects**: Time-based audio
   effects, such as delays, reverbs, or
   modulators, require precise timing control to
   produce the desired results. When working with
   time, you should ensure that these effects are
   updated and processed correctly based on the
   current time and tempo.

6. **Synchronization**: In some cases, you may
   need to synchronize your audio program with
   external sources, such as MIDI clocks or other
   audio applications. This may require
   implementing mechanisms to synchronize your
   program's internal time with external
   references accurately.

7. **Latency**: Audio latency is the time it takes
   for an audio signal to travel through the audio
   processing chain from input to output. When
   working with time in an audio program, it's
   important to consider and minimize latency to
   maintain accurate timing and a responsive user
   experience.

8. **Time units**: Time can be represented in
   various units, such as samples, milliseconds,
   or musical units like beats or bars. You should
   choose the appropriate time units for your
   application and ensure that all time-related
   calculations and conversions are accurate.

By carefully considering these factors and
concepts, you can ensure that your audio program
handles time accurately and efficiently, providing
a high-quality and responsive user experience.

9> can you show me the ins and outs of working
with time?

Working with time in an audio program involves
various aspects, from basic time calculations to
advanced synchronization and latency
management. Here's an overview of the ins and outs
of working with time in audio programming:

1. **Time units**: Choose the appropriate time
   units for your application. Common time units
   in audio programming include:

   - Samples: Discrete audio data points in
     a digital audio stream.

   - Milliseconds: A common unit for time-based
     effects like delay or reverb.

   - Beats or bars: Musical units that correspond
     to the tempo and time signature.

2. **Time conversions**: Converting between time
   units is a common task in audio
   programming. Make sure you handle these
   conversions correctly:

   - To convert between samples and milliseconds,
     use the sampling rate: 
     `ms = (samples / sampling_rate) * 1000` 
     and `samples = (ms / 1000) * sampling_rate`.

   - To convert between musical units (beats or
     bars) and milliseconds, use the tempo (in
     beats per minute): 
     `ms = (beats / tempo) * 60000` 
     and `beats = (ms / 60000) * tempo`.

3. **Timing accuracy**: Use high-resolution timing
   sources, such as sample counters or
   high-resolution timers, to ensure accurate
   timing. This is crucial for maintaining
   synchronization and producing precise audio
   effects.

4. **Buffer size**: Be aware of the buffer size
   when processing audio. Time-based audio effects
   may need to be updated and processed on
   a per-buffer basis to maintain accurate timing
   and synchronization.

5. **Tempo and time signature**: Take tempo and
   time signature into account when working with
   time-dependent audio processes. This is
   essential for musical applications like
   sequencers or arpeggiators. Update time-based
   effects and audio parameters accordingly as the
   tempo or time signature changes.

6. **Synchronization**: Implement mechanisms for
   synchronizing your audio program with external
   sources, such as MIDI clocks or other audio
   applications. This may involve sending and
   receiving timing information or adjusting your
   program's internal time to match external
   references.

7. **Latency management**: Minimize audio latency
   by optimizing your audio processing chain,
   choosing appropriate buffer sizes, and using
   efficient algorithms. Be aware of the latency
   introduced by your program and compensate for
   it when necessary, such as by delaying audio
   events or aligning synchronization points.

8. **Time-based effects**: When implementing
   time-based audio effects, ensure they are
   updated and processed correctly based on the
   current time and tempo. This may involve
   interpolating between audio samples, modulating
   effect parameters, or adjusting buffer lengths.

By understanding and applying these concepts, you
can effectively work with time in audio
programming, creating accurate, synchronized, and
responsive audio applications.

10> /file surge-rs/surge-timeunit/src/unit.rs

The `TimeUnit` struct you've provided is used to
manage time-related information in an audio
program, specifically in the context of the Surge
synthesizer system. The struct contains various
fields related to time, tempo, time signature, and
sample rate. Let's dive into the details of this
implementation:

1. **Fields**:

   - `ppq_pos`: Position in pulses per quarter
     note (PPQ). This represents the current
     position in the musical timeline and is used
     to sync various events, like notes or
     automation.

   - `tempo`: The current tempo in beats per
     minute (BPM). This value is essential for
     time-based effects and synchronization with
     other music applications.

   - `time_sig_numerator` and
     `time_sig_denominator`: The current time
     signature as a numerator (top number) and
     denominator (bottom number) pair, e.g., 4/4
     or 3/4.

   - `temposyncratio`: A value representing the
     ratio of the current tempo to the reference
     tempo of 120 BPM. This is useful for scaling
     time-dependent parameters or calculations.

   - `temposyncratio_inv`: The inverse of
     `temposyncratio`. It represents the ratio of
     the reference tempo of 120 BPM to the current
     tempo.

   - `songpos`: The current position in the song
     timeline, usually measured in beats or bars.

   - `srunit`: A `SampleRateHandle` that stores
     the current sample rate information.

2. **Methods**:

   - `new`: Constructs a new `TimeUnit` instance,
     initializing its fields with default values.

   - `init`: Initializes the `TimeUnit` instance
     with default values. It is called within the
     `new` method.

   - `update`: Updates the internal values of the
     `TimeUnit` instance based on the current
     state of its fields, such as the `ppq_pos`
     and `tempo`.

The `TimeUnitHandle` struct is a wrapper around
the `TimeUnit` struct, providing methods to access
and modify the `TimeUnit` instance while managing
its state using `Rc<RefCell<TimeUnit>>`.

1. **Methods**:

   - `new`: Constructs a new `TimeUnitHandle`
     instance.

   - Various `pub fn` methods: Provide access to
     the `TimeUnit` fields through the
     `TimeUnitHandle`, allowing you to get or set
     values such as the `temposyncratio`, `tempo`,
     and `songpos`.

   - `update`: Calls the `update` method on the
     inner `TimeUnit` instance.

By using the `TimeUnit` and `TimeUnitHandle`
structs, you can effectively manage time-related
information in your audio program, making it
easier to synchronize events, handle tempo
changes, and work with time-based effects.
