# surge-stepseq

## StepSequencer

The `StepSequencer` is a struct that represents
a step sequencer, which is a fundamental component
of any synthesizer system. The `StepSequencer`
struct contains several fields, including the
current step, loop start and end points, shuffle
settings, and trigger mask. The sequencer can be
mutated using various methods, such as `set_step`,
`set_loop`, `set_shuffle`, `set_trigmask`,
`mutate_sequence`, `random_sequence`,
`load_sequence`, `save_sequence`, `get_mute_mask`,
`mute_all`, `mute_group`, `mute_step`,
`unmute_all`, `unmute_group`, and
`unmute_step`. The sequencer can also be tested
using the `test_stepsequencer_default` and
`test_stepsequencer_setters` methods.

## Mathematics

A step sequencer can be mathematically represented
as a series of steps, each of which can have
a value or probability associated with it. The
probabilities of each step can be used to generate
random sequences or mutate existing ones. The
probability of a particular step `i` can be
represented using the following equation:

`P(i) = p_i`

where `p_i` is the probability of the `i`th step.

The trigger mask can also be mathematically
represented as a binary sequence, where each bit
represents whether or not a step is muted. The
probability of a step being triggered can be
calculated using the trigger mask and the current
step index.

The `apply_swing` method can also be used to apply
a swing rhythm to the sequencer by delaying the
playback of certain steps based on a swing factor.

## Usage

The `StepSequencer` struct can be used in any Rust
program that requires a step sequencer
component. It provides a flexible and powerful
interface for generating and manipulating
sequences of values or probabilities.

133> what else can we do to improve our step
sequencer?

There are many ways to improve a step sequencer,
and the specific improvements will depend on the
desired functionality and intended use cases. Here
are a few general ideas:

1. Add more control parameters: Depending on the
   use case, it might be useful to add additional
   parameters to control various aspects of the
   sequencer. For example, you could add
   a parameter to control the number of steps, or
   a parameter to specify the pattern length in
   beats.

2. Add more randomization options: A step
   sequencer can become more versatile if it
   offers a wide range of randomization options,
   such as randomizing step velocity, probability,
   and duration.

3. Add more advanced sequencing options: More
   advanced sequencing options such as
   polyrhythms, euclidean rhythms, or
   probabilistic rhythms can add depth and
   complexity to a step sequencer.

4. Add modulation options: You could add the
   ability to modulate various parameters of the
   sequencer, such as step probability or
   velocity, using an LFO or other modulation
   sources.

5. Add support for different time signatures:
   Depending on the intended use case, it may be
   useful to add support for different time
   signatures, such as 3/4 or 5/4 time.

6. Improve the interface: Improving the interface,
   such as adding visual feedback, can make the
   step sequencer easier and more intuitive to
   use.

7. Improve the timing accuracy: Depending on the
   intended use case, improving the timing
   accuracy of the step sequencer could be
   important. This could involve improving the
   resolution of the sequencer or implementing
   more precise timing mechanisms.

134> what else? i know you have some more ideas up
in there!

Sure, here are some more ideas:

1. Probability-based sequencing: Instead of having
   a fixed sequence of steps, we could introduce
   probabilities for each step, so that the
   sequencer can generate different patterns each
   time it's triggered. This could be especially
   useful for creating more organic and evolving
   sequences.

2. Pattern chaining and branching: We could allow
   users to chain multiple step sequences
   together, or create branches within a sequence,
   to create more complex patterns.

3. Parameter modulation: We could allow users to
   modulate parameters of the step sequencer, such
   as step duration, probability, or velocity,
   using LFOs or other modulation sources.

4. User-defined scales and chord progressions: We
   could allow users to define custom scales and
   chord progressions, so that the step sequencer
   can be synced to the harmonic progression of
   a track.

5. MIDI and CV output: We could add MIDI and CV
   output capabilities to the step sequencer, so
   that it can be used to control external gear.

6. Real-time recording and editing: We could allow
   users to record and edit step sequences in
   real-time, using a MIDI controller or other
   input device.

7. Advanced sequencing modes: We could add
   advanced sequencing modes, such as
   arpeggiators, polyrhythms, and
   probability-based triggers, to create even more
   complex and dynamic sequences.

135> pattern chaining and branching! that one is
pretty nice eh?

Yes, pattern chaining and branching can add a lot
of depth and complexity to a step sequencer. With
pattern chaining, you can create longer sequences
by linking together multiple shorter patterns,
while pattern branching allows you to create
alternate versions of a pattern that can be
triggered depending on certain conditions. Both of
these techniques can help you create more
interesting and varied musical sequences.

136> could you train a neural network to choose
interesting sequences?

It is certainly possible to train a neural network
to generate interesting step sequences. One
approach could be to use a generative adversarial
network (GAN) or a variational autoencoder (VAE)
to learn a representation of a set of step
sequences and then generate new sequences from
that learned representation. Another approach
could be to use a recurrent neural network (RNN)
to generate new sequences based on patterns
learned from a training set of sequences.

However, it's important to keep in mind that the
definition of "interesting" is subjective and
varies depending on the context and musical
style. Additionally, generating interesting
sequences requires a deep understanding of music
theory and structure, which may be difficult to
encode in a neural network. Therefore, it's likely
that any generated sequences would need to be
further refined and curated by a human musician to
ensure they fit the desired context and musical
style.

137> interesting -- what about something which can
adapt to the particular style of the musician who
is using the stepsequencer?

Yes, that is also possible. One approach is to use
a variant of a generative model called
a conditional generative model. In a conditional
generative model, the model learns to generate
data based on a given input condition. In the case
of a step sequencer, the input condition could be
the musician's playing style, such as the timing
and rhythm of their past note sequences. The
neural network could then be trained to generate
new note sequences that match the input style.

One specific type of conditional generative model
that could be used for this is a recurrent neural
network with an attention mechanism. The recurrent
neural network would be used to generate the note
sequence, while the attention mechanism would help
the network to focus on the musician's playing
style when generating the sequence.

Of course, training such a model would require
a large amount of high-quality training data that
includes the musician's playing style, and a lot
of computational power to train the
model. However, if done successfully, such a model
could potentially help musicians to quickly
generate interesting and personalized note
sequences that match their playing style.
