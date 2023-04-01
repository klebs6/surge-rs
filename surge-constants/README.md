# surge-constants

This Rust crate provides a collection of constants
used in the Surge synthesizer system. These
constants are used in various modules throughout
the Surge suite to provide consistent and
optimized behavior.

The `surge-constants` crate includes constants
such as oversampling factor for the oscillator,
block size for processing audio, the number of
coefficients used by the FIR interpolator, and the
maximum number of sub-tables that can be used,
among others. These constants are used in
conjunction with the `surge-core` crate to
implement various audio effects.

For example, the `HPF_CYCLE_LOSS` constant is used
in the implementation of the high-pass filter
effect to control the shape of the filter. The
higher the value of `HPF_CYCLE_LOSS`, the more
aggressive the high-pass filtering will be,
resulting in more low frequencies being removed
from the audio signal.

The `surge-constants` crate also includes
constants used in the implementation of the MIDI
module, such as the frequency of MIDI note 0 and
the MIDI note number for A4.

Overall, the `surge-constants` crate provides
a convenient way to access and use the various
constants used in the Surge synthesizer system.

71> thanks! could you please describe how some of
these constants might be used in other projects?

Sure! Here are some possible uses for some of the
constants:

- `OSC_OVERSAMPLING` and `BLOCK_SIZE` could be
  used to determine the size of input and output
  buffers for processing audio in other digital
  signal processing (DSP) projects.

- `BLOCK_SIZE_OS_QUAD` could be used to determine
  the size of a buffer used for computing multiple
  oscillator outputs simultaneously in other DSP
  projects.

- `FIR_IPOL_N` and `FIR_IPOL_M` could be used to
  determine the number of taps and size of the
  kernel buffer for FIR filtering in other
  resampling projects.

- `HPF_CYCLE_LOSS` could be used as a parameter
  for controlling the shape of a high-pass filter
  in other DSP projects.

- `MAX_VOICES` could be used to limit the number
  of voices in a polyphonic synthesizer project.

- `N_SCENE_PARAMS` could be used as a reference
  for the number of scene parameters in
  a graphical user interface (GUI) for
  a synthesizer project.

- `WINDOW_SIZE_X` and `WINDOW_SIZE_Y` could be
  used to set the size of the GUI window in
  a synthesizer project.

- `MAX_WAVETABLE_SAMPLES` could be used as a limit
  for the number of samples in a wavetable in
  a synthesizer project.

- `NUM_CONTROLINTERPOLATORS` could be used to set
  the number of control interpolators available
  for smoothing parameter changes in a DSP
  project.

- `CONCERT_A_HZ` could be used as a reference
  frequency for tuning musical instruments in
  other audio projects.

72> cool thanks! do you think it makes sense to
keep a separate crate for global constants such as
this in the context of a rust works pace?

In a Rust workspace, it can make sense to have
separate crates for different functionalities,
including global constants. This can help with
organization, maintainability, and reuse of
code. By having a separate crate for global
constants, it can be easily shared and used across
multiple projects within the workspace without
duplicating code. Additionally, it can help keep
the main codebase cleaner and easier to manage by
separating out non-functional code into its own
crate.
