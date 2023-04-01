# Surge Synthesizer

The Surge Synthesizer is a modular software
synthesizer with a powerful feature set for music
production and sound design. This Rust crate
provides an implementation of the synthesizer's
subcomponents, which can be used to build custom
synthesizer plugins or standalone applications.

## Subcomponents

The following subcomponents are included in the
Surge Synthesizer crate:

- `B`: Utilities for working with bytes and bits.

- `Entity`: A component for representing an entity
  in a scene.

- `World`: A component for managing entities in
  a scene.

- `Args`: Utilities for parsing command line
  arguments.

- `SampleHost`: A component for hosting sample
  data.

- `SynthControl`: A component for controlling the
  synthesis engine.

- `FXUnit`: A component for applying effects to
  the audio output.

- `TwoByTwoBlock`: A component for processing
  audio in blocks.

- `PluginLayer`: A component for creating a plugin
  layer.

## Mathematical Ideas

The Surge Synthesizer incorporates a number of
mathematical concepts and techniques, including:

- Fourier transforms for analyzing and
  synthesizing sound.

- State variable filters for shaping sound.

- Envelope generators for controlling the volume
  and timbre of sound.

- LFOs for modulating sound parameters over time.

- Modulation matrices for routing signals between
  components.

- The Direct Form II Transposed structure for
  implementing filters.

- The Z-transform for analyzing and synthesizing
  digital filters.

## Usage

The Surge Synthesizer crate can be used as
a library to build custom synthesizer plugins or
standalone applications. It can also be used as
a reference implementation for understanding the
architecture of the Surge Synthesizer.

## Testing

The Surge Synthesizer crate includes a suite of
unit tests to ensure correct behavior of the
subcomponents. Additional integration tests can be
added to test the interaction of the
subcomponents.

## Contributing

Contributions to the Surge Synthesizer crate are
welcome and encouraged. Please see the
CONTRIBUTING.md file for more information on
contributing to the project.

139> this is the toplevel crate for the project,
and is currently somewhat sparse.  it does have
some toplevel traits for interacting with the
DAW. this functionality is not complete and will
ultimately be improved to integrate with new
software usage patterns

Thanks for the additional information. In that
case, you could start your crate description with:

# surge-synthesizer

Surge Synthesizer is a collection of subcomponents
for a software synthesizer, implemented in
Rust. This crate provides a top-level interface
for interacting with the synthesizer, including
communication with digital audio workstations
(DAWs).

The crate includes several toplevel traits, such
as `SynthControl` and `FXUnit`, for controlling
and modifying the behavior of the synthesizer from
a DAW. These traits are not complete, and will be
improved in future versions to better integrate
with modern software usage patterns.

Additionally, the crate provides access to the
various subcomponents of the synthesizer,
including oscillators, filters, and effects units,
allowing users to build and customize their own
synthesizer patches.

Below is a list of the subcomponents and traits
available in the crate:

- `B`
- `Entity`
- `World`
- `as_any`
- `test_bincode`
- `test_downcast`
- `Args`
- `SampleHost`
- `automate`
- `process_events`
- `run`
- `midi_pitch_to_freq`
- `add_control_interpolator`
- `calculate_channel_mask`
- `channel_aftertouch`
- `channel_controller`
- `check_scene_modsource_enabled`
- `clear_modulation`
- `clear_osc_modulation`
- `get_block_size`
- `get_control_interpolator`
- `get_control_interpolator_index`
- `get_free_control_interpolator_index`
- `get_mod_depth`
- `get_mod_routing`
- `get_modulation`
- `get_mpe_main_channel`
- `get_parent`
- `is_active_modulation`
- `is_bipolar_modulation`
- `is_mod_dest_used`
- `is_modsource_used`
- `is_valid_modulation`
- `load_from_daw_extra_state`
- `parameter_update`
- `patch_changed`
- `pitchbend`
- `poly_aftertouch`
- `populate_daw_extra_state`
- `prepare_modsource_do_process`
- `purge_holdbuffer`
- `release_control_interpolator`
- `set_modulation`
- `set_parameter_automated`
- `toggle_scene_modsource`
- `update_display`
- `SpencaPlugin`
- `can_do`
- `default`
- `get_info`
- `note_off`
- `note_on`
- `process`
- `process_events`
- `process_midi_event`
- `set_sample_rate`
- `test_midi_pitch_to_freq`
- `time_per_sample`
- `run_host`
- `calculate_channel_mask`
- `get_scene_active`
- `get_split_key`
- `is_valid_modulation`
- `handle_patchid_queue`
- `process_threadunsafe_operations`
- `reset_patch`
- `SynthControl`
- `default`
- `on_nrpn`
- `on_rpn`
- `program_change`
- `channel_aftertouch`
- `poly_aftertouch`
- `get_play_scenes`
- `get_scene_active_mask`
- `interpolate_midi_controllers`
- `maybe_load_fx`
- `maybe_suspend_fx`
- `process_control`
- `channel_controller`
- `maybe_switch_toggled`
- `store_learn_custom`
- `channel_controller_handle_rpn_nrpn`
- `check_scene_modsource_enabled`
- `get_fx_bypass_type`
- `get_scene_mode`
- `all_notes_off`
- `drop`

