# surge-scene

The `SurgeScene` struct represents a single voice
scene in the Surge synthesizer. It contains
a collection of oscillators, filters, envelopes,
LFOs, modulation routings, modulation sources,
step sequencers, and various effects units that
are used to create the sound of the scene. The
struct also contains references to handles for
various time-related units, MIDI and MPE input
units, a tuner, a synth input, and a collection of
voices.

The `SurgeScene` struct has functions for playing
and releasing notes, handling parameter changes,
and managing the state of the scene. These
functions interact with the various components of
the scene to create and modify its sound output.

# surge-patch

A `SurgePatch` is a collection of settings and
parameters that define the sound of
a synthesizer. It consists of a `metadata` struct
that stores information about the patch, such as
its name, author, and description. The `params`
array contains the values of all the patch
parameters, which can be adjusted to modify the
sound.

A `SurgePatch` also contains one or more
`SurgeScene` structs, which define the
configuration for individual sounds or voices
within the patch. Each scene contains settings for
oscillators, filters, envelopes, and modulation
sources that are used to create a particular
sound.

In addition to scenes, a `SurgePatch` may also
contain one or more `SurgeEffect` structs, which
define effects such as reverb or delay that can be
applied to the sound.

The patch may also include optional data for
tuning, keymapping, and pitch bend range, as well
as a flag indicating whether the patch is
configured for MPE (MIDI Polyphonic Expression)
mode.

114> what is a patch in the context of a software
synthesizer? how could we improve on its design?

In the context of a software synthesizer, a patch
represents a set of parameters that define the
sound of the synthesizer. It includes things like
oscillator settings, filter settings, envelope
settings, modulation sources and destinations, and
so on.

The design of a patch can be improved in a few
ways:

1. User interface: The way patches are presented
   to the user can have a big impact on
   usability. An intuitive and easy-to-use
   interface can make it easier for musicians to
   create and save their own patches.

2. Parameter organization: The way parameters are
   organized within a patch can also affect
   usability. Grouping related parameters together
   and using clear and consistent names can make
   it easier to understand and tweak a patch.

3. Patch sharing: Making it easy for users to
   share and download patches can be a great way
   to build a community around a synthesizer. This
   can be done through online forums, preset
   libraries, or built-in patch sharing features
   within the software itself.

4. Integration with hardware: If the synthesizer
   is designed to work with external hardware,
   integrating patch management with that hardware
   can be a big help. For example, allowing users
   to save patches directly to a connected
   hardware synth can be more convenient than
   having to transfer them manually.

Overall, a well-designed patch system can be a key
factor in the success of a software synthesizer,
both in terms of usability and community
engagement.

115> could you provide  more detailed information
on each of these points?

Sure, here are more details on each of the points:

1. **Complexity:** A patch can have a lot of
   parameters, some of which may not be relevant
   to the user's needs. This can make it difficult
   for the user to navigate and tweak the patch to
   their liking. A more streamlined approach could
   be to group the parameters into categories or
   sections that make sense for the type of sound
   the patch is meant to create. For example, an
   "envelope" section that includes attack, decay,
   sustain, and release parameters, or a "filter"
   section that includes cutoff, resonance, and
   filter type parameters.

2. **Ease of use:** Related to the complexity
   issue, some patches may be difficult to use for
   inexperienced users. Providing helpful
   documentation or tooltips can go a long way in
   improving the user experience. Additionally,
   having a user-friendly interface that visually
   represents the parameters being adjusted can
   make it easier for the user to understand the
   impact of their changes.

3. **Flexibility:** Patches may be designed with
   a specific sound or use case in mind, which can
   limit their usefulness in other
   contexts. Allowing for more flexibility in
   parameter ranges or adding more modularity to
   the patch design can increase its versatility
   and usefulness in different musical contexts.

4. **Naming conventions:** Patches may have
   complex or cryptic names that make it difficult
   for users to understand what they are meant to
   do. Clear and descriptive naming conventions
   can make it easier for users to find the
   patches that suit their needs.

5. **Compatibility:** Patches may not be
   compatible with different hardware or software
   environments, which can limit their usefulness
   to certain users. Ensuring that patches are
   compatible with a range of systems and
   environments can increase their reach and
   accessibility to a wider audience.

116> cool, thanks -- and in the context of
a synthesizer, what is a scene?

In the context of a synthesizer, a scene is
a collection of settings and parameters that
define a specific sound or sound design. It
typically includes a combination of oscillators,
filters, modulation sources, and effects, as well
as other settings such as envelope parameters and
routing.

Scenes are often used to create presets or
starting points for sound design, and can be
edited and adjusted to create unique sounds. They
can also be used to quickly switch between
different sound setups during performance, without
needing to manually adjust each individual
setting.

In some synthesizers, scenes may also include
settings for controlling multiple voices or
patches at once, such as assigning a common LFO or
filter cutoff to all voices. Scenes may be
organized into banks or categories, making it
easier to find and switch between different
sounds.

117> so, it is something like the instantiation of
a set of synthesizer components.  is that correct?

Yes, that's correct. A scene is essentially
a snapshot of a particular configuration of
synthesizer components, including oscillators,
filters, envelopes, LFOs, and modulation
sources. Each scene represents a specific sound or
instrument that can be played and controlled by
the user.

118> why do we only use two?

It's likely that the number of sends in the
`SurgeScene` is limited to two for performance and
complexity reasons. Each send requires additional
processing resources, so adding more sends would
increase the processing load and potentially
introduce latency. Additionally, having a fixed
number of sends simplifies the routing and
processing logic, making it easier to implement
and maintain. Finally, in practice, two sends may
be sufficient for many use cases, particularly
when combined with the other processing
capabilities of the synthesizer.

119> interesting -- why not just elide this layer
of abstraction and deal entirely in the
subcomponents?

Using scenes allows for a higher level of
abstraction that can be useful when working with
complex patches that involve many different
components. Instead of dealing with individual
oscillators, filters, and envelopes separately,
a scene encapsulates all of these elements into
a single entity that can be saved, loaded, and
manipulated as a single unit.

Furthermore, a scene can help organize the
components in a meaningful way that can aid in
sound design. For example, a scene could represent
a particular sound, such as a piano or a guitar,
and all of the relevant components needed to
create that sound could be grouped together within
the scene. This can make it easier for a sound
designer to create and manipulate complex sounds.

Overall, using scenes can help simplify the
process of sound design and patch creation, and
provide a more intuitive way of organizing and
manipulating synthesizer components.

120> how do scenes and patches interact?

Scenes and patches are related in that a patch is
made up of one or more scenes. In other words,
a patch is a collection of scenes that can be
played together. Each scene within a patch can be
thought of as a separate "layer" of the patch,
with its own set of settings and parameters.

When a patch is loaded into the synthesizer, it
typically contains multiple scenes that are
organized in a specific way to create a particular
sound. The user can then select which scene(s) to
play at a given time, allowing them to create
complex sounds by layering multiple scenes
together.

So, patches provide a higher-level organizational
structure that allows the user to combine and
manipulate multiple scenes, while scenes provide
the individual components that make up the sound
of the patch.
