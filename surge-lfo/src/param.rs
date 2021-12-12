ix!();

/**
  | I have used the ordering here in CLFOGui to
  | iterate.
  |
  | Be careful if rate or release move from
  | first/last position.
  */
enhanced_enum![
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

rt![LfoParam];

impl Param for LfoParam {/* TODO */
    fn control_group(&self) -> ControlGroup { ControlGroup::Lfo } 
}

impl LfoParam {
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
