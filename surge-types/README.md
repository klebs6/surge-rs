## surge-types

A Rust crate for the Surge Synthesizer
system. This crate provides various types and
traits used throughout the Surge Synthesizer
codebase.

### Math Concepts

- `Mask32`: This token represents a 32-bit
  bitmask. Bitmasks are often used in computer
  programming to manipulate binary data. They are
  usually represented as a series of 1s and
  0s. The `Mask32` type can be used to perform
  bitwise operations on 32-bit data.

- `NumberOfBlocks`: This token represents the
  number of audio blocks in a given signal
  chain. In digital audio processing, audio
  signals are often split into blocks or frames,
  which are processed independently. The
  `NumberOfBlocks` type can be used to calculate
  the number of audio blocks needed for a given
  audio signal.

- `OutputDataPresent`: This token represents
  whether or not output data is present. In
  digital audio processing, output data refers to
  the audio signal that is sent to a speaker or
  other audio device. The `OutputDataPresent` type
  can be used to determine whether or not output
  data is present, which can be useful for
  managing audio signal flow.

- `PitchBendCfg`: This token represents the pitch
  bend configuration used in the
  synthesizer. Pitch bend is a musical effect that
  changes the pitch of a note over time. The
  `PitchBendCfg` type can be used to configure the
  pitch bend behavior of the synthesizer.

- `OscillatorOut`: This token represents the
  output of an oscillator. In audio synthesis, an
  oscillator generates a periodic waveform, such
  as a sine wave or a square wave. The
  `OscillatorOut` type can be used to represent
  the output of an oscillator, which can be used
  as an audio signal.

- `Ringout`: This token represents a ring
  modulator output. A ring modulator is a type of
  audio effect that multiplies two audio signals
  together. The `Ringout` type can be used to
  represent the output of a ring modulator.

- `FilterUnit`: This token represents a filter
  unit. In audio synthesis, a filter is used to
  modify the frequency content of an audio
  signal. The `FilterUnit` type can be used to
  represent a filter, which can be used to process
  audio signals.

- `Output`: This token represents the output of
  a signal chain. In audio processing, a signal
  chain is a series of audio processing blocks
  connected in series. The `Output` type can be
  used to represent the output of a signal chain.

- `control_group`: This token represents a group
  of control parameters. In audio synthesis,
  control parameters are used to modify the
  behavior of audio processing blocks. The
  `control_group` type can be used to group
  related control parameters together.

- `on`: This token represents whether a control
  parameter is enabled or not. Control parameters
  can often be enabled or disabled to modify the
  behavior of audio processing blocks. The `on`
  type can be used to represent the on/off state
  of a control parameter.

- `runtime_array`: This token represents
  a dynamically-sized array. In Rust, arrays are
  usually statically-sized, meaning that their
  size is determined at compile time. The
  `runtime_array` type can be used to represent
  arrays whose size is determined at runtime.

- `uses_wavetabledata`: This token represents
  whether or not a synthesizer block uses
  wavetable data. Wavetable synthesis is a type of
  audio synthesis that uses precomputed waveform
  samples. The `uses_wavetabledata` type can be
  used to determine whether or not a synthesizer
  block uses wavetable data.

- `blocks`: This token represents a series of
  audio processing blocks. In audio processing,
  a series of processing blocks can be connected
  in series to form a signal chain. The `blocks`
  type can be used to represent

2> thanks! could you keep going?

- `feedback_scale`: This token represents the
  feedback scale used in a signal processing
  block. Feedback is a technique used in audio
  processing to create complex audio effects. The
  `feedback_scale` type can be used to configure
  the amount of feedback used in a signal
  processing block.

- `control_type`: This token represents the type
  of a control parameter. Control parameters can
  have different types, such as integer or
  floating-point numbers. The `control_type` type
  can be used to represent the type of a control
  parameter.

- `default`: This token represents the default
  value of a control parameter. The `default` type
  can be used to represent the default value of
  a control parameter.

- `default_value`: This token represents the
  default value of a control parameter. The
  `default_value` type can be used to represent
  the default value of a control parameter.

- `index`: This token represents the index of an
  element in an array or collection. The `index`
  type can be used to represent the index of an
  element in an array or collection.

- `index_mut`: This token represents a mutable
  reference to an element in an array or
  collection. The `index_mut` type can be used to
  represent a mutable reference to an element in
  an array or collection.

- `is`: This token represents a boolean value
  indicating whether a control parameter is
  enabled or not. The `is` type can be used to
  represent the on/off state of a control
  parameter.

- `is_dual`: This token represents whether
  a control parameter has dual functionality. Dual
  functionality means that a control parameter can
  control two different parameters depending on
  its state. The `is_dual` type can be used to
  represent the dual functionality of a control
  parameter.

- `is_serial`: This token represents whether
  a signal processing block is in serial mode. In
  serial mode, audio signals are processed one
  after the other, in sequence. The `is_serial`
  type can be used to represent whether a signal
  processing block is in serial mode.

- `is_stereo`: This token represents whether
  a signal processing block processes stereo audio
  signals. Stereo audio signals consist of two
  audio channels, left and right. The `is_stereo`
  type can be used to represent whether a signal
  processing block processes stereo audio signals.

- `is_wide`: This token represents whether
  a control parameter has a wide range of
  values. Control parameters with a wide range of
  values can control a wider range of audio
  processing parameters. The `is_wide` type can be
  used to represent the wide range of a control
  parameter.

- `max_value`: This token represents the maximum
  value of a control parameter. The `max_value`
  type can be used to represent the maximum value
  of a control parameter.

- `min_value`: This token represents the minimum
  value of a control parameter. The `min_value`
  type can be used to represent the minimum value
  of a control parameter.

- `modulateable`: This token represents whether
  a control parameter is
  modulateable. Modulateable control parameters
  can be controlled by other audio signals, such
  as LFOs or envelopes. The `modulateable` type
  can be used to represent whether a control
  parameter is modulateable.

- `moverate`: This token represents the rate of
  change of a control parameter. The `moverate`
  type can be used to represent the rate of change
  of a control parameter.

- `switch`: This token represents a switch
  parameter. Switch parameters can be used to turn
  on or off certain audio processing blocks or
  features. The `switch` type can be used to
  represent a switch parameter.

- `value_type`: This token represents the type of
  a control parameter value. Control parameters
  can have different value types, such as integer
  or floating-point numbers. The `value_type` type
  can be used to represent the value type
