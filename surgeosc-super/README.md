The AbstractBlitOperator handles a model where an
oscillator generates an impulse buffer, but
requires pitch tuning, drift, FM, and DAC
emulation.

Unison just replicates the voices in memory
running the entire oscillator set for each voice,
with a few parameters split - notably drift - and
the state storage outlined below split by
voice. As such the state is often indexed by
a voice. For the rest of this description I'll
leave out the unison splits. Similarly the stereo
implementation just adds pairs (for oscbuffer
there is oscbufferR and so on) and here I'll just
document the mono implementation

## Overall operating model

Assume we have some underlying waveform which we
either have in memory or which we generate using
an algorithm. At different pitches we want to
advance through that waveform at different speeds,
take the implied impulses for the moment in time
and simulate a DAC outputting that. The common
form of that waveform is that it is digital
- namely it is represented as a set of impulse
values at a set of times - but those times do not
align with the sample points.

In code that looks as follows

- The oscillator has a phase pointer (oscstate)
  which indicates where we are in the internal
  stream.

- At any given moment, we can generate the next
  chunk of samples for our frequency which is done
  in the 'convolute' method and store them in
  a buffer. This buffer is a fixed number of
  samples, but convolute generates a fixed amount
  of phase space coverage, so at higher frequency
  we need to convolute more often (cover more
  phase space at constant sample space).

- in our process loop, we extract those samples
  from the buffer to the output. At any given
  convolution moment, we store into the buffer at
  the state variable bufpos.

- If we have extracted our set of samples, we
  re-generate the next chunk by convoluting.

So basically we have a couple of arrows pointing
around. oscstate, which shows us how much phase
space is covered up to bufpos of buffer and the
simple march of time that comes from calling
process_block. When we are out of state space
(namely, oscstate < BLOCK_SIZE * wavelength) we
need to reconvolve and fill our buffer and
increase our oscstate pointer. So in the process
block it looks like oscstate counts down and
convolute pushes it up, but what really is
happening is oscstate counts down because bufpos
moves forwards, and convolute gives us more valid
buffer ahead of us. When we beyond the end of the
oscillator buffer we need to wrap our pointer.

The storage buffer is sized so there is enough
room to run a FIR model of the DAC forward in time
from the point of the current buffer. This means
when we wrap the buffer position we need to copy
the back-end FIR buffer into the front of the new
buffer. Other than that subtletly the buffer is
just a ring.

There's lots more details but that's the basic
operating model you will see in ::process_block
once you know that ::convolute generates the next
blat of samples into the oscbuffer structure.

The calculation which happens when we do the
convolution exists in the various oscbuffers and
the current position we have extracted lives in
the bufpos variable. So at a given point, oscstate
tells us how much phase space is left if we
extract from bufpos onwards.

The convolute method, then, is the heart of the
oscillator. It generates the signal moving
forwards which we push out through the buffer. In
the AbstractBlitOscillator subclasses, it works on
a principle of simulating a DAC for
a voice. A little theory.

We know that in a theoretical basis, a digital
signal is a stream of delta impulses at the sample
point, but we also knwo that delta impulses have
infinite frequency response, so especially as you
get closer to the nyquist frequency, you end up
with very nasty aliasing problems. Sample a 60hz
sin wav at 100 hz and you can immediately see the
40 hz artefact. So what you want to do is replace
the delta with a function that has the time
response matching a perfect low pass filter, which
is a rect in frequency space or a sinc in time
space. So basically at each point where you
generate signal you want to rather than just
taking that signal, increase the signal by the
sinc-smeared energy of the change in signal.

Or: Rather than "output = zero-order samples of
underlyer" do "output += (change in underlyer)
x (sinc)" where x is a convolution operator. Since
sinc has infinite support, though, we can't use
that really, so have to use a windowed sinc.

Once we have committed to convolving an exact but
differently aligned impulse stream into our sample
output, though, we have the opportunity to exactly
align the time of that impulse convoultion with
the moment between the samples when the actual
impulse occrs.