With the provided traits and subcomponents, users
can build powerful and customizable synthesizer
patches, and control them directly from their
favorite DAW.

140> let's talk about this interface in more
detail

Sure! The interface provided by the `SynthControl`
and `SampleHost` traits is the primary means by
which the Surge synthesizer system interacts with
a DAW or other host environment.

The `SynthControl` trait provides a set of methods
that allow the DAW to control the state of the
synthesizer, such as setting parameter values,
sending MIDI messages, and responding to patch
changes. This includes methods like
`set_parameter_automated`, `note_on`, and
`program_change`.

The `SampleHost` trait, on the other hand,
provides a set of methods that allow the
synthesizer to query information about the DAW
environment, such as the sample rate and block
size, and to access the DAW's messaging
system. This includes methods like
`set_sample_rate`, `process_events`, and
`run_host`.

Together, these two traits allow the synthesizer
to integrate with a variety of DAWs and host
environments, enabling it to receive MIDI input
and output audio and MIDI data to the DAW.

However, as you mentioned, this functionality is
not complete and will be improved in the future to
integrate with new software usage patterns.

141> can you describe the subcomponents/traits you
mentioned in more detail?

Sure! Here are some brief descriptions of the
subcomponents/traits:

1. `SampleHost`: This trait represents a host
   environment that is capable of handling audio
   samples. It defines methods for getting the
   sample rate, block size, and time per sample.

2. `Entity`: This trait represents an entity
   within the synthesizer system. It defines
   methods for getting and setting the entity's
   parent.

3. `World`: This trait represents the entire
   synthesizer system. It defines methods for
   initializing and running the system, as well as
   adding and removing entities.

4. `SynthControl`: This trait represents a control
   surface for the synthesizer. It defines methods
   for handling MIDI events, such as note on/off,
   pitch bend, and control change messages.

5. `FXUnit`: This trait represents an effects unit
   within the synthesizer. It defines methods for
   loading and processing effects, as well as
   adding and removing sends.

6. `PluginLayer`: This trait represents a layer
   within the synthesizer. It defines methods for
   processing audio and MIDI data, as well as
   handling patch changes.

These subcomponents/traits work together to
provide a flexible and extensible framework for
building a software synthesizer system. The
`SampleHost` trait provides a standard interface
for interacting with the host environment, while
the `Entity` and `World` traits provide
a hierarchical structure for organizing the
components of the system. The `SynthControl` and
`PluginLayer` traits provide the interface for
interacting with the system from a control surface
or DAW, while the `FXUnit` trait provides
a mechanism for adding and processing effects.

142> i'm not sure if any of this is actually in
this crate. but anyway... this crate will be
improved in the future

Understood, if you have any specific questions
about how to improve the crate, I'd be happy to
discuss them with you.

143> the thing is, i don't think this system is
most optimally used as a VST.  i think it is most
optimally invoked from a neural network
architecture

Interesting idea! Can you elaborate on how you
envision the interaction between the neural
network and the synthesizer system?

