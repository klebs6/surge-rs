ix!();

enhanced_enum![
    AdsrParam {
        Attack,
        Decay,
        Sustain,
        Release,
        AttackShape,
        DecayShape,
        ReleaseShape,
        Mode,
    }
];

rt![AdsrParam];

impl Param for AdsrParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Env } 
}

impl AdsrParam {
    #[inline] pub fn new_runtime() -> AdsrParamArrayRT {
        let x  = AdsrParamArrayRT::new_with( 
            |x| match x {
                AdsrParam::Attack       => AdsrParamRT::new(AdsrParam::Attack),
                AdsrParam::Decay        => AdsrParamRT::new(AdsrParam::Decay),
                AdsrParam::Sustain      => AdsrParamRT::new(AdsrParam::Sustain),
                AdsrParam::Release      => AdsrParamRT::new(AdsrParam::Release),
                AdsrParam::AttackShape  => AdsrParamRT::new(AdsrParam::AttackShape),
                AdsrParam::DecayShape   => AdsrParamRT::new(AdsrParam::DecayShape),
                AdsrParam::ReleaseShape => AdsrParamRT::new(AdsrParam::ReleaseShape),
                AdsrParam::Mode         => AdsrParamRT::new(AdsrParam::Mode),
            }
        );
    }
}