So the convolution  has to manage a couple of
dimensions of time. When we call :;convolute
remember, it is because we don't have enough
buffer phase space computed for our current
block. So ::convolute is filling a block in the
"future" of our current pointer. That means we can
actually use a sligntly non-causal filter into the
oscstate future. So mechanically we end up
implementing "oscbuffer [i + futurelook]
= sum(impulse chage) * impulse[i]"

Surge adds one last wrinkle, which is that impulse
function depends on how far between a sample you
are. The peak of the function should happen
exactly at the point intra-sample. To do that it
makes two tables. The first is a table of the
windowed sinc at 256 steps between 0 and
1 sample. The second is the derivative of that
windowed function with respect to position which
allows us to make a first order taylor correction
to the window. Somewhat confusingly, but very
efficiently, these two tables are stored in one
data structure "sinctable", with an indexing
structure that gives a window, a window
derivative, the next window, the next window
derivative, etc... 

But the end result is we do a calclulation which
amounts to

```c
while( our remaining osc state doesn't cover enough phase space ) <<- this is in process block
   convolute <<- do this call
     Figure out our next impulse and change in impulse. Call that change g.
     figure out how far in the future that impulse spans. Call that delay.
     fill in the oscbuffer in that future with the windowed impulse
         oscbuffer[pos + i] = oscbuffer[pos + i] + g * ( sincwindow[i] + dt * dsincwindow[i] )
     advance oscstate by the amount of phase space we have covered

```

Unfortunately, to do this efficiently the code is
a bit inscrutable, hence this comment. Also some
of the variable names (lipol128 is not an obvious
name for the 'dt' above) makes the code hard to
follow. As such, in this implementation I've added
quite a lot of comments to the ::convolute method.

At the final stage, the system layers on a simple
3 coefficient one delay biquad filter into the
stream based on character, copies the buffer to
the output, and then manages pointer wraparounds
and stuff. That's all pretty mechanical.

## Surge Super Oscillator

The Surge Super Oscillator is a powerful and
versatile synthesizer component that provides
a range of advanced synthesis features. It is
designed to produce complex and interesting sound
textures, with a focus on flexibility and user
control.

At its core, the Surge Super Oscillator is based
on a set of advanced mathematical algorithms that
allow it to generate a wide range of waveforms and
sound textures. These algorithms are implemented
using Rust's high-performance programming
constructs, making the oscillator both efficient
and easy to use.

The oscillator includes a range of features that
allow for advanced modulation and control. It
supports various control types, including
envelopes, LFOs, and MIDI input, and provides
a range of modulateable parameters that can be
used to shape and control the output signal.

One of the key features of the Surge Super
Oscillator is its ability to perform advanced
convolution and filtering operations. This allows
for precise control over the frequency and phase
response of the oscillator, and allows for the
creation of complex and dynamic sound textures.

The Surge Super Oscillator also includes a range
of performance optimization features, including
configurable block sizes and update lag values,
which allow for fine-grained control over the
processing performance and latency.

Overall, the Surge Super Oscillator is a powerful
and versatile synthesizer component that provides
a range of advanced synthesis features. With its
advanced mathematical algorithms, flexible
modulation capabilities, and advanced convolution
and filtering features, it is well-suited for
a wide range of music production applications.

27> /file surge-rs/surgeosc-super/src/init.rs

Looking at the code snippet you provided, it
appears that the initialization process for the
Surge Super Oscillator involves several key steps:

1. Initializing internal state: The first line of
   the `init` function sets the `first_run` flag
   to `true`, indicating that this is the first
   time the oscillator is being initialized.

2. Setting the oscillator mode: The
   `get_character` function is called to retrieve
   the current oscillator mode, which is used to
   set the oscillator coefficients using the
   `set_coeff_by_character` function.

