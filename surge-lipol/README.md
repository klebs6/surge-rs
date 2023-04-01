# surge-lipol

A Rust crate that implements the LiPol (linear
interpolator) algorithm for audio processing.

A LiPol is a type of audio processing algorithm
that linearly interpolates between two values over
time, making it useful for smoothly transitioning
between audio values, for example when fading in
or out.

The crate includes a struct `LiPol<T>` that
implements the LiPol algorithm. It contains the
following fields:

- `v`: the current audio value

- `new_v`: the target audio value (used for
  interpolation)

- `dv`: the rate of change between `v` and `new_v`

- `bs_inv`: the inverse of the block size, used
  for calculating `dv`

- `first_run`: a boolean flag indicating whether
  this is the first time the LiPol has been used
  (used for special first-run behavior)

The `LiPol` struct also includes several methods
for working with the LiPol algorithm:

- `new(blocksize: usize)`: creates a new `LiPol`
  with the given block size

- `set_blocksize(&mut self, n: usize)`: sets the
  block size of this `LiPol` to `n`

- `instantize(&mut self)`: sets the current audio
  value to the target audio value and resets the
  rate of change to zero

- `process(&mut self)`: advances the current audio
  value by the current rate of change

- `get_target_value(&mut self) -> T`: gets the
  current target audio value

- `reset(&mut self, blocksize: usize)`: resets
  this `LiPol` with the given block size

- `new_value(&mut self, x: T)`: sets the target
  audio value to `x` and calculates the new rate
  of change

The crate also includes some utility functions for
testing and initializing the LiPol, such as
`default()` for initializing with a default block
size, and `test_set_blocksize()` for testing the
`set_blocksize()` method.

Overall, `surge-lipol` provides a simple and
efficient implementation of the LiPol algorithm in
Rust, making it a useful tool for smoothly
transitioning between audio values in digital
audio processing applications.

### Mathematical Analysis

The LIPOL algorithm is a form of linear
interpolation that approximates values between two
points using a straight line. Given two points
(x0, y0) and (x1, y1), the LIPOL algorithm can be
used to approximate a value y at a point x between
x0 and x1. The formula for LIPOL is:

```
y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
```

The surge-lipol crate provides an implementation
of the LIPOL algorithm that can be used for
digital signal processing in the context of the
surge synthesizer system. It also includes various
tests to ensure the correctness of the
implementation.

Overall, the surge-lipol crate provides a powerful
tool for approximating values in between two
points and is useful for digital audio signal
processing.

6> /file src/lipol_ps.rs

This code appears to be an implementation of
a linear interpolator algorithm for audio
processing, with support for both scalar and
vector operations. The `LiPol` struct is a generic
struct that stores the current audio value, the
target audio value, the rate of change between the
two, and other necessary variables for performing
the linear interpolation. The `LipolPs` struct is
a specialized struct that uses the SSE instruction
set to perform vector operations on the audio
values. It also provides additional functions for
setting the target audio value in various ways and
for initializing and storing blocks of line
segments. Overall, the code seems to be
a well-optimized implementation of a common audio
processing algorithm.

7> /file src/mac.rs

This is more code from the same crate. The two
functions `mac_block_to` and `mac_2_blocks_to` use
the `LipolPs` struct to perform
a multiply-accumulate operation on blocks of data.

The `mac_block_to` function takes a source block
of `f32` values at `src` and a destination block
of `f32` values at `dst`, and multiplies each
element in the source block with the corresponding
element in a line segment defined by the `LipolPs`
struct, and then adds the result to the
corresponding element in the destination block.

The `mac_2_blocks_to` function is similar, but
takes two source blocks at `src1` and `src2`, and
two destination blocks at `dst1` and `dst2`, and
multiplies each element in the corresponding pair
of source blocks with the corresponding element in
a line segment defined by the `LipolPs` struct,
and then adds the result to the corresponding
element in the corresponding pair of destination
blocks.

