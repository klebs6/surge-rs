### surge-qfunit

50> /file surge-rs/surge-qfunit/src/qf_unit_state.rs

This source file defines the `QuadFilterUnitState`
struct, which represents the state of a quad
filter unit. The struct contains fields for the
filter coefficients, registers, delay buffers,
activity masks, and comb write positions.

The struct provides methods for accessing and
modifying the register values. The `new` method
initializes a new `QuadFilterUnitState` instance
with zeroed-out registers and an empty delay
buffer. The `TablesHandle` struct is also passed
in as a parameter and stored as a field in the
`QuadFilterUnitState`.

Note that this source file makes use of the
`z128!` macro, which returns an array of four
`__m128` vectors initialized to zero. These
vectors are used throughout the codebase to
represent four single-precision floating point
values packed into a 128-bit SIMD register.

51> /file surge-rs/surge-qfunit/src/qf_chain_state.rs

This is the source code for the `QuadFilterChain`
and `QuadFilterChainState` structs.

### QuadFilterChain

`QuadFilterChain` is a struct that represents
a chain of `QuadFilterChainState` structs. It is
typically used with `MAX_VOICES >> 2` elements,
and each `QuadFilterChainState` performs filtering
for four voices.

### QuadFilterChainState

`QuadFilterChainState` is a struct that represents
the state of a quad filter chain. It contains
several fields that hold filter coefficients,
registers, and delay buffers, as well as gain,
feedback, mix, and drive parameters for each of
four filter units. It also contains fields for
wavedata, feedback stereo channels, and various
other filter-related parameters.

The `QuadFilterChainState` struct has a method
`default_unit_state` that returns an array of four
`QuadFilterUnitState` structs, which represent the
state of individual filter units in the quad
filter chain.

The `QuadFilterChainState` struct also has several
helper methods that create `__m128` vectors filled
with zeros.

52> /file surge-rs/surge-qfunit/src/do_dual1.rs

It looks like the `do_dual1` method is one of
several methods that perform filtering on a quad
filter chain state. This method takes in four
parameters:

- A mutable reference to a `WaveshaperState`

- A mutable reference to a `QuadFilterChainState`

- A mutable reference to an `FbqGlobal`

- A pointer to a `f32` that will hold the left
  channel output

- A pointer to a `f32` that will hold the right
  channel output

The method performs the following operations:

- Initializes a constant value `hb_c` with the
  value `0.5`

- For each sample in a block of audio data:

  - Updates the `feedback` value of the
    `QuadFilterChainState` by adding the
    `d_feedback` value

  - Multiplies the `feedback` value by the
    `feedback_line_l` value of the
    `QuadFilterChainState` and applies a softclip
    function to the result

  - Adds the `feedback` value to the `dl[k]` and
    `dr[k]` values of the `QuadFilterChainState`
    and stores the result in `x` and `y`,
    respectively

  - If a filter identified by the constant
    `A_FILTER_ACTIVE` is active, applies the
    filter to `x` using the first unit state of
    the `QuadFilterChainState`

  - If a filter identified by the constant
    `B_FILTER_ACTIVE` is active, applies the
    filter to `y` using the second unit state of
    the `QuadFilterChainState`

  - Adds the `d_mix1` and `d_mix2` values of the
    `QuadFilterChainState` to the `mix1` and
    `mix2` values, respectively

  - Calculates the mixed output by multiplying `x`
    by `mix1` and `y` by `mix2` and adding the
    results

  - If a waveshaper identified by the constant
    `WAVESHAPER_ACTIVE` is active, applies the
    waveshaper to the mixed output using the
    `WaveshaperState` and `drive` value of the
    `QuadFilterChainState`

  - Adds the `d_gain` value of the
    `QuadFilterChainState` to the `gain` value

  - Multiplies the mixed output by the `gain`
    value and applies a mask that is the result of
    applying a bitwise `and` operation to the
    `active` value of the first unit state of the
    `QuadFilterChainState` and the mixed output

  - Stores the masked output in the
    `feedback_line_l` value of the
    `QuadFilterChainState`

  - Writes the left and right channel output
    values by calling the `qfc_write_outputs`
    function with the appropriate arguments.

53> /file surge-rs/surge-qfunit/src/do_ring.rs

These two methods seem to perform filtering and
processing on audio data using
a QuadFilterChainState struct, which contains
various data related to the state of the filter
chain. The first method, `do_dual1`, appears to
implement a dual filter topology, where two
independent filters are applied to the audio data
in parallel. The second method, `do_ring`,
implements a ring modulation topology, where the
output of one filter is used to modulate the input
of the other filter.

Both methods perform operations on the
`QuadFilterChainState` struct's fields, such as
`unit_state`, which appears to contain information
about each filter unit in the chain, and `gain`,
which seems to determine the overall gain applied
to the processed audio signal. Other fields like
`feedback` and `mix1` and `mix2` appear to be used
for intermediate calculations.

The methods also seem to use some external state,
such as `wss`, a mutable reference to
a `WaveshaperState` struct, and `fbq`, a mutable
reference to a `FbqGlobal` struct. These
references may be used to pass information between
different parts of the program, or to access
shared state.

Finally, both methods contain a call to
`qfc_write_outputs`, which is not shown here but
probably performs some output operations like
writing the filtered audio data to a buffer.

54> /file surge-rs/surge-qfunit/src/do_serial1.rs