3. Clearing output buffers: The
   `blitter.clear_all_oscout()` function is called
   to clear the output buffers used by the
   oscillator.

4. Setting buffer position and DC offset: The
   buffer position and DC offset are set to their
   default values of `0` and `0.0`, respectively.

5. Setting the update rate: The `set_rate_all`
   function is called to set the update rate for
   the oscillator to `0.05`.

6. Setting unison count: The `unicount` parameter
   is retrieved using the `pvali` function, which
   is then used to set the unison count for the
   oscillator.

7. Preparing unison buffers: The `prepare_unison`
   function is called to prepare the unison
   buffers used by the oscillator.

8. Clearing internal tracking: Finally, the
   `clear_tracking` function is called to clear
   any internal tracking information used by the
   oscillator.

Overall, the initialization process for the Surge
Super Oscillator is a critical step in preparing
the oscillator for use. By initializing internal
state, setting oscillator mode and coefficients,
clearing output buffers, and setting key
parameters such as the update rate and unison
count, the oscillator is able to produce
high-quality output signals with a wide range of
synthesis features.

28> /file surge-rs/surgeosc-super/src/new.rs

Looking at the code snippet you provided, it
appears that the Surge Super Oscillator is created
using the `new` function. This function takes
several input parameters, including handles to
a tuner, tables, and sample rate.

The `new` function initializes a new instance of
the `SurgeSuperOscillator` struct, setting the
various fields to their default values. The
`drift`, `master_osc`, `out`, `blitter`, `params`,
`osc_params`, `li_hpf`, `li_dc`,
`li_integratormult`, `fm_phase`, `first_run`,
`dc`, `dc_uni`, `elapsed_time`, `last_level`,
`pwidth`, `pwidth2`, `pitch`, `fm_depth`,
`integrator_mult`, `l_pw`, `l_pw2`, `l_shape`,
`l_sub`, `l_sync`, `fm_delay`, `fm_mul_inv`,
`coeff_b0`, `coeff_b1`, and `coeff_a1` fields are
all set to their default values.

Additionally, the `tables`, `tuner`, and `srunit`
handles are cloned and stored in the new
`SurgeSuperOscillator` instance.

Overall, the `new` function provides a convenient
way to create a new instance of the Surge Super
Oscillator, with default settings and handles to
key system resources. By providing a flexible and
easy-to-use initialization process, the Surge
Super Oscillator is able to provide advanced
synthesis features and high-quality output signals
for a wide range of music production applications.

29> /file surge-rs/surgeosc-super/src/params.rs

Sure, I'd be happy to help.

The SurgeSuperOscillator is a powerful component
of the larger Surge synthesizer system, and it can
be described as a highly-parametrized,
high-quality sample and hold oscillator. The
oscillator is designed to produce complex
waveforms with a high degree of precision and
flexibility, making it a valuable tool for any
musician or sound designer.

To get started with the SurgeSuperOscillator, you
will need to initialize it with the `init()`
function. This function sets up the oscillator's
initial state, including its parameters and
buffers. Once the oscillator is initialized, you
can create a new instance of it using the `new()`
function.

The SurgeSuperOscillator is highly parametrized,
with a wide range of parameters that can be used
to adjust its behavior. These parameters include
`Shape`, `Width`, `SubWidth`, `SubLevel`,
`SyncPitch`, `UniSpread`, `UniCount`, and
`Character`. Each of these parameters has
a default value, a minimum and maximum value, and
a value type, which determines the type of data
that can be used to adjust it. The oscillator's
parameters can be set using the `set_param()`
function.

The oscillator is also modulateable, meaning that
its parameters can be adjusted dynamically using
modulation sources such as LFOs and envelopes. The
`moverate()` function determines the rate at which
each parameter can be modulated.

The SurgeSuperOscillator is a complex algorithm
that involves a number of mathematical
calculations, including convolutions and impulse
responses. These calculations are optimized for
high performance and low latency, allowing the
oscillator to produce complex waveforms in real
time.

