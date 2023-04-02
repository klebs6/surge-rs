crate::ix!();

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

enhanced_enum![

    // defines different types of saturators that
    // can be used in the non-linear state filter. 
    //
    // It includes `Tanh`, which uses a hyperbolic
    // tangent function to saturate the output, and
    // `SoftClip`, which uses a soft-clipping function
    // to saturate the output.
    //
    NLSFSaturator {
        Tanh,
        SoftClip,
    }
];