Both functions use a macro `lipol_ps_sse_block!`
to iterate over the blocks of data and apply the
operation to each element. The `unsafe` keyword is
used to indicate that the functions may access
memory in an unsafe manner, and that the caller is
responsible for ensuring that the memory is valid
and properly aligned. The `TryInto` trait is used
to convert the `NQ` argument to a `usize` value,
and any errors are reported using the `Debug`
trait.

8> /file src/multiply.rs

This is another part of the `crate::LipolPs`
module. It contains several unsafe functions that
are used for multiplying and saturating blocks of
f32 values using the Single Instruction Multiple
Data (SIMD) instructions provided by the SSE
(Streaming SIMD Extensions) instruction set.

The first function `multiply_block_to` multiplies
a block of `f32` values in the `src` pointer with
two `__m128` vectors, `y1` and `y2`, and stores
the result in the `dst` pointer. The
multiplication is done in blocks of 2.

The second function `multiply_2_blocks_to`
multiplies two blocks of `f32` values in the
`src1` and `src2` pointers with two `__m128`
vectors, `y1` and `y2`, and stores the result in
the `dst1` and `dst2` pointers. The multiplication
is done in blocks of 2.

The third function `multiply_2_blocks` multiplies
two blocks of `f32` values in the `src1` and
`src2` pointers with two `__m128` vectors, `y1`
and `y2`. The multiplication is done in blocks of
2.

The fourth function `multiply_block` multiplies
a block of `f32` values in the `src` pointer with
two `__m128` vectors, `y1` and `y2`. The
multiplication is done in blocks of 8.

The fifth function `multiply_block_sat1` saturates
the result of the multiplication to a maximum
value of 1.0. The multiplication is done in blocks
of 8.

9> /file src/sse_block.rs

This is a Rust macro definition that generates
code for performing a specific calculation using
SIMD instructions. The macro is named
`lipol_ps_sse_block`, and it takes four arguments:
an identifier `$self`, an expression `$nquads`, an
expression `$n`, and a code block `$code`.

The macro generates code that performs a loop over
the range `0..$nquads` or `0..($nquads << 2)`
(depending on the value of `$n`). The loop body
executes the `$code` block, passing it four
arguments: an index `i`, and three mutable 128-bit
SIMD vectors of floating-point numbers named `y1`,
`y2`, and `dy`.

The `y1` and `dy` variables are initialized to
zero using the `z128![]` macro, and then
initialized with appropriate values for the
specific calculation being performed using the
`initblock` method on the `$self` object.

The `y2` variable is computed by adding `y1` and
`dy` element-wise using the `_mm_add_ps`
function. The `dy` variable is updated by
multiplying it by a constant value defined using
the `m128_two![]` macro.

The `$code` block contains the actual computation
being performed using SIMD instructions./f It uses
the `_mm_addi_ps` and `_mm_mul_ps` functions to
performle addition and multiplication of 128-bit
SIMD vectors of floating-point numbers,
respectively.

10> /file src/trixpan.rs

This code defines the implementation of several
methods for the `LipolPs` struct.

The `trixpan_left` and `trixpan_right` methods
perform a specific panning calculation on two
input channels, which is used in the
`trixpan_blocks` method. These methods use SIMD
instructions to perform the calculations.

The `trixpan_blocks_do_quad` method takes pointers
to left and right channels, pointers to output
left and right channels, a mutable reference to
a SIMD vector `y`, and a mutable reference to
a SIMD vector `dy`. It uses `trixpan_left` and
`trixpan_right` to calculate the left and right
output channels for a quad of samples, and updates
`y` and `dy` with the new values for the next quad
of samples.

The `trixpan_blocks` method takes pointers to left
and right channels, pointers to output left and
right channels, and a number of quads to
process. It initializes `y` and `dy`, and then
calls `trixpan_blocks_do_quad` for each quad of
samples, passing the appropriate pointers and
references.

The code makes use of several macros defined in
the `ix` module, including `z128!`, `m128_one!`,
and `m128_two!`, which initialize SIMD vectors to
zero, one, and two, respectively. It also defines
the `lipol_ps_sse_block` macro, which generates
code that performs a specific calculation using
SIMD instructions.