In summary, the SurgeSuperOscillator is
a highly-parametrized, high-quality sample and
hold oscillator that is designed to produce
complex waveforms with a high degree of precision
and flexibility. Its many parameters and
modulateable nature make it a powerful tool for
musicians and sound designers, and its optimized
algorithm ensures low latency and high
performance.

30> /file surge-rs/surgeosc-super/src/set.rs

The `SurgeSuperOscillator` has several functions
for setting its properties.

The `get_character()` function returns the current
`CharacterMode` of the oscillator.

The `set_rate_all(rate: f32)` function sets the
rate of several lag processors in the oscillator,
including `l_pw`, `l_pw2`, `l_shape`, `l_sub`, and
`l_sync`.

The `clear_tracking()` function sets the values of
`last_level` and `elapsed_time` arrays to zero.

The `set_coeff_by_character(mode: CharacterMode)`
function sets the filter coefficients based on the
current `CharacterMode` of the oscillator. The
filter coefficients determine the coloration of
the oscillator's output, with different modes
producing warmer, neutral, or brighter sounds. The
function calculates the filter coefficients based
on the current samplerate and sets them to the
`coeff_b0`, `coeff_b1`, and `coeff_a1` variables.

31> /file surge-rs/surgeosc-super/src/set_pitch.rs

The `set_pitch` method is used to set the pitch of
the `SurgeSuperOscillator` in Hertz. It takes in
two arguments: `pitch`, which is the target pitch
in Hertz, and `is_display`, a boolean flag that
determines whether the pitch is being set for
display purposes or not.

If `is_display` is `true`, the method sets the
`n_unison` parameter to 1, which indicates that
there is only one oscillator. Otherwise, it sets
the `n_unison` parameter to the value specified by
the `UniCount` parameter, which determines the
number of oscillators to be used.

The method updates the lag values using the
`update_lagvals` function, which calculates the
lag values based on the current pitch value. It
then iterates over each oscillator and sets its
state depending on the `Retrigger` parameter and
the `is_display` flag. If `Retrigger` is `true` or
`is_display` is `true`, it resets the oscillator's
state and sets the `last_level` value to
`-0.4`. Otherwise, it calculates the detune value
based on the `UniSpread` parameter and a random
value, and sets the oscillator's state
accordingly.

The method also resets several other parameters
for each oscillator, including `dc_uni`, `state`,
`pwidth`, `driftlfo`, and `driftlfo2`.

32> /file surge-rs/surgeosc-super/src/sso.rs

The `SurgeSuperOscillator` is an oscillator type
that generates complex waveforms by combining
multiple simpler waveforms, using various
parameters to shape and modulate the resulting
sound. The oscillator uses a blitter, which is
a technique for generating band-limited waveforms,
to produce its output. The `SSOParamArrayRT` type
is used to store and manage the oscillator's
parameters, which can be modulated in real-time.

The oscillator has a number of properties and
methods for setting and controlling its
behavior. For example, it has a `set_pitch` method
for setting its pitch, a `set_rate_all` method for
setting the rate of all its internal modulators,
and a `clear_tracking` method for resetting its
tracking values. It also has a number of
properties for controlling various aspects of its
sound generation, such as its drift, FM depth, and
waveform shape.

The oscillator is implemented using a combination
of Rust code and inline assembly (using the
`inline_asm!` macro), which allows it to achieve
high performance while remaining portable across
different platforms. The code uses various
optimization techniques, such as loop unrolling
and SIMD instructions, to further improve
performance.

33> /file surge-rs/surgeosc-super/src/update_lagvals.rs

This code defines an implementation of the
`update_lagvals` function for the
`SurgeSuperOscillator` struct.

The `update_lagvals` function updates the values
of several internal `Lag` objects used by the
oscillator. These objects model low-pass filters
that smooth out changes in various oscillator
parameters, like `Width` and `Shape`.

