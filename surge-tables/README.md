# surge-tables

**SurgeTables: Waveshaping and Envelope Utilities for the Surge Synthesizer System**

SurgeTables is a Rust crate that provides
a variety of waveshaping, envelope, and sinc
tables for the Surge Synthesizer System. It
includes utilities for generating and accessing
tables, as well as mathematical functions to
support synthesis operations.

# SurgeTables

SurgeTables is a collection of tables and
utilities used in the Surge Synthesizer System,
supporting various waveshaping, envelope, and sinc
operations. The crate is divided into several
subcomponents, each focusing on a specific aspect
of sound synthesis.

## SineTables

SineTables contains precomputed sine wave values
for efficient lookup during synthesis. These sine
wave values are stored in different formats, such
as `f32`, `f64`, and `i16`.

## WaveshapeTables

WaveshapeTables provides tables and utilities for
waveshaping synthesis. Waveshaping is a nonlinear
distortion technique used to modify the harmonic
content of a signal. It supports functions such as
`lookup_waveshape` and `lookup_waveshape_warp` for
accessing these tables and applying the desired
transformations.

## GainTables

GainTables provides utilities for converting
between decibel (dB) and linear scales, as well as
functions for applying clipping and scaling to
audio signals. The `db_to_linear` function, for
example, converts a decibel value to its
corresponding linear gain value:

```
linear_gain = 10^(dB/20)
```

## EnvelopeTables

EnvelopeTables provides tables and functions for
handling the linear and low-pass filtered (LPF)
envelope rates in the synthesizer. The
`envelope_rate_linear` and `envelope_rate_lpf`
functions provide utilities for controlling these
rates.

## SincTables

SincTables contains tables and utilities for
windowed sinc functions, which are used for
resampling and other signal processing tasks. The
`windowed_sinc` function generates a sinc function
windowed by a specified window function:

```
windowed_sinc(x) = sinc(x) * window_function(x)
```

where `sinc(x) = sin(πx) / (πx)` for `x ≠ 0` and
`sinc(0) = 1`.

## MaybeOwningTablesHandle and TablesHandle

These types provide a convenient way to handle and
manage the lifetime of the tables created by the
various subcomponents in the crate.

---

With SurgeTables, users can efficiently access and
manipulate a variety of tables and mathematical
functions to support the waveshaping, envelope,
and sinc operations in the Surge Synthesizer
System.

2> /file surge-rs/surge-tables/src/envelope.rs

**EnvelopeTables**

1. What is this table used for?

   The `EnvelopeTables` struct contains two
   tables: `table_envrate_lpf` and
   `table_envrate_linear`. These tables are used
   for efficient lookup of envelope rates in the
   Surge Synthesizer System. The envelope rates
   determine how quickly an envelope progresses
   through its stages (attack, decay, sustain,
   release) and are crucial for shaping the
   amplitude and timbre of a synthesized sound.

2. Which calculations are involved in the
   precomputation?

   The precomputation involves the calculation of
   two tables:

   - `table_envrate_lpf`: This table contains
     values related to the low-pass filtered (LPF)
     envelope rates. The precomputation involves
     an exponential decay calculation with a time
     constant based on the sample rate and
     a power-of-two mapping.

   - `table_envrate_linear`: This table contains
     values related to the linear envelope
     rates. The precomputation involves a simple
     division by the time constant based on the
     sample rate and a power-of-two mapping.

3. How much performance are we saving?

   By precomputing these envelope rate values and
   storing them in tables, we save performance by
   avoiding the need to recalculate these values
   in real-time during the synthesis process. This
   can lead to significant performance
   improvements, especially when dealing with
   complex synthesizer patches and polyphony.

4. How can we improve this table?

   Some possible improvements to the
   `EnvelopeTables` include:

   - Interpolation: The current implementation
     uses linear interpolation to access the table
     values. Other interpolation methods, such as
     cubic or spline interpolation, could provide
     more accurate results at the cost of
     additional complexity.

   - Adaptive table size: The current
     implementation uses a fixed table size of
     512. Allowing for user-defined table sizes
     could enable users to balance memory usage
     and lookup accuracy based on their specific
     needs.

   - Alternative precomputation methods: Exploring
     different methods for generating the envelope
     rate values in the precomputation step could
     lead to more accurate or faster results. This
     may involve using different mathematical
     approximations or algorithms for the
     exponential decay and linear rate
     calculations.

