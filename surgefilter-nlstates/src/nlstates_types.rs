crate::ix!();

/// This is an enhanced enumeration that defines different types of non-linear state filters,
/// including low-pass, high-pass, band-pass, notch, and all-pass. 
///
/// It allows you to define these filter types as values of the enumeration, making them easier to
/// use in your code.
///
enhanced_enum![
    NLSFType {
        LowPass,
        HighPass,
        BandPass,
        Notch,
        Allpass,
    }
];

/// This is a coefficient index that specifies the different coefficients used in a non-linear
/// state filter. 
///
/// Specifically, it defines the `A1`, `A2`, `B0`, `B1`, and `B2` coefficients that are used in the
/// filter calculations.
///
coeffidx![
    NLSFCoeff;
    A1,
    A2,
    B0,
    B1,
    B2
];

/// This is another coefficient index that defines the different state variables used in
/// a non-linear state filter. 
///
/// It defines `Z1` and `Z2` as the first and second `z-1` state for the first stage, `Z3` and `Z4`
/// as the first and second `z-1` state for the second stage, and so on up to the fourth stage
/// (`Z7` and `Z8`).
///
coeffidx![
    NLSFState;
    Z1, // 1st z-1 state for first  stage
    Z2, // 2nd z-1 state for first  stage
    Z3, // 1st z-1 state for second stage
    Z4, // 2nd z-1 state for second stage
    Z5, // 1st z-1 state for third  stage
    Z6, // 2nd z-1 state for third  stage
    Z7, // 1st z-1 state for fourth stage
    Z8 // 2nd z-1 state for fourth stage
];

/// `C`: This is a type alias that represents the `NLSFCoeff` type. 
///
/// It is used to make the code more readable and maintainable.
///
pub type C = NLSFCoeff;

/// `R`: This is a type alias that represents the `NLSFState` type. 
///
/// It is used to make the code more readable and maintainable.
///
pub type R = NLSFState;

/// This is an enhanced enumeration that defines different types of saturators that can be used in
/// the non-linear state filter. 
///
/// It includes `Tanh`, which uses a hyperbolic tangent function to saturate the output, and
/// `SoftClip`, which uses a soft-clipping function to saturate the output.
///
enhanced_enum![
    NLSFSaturator {
        Tanh,
        SoftClip,
    }
];