The function first retrieves the values of these
parameters from the `self.params` array using
a macro defined using the `macro_rules`
syntax. The macro simply returns the value of the
specified parameter.

Next, the function updates the `Lag` objects with
new values for `sync`, `pw`, `pw2`, `shape`, and
`sub` using the `new_value` method of each `Lag`
object. The `limit_range` function is used to
ensure that the values of these parameters fall
within valid ranges.

The function then calculates the pitch multiplier
`pp` using the `n2p_tuningctr` method of the
`TunerHandle` object stored in `self.tuner`, which
converts a MIDI note number to a frequency
ratio. This frequency ratio is added to the `sync`
parameter and converted back to a pitch multiplier
using the `n2p_tuningctr` method.

The function calculates the inverse of the
oscillator's sampling rate times 4 and stores it
in the `invt` variable.

Finally, the function calculates the value of the
high-pass filter used by the `LiPolPs` object
stored in `self.li_hpf` by taking the minimum of
the `SSO_INTEGRATOR_HPF` constant and
`SSO_HPF_CYCLE_LOSS` raised to the power of
`invt`. This value is passed to the `set_target`
method of the `LiPolPs` object to update its
internal state.

If `is_init` is `true`, the function calls the
`instantize` method of each `Lag` and `LiPolPs`
object to reset their internal state.

34> /file surge-rs/surgeosc-super/src/process/process.rs

This code defines the `process_block` function for
the `SurgeSuperOscillator` type, which implements
the `OscillatorProcess` trait. This function is
called to generate a block of audio samples for
the oscillator.

At the beginning of the function, the `pitch` and
`drift` fields of the oscillator are set based on
the configuration passed in as an argument. The
`blitter.pitchmult_inv` and `blitter.pitchmult`
fields are then set based on the `pitch` field,
and the `update_lagvals` function is called to
update the internal lag processors based on the
current parameter values.

Next, the `process_block_fm` or
`process_block_nofm` function is called based on
whether frequency modulation is enabled or
not. These functions generate a block of audio
samples based on the current oscillator parameters
and write them into the oscillator's `blitter`
buffer.

After the audio samples have been generated,
a high-pass filter is applied to the block using
the `li_hpf` LipolPs processor. The output of the
high-pass filter is then multiplied by the current
pitch attenuation and DC offset attenuation
factors.

Finally, the generated audio samples are stored in
the oscillator's `blitter` buffer, and the buffer
position is updated to prepare for the next block.

35> /file surge-rs/surgeosc-super/src/process/no_fm.rs

This function processes a block of samples for the
oscillator when FM modulation is not used. It
takes as input an `OscillatorProcessBlockCfg`
struct, which contains information about the
pitch, drift, stereo status, and whether FM is
used. The function first calculates the amount of
phase space to be covered, based on the oversample
block size and pitch. Then, for each voice in the
oscillator's `blitter`, it generates samples and
fills the buffer until the oscillator and sync
states have covered at least the required amount
of phase space. Finally, it processes the
generated samples.

If `self.l_sync.v` is greater than 0, then it
means that sync is active, and the function will
fill the `syncstate` traversal until it has
covered the required amount of phase
space. Otherwise, it will fill the `oscstate`
traversal. The `convolute` function is called to
fill the buffer for the voice. Once enough samples
have been generated to cover the required phase
space, the function processes the generated
samples.

36> /file surge-rs/surgeosc-super/src/process/handle_wrap.rs

This function handles the buffer wrap when the
buffer position reaches the end of the
buffer. When this happens, the function copies the
overlapping samples from the end of the buffer to
the beginning of the buffer. It uses SIMD
instructions to load and store the samples in
blocks of 4.

First, it defines a constant SZ as FIR_IPOL_N
(interpolation filter order) divided by 4. It then
creates arrays of SSE registers overlap_l,
overlap_r (for stereo mode), and dcoverlap of size
SZ to store the overlapping samples.