3> /file surge-rs/surge-tables/src/gain.rs

**GainTables**

1. What is this table used for?

   The `GainTables` struct contains a table called
   `table_db`, which is used for efficient
   conversion between decibel (dB) and linear
   scales. This is useful for operations involving
   gain, attenuation, or other processes where
   logarithmic scales are involved. The table also
   provides a `clipscale` function to calculate
   the scaling factor depending on the filter
   subtype (Rough or Smooth).

2. Which calculations are involved in the
   precomputation?

   The precomputation involves the calculation of
   a single table:

   - `table_db`: This table contains values for
     converting decibel values to linear gain
     values. The precomputation involves
     calculating the linear gain for a given dB
     value using the formula: `linear_gain
     = 10^(0.05 * (dB - 384))`.

3. How much performance are we saving?

   By precomputing these decibel-to-linear gain
   values and storing them in a table, we save
   performance by avoiding the need to recalculate
   these values in real-time during the synthesis
   process. This can lead to noticeable
   performance improvements, especially when
   processing audio signals that require frequent
   conversions between decibel and linear scales.

4. How can we improve this table?

   Some possible improvements to the `GainTables`
   include:

   - Interpolation: The current implementation
     uses linear interpolation to access the table
     values. Other interpolation methods, such as
     cubic or spline interpolation, could provide
     more accurate results at the cost of
     additional complexity.

   - Adaptive table size: The current
     implementation uses a fixed table size of
     512. Allowing for user-defined table sizes
     could enable users to balance memory usage
     and lookup accuracy based on their specific
     needs.

   - Additional gain-related functions: Expanding
     the `GainTables` struct to include other
     gain-related functions, such as
     linear-to-decibel conversions or different
     types of gain scaling, could make the table
     more versatile and useful for a wider range
     of audio processing tasks.

4> /file surge-rs/surge-tables/src/sinc.rs

**SincTables**

1. What is this table used for?

   The `SincTables` struct contains three tables:
   `table`, `table_1x`, and `table_i16`. These
   tables store precomputed windowed sinc function
   values, which are commonly used in digital
   signal processing, particularly in
   interpolation and resampling tasks. By
   precomputing these values, the `SincTables` can
   be used to perform efficient interpolation in
   real-time audio processing.

2. Which calculations are involved in the
   precomputation?

   The precomputation involves the calculation of
   three tables:

   - `table`: This table stores windowed sinc
     function values for a specific cutoff
     frequency `cutoff`.

   - `table_1x`: This table stores windowed sinc
     function values for a different cutoff
     frequency `cutoff1x`.

   - `table_i16`: This table stores windowed sinc
     function values for another cutoff frequency
     `cutoff_i16`, but in 16-bit integer format.

   The precomputation calculates the windowed sinc
   function values using the following steps:

   1. Calculate the time `t` for each table entry
      using the `get_table_t` function.

   2. Calculate the windowed sinc function value
      using the `windowed_sinc` function, which
      involves the product of a symmetric Blackman
      window and the sinc function with the given
      cutoff frequency.

   3. Store the calculated values in the
      respective tables.

3. How much performance are we saving?

   By precomputing the windowed sinc function
   values and storing them in tables, the
   performance of real-time audio processing
   tasks, such as interpolation and resampling,
   can be significantly improved. This is because
   the computationally expensive sinc and Blackman
   window calculations are avoided during
   real-time processing, and the precomputed
   values can be quickly accessed from the tables.

4. How can we improve this table?

   Some possible improvements to the `SincTables`
   include:

   - Adaptive table sizes: Allowing for
     user-defined table sizes could enable users
     to balance memory usage and lookup accuracy
     based on their specific needs.

   - Additional interpolation methods: Expanding
     the `SincTables` struct to include other
     interpolation methods could make the table
     more versatile and useful for a wider range
     of audio processing tasks.

   - Optimizing sinc and window functions: The
     current implementation calculates the
     windowed sinc values using standard Blackman
     window and sinc functions. Optimized versions
     of these functions, such as those found in
     specialized libraries, could potentially
     improve the accuracy or performance of the
     tables.

5> /file surge-rs/surge-tables/src/sine.rs

**SineTables**