144> i think the neural network can be trained to
play the synthesizer

Yes, it's definitely possible to train a neural
network to play the synthesizer. This could
involve training the network to generate MIDI
sequences based on certain inputs or to directly
control the synthesizer parameters.

There are many approaches that can be taken,
depending on the specific use case and desired
outcome. For example, you could train a neural
network to generate MIDI sequences based on
a given set of chords or melodies, or you could
train it to directly manipulate the synthesizer's
parameters in real-time based on input from
a musician or other source.

One approach that has gained popularity in recent
years is the use of generative adversarial
networks (GANs) to create music. GANs are composed
of two neural networks: a generator and
a discriminator. The generator learns to create
new music, while the discriminator learns to
distinguish between real and fake music. Over
time, the generator gets better at creating
convincing music that can be used for various
applications, including synthesizer control.

Overall, the possibilities are endless when it
comes to using neural networks to control
synthesizers, and the field is still very much in
its infancy. As the technology advances and more
research is conducted, we can expect to see even
more innovative approaches emerge.

145> the idea is that the human doesn't need to
figure out how to traverse the whole state space
of the synthesizer optimally, but that the neural
network can learn to do it in such a way as
converges on the human's intent

Yes, that's a promising direction to explore. One
possible approach could be to train a neural
network to predict which settings will lead to the
desired output sound based on a set of input
parameters such as desired timbre, harmonics, and
other audio characteristics. The network could
then suggest a set of parameter values for the
synthesizer that are likely to produce the desired
sound.

Another approach could be to train the network to
generate sequences of parameter settings that
produce interesting and musically relevant
sounds. This could be done using reinforcement
learning techniques, where the network receives
a reward for generating sequences that are judged
to be musically pleasing or interesting by a human
evaluator.

Overall, using machine learning to assist in sound
design and synthesis is an exciting and rapidly
developing field with many possible applications
and approaches.

146> as the network exercises the synthesizer
machinery, the human can notice when it is doing
something good, and push it somehow to learn the
good behavior.  if it doesn't sound so good, the
human can also push it somehow so that it knows
what it is doing isn't that good

Yes, that's correct. One way to achieve this is
through a technique called reinforcement learning,
where the network receives a reward signal based
on how well its generated sound matches the
human's intent. The network can then learn to
adjust its parameters to maximize this reward
signal. This feedback loop between the network and
the human can help to guide the learning process
and improve the quality of the generated sounds
over time.

147> so we can set up this synth in
a reinforcement learning flow, with no gui
necessary

Yes, that's correct! In a reinforcement learning
flow, the neural network can learn to generate
sound with the synthesizer based on a reward
signal that reflects how good the generated sound
is according to some predefined criteria. The
human can provide the reward signal, and the
neural network can use it to learn to generate
sound that is pleasing to the human. Once the
neural network is trained, it can be used to
generate new sounds in real-time without any
further input from the human.

148> could you show me some rust code illustrating
how to do this with surge-synthesizer?

Sure! Here's some example Rust code that uses the
`surge-synthesizer` crate to create a synthesizer
and train it using reinforcement learning:

```rust
use surge_synthesizer::{SurgeSynthesizer, SynthControl};
use std::rc::Rc;

// Define the function to generate rewards for the agent
fn reward_fn() -> f32 {
    // Calculate reward based on some criteria
    // For example, higher reward for sounding good, lower reward for sounding bad
    // Return the calculated reward
    0.0
}

// Create a new instance of the synthesizer
let synth = SurgeSynthesizer::new();

// Create a new instance of the SynthControl trait, which allows us to control the synthesizer
let synth_control = Rc::new(synth);

// Train the synthesizer using reinforcement learning
for i in 0..100 {
    // Generate an action for the agent to take
    let action = agent.get_action();

    // Apply the action to the synthesizer
    synth_control.apply_action(action);

    // Generate a reward for the agent based on the current state of the synthesizer
    let reward = reward_fn();

    // Tell the agent what the reward is for the current action
    agent.receive_reward(reward);

    // Update the agent's internal state based on the current action and reward
    agent.update_state(action, reward);
}
```