The function then creates a zero SSE register
using `_mm_setzero_ps()`.

In the loop, it loads the samples from the end of
the buffer into overlap_l, overlap_r, and
dcoverlap using `_mm_load_ps()`. It then stores
these samples into the beginning of the buffer
using `_mm_store_ps()`. After storing, it also
clears the samples from the end of the buffer by
storing the zero SSE register.

If stereo mode is enabled, it repeats this process
for overlap_r.

37> /file surge-rs/surgeosc-super/src/process/fm.rs

The `process_block_fm` method processes the block
of the oscillator when FM is enabled. For each
sample in the block, it retrieves the value of the
`master_osc` and calculates the FM multiplier as
`fmmul = limit_range(1.0 + cfg.fm_depth
* master_osc, 0.1, 1.9)`. The amount of phase
space to cover is then calculated as `a
= self.blitter.pitchmult * fmmul`.

For each voice in the unison, the method enters
a loop that fills the oscillator buffer and FM
modulation buffer as long as the amount of phase
space to cover is greater than the current state
of the oscillator or the sync state. While the
loop is running, the `fm_mul_inv` is set to the
reciprocal of the FM multiplier to avoid
division. Finally, the method subtracts the amount
of phase space from the oscillator state and sync
state, and processes the block.

38> /file surge-rs/surgeosc-super/src/process/block.rs

This code seems to be implementing the core
processing logic for a digital oscillator using
Surge's SuperOscillator algorithm. The
SuperOscillator generates multiple copies of
a waveform, slightly detuned from each other, to
create a rich, thick sound. The algorithm uses
a technique called oversampling to reduce
aliasing, which occurs when the waveform is
sampled at too low a rate.

The `process_block` method is called to process
a block of samples. The method takes an
`OscillatorProcessBlockCfg` argument, which
contains configuration information for the block
processing. Within the `process_block` method,
several internal parameters of the oscillator are
updated, and then either `process_block_fm` or
`process_block_nofm` is called based on whether or
not frequency modulation is being used.

In `process_block_fm`, the algorithm generates
multiple copies of the waveform, each detuned from
the others, and then applies frequency modulation
to them. The method uses a nested loop to process
the block sample-by-sample, and calls the
`convolute` method for each sample to generate the
output signal.

In `process_block_nofm`, the algorithm generates
multiple copies of the waveform without applying
frequency modulation. The method also uses
a nested loop to process the block
sample-by-sample, and calls the `convolute` method
for each sample to generate the output signal.

The `maybe_handle_wrap` method is called to handle
the case where the output buffer wraps around to
the beginning of the buffer. The method copies the
overlapping samples from the end of the previous
buffer to the beginning of the new buffer to
ensure continuity in the output signal.

The `do_block` method is called for each sample in
the block and performs the core processing for the
oscillator. It applies a high-pass filter to the
waveform, subtracts the DC offset, and applies
a character filter to shape the output signal. The
resulting signal is then stored in the output
buffer. The method also handles the stereo case
where two signals are generated for left and right
channels.

39> /file surge-rs/surgeosc-super/src/convolute/stereo.rs

This function convolves the stereo output of the
oscillator with the sinc function to apply
interpolation to the waveform. The function takes
in the following arguments:

- `g`: The gain to apply to the left channel.

- `g_r`: The gain to apply to the right channel.

- `delay`: The delay in samples to apply to the
  sinc function.

- `m`: The index of the sinc table to start
  convolving from.

- `lipol128`: An `__m128` vector containing the
  lipshitz value.

The function first loads the left and right gains
into `__m128` vectors and shuffles their values so
that each element of the vector contains the same
gain value. The function then loops through each
sample in the FIR interpolation buffer (the
`FIR_IPOL_N` samples at the end of the
`oscbuffer_l` and `oscbuffer_r` arrays), and loads
the corresponding samples from the sinc table. The
function then multiplies the loaded/ sinc values
by the `lipol128` vector, which contains the
Lipshitz value for the current sample. The
function then multiplies the resulting sinc values
by the left and right gain values and adds them to
the current sample value in the `oscbuffer_l` and
`oscbuffer_r` arrays, respectively. Finally, the
updated sample values are stored back into the
`oscbuffer_l` and `oscbuffer_r` arrays using
`_mm_storeu_ps`.

