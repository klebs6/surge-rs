# `surge-param`

The `surge-param` crate provides a set of
parameter types and associated functions that can
be used for implementing parameterized synthesis
and processing algorithms. The main goal of this
crate is to enable the creation of flexible,
expressive, and customizable software synthesizers
and audio effects.

## `ParamType`

The `ParamType` trait defines a common interface
for parameters, which can be controlled and
manipulated in real-time by users or by other
components of the synthesizer. The trait provides
a variety of methods to get and set parameter
values, display parameter information, and more.

## `ControlStyle`

The `ControlStyle` enum describes the type of
control used to manipulate a parameter. This can
include knobs, sliders, buttons, or any other
input device that can be used to manipulate
a value.

## `ParamRT`

The `ParamRT` struct provides real-time support
for parameter updates and modifications. This
includes methods to handle parameter automation,
as well as functions to get and set parameter
values.

## `ParameterMeta`

The `ParameterMeta` struct defines metadata
associated with a parameter, such as the
parameter's name, description, and default value.

## `Error`

The `Error` enum contains a set of error types
that can occur when working with parameters.

## `Output`

The `Output` struct represents a parameter output,
which can be used to output parameter values to
other components of the synthesizer or to external
software.

## `PData`

The `PData` struct is used to hold parameter data.

## `ParamUserData`

The `ParamUserData` struct is used to store user
data associated with a parameter.

## `sub`

The `sub` module contains a set of functions and
types related to parameter manipulation and
processing.

## `try_into`

The `try_into` module provides a set of functions
to convert between different parameter types.

## `ControlGroupEntry`

The `ControlGroupEntry` struct defines a single
entry in a control group, which is a collection of
related parameters that can be controlled
together.

## `bound_value`

The `bound_value` function is used to limit
a value to a certain range.

## `limit_range`

The `limit_range` function is used to limit
a parameter's range to a specific range.

## `normalized2value`

The `normalized2value` function converts
a normalized parameter value to a real-world
value.

## `value2normalized`

The `value2normalized` function converts
a real-world parameter value to a normalized
value.

43> thanks -- what kinds of parameters do we have?

The `surge-param` crate defines several types of
parameters that can be used to manipulate the
sound produced by a synthesizer or other audio
processing system. Here are some of the main
parameter types:

- **Modulateable**: These are parameters that can
  be modulated by an LFO, envelope, or other
  modulation source. The `modulateable` flag
  indicates whether a parameter can be modulated
  or not.

- **Morph**: These are parameters that can be
  morphed between different values or
  settings. The `morph` and `morph_alt` functions
  allow for interpolation between different
  parameter values.

- **Snap**: These are parameters that can be
  snapped to a specific value or a set of discrete
  values. The `snap` function allows for snapping
  a parameter value to the nearest snap value.

- **Extend Range**: These are parameters that can
  have their range extended beyond their initial
  limits. The `extend_range` function allows for
  setting the new minimum and maximum values for
  the parameter.

- **Tempo Sync**: These are parameters that can be
  synced to a specific tempo or beat. The
  `can_temposync` flag indicates whether
  a parameter can be tempo synced or not, and the
  `tempo_sync_notation_value` function allows for
  getting the tempo-synced value of the parameter.

- **Control Type**: These are parameters that have
  a specific control type, such as a knob, fader,
  or button. The `control_type` function allows
  for getting the control type of a parameter.

- **Control Style**: These are parameters that
  have a specific control style, such as linear or
  logarithmic. The `control_style` function allows
  for getting the control style of a parameter.

- **Control Group**: These are parameters that
  belong to a specific control group, which can be
  used to organize parameters into logical
  groups. The `control_group` function allows for
  getting the control group of a parameter.

- **Affect Other Parameters**: These are
  parameters that can affect the value of other
  parameters. The `affect_other_parameters` flag
  indicates whether a parameter can affect other
  parameters or not.

44> what sorts of control styles can we have?

The `ControlStyle` enum in the `surge-param` crate
defines several possible control styles for
a parameter. These styles are:

- `Direct`: This control style allows the
  parameter to be set directly, without any
  additional processing or modulation. This is the
  simplest control style.

- `DirectWithModulation`: This control style
  allows the parameter to be modulated, but only
  by a fixed modulation source that is associated
  with the parameter.

- `DirectWithModulationMultiplier`: This control
  style is similar to `DirectWithModulation`, but
  allows the modulation amount to be scaled by
  a separate control.

