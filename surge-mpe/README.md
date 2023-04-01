# Surge MPE Crate Description

## MPEUnit

The `MPEUnit` is a subcomponent of the Surge
synthesizer system that implements the MIDI
Polyphonic Expression (MPE) protocol. The MPE
protocol allows for expressive control of
individual notes in a polyphonic context, enabling
features such as per-note pitch bend and per-note
pressure sensitivity.

The `MPEUnit` includes methods to initialize the
unit, get and set the MPE main channel, get and
set the enabled state, get and set the global
pitch bend range, and get the default values for
various MPE parameters.

## MPEUnitHandle

The `MPEUnitHandle` is a handle to an instance of
the `MPEUnit`, allowing for convenient access to
its methods.

## Mathematical Analysis

The MPE protocol involves a number of mathematical
concepts, such as pitch bend and pressure
sensitivity. Here are some relevant equations:

### Pitch Bend

Pitch bend is a technique used to change the pitch
of a note in a continuous manner. In the MPE
protocol, pitch bend can be applied per-note. The
pitch bend value is represented as a 14-bit signed
integer, where a value of 0 represents no pitch
bend, a value of 8192 represents a pitch bend of
1 semitone up, and a value of -8191 represents
a pitch bend of 1 semitone down.

The pitch bend value can be converted to a pitch
ratio using the following equation:

```
ratio = 2 ^ (bend / 8192 / 12)
```

where `bend` is the pitch bend value.

### Pressure Sensitivity

Pressure sensitivity is a technique used to
control various parameters based on the pressure
applied to a note after it has been triggered. In
the MPE protocol, pressure sensitivity can be
applied per-note as polyphonic aftertouch. The
pressure value is represented as a 7-bit unsigned
integer, with a range of 0-127.

The pressure value can be scaled to a range
between 0 and 1 using the following equation:

```
value = pressure / 127
```

## Conclusion

The `Surge MPE` crate provides a set of tools for
implementing the MIDI Polyphonic Expression (MPE)
protocol in the Surge synthesizer system. The MPE
protocol allows for expressive control of
individual notes in a polyphonic context, and
includes mathematical concepts such as pitch bend
and pressure sensitivity. The `MPEUnit` and
`MPEUnitHandle` provide a convenient interface for
accessing MPE functionality within the Surge
synthesizer system.

25> thanks -- could you teach me the MPE protocol?

Sure, I can give you an overview of the MIDI
Polyphonic Expression (MPE) protocol.

MPE is an extension of the standard MIDI protocol
that enables per-note expression control in
a polyphonic context. The MPE protocol was
developed to address the limitations of
traditional MIDI in expressive electronic music
performance, specifically in the context of
polyphonic instruments such as keyboards.

In traditional MIDI, a single channel is used to
transmit data for all voices of a polyphonic
instrument, such as a keyboard. This means that
all notes on the keyboard share the same
expression controls, such as pitch bend and
modulation. This can limit the expressiveness of
a performance, as it is not possible to apply
expression control to individual notes.

In the MPE protocol, each note is assigned its own
MIDI channel and can be addressed and controlled
independently. This enables per-note expression
control, such as individual pitch bend and
pressure sensitivity. MPE uses the same MIDI
message format as traditional MIDI, with the
addition of a few new messages and parameters.

Here are some key features of the MPE protocol:

- **Per-note MIDI channels**: Each note is
  assigned its own MIDI channel, allowing for
  independent control of expression.

- **Per-note pitch bend**: Each note can have its
  own pitch bend value, enabling microtonal and
  glissando effects.

- **Per-note pressure sensitivity**: Each note can
  have its own pressure sensitivity value,
  allowing for independent control of timbre and
  dynamics.

- **Global pitch bend range**: The MPE protocol
  includes a global pitch bend range parameter
  that sets the maximum pitch bend range for all
  notes. This enables consistent pitch bend
  behavior across multiple instruments and
  controllers.

- **Polyphonic aftertouch**: The MPE protocol
  includes support for polyphonic aftertouch,
  which allows for independent pressure
  sensitivity for multiple notes at once.

- **Note on/off messages**: MPE uses the same note
  on/off messages as traditional MIDI, but with
  additional parameters for channel assignment and
  expression control.

