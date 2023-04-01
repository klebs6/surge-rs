## surge-voice

A Rust crate for the Surge Synthesizer
system. This crate provides various types and
functions related to voice processing and
modulation in the Surge Synthesizer.

### Math Concepts

- `calc_levels`: This token represents the
  calculation of modulation levels for
  a voice. Modulation levels are often used in
  audio synthesis to control the amount of
  modulation applied to a signal. The
  `calc_levels` function can be used to calculate
  modulation levels for a voice.

- `legato`: This token represents the legato mode
  for a voice. Legato mode is a playing technique
  in which a note is played smoothly and without
  interruption. The `legato` type can be used to
  represent the legato mode for a voice.

- `update_portamento`: This token represents the
  update of portamento parameters for
  a voice. Portamento is a musical effect in which
  the pitch of a note slides up or down to
  a target pitch. The `update_portamento` function
  can be used to update portamento parameters for
  a voice.

- `OscillatorRuntime`: This token represents the
  runtime data for an oscillator. In audio
  synthesis, an oscillator generates a periodic
  waveform, such as a sine wave or a square
  wave. The `OscillatorRuntime` type can be used
  to represent the runtime data for an oscillator.

- `gen_process_cfg`: This token represents the
  generation of process configuration data for
  a voice. Process configuration data is often
  used in audio synthesis to configure the
  processing of audio signals. The
  `gen_process_cfg` function can be used to
  generate process configuration data for a voice.

- `gen_oscillator_runtime`: This token represents
  the generation of runtime data for an
  oscillator. Runtime data is often used in audio
  synthesis to store the state of a processing
  block between processing blocks. The
  `gen_oscillator_runtime` function can be used to
  generate runtime data for an oscillator.

- `FilterBlockData`: This token represents the
  data for a filter block. In audio synthesis,
  a filter is used to modify the frequency content
  of an audio signal. The `FilterBlockData` type
  can be used to represent the data for a filter
  block.

- `FilterBlockState`: This token represents the
  state of a filter block. The `FilterBlockState`
  type can be used to represent the state of
  a filter block.

- `VoiceUpdateQFCSCfg`: This token represents the
  configuration data for updating a voice's Quad
  Filter Block State. Quad Filter Block State is
  used in audio synthesis to store the state of
  a quad filter block between processing
  blocks. The `VoiceUpdateQFCSCfg` type can be
  used to represent the configuration data for
  updating a voice's Quad Filter Block State.

- `set_quad_filterblock`: This token represents
  the function to set the Quad Filter Block for
  a voice. Quad Filter Block is used in audio
  synthesis to store the state of a quad filter
  block between processing blocks. The
  `set_quad_filterblock` function can be used to
  set the Quad Filter Block for a voice.

- `get_temposyncratio`: This token represents the
  calculation of tempo sync ratio for
  a voice. Tempo sync ratio is often used in audio
  synthesis to synchronize audio effects with the
  tempo of a song. The `get_temposyncratio`
  function can be used to calculate the tempo sync
  ratio for a voice.

- `release`: This token represents the release
  phase of an audio envelope. In audio synthesis,
  an envelope is used to control the amplitude or
  frequency of an audio signal over time. The
  `release` type can be used to represent the
  release phase of an audio envelope.

- `uber_release`: This token represents the uber
  release phase of an audio envelope. The
  `uber_release` type can be used to represent the
  uber release phase of an audio envelope.

- `calc_pan`: This token represents the
  calculation of pan position for a voice. Pan
  position is used in audio synthesis to position
  an audio signal in the stereo field. The
  `calc_pan` function can be used to calculate the
  pan position for a voice.

- `SyncQFBRegistersCfg`: This token represents the
  configuration data for synchronizing the
  registers of a Quad Filter Block. Quad Filter
  Block is used in audio synthesis to modify the
  frequency content of an audio signal. The
  `SyncQFBRegistersCfg` type can be used to
  represent the configuration data for
  synchronizing the registers of a Quad Filter
  Block.

- `sync_registers_from_qfb`: This token represents
  the function to synchronize the registers of
  a Quad Filter Block. The
  `sync_registers_from_qfb` function can be used
  to synchronize the registers of a Quad Filter
  Block.

- `process_ring`: This token represents the
  processing of a ring modulation effect. Ring
  modulation is used in audio synthesis to create
  complex audio effects by multiplying two audio
  signals together. The `process_ring` function
  can be used to process a ring modulation effect.

- `create_voice_modsources`: This token represents
  the creation of modulation sources for
  a voice. Modulation sources are often used in
  audio synthesis to modify the behavior of audio
  processing blocks. The `create_voice_modsources`
  function can be used to create modulation
  sources for a voice.

- `create_voice_oscillators`: This token
  represents the creation of oscillators for
  a voice. Oscillators generate periodic
  waveforms, such as sine waves or square waves,
  which are used as audio signals in audio
  synthesis. The `create_voice_oscillators`
  function can be used to create oscillators for
  a voice.

- `create_voice_osclevels`: This token represents
  the creation of oscillator levels for
  a voice. Oscillator levels are often used in
  audio synthesis to control the amount of
  modulation applied to an oscillator. The
  `create_voice_osclevels` function can be used to
  create oscillator levels for a voice.

- `calc_routes`: This token represents the
  calculation of modulation routes for
  a voice. Modulation routes are used in audio
  synthesis to route modulation sources to control
  parameters. The `calc_routes` function can be
  used to calculate modulation routes for a voice.