- `Bipolar`: This control style allows the
  parameter to be set with a bipolar value,
  i.e. a value that can be positive or
  negative. This is useful for parameters that
  control a range of values centered around zero,
  such as a pan control.

- `BipolarWithModulation`: This control style is
  similar to `DirectWithModulation`, but allows
  for bipolar modulation.

- `BipolarWithModulationMultiplier`: This control
  style is similar to
  `DirectWithModulationMultiplier`, but allows for
  bipolar modulation.

- `TwoPole`: This control style allows the
  parameter to be set with a two-pole value,
  i.e. a value that can be positive or negative
  with a defined maximum absolute value. This is
  useful for parameters that control a range of
  values with a limited amplitude, such as
  a filter cutoff frequency.

- `TwoPoleWithModulation`: This control style is
  similar to `DirectWithModulation`, but allows
  for two-pole modulation.

- `TwoPoleWithModulationMultiplier`: This control
  style is similar to
  `DirectWithModulationMultiplier`, but allows for
  two-pole modulation.

- `Enum`: This control style allows the parameter
  to be set with a discrete set of values. This is
  useful for parameters that have a limited set of
  meaningful values, such as a waveform selector.

- `EnumWithModulation`: This control style is
  similar to `DirectWithModulation`, but allows
  for modulation of an enum parameter.

- `EnumWithModulationMultiplier`: This control
  style is similar to
  `DirectWithModulationMultiplier`, but allows for
  modulation of an enum parameter.

45> interesting -- thanks! what else should we
know about parameters in surge?

Here are some additional things to know about
parameters in Surge:

- Parameters can be used for a wide range of tasks
  beyond simply controlling audio synthesis,
  including controlling MIDI output, controlling
  visualizations, and more.

- Surge allows users to create custom parameter
  types using the `ParamType` enum, enabling
  fine-grained control over how parameters behave.

- Parameters in Surge can be modulated in
  real-time using a wide range of modulation
  sources, including LFOs, envelopes, and MIDI CC
  messages.

- The parameter system in Surge is designed to be
  very flexible and extensible, enabling users to
  easily create and manipulate complex parameter
  structures for use in their own projects.

- Surge also includes a number of built-in tools
  for working with parameters, including functions
  for setting parameter values smoothly and
  intelligently, and tools for synchronizing
  parameters to MIDI clock or other timing
  sources.

- Finally, Surge provides a powerful preset system
  that enables users to store and recall specific
  parameter settings, making it easy to save and
  reuse specific sound configurations across
  multiple projects.

46> what distinguishes runtime data for
a parameter and compile time data? what sorts of
things can we know about a parameter at compile
time? what can we know about a parameter that
isn't available until runtime?

In the context of software development, compile
time refers to the time during which the source
code is being compiled into machine code by
a compiler. In contrast, runtime refers to the
time during which the compiled machine code is
being executed by the computer.

In the context of Surge, there are certain aspects
of a parameter that can be determined at compile
time, and other aspects that are only known at
runtime.

At compile time, we can know things like the
parameter's data type, name, default value,
minimum and maximum values, and any control styles
that are associated with the parameter. These are
typically defined in the parameter's struct or
enum definition in the code.

At runtime, we can know things like the current
value of the parameter, whether it is being
modulated by any modulation sources, and any MIDI
or OSC messages that are being received that
affect the parameter. These aspects of the
parameter are determined by the state of the
software and the user's interaction with it.

In general, compile time data is static and fixed,
while runtime data is dynamic and can change
frequently.

47> what else can we know in each of these two
categories?

At compile time, we can know static information
about the parameter such as its name, display
information, default value, control style, and
allowed value range. This information is fixed and
can be accessed without running the code.

At runtime, we can know dynamic information about
the parameter such as its current value,
modulation value, and automation state. This
information can change during program execution
and can be accessed and updated using various
methods provided by the parameter API.

48> how much memory does a parameter take at
runtime?

The amount of memory that a parameter takes at
runtime depends on the specific implementation and
data types used for that parameter. In the case of
the `surge-param` crate, the `ParamRT` struct used
to represent a runtime parameter stores several
fields, including the parameter's current value,
modulation value, and modulation rate, as well as
various flags and metadata. The size of this
struct will depend on the specific data types used
for each field, as well as any padding or
alignment requirements dictated by the target
architecture.

In general, the amount of memory used by
a parameter will depend on its data type, the
number of parameters used in the system, and the
overall memory requirements of the system. It's
important to carefully manage memory usage in any
software system, and to avoid unnecessary memory
allocation or data duplication.