11> /file src/fade.rs

These two unsafe methods belong to the `LipolPs`
struct implementation in Rust, and they are used
for fading blocks of audio data. Both methods take
a pointer to the source buffer(s), a pointer to
the destination buffer(s), and an integer `nquads`
that specifies the number of quads (groups of four
`f32` values) to process. The methods use SSE
intrinsics to perform the fades, which allows for
parallel processing of the data using SIMD
instructions.

The first method, `fade_block_to`, takes two
source buffers `src1` and `src2` and fades them
into the destination buffer `dst`. The fade is
based on the current state of the `LipolPs`
object, which determines the fading
parameters. The method is unsafe because the user
needs to make sure that the input buffers have at
least `nquads` elements, and the function can
safely access them.

The second method, `fade_2_blocks_to`, is similar
to the first one, but it takes four source buffers
`src11`, `src12`, `src21`, and `src22`, and fades
them into two destination buffers `dst1` and
`dst2`. The method performs two fades in parallel,
writing the results to `dst1` and `dst2`
respectively. Like the first method, this method
is also unsafe, and the user needs to make sure
that the input buffers have at least `nquads`
elements and the function can safely access them.

Both methods use a `lipol_ps_sse_block!` macro,
which provides a loop over the data that needs to
be processed, taking into account the fact that
SSE instructions operate on 128-bit registers, and
therefore the data needs to be processed in 4-byte
(32-bit) chunks. The methods also use the
`_mm_mul_ps` and `_mm_add_ps` functions, which are
intrinsic functions that allow for low-level
manipulation of 128-bit SSE registers.

12> /file src/add.rs

This is the implementation of two methods for the
`LipolPs` struct: `add_block` and
`subtract_block`. Both of them take a pointer to
a memory region and the number of quadwords to
operate on, and use vectorized SSE instructions to
perform the corresponding operation on the
quadwords in the memory region.

Both methods are marked as `unsafe` because the
caller is responsible for ensuring that the memory
pointed to by the pointer is valid and that the
quadwords can be accessed
contiguously. Additionally, the pointer must be
a valid pointer to a mutable slice of `f32`
values.

The `add_block` method adds the given block of
quadwords to the `LipolPs` instance, while the
`subtract_block` method subtracts the
corresponding values from the current state for
a block of quadwords in the memory region.

These methods use a macro called
`lipol_ps_sse_block!` which generates SSE2 vector
code that operates on a block of quadwords. The
macro takes a closure which operates on four
references to `__m128` values, which are used to
hold the current state of the `LipolPs`
instance. The closure also takes an index value,
which is used to access the quadwords in the
memory region.

Overall, these methods provide efficient
vectorized operations on blocks of quadwords,
making use of the SSE2 instructions provided by
modern CPUs.

4> thanks! could you teach me how a linear
interpolator works?

Sure! I'd be happy to explain how a linear
interpolator works.

Linear interpolation is a method of approximating
values in between two known points by drawing
a straight line between them. The basic idea is to
use the slope of the line connecting the two
points to estimate the value at a point between
them. The closer the point is to one of the known
points, the closer the approximation will be to
the actual value at that point.

In the context of digital signal processing,
linear interpolation is often used to increase the
sample rate of a signal. For example, if a signal
is sampled at a rate of 44100 samples per second,
but we need to process it at a rate of 48000
samples per second, we can use linear
interpolation to estimate the values of the signal
at the extra sample points. We would take two
adjacent samples of the signal and draw a straight
line between them. Then we would use the slope of
that line to estimate the value of the signal at
the extra sample points.

To illustrate this, let's say we have a signal
with two samples: x0 = 1 and x1 = 5, and their
corresponding values: y0 = 2 and y1 = 4. If we
want to estimate the value of the signal at
a point x = 2.5, we can use the LIPOL formula:

```
y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
```

Plugging in the values, we get:

