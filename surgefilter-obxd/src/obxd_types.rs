crate::ix!();

pub const OBXD_SSEW: i32 = 4;

coeffidx![
    Obxd12dBCoeff;
    G12,
    R12,
    MultiMode,
    BandPass,
    SelfOscPush
];

coeffidx![
    Obxd24dBCoeff;
    G24,
    R24,
    Rcor24,
    Rcor24inv,
    PoleMix,       // mm
    PoleMixInvInt, // mmch
    PoleMixScaled // mmt
];

enhanced_enum![
    ObxdParams {
        S1,
        S2,
        S3,
        S4,
    }
];

enhanced_enum![
    Poles {
        TwoPole,
        FourPole
    }
];