1. What is this table used for?

   The `SineTables` struct contains two tables:
   `table_sin` and `table_sin_offset`. These
   tables store precomputed sine function values
   and their corresponding offsets. By
   precomputing these values, the `SineTables` can
   be used to perform efficient trigonometric
   calculations, such as oscillations and
   frequency modulation, in real-time audio
   processing.

2. Which calculations are involved in the
   precomputation?

   The precomputation involves the calculation of
   two tables:

   - `table_sin`: This table stores sine function
     values for angles between 0 and 2π, uniformly
     spaced with a step size of 2π
     / `SIN_TABLE_SIZE_D`.

   - `table_sin_offset`: This table stores the
     difference between consecutive sine function
     values in `table_sin`.

   The precomputation calculates the sine function
   values and their offsets using the following
   steps:

   1. Calculate the angle `t` for each table entry
      as 2π * (i / `SIN_TABLE_SIZE_D`), where i is
      the index of the entry.

   2. Calculate the sine function value `sin(t)`
      and store it in `table_sin`.

   3. Calculate the sine function value `sin(t
      + 2π / SIN_TABLE_SIZE_D)` and store the
      difference between this value and `sin(t)`
      in `table_sin_offset`.

3. How much performance are we saving?

   By precomputing the sine function values and
   their offsets and storing them in tables, the
   performance of real-time audio processing tasks
   that involve trigonometric calculations can be
   significantly improved. This is because the
   computationally expensive sine calculations are
   avoided during real-time processing, and the
   precomputed values can be quickly accessed from
   the tables.

4. How can we improve this table?

   Some possible improvements to the `SineTables`
   include:

   - Adaptive table sizes: Allowing for
     user-defined table sizes could enable users
     to balance memory usage and lookup accuracy
     based on their specific needs.

   - Additional trigonometric functions: Expanding
     the `SineTables` struct to include other
     trigonometric functions, such as cosine or
     tangent, could make the table more versatile
     and useful for a wider range of audio
     processing tasks.

   - Optimizing sine function calculations: The
     current implementation calculates the sine
     values using the standard sine
     function. Optimized versions of these
     functions, such as those found in specialized
     libraries, could potentially improve the
     accuracy or performance of the tables.

6> /file surge-rs/surge-tables/src/waveshape.rs

**WaveshapeTables**

1. What is this table used for?

   The `WaveshapeTables` struct contains a table
   of precomputed waveshaping function values that
   can be used to quickly modify the shape of
   audio waveforms in real-time audio
   processing. By using these precomputed values,
   the performance of audio processing tasks
   involving waveshaping can be significantly
   improved.

2. Which waveshaping functions are included?

   The `WaveshapeTables` struct includes
   precomputed values for five different
   waveshaping functions:

   1. `wst_tanh`: Hyperbolic tangent function.

   2. `wst_hard`: A hard clipping function, based
      on the fifth power of the absolute value of
      x, followed by a hyperbolic tangent and
      a power of 0.2.

   3. `wst_asym`: An asymmetric waveshaping
      function using the "shafted_tanh" function,
      which is shifted by 0.5.

   4. `wst_sine`: Sine function.

   5. `wst_digi`: A digital-style waveshaping
      function using the hyperbolic tangent
      function with a different scaling factor.

3. How much performance are we saving?

   By precomputing the waveshaping function values
   and storing them in tables, the performance of
   real-time audio processing tasks involving
   waveshaping can be significantly improved. This
   is because the computationally expensive
   waveshaping function calculations are avoided
   during real-time processing, and the
   precomputed values can be quickly accessed from
   the tables.

4. How can we improve this table?

   Some possible improvements to the
   `WaveshapeTables` include:

   - Adaptive table sizes: Allowing for
     user-defined table sizes could enable users
     to balance memory usage and lookup accuracy
     based on their specific needs.

   - Additional waveshaping functions: Expanding
     the `WaveshapeTables` struct to include more
     waveshaping functions could make the table
     more versatile and useful for a wider range
     of audio processing tasks.

   - Optimizing waveshaping function calculations:
     The current implementation calculates the
     waveshaping function values using standard
     functions. Optimized versions of these
     functions, such as those found in specialized
     libraries, could potentially improve the
     accuracy or performance of the tables.

   - Interpolation: Implementing interpolation
     between table entries could improve the
     accuracy of the waveshaping functions when
     using the lookup methods.
