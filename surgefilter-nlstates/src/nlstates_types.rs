crate::ix!();

enhanced_enum![
    NLSFType {
        LowPass,
        HighPass,
        BandPass,
        Notch,
        Allpass,
    }
];

coeffidx![
    NLSFCoeff;
    A1,
    A2,
    B0,
    B1,
    B2
];

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

pub type C = NLSFCoeff;
pub type R = NLSFState;

enhanced_enum![
    NLSFSaturator {
        Tanh,
        SoftClip,
    }
];