```
y = 2 + (2.5 - 1) * ((4 - 2) / (5 - 1))
  = 2 + 1.5 * (2 / 4)
  = 2.75
```

So the estimated value of the signal at x = 2.5 is
2.75.

Of course, this is just a simple example. In
practice, we would use many more samples to
interpolate the signal and reduce the error in the
estimation. Additionally, there are other
interpolation methods such as cubic interpolation
that can be used to further improve the accuracy
of the estimation.

5> which methods do we have for linear
interpolation?

Linear interpolation involves approximating values
in between two known points by drawing a straight
line between them. There is only one formula for
linear interpolation, which is commonly referred
to as the "LIPOL" formula. LIPOL stands for
"linear polar" and is also known as the "lerp"
formula or the "lerp" function.

The LIPOL formula is:

```
y = y0 + (x - x0) * ((y1 - y0) / (x1 - x0))
```

where x0 and y0 are the coordinates of the first
known point, x1 and y1 are the coordinates of the
second known point, x is the x-coordinate of the
point we want to approximate, and y is the
estimated value of the point.

While there is only one formula for linear
interpolation, there are different ways to
implement it in code. Some programming languages
provide built-in functions or libraries for linear
interpolation, while others require manual
implementation. In general, linear interpolation
is a simple and computationally efficient method
for approximating values in between two known
points, and is widely used in digital signal
processing and computer graphics.

6> which instructions do we use for this on arm?

On ARM processors, the instructions used for
linear interpolation depend on the specific
architecture and instruction set. However, the
most common instruction used for linear
interpolation on ARM is the FMA (Fused
Multiply-Add) instruction. The FMA instruction is
a single instruction that performs
a multiplication and addition in one step, which
can improve performance in some cases.

In particular, the ARM NEON instruction set
provides a set of SIMD (Single Instruction
Multiple Data) instructions that can be used for
linear interpolation. These instructions operate
on multiple data elements in parallel, which can
be useful for processing large blocks of audio or
video data.

The NEON instruction set includes instructions for
loading and storing data from memory, as well as
arithmetic and logical operations on vectors of
data. To perform linear interpolation using NEON
instructions, you would typically load the input
data into a vector register, apply the LIPOL
formula to the vector elements, and then store the
output data back to memory.

Here's an example of how you might use NEON
instructions to perform linear interpolation in
ARM assembly language:

```asm
// Load input data into vector registers
vld1.32 {d0}, [r0]  // Load x0 and y0 into d0
vld1.32 {d1}, [r1]  // Load x1 and y1 into d1
vld1.32 {d2}, [r2]  // Load x values into d2

// Compute slope and delta x
vsub.f32 d3, d1, d0  // Compute delta y (y1 - y0) in d3
vsub.f32 d4, d2, d0  // Compute delta x (x - x0) in d4
vdiv.f32 d5, d3, d4  // Compute slope (delta y / delta x) in d5

// Compute interpolated values
vsub.f32 d6, d2, d0  // Compute delta x (x - x0) in d6
vmul.f32 d7, d6, d5  // Multiply delta x by slope to get delta y (d7)
vadd.f32 d8, d7, d0  // Add delta y to y0 to get interpolated value (d8)

// Store output data to memory
vst1.32 {d8}, [r3]  // Store interpolated values back to memory
```

This example loads the input data into vector
registers using the `vld1` instruction, computes
the slope and delta x using the `vsub` and `vdiv`
instructions, computes the interpolated values
using the `vmul` and `vadd` instructions, and
stores the output data back to memory using the
`vst1` instruction. This code assumes that the
input and output data are 32-bit floating-point
values, and that the input data is stored in
memory at addresses `r0`, `r1`, and `r2`, and that
the output data is stored in memory at address
`r3`.

7> what are some caveats when we do linear
interpolation? are there better forms of
interpolation for certain tasks?

While linear interpolation is a simple and useful
technique, there are some caveats to be aware of:

1. Linear interpolation assumes a straight line
   between the two known points. If the function
   being interpolated is not linear, the
   approximation may be inaccurate, especially for
   values far away from the known points.