- **MPE-compatible devices**: MPE requires devices
  that are specifically designed to support the
  protocol, including keyboards and
  synthesizers. Some software synthesizers also
  support MPE.

In summary, the MPE protocol enables per-note
expression control in a polyphonic context,
allowing for more expressive electronic music
performances. It achieves this by assigning each
note its own MIDI channel and including support
for per-note pitch bend, pressure sensitivity, and
other expression controls.

26> thanks -- what should we keep in mind while
implementing it?

Implementing the MPE protocol can be challenging,
as it requires careful consideration of a number
of technical details. Here are some things to keep
in mind while implementing the MPE protocol:

1. **Channel assignment**: Each note must be
   assigned to its own MIDI channel, and the total
   number of channels used must not exceed the
   maximum number supported by the target
   device. It is important to ensure that channel
   assignment is handled correctly to avoid
   conflicts and ensure proper expression control.

2. **Pitch bend range**: The MPE protocol includes
   a global pitch bend range parameter that sets
   the maximum pitch bend range for all notes. It
   is important to ensure that this parameter is
   set correctly to avoid unexpected pitch bend
   behavior.

3. **Pressure sensitivity**: The MPE protocol
   includes support for polyphonic aftertouch,
   which allows for independent pressure
   sensitivity for multiple notes at once. It is
   important to ensure that pressure sensitivity
   is implemented correctly to allow for
   expressive control of timbre and dynamics.

4. **Note on/off messages**: MPE uses the same
   note on/off messages as traditional MIDI, but
   with additional parameters for channel
   assignment and expression control. It is
   important to ensure that note on/off messages
   are handled correctly to enable proper
   expression control.

5. **MPE-compatible devices**: MPE requires
   devices that are specifically designed to
   support the protocol, including keyboards and
   synthesizers. It is important to ensure that
   all devices used in an MPE setup are compatible
   with the protocol to avoid unexpected behavior.

6. **Testing**: Implementing the MPE protocol can
   be complex, and it is important to thoroughly
   test an implementation to ensure that it is
   working correctly. This may involve testing
   with a variety of MPE-compatible devices and
   controllers.

In summary, implementing the MPE protocol requires
careful consideration of a number of technical
details. It is important to ensure that channel
assignment, pitch bend range, pressure
sensitivity, note on/off messages, and device
compatibility are all handled correctly to enable
expressive control of individual notes in
a polyphonic context. Thorough testing is also
important to ensure that an implementation is
working correctly.

27> any caveats?

Yes, there are some caveats to keep in mind when
implementing the MPE protocol:

1. **MIDI message bandwidth**: The MPE protocol
   uses more MIDI messages than traditional MIDI,
   as each note requires its own MIDI channel and
   associated expression control messages. This
   can increase the amount of MIDI data being
   transmitted, which may impact performance on
   slower or older MIDI interfaces.

2. **MPE-compatible devices**: While the MPE
   protocol is becoming increasingly common in new
   electronic music hardware and software, not all
   devices and controllers support the
   protocol. It is important to ensure that all
   devices and controllers used in an MPE setup
   are compatible with the protocol to avoid
   unexpected behavior.

3. **Per-note tuning**: The MPE protocol enables
   microtonal and glissando effects through
   per-note pitch bend, but this can also present
   challenges for tuning. It is important to
   ensure that tuning is handled correctly to
   avoid pitch drift and dissonance.

4. **Implementation complexity**: Implementing the
   MPE protocol can be complex, as it requires
   careful handling of channel assignment, pitch
   bend range, pressure sensitivity, and note
   on/off messages. This can be challenging to
   implement correctly, particularly in real-time
   performance contexts.

5. **Compatibility with traditional MIDI**: While
   the MPE protocol is designed to extend the
   functionality of traditional MIDI, it is not
   fully backward-compatible with traditional MIDI
   devices and controllers. It is important to
   ensure that MPE implementations can gracefully
   handle situations where traditional MIDI
   devices or controllers are used in an MPE
   setup.

In summary, implementing the MPE protocol can
present a number of challenges related to MIDI
message bandwidth, device compatibility, per-note
tuning, implementation complexity, and
compatibility with traditional MIDI. It is
important to carefully consider these caveats and
design an implementation that addresses them
appropriately.
