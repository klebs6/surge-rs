/*!
  | Overall, this code defines the different
  | parameters that can be controlled for an LFO
  | and provides functionality to create runtime
  | representations of these parameters.
  |
  */

crate::ix!();

enhanced_enum![

    // This code defines an enhanced enum called
    // `LfoParam`, which represents the different
    // parameters of an LFO (low-frequency
    // oscillator) that can be controlled. 
    //
    // The `enhanced_enum!` macro generates code for
    // the enum with additional functionality. 
    //
    // I have used the ordering here in CLFOGui to
    // iterate.
    // 
    // Be careful if rate or release move from
    // first/last position.
    //
    LfoParam {
        Rate,
        Shape,
        StartPhase,
        Magnitude,
        Deform,
        Trigmode,
        Unipolar,
        Delay,
        Hold,
        Attack,
        Decay,
        Sustain,
        Release,
    }
];

// The `rt!` macro registers the enum with
// a global registry for runtime parameters.
//
rt![LfoParam];

/// The `ParameterInterface` trait is implemented for
/// `LfoParam`, indicating that it is a parameter
/// that can be controlled. 
///
/// The `control_group` method returns
/// `ControlGroup::Lfo`, which specifies that
/// these parameters are part of the LFO control
/// group.
///
impl ParameterInterface for LfoParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Lfo } 
}

impl LfoParam {

    /// The `LfoParam` enum has a method
    /// `new_runtime()` that returns an array of
    /// `LfoParamRT` values, which are runtime
    /// representations of the `LfoParam` values. 
    ///
    /// The `LfoParamArrayRT` is a wrapper around
    /// an array of `LfoParamRT` values. 
    ///
    /// The `new_with` method takes a closure that
    /// maps each `LfoParam` value to a new
    /// `LfoParamRT` value.
    ///
    pub fn new_runtime() -> LfoParamArrayRT {
        LfoParamArrayRT::new_with( |x| match x {
            LfoParam::Rate       => LfoParamRT::new(LfoParam::Rate),
            LfoParam::Shape      => LfoParamRT::new(LfoParam::Shape),
            LfoParam::StartPhase => LfoParamRT::new(LfoParam::StartPhase),
            LfoParam::Magnitude  => LfoParamRT::new(LfoParam::Magnitude),
            LfoParam::Deform     => LfoParamRT::new(LfoParam::Deform),
            LfoParam::Trigmode   => LfoParamRT::new(LfoParam::Trigmode),
            LfoParam::Unipolar   => LfoParamRT::new(LfoParam::Unipolar),
            LfoParam::Delay      => LfoParamRT::new(LfoParam::Delay),
            LfoParam::Hold       => LfoParamRT::new(LfoParam::Hold),
            LfoParam::Attack     => LfoParamRT::new(LfoParam::Attack),
            LfoParam::Decay      => LfoParamRT::new(LfoParam::Decay),
            LfoParam::Sustain    => LfoParamRT::new(LfoParam::Sustain),
            LfoParam::Release    => LfoParamRT::new(LfoParam::Release),
        })
    }
}