These are three methods of a Rust module, likely
related to audio processing. The methods implement
different filter topologies, which are used to
process audio signals. All three methods share
a similar structure where they iterate over
a block of audio samples, process the samples with
different filters, mix the results, and output the
final signal.

The `do_dual1` method seems to implement
a dual-filter topology with feedback. The
`do_ring` method seems to implement a ring
modulation topology with feedback. The
`do_serial1` method seems to implement a serial
filter topology without feedback.

Each method takes a `WaveshaperState` struct,
a `QuadFilterChainState` struct, a `FbqGlobal`
struct, and two `*mut f32` pointers to output the
left and right channels. The
`QuadFilterChainState` struct seems to contain the
state of the different filters, while the
`FbqGlobal` struct contains pointers to the filter
functions.

Overall, these methods appear to be low-level,
performance-critical implementations of audio
filters that leverage the SIMD capabilities of
modern CPUs for speed.

55> /file surge-rs/surge-qfunit/src/do_stereo.rs

All of these functions seem to be part of a Rust
crate called `QFC` (Quad Filter Chain), which
implements various digital signal processing (DSP)
algorithms, such as filtering, waveshaping, and
mixing, among others. Each function seems to be
a different implementation of the `do_` prefix,
which represents a specific processing topology
for the filter chain. The different
implementations seem to be optimized for different
filter chains, and they take various boolean
constants as parameters to determine which
processing blocks to activate for a specific
filter chain. All of these functions seem to be
operating on `f32` values using the SIMD
instruction set provided by the `sse` Rust crate.

56> so, we have several hardcoded filter chains
for different topologies

Yes, that's correct. These functions implement
different filter topologies for the
QuadFilterChainState struct. The different
topologies correspond to different ways of
connecting the two filters in series or in
parallel. The specific topologies are chosen by
calling the corresponding function based on the
desired filter configuration.

57> what about QuadFilterChainState itself -- how
is this used?

`QuadFilterChainState` is a struct used to store
the state of a quad filter chain. It contains
several arrays and values that are used to store
the filter coefficients and state, as well as the
mixing and feedback values used by the filter
chain.

Here is the definition of the
`QuadFilterChainState` struct:

```rust
pub struct QuadFilterChainState {
    pub dl:               [__m128; BLOCK_SIZE_OS],
    pub dr:               [__m128; BLOCK_SIZE_OS],
    pub feedback_line_l:  __m128,
    pub feedback:        __m128,
    pub mix1:            __m128,
    pub mix2:            __m128,
    pub gain:            __m128,
    pub d_feedback:      __m128,
    pub d_mix1:          __m128,
    pub d_mix2:          __m128,
    pub d_gain:          __m128,
    pub unit_state:      [UnitState; 2],
    pub active:          [bool; 4],
    pub drive:           __m128,
    pub d_drive:         __m128,
    pub ws_lpf:          __m128,
}
```

Some of the key fields in this struct include:

- `dl` and `dr`: These are arrays of 4-element
  vectors (`__m128`) that store the input audio
  data for the left and right channels,
  respectively.

- `feedback_line_l` and `feedback`: These are
  vectors that store the feedback values for the
  left channel filter chain.

- `mix1` and `mix2`: These are vectors that store
  the mixing values for the left and right channel
  filter chains, respectively.

- `gain`: This is a vector that stores the overall
  gain for the filter chain.

- `d_feedback`, `d_mix1`, `d_mix2`, `d_gain`, and
  `d_drive`: These are vectors that store the
  delta values (changes in value) for the
  feedback, mixing, gain, and drive parameters.

- `unit_state`: This is an array of two
  `UnitState` structs that store the state for the
  two filter units in the filter chain.

- `active`: This is an array of four booleans that
  indicate which filter units and channels are
  active in the filter chain.

- `drive` and `ws_lpf`: These are vectors that
  store the drive and low-pass filter values used
  by the waveshaper.

The `QuadFilterChainState` struct is passed as an
argument to the filter chain functions, which use
its fields to perform the filtering and mixing
operations. The values in the
`QuadFilterChainState` struct are updated during
each call to the filter chain functions to reflect
the changes in the filter parameters and audio
data.

58> interesting -- what distinguishes it from
QuadFilterUnitState?

`QuadFilterUnitState` is a per-unit state struct
that holds the state information for each of the
four filter units in the `QuadFilterChain`. It
contains parameters such as filter coefficients,
filter state variables, and the like.

On the other hand, `QuadFilterChainState` is
a struct that holds the state information for the
entire `QuadFilterChain`. This includes the four
`QuadFilterUnitState` structs, as well as state
variables for the entire filter chain, such as
feedback values, mix values, and gain values.

In other words, `QuadFilterUnitState` contains the
state information for a single filter unit, while
`QuadFilterChainState` contains the state
information for the entire filter chain.

59> thanks -- it seems like we can select which
topology to activate, which leverages several quad
filter units.  the chain is the overarching
structure which manages the whole flow

Yes, that's correct. The `QuadFilterChainState` is
the main structure that manages the flow of audio
through the different quad filter units and other
processing stages. It provides the necessary state
information for each quad filter unit, such as the
filter coefficients, filter state, and feedback
parameters, as well as the mixing coefficients for
combining the filtered signals. By activating
different combinations of quad filter units, the
quad filter chain can implement different filter
topologies.

On the other hand, the `QuadFilterUnitState`
contains the state information for each individual
quad filter unit, such as the filter coefficients
and state, and the gain and active flag. Each quad
filter unit is responsible for processing a single
channel of audio, and the `QuadFilterUnitState` is
used to manage the state of each unit.