40> /file surge-rs/surgeosc-super/src/convolute/mono.rs

Great, let me know if there's anything else I can
help with!

41> /file surge-rs/surgeosc-super/src/convolute/get_t.rs

This is an implementation of the `get_t` function
in the `SurgeSuperOscillator` struct. It takes in
two floating point values, `sync` and `detune`,
and returns a tuple of two floating point values,
`t` and `t_inv`.

The `get_t` function first checks whether the
`absolute` flag of the `SSOParam::UniSpread`
parameter is set. If it is, it calculates the
`note` variable by multiplying `detune` by the
inverse pitch multiplier, then scaling it based on
the note number and adding the `sync` value. The
inverse pitch multiplier is calculated using the
`n2pinv` function of the `tuner` object, which
takes a MIDI note number as input and returns the
inverse of its frequency in Hertz. The resulting
`note` value is then converted back to an inverse
frequency using the `n2pinv` function, and
returned as `t`.

If the `absolute` flag is not set, the `get_t`
function simply adds `sync` and `detune`, and
passes the result to the `n2pinv_tuningctr`
function of the `tuner` object, which also returns
the inverse of the frequency in Hertz.

Finally, `t` and its inverse `t_inv` are returned
as a tuple.

42> /file surge-rs/surgeosc-super/src/convolute/get_g.rs

This is a snippet of Rust code from the Surge
synthesizer project. It defines some methods of
the `SurgeSuperOscillator` struct, which is part
of the implementation of the oscillator module.

The `convolve_stereo` method convolves the audio
data in the left and right buffers of a stereo
audio signal with a given filter kernel, with the
output signal also being stereo. This method makes
use of the SSE (Streaming SIMD Extensions)
instruction set, which is a set of instructions
that can be used to perform parallel operations on
multiple data elements simultaneously. This allows
for more efficient processing of audio signals.

The `convolve_mono` method is similar to
`convolve_stereo`, but is used for convolving
a mono audio signal with a filter kernel.

The `get_t` method calculates the current position
in time of the oscillator based on its current
pitch and detune settings.

The `get_delay` method calculates the delay in
samples that should be applied to the audio signal
based on the oscillator's frequency modulation and
position in the cycle.

The `impulse_zero`, `impulse_one`, `impulse_two`,
and `impulse_three` methods are used to generate
a 4-impulse cycle for generating square, saw, and
sub waveforms. These methods calculate the height
of each impulse and the change in level from the
prior impulse.

The `get_g` method calculates the change in level
for the current impulse of the 4-impulse cycle,
based on the waveform type and sub-cycle position,
and applies a stereo pan if required.

43> /file surge-rs/surgeosc-super/src/convolute/convolute.rs

The `convolute` function is a method of the
`Convolute` trait that is implemented for the
`SurgeSuperOscillator` struct. It performs the
convolution of a single sample of the oscillator's
waveform with the sinc function of the filter
kernel.

The function takes a `ConvolutionCfg` argument
that specifies the voice index, stereo flag, and
FM flag. It starts by computing the detune value,
the wave shape (`wf`) and sub-cycle phase (`sub`)
for the current sample, and the `ipos` value that
indicates the current position in phase space. If
sync is enabled, the `ipos` value is computed
using the sync state instead of the oscillator
state.

The function then calculates the delay value based
on the FM flag and `ipos`, and the `m` and
`lipol128` values that are used to calculate the
fractional part of the filter kernel that needs to
be convolved with the current sample.