- `switch_toggled`: This token represents the
  toggling of a switch parameter. Switch
  parameters can be used to turn on or off certain
  audio processing blocks or features. The
  `switch_toggled` function can be used to toggle
  a switch parameter.

- `maybe_toggle_filter`: This token represents the
  toggling of a filter parameter. The
  `maybe_toggle_filter` function can be used to
  toggle a filter parameter.

- `VoiceToggleSoloCfg`: This token represents the
  configuration data for toggling the solo mode
  for a voice. Solo mode is often used in audio
  mixing to isolate a single track or
  instrument. The `VoiceToggleSoloCfg` type can be
  used to represent the configuration data for
  toggling the solo mode for a voice.

- `maybe_toggle_solo`: This token represents the
  toggling of the solo mode for a voice. The
  `maybe_toggle_solo` function can be used to
  toggle the solo mode for a voice.

- `set_path`: This token represents the setting of
  a voice's signal path. A signal path is a series
  of audio processing blocks connected in
  series. The `set_path` function can be used to
  set a voice's signal path.

- `solo`: This token represents the solo mode for
  a voice. Solo mode is often used in audio mixing
  to isolate a single track or instrument. The
  `solo` type can be used to represent the solo
  mode for a voice.

- `maybe_calc_poly_aftertouch`: This token
  represents the calculation of polyphonic
  aftertouch data for a voice. Polyphonic
  aftertouch is often used in electronic

5> thanks! could you teach me more about voices?
how do they work?

Sure! In audio synthesis, a voice is a unit of
sound that is generated by an individual
synthesizer. A synthesizer can produce multiple
voices simultaneously, each with its own pitch,
timbre, and other characteristics.

A voice in a synthesizer typically consists of
several components, including one or more
oscillators, a filter, and an envelope
generator. The oscillators generate the basic
waveform for the voice, while the filter modifies
the frequency content of the waveform. The
envelope generator shapes the amplitude and/or
frequency of the sound over time.

In addition to these basic components, a voice in
a synthesizer can also include modulation sources,
such as LFOs or envelopes, which can be used to
modulate various parameters of the voice,
including the pitch, amplitude, and filter cutoff.

When a note is played on a synthesizer, the
synthesizer assigns a free voice to the note, and
the assigned voice generates the sound for the
note. If all of the available voices are currently
in use, the synthesizer will either steal a voice
from an existing note or simply ignore the new
note.

Voices in a synthesizer can be polyphonic or
monophonic. In polyphonic mode, the synthesizer
can play multiple notes simultaneously, with each
note assigned to its own voice. In monophonic
mode, the synthesizer can only play a single note
at a time, and all notes are assigned to the same
voice.

Voices in a synthesizer can also be modified in
real-time using various performance techniques,
such as pitch bending, vibrato, and modulation
wheel. These techniques can be used to create
expressive and dynamic performances on the
synthesizer.

6> how much memory do they need? how many of them
do we need to get a decent synthesizer?

The amount of memory required for a voice in
a synthesizer can vary depending on the complexity
of the voice and the quality of the sound. In
general, a voice in a software synthesizer can
consume anywhere from a few kilobytes to several
megabytes of memory, depending on the complexity
of the voice and the quality of the sound.

The number of voices required for a decent
synthesizer can also vary depending on the
specific needs of the user. In general,
a polyphonic synthesizer will require more voices
than a monophonic synthesizer. The number of
voices required can also depend on the type of
music being produced, as some genres may require
more complex and layered sounds than others.

As an example, a simple monophonic synthesizer
with one oscillator, a filter, and an envelope
generator can be created using just a few
kilobytes of memory, and can produce a wide range
of basic sounds. A more complex polyphonic
synthesizer with multiple oscillators, filters,
and modulation sources can require several
megabytes of memory per voice, and may require
dozens of voices to create more complex and
layered sounds.

Ultimately, the number of voices required for
a synthesizer will depend on the specific needs of
the user and the complexity of the sounds being
produced.

7> are there alternative techniques we can use to
achieve the same effect?

Yes, there are alternative techniques that can be
used to achieve similar effects as traditional
synthesis techniques, and these techniques can
often be more memory-efficient and computationally
efficient.

One such alternative technique is sample-based
synthesis, which involves using pre-recorded audio
samples as the building blocks for creating
sounds, rather than generating sounds from scratch
using synthesis techniques. Sample-based synthesis
can be more memory-efficient than traditional
synthesis, as it relies on pre-recorded samples
that can be reused across multiple sounds.

Another alternative technique is physical modeling
synthesis, which involves simulating the physics
of real-world instruments to create
sounds. Physical modeling synthesis can be
computationally efficient, as it does not require
complex mathematical calculations to generate
sounds from scratch.

Granular synthesis is another alternative
technique that involves breaking up audio samples
into small "grains" and reassembling them in
different ways to create new sounds. This
technique can be memory-efficient, as it allows
for a large number of sounds to be created using
a relatively small amount of memory.

Lastly, wavetable synthesis involves using
pre-recorded waveforms, called wavetables, to
create sounds. This technique can be more
memory-efficient than traditional synthesis, as it
relies on a small set of pre-recorded waveforms
that can be reused across multiple sounds.

Each of these alternative techniques has its own
strengths and weaknesses, and the choice of
technique will depend on the specific needs of the
user and the characteristics of the sounds being
produced.
