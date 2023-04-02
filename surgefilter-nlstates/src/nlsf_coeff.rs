crate::ix!();

// This is a coefficient index that specifies the
// different coefficients used in a non-linear
// state filter. 
//
// Specifically, it defines the `A1`, `A2`, `B0`,
// `B1`, and `B2` coefficients that are used in
// the filter calculations.
//
coeffidx!{
    NLSFCoeff;
    A1,
    A2,
    B0,
    B1,
    B2
}