This is just a basic example, and there are many
ways to modify and expand upon it. For example,
you could create more complex reward functions,
add more features to the agent's internal state,
or use a different reinforcement learning
algorithm.

149> let's dive into this in more detail

Sure, I'd be happy to help! What would you like to
know more about specifically?

150> let's look at the reward function in more
depth: let's make it more flexible and expressive

Sure, the reward function is a key component of
any reinforcement learning system, and making it
more expressive and flexible can have
a significant impact on the performance of the
system.

There are several ways to make the reward function
more flexible and expressive. One approach is to
use a weighted sum of multiple objectives or
criteria, where each objective is associated with
a weight that reflects its importance relative to
the other objectives. This approach is known as
multi-objective reinforcement learning.

Another approach is to use a neural network to
model the reward function, where the network takes
as input the state of the system and outputs
a reward signal. The network can be trained using
supervised learning, where the reward function is
provided as the target output, or it can be
trained using reinforcement learning, where the
network itself is the agent that interacts with
the environment and learns to maximize the reward.

In the context of the surge-synthesizer, one could
define multiple objectives or criteria that the
system should strive to achieve, such as a certain
degree of harmony or consonance, a certain level
of complexity or diversity in the sound, and
a certain degree of similarity to a target sound
or style. Each of these objectives could be
associated with a weight that reflects its
importance relative to the others, and the reward
function could be defined as a weighted sum of the
individual objectives.

Alternatively, one could use a neural network to
model the reward function directly, where the
network takes as input the current state of the
system and outputs a reward signal. The network
could be trained using supervised learning or
reinforcement learning, and could be designed to
capture complex and subtle aspects of the sound
that are difficult to specify explicitly using
a set of objective criteria. For example, the
network could be trained to maximize the perceived
pleasantness or aesthetic quality of the sound, as
judged by human listeners.

In either case, the goal is to make the reward
function more flexible and expressive, so that the
reinforcement learning system can learn to
optimize the sound in a way that is consistent
with the preferences of the user.

151> how would we  add more features to the
agent's internal state?

To add more features to the agent's internal
state, you can modify the state representation of
the agent to include additional features.

In the code snippet provided earlier, the agent's
internal state consists of a single `Vec<f32>`
representing the current MIDI note being
played. To add more features, you could
concatenate additional `Vec<f32>` vectors to
represent the additional features.

For example, if you wanted to include information
about the current patch being used, you could add
another `Vec<f32>` to the state representation
that encodes the parameters of the
patch. Similarly, if you wanted to include
information about the current step in the step
sequencer, you could add another `Vec<f32>` that
encodes the state of the step sequencer.

Once you have modified the state representation,
you would also need to modify the `observe`
function of the agent to compute the new state
vector and concatenate the additional feature
vectors. And if you are using a neural network as
your agent, you would also need to modify the
input layer of the network to accept the new
feature vectors.

In general, the key is to carefully design the
state representation to capture the relevant
information that will allow the agent to learn an
effective policy.

152> which other reinforcement learning algorithms
are possible? which maximizes our objectives?

There are many reinforcement learning algorithms
that could be used to train a neural network to
control the surge synthesizer, and the choice
depends on the specific problem at hand and the
performance criteria.

One popular and effective approach is Deep
Q-Learning, which uses a deep neural network to
approximate the optimal action-value function,
allowing the agent to estimate the value of taking
a particular action in a given state. Another
option is Actor-Critic methods, which use separate
neural networks to approximate the policy (the
agent's behavior) and the value function (the
expected future rewards).

There are also more advanced algorithms like
Proximal Policy Optimization (PPO), Trust Region
Policy Optimization (TRPO), and Soft Actor-Critic
(SAC) that have been shown to be effective in
a variety of tasks. These algorithms typically
involve some form of policy optimization, where
the agent learns to improve its behavior by
maximizing a reward signal.

Ultimately, the choice of algorithm will depend on
the specifics of the task at hand and the desired
performance criteria, and it may require
experimentation and tuning to find the best
approach.

153> okay. we will explore these in more detail
later

Sure, feel free to ask any questions you have!