2. Linear interpolation can introduce artifacts
   such as "aliasing" in digital signal
   processing. Aliasing occurs when the signal
   being interpolated contains high-frequency
   components that are not properly filtered,
   resulting in distortion or noise in the output.

3. Linear interpolation can also introduce
   "overshoot" or "undershoot" errors, especially
   if the input signal has discontinuities or
   sharp changes in slope. Overshoot or undershoot
   errors occur when the approximation exceeds the
   actual value or falls short of it,
   respectively.

To address these issues, there are several other
interpolation methods that may be more appropriate
for certain tasks:

1. Piecewise linear interpolation: This method
   involves dividing the input range into smaller
   intervals and performing linear interpolation
   separately for each interval. This can reduce
   the error in the approximation and better
   capture the behavior of nonlinear functions.

2. Polynomial interpolation: This method involves
   fitting a polynomial of degree n to the n+1
   known points, and using the polynomial to
   approximate values in between them. This can
   provide a more accurate approximation than
   linear interpolation, especially for functions
   with higher degrees of curvature.

3. Cubic spline interpolation: This method
   involves fitting a piecewise cubic polynomial
   to the known points, such that the function and
   its first and second derivatives are continuous
   across the interval. This can provide a smooth
   and accurate approximation, and is commonly
   used in computer graphics and animation.

4. Fourier series interpolation: This method
   involves approximating the function as a sum of
   sine and cosine waves of varying frequency, and
   using the Fourier coefficients to estimate
   values in between the known points. This can
   provide a highly accurate approximation for
   periodic functions, but may be less suitable
   for non-periodic functions.

Overall, the choice of interpolation method
depends on the specific task and the
characteristics of the function being
approximated. Linear interpolation is a useful and
simple method, but may not always be the best
choice for every situation.

8> could you describe how each of these four
methods applies to the domain of audio?

Sure, I'd be happy to describe how each of these
four interpolation methods applies to the domain
of audio.

1. Piecewise linear interpolation: This method is
   commonly used in audio signal processing for
   up-sampling (increasing the sampling rate of
   a signal). In this context, the input signal is
   divided into small intervals, and linear
   interpolation is used to estimate the values of
   the signal at the extra sample
   points. Piecewise linear interpolation is
   simple and computationally efficient, and can
   provide a reasonable approximation of the
   original signal at a higher sampling rate.

2. Polynomial interpolation: This method is less
   commonly used in audio signal processing due to
   its tendency to introduce "ringing" artifacts
   in the output signal. Ringing occurs when the
   polynomial overshoots or undershoots the actual
   signal, resulting in oscillations or "echoes"
   in the output. However, polynomial
   interpolation can be useful for certain tasks
   such as frequency analysis or spectral
   modeling, where a smooth and accurate
   representation of the signal is desired.

3. Cubic spline interpolation: This method is
   commonly used in audio signal processing for
   a variety of tasks such as pitch shifting, time
   stretching, and resampling. In these contexts,
   cubic spline interpolation can provide a smooth
   and accurate approximation of the original
   signal, while preserving important features
   such as transients and harmonics. Cubic spline
   interpolation can also be used for waveform
   shaping and envelope generation, where a smooth
   and continuous curve is desired.

4. Fourier series interpolation: This method is
   commonly used in audio signal processing for
   waveform synthesis and resynthesis. In this
   context, the Fourier series coefficients of
   a waveform are computed, and used to synthesize
   or resynthesize the waveform at different
   frequencies or sampling rates. Fourier series
   interpolation can provide a highly accurate
   representation of the original waveform,
   especially for periodic signals, and can be
   used for tasks such as additive synthesis,
   frequency modulation, and wavetable synthesis.

Overall, the choice of interpolation method in
audio signal processing depends on the specific
task and the desired characteristics of the output
signal. While piecewise linear interpolation is
simple and computationally efficient, other
methods such as cubic spline and Fourier series
interpolation can provide a more accurate and
versatile representation of the original signal.