Next, the function calls the `get_g` method to
calculate the change in level (`g`) for the
current sample, and applies this change to the
left and right channels if stereo output is
enabled. Finally, the function calls either
`convolve_mono` or `convolve_stereo` to perform
the actual convolution with the sinc function of
the filter kernel.

If `CONVOLVE_NODC` is not set, the function also
calculates the DC offset for the current sample
and updates the `dc_uni` buffer and `dcbuffer`
with this value. Finally, the function updates the
oscillator state and state machine for the next
sample.

44> thanks! what do you think overall about this
oscillator?

As an AI language model, I don't have personal
opinions, but I can provide you with some
technical information about the oscillator.

The Surge SuperOscillator is a powerful and
versatile digital oscillator that can produce
a wide variety of sounds, including classic
waveforms such as saw, square, and triangle, as
well as more complex sounds such as
sub-oscillators and detuned unison voices.

It achieves its versatility through a combination
of advanced features, including frequency
modulation, sync, and a state machine that
generates a sequence of impulses to create its
output waveform. Additionally, it uses convolution
to apply a filter to the waveform, allowing for
a rich and complex sound that can be shaped in
many ways.

Overall, the Surge SuperOscillator is a highly
capable and flexible oscillator that can be used
to create a wide range of sounds in a variety of
contexts, making it a valuable tool for sound
designers and musicians alike.

45> could you describe for me the state machine?

Certainly! The state machine in this code is used
to generate a four-impulse cycle that produces
a square, sawtooth, and sub-waveform. The state
machine is implemented using an enhanced Rust
enum, which allows for convenient pattern matching
over the states.

Here are the four states of the state machine and
their respective actions:

- `ImpulseZero`: Calculates the height of the
  first impulse of the cycle and returns the
  change from the prior level at this impulse.

- `ImpulseOne`: Returns the change in level for
  the second impulse of the cycle.

- `ImpulseTwo`: Returns the change in level for
  the third impulse of the cycle.

- `ImpulseThree`: Returns the change in level for
  the fourth impulse of the cycle.

Each time the state machine is convolved, the
current state is checked, and the corresponding
action is taken to generate the next impulse in
the cycle. After each convolution, the state
pointer is advanced to the next state.

46> thanks! could you describe for me the
convolution algorithm in more depth? 

Certainly! Let me see if I can explain it in a way
that's both informative and entertaining.

Convolution is a fundamental operation in digital
signal processing that plays a crucial role in the
generation of sound. In essence, it involves
multiplying two functions together and then
integrating the result. In the context of audio
synthesis, one of the functions is typically the
audio waveform generated by the oscillator, and
the other is a filter kernel that defines the
desired spectral response of the resulting sound.

The convolution algorithm used in the
SurgeSuperOscillator is a relatively
straightforward implementation of this
concept. The first step is to calculate the
current position of the oscillator in its phase
space, which is represented as a number between
0 and 2^24. This position is then used to
determine the delay between the current sample
being generated and the sample that will be
convolved with it.

The next step is to calculate the fractional part
of the position, which is then used to interpolate
the filter kernel. This interpolation is performed
using a polyphase FIR filter, which is a common
technique for efficiently implementing FIR filters
on modern CPUs. In essence, the filter kernel is
divided into a number of small, overlapping
sub-filters, each of which is applied to a portion
of the input signal. The outputs of these
sub-filters are then combined to produce the final
output.

Once the filter kernel has been interpolated, the
resulting waveform is multiplied by the current
sample of the oscillator waveform, and the product
is added to the output buffer at the appropriate
delay position. This process is repeated for each
sample in the audio buffer, resulting in the final
audio waveform that is output to the speakers.

Of course, there are many nuances and
optimizations that go into this process that
I haven't touched on here, but I hope this gives
you a sense of the basic principles
involved. Convolution is a powerful and versatile
tool for shaping audio signals, and the
implementation used in the SurgeSuperOscillator is
just one example of how it can be used to create
rich and complex sounds.
