# surge-imports

This crate encapsulates the third-party dependencies
for the `surge*` Rust workspace.

The `surge*` Rust workspace consists of multiple crates
that make up the Surge synthesizer system. This crate
imports and re-exports the necessary dependencies for
the system, such as SIMD instruction sets, FFT
algorithms, and more.

By using this crate, the user of the `surge*` system can
more easily manage the dependencies and ensure
compatibility between the different subcomponents of
the system.
