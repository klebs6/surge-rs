crate::ix!();

enhanced_enum![
    AllpassReverbParam {
        PreDelay,
        RoomSize,
        DecayTime,
        Diffusion,
        BuildUp,
        Modulation,
        LFDamping,
        HFDamping,
        Mix,
        Width,
        ReturnLevel,
    }
];

rt![AllpassReverbParam];

impl ParameterInterface for AllpassReverbParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
    fn control_type(&self) -> ControlType {
        match self {
            AllpassReverbParam::PreDelay    => ControlType::ReverbPreDelayTime,
            AllpassReverbParam::RoomSize    => ControlType::PercentBidirectional,
            AllpassReverbParam::DecayTime   => ControlType::ReverbTime,
            AllpassReverbParam::Diffusion   => ControlType::Percent,
            AllpassReverbParam::BuildUp     => ControlType::Percent,
            AllpassReverbParam::Modulation  => ControlType::Percent,
            AllpassReverbParam::LFDamping   => ControlType::Percent,
            AllpassReverbParam::HFDamping   => ControlType::Percent,
            AllpassReverbParam::Mix         => ControlType::Percent,
            AllpassReverbParam::Width       => ControlType::Percent,
            AllpassReverbParam::ReturnLevel => ControlType::Percent,
        }
    }
    fn default_value(&self) -> PData {
        match self {
            AllpassReverbParam::PreDelay    => PData::Float(-4.0),
            AllpassReverbParam::RoomSize    => PData::Float(0.0),
            AllpassReverbParam::DecayTime   => PData::Float(0.75),
            AllpassReverbParam::Diffusion   => PData::Float(1.0),
            AllpassReverbParam::BuildUp     => PData::Float(1.0),
            AllpassReverbParam::Modulation  => PData::Float(0.5),
            AllpassReverbParam::LFDamping   => PData::Float(0.2),
            AllpassReverbParam::HFDamping   => PData::Float(0.2),
            AllpassReverbParam::Mix         => PData::Float(0.33),
            AllpassReverbParam::Width       => PData::Float(0.75),
            AllpassReverbParam::ReturnLevel => PData::Float(0.75),
        }
    }
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
    fn min_value(&self) -> PData {
        match self {
            AllpassReverbParam::PreDelay    => PData::Float(-4.0),
            AllpassReverbParam::RoomSize    => PData::Float(-1.0),
            AllpassReverbParam::DecayTime   => PData::Float(-4.0),
            AllpassReverbParam::Diffusion   => PData::Float(0.0),
            AllpassReverbParam::BuildUp     => PData::Float(0.0),
            AllpassReverbParam::Modulation  => PData::Float(0.0),
            AllpassReverbParam::LFDamping   => PData::Float(0.0),
            AllpassReverbParam::HFDamping   => PData::Float(0.0),
            AllpassReverbParam::Mix         => PData::Float(0.0),
            AllpassReverbParam::Width       => PData::Float(0.0),
            AllpassReverbParam::ReturnLevel => PData::Float(0.0),
        }
    }
    fn max_value(&self) -> PData {
        match self {
            AllpassReverbParam::PreDelay    => PData::Float(1.0),
            AllpassReverbParam::RoomSize    => PData::Float(1.0),
            AllpassReverbParam::DecayTime   => PData::Float(6.0),
            AllpassReverbParam::Diffusion   => PData::Float(1.0),
            AllpassReverbParam::BuildUp     => PData::Float(1.0),
            AllpassReverbParam::Modulation  => PData::Float(1.0),
            AllpassReverbParam::LFDamping   => PData::Float(1.0),
            AllpassReverbParam::HFDamping   => PData::Float(1.0),
            AllpassReverbParam::Mix         => PData::Float(1.0),
            AllpassReverbParam::Width       => PData::Float(1.0),
            AllpassReverbParam::ReturnLevel => PData::Float(1.0),
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            AllpassReverbParam::PreDelay    => ValType::VtFloat,
            AllpassReverbParam::RoomSize    => ValType::VtFloat,
            AllpassReverbParam::DecayTime   => ValType::VtFloat,
            AllpassReverbParam::Diffusion   => ValType::VtFloat,
            AllpassReverbParam::BuildUp     => ValType::VtFloat,
            AllpassReverbParam::Modulation  => ValType::VtFloat,
            AllpassReverbParam::LFDamping   => ValType::VtFloat,
            AllpassReverbParam::HFDamping   => ValType::VtFloat,
            AllpassReverbParam::Mix         => ValType::VtFloat,
            AllpassReverbParam::Width       => ValType::VtFloat,
            AllpassReverbParam::ReturnLevel => ValType::VtFloat,
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            AllpassReverbParam::PreDelay    => 1.0,
            AllpassReverbParam::RoomSize    => 1.0,
            AllpassReverbParam::DecayTime   => 1.0,
            AllpassReverbParam::Diffusion   => 1.0,
            AllpassReverbParam::BuildUp     => 1.0,
            AllpassReverbParam::Modulation  => 1.0,
            AllpassReverbParam::LFDamping   => 1.0,
            AllpassReverbParam::HFDamping   => 1.0,
            AllpassReverbParam::Mix         => 1.0,
            AllpassReverbParam::Width       => 1.0,
            AllpassReverbParam::ReturnLevel => 1.0,
        }
    }
}

impl AllpassReverbParam {
    pub fn new_runtime() -> AllpassReverbParamArrayRT {
        AllpassReverbParamArrayRT::new_with(|x| match x {
            AllpassReverbParam::PreDelay    => AllpassReverbParamRT::new(AllpassReverbParam::PreDelay),
            AllpassReverbParam::RoomSize    => AllpassReverbParamRT::new(AllpassReverbParam::RoomSize),
            AllpassReverbParam::DecayTime   => AllpassReverbParamRT::new(AllpassReverbParam::DecayTime),
            AllpassReverbParam::Diffusion   => AllpassReverbParamRT::new(AllpassReverbParam::Diffusion),
            AllpassReverbParam::BuildUp     => AllpassReverbParamRT::new(AllpassReverbParam::BuildUp),
            AllpassReverbParam::Modulation  => AllpassReverbParamRT::new(AllpassReverbParam::Modulation),
            AllpassReverbParam::LFDamping   => AllpassReverbParamRT::new(AllpassReverbParam::LFDamping),
            AllpassReverbParam::HFDamping   => AllpassReverbParamRT::new(AllpassReverbParam::HFDamping),
            AllpassReverbParam::Mix         => AllpassReverbParamRT::new(AllpassReverbParam::Mix),
            AllpassReverbParam::Width       => AllpassReverbParamRT::new(AllpassReverbParam::Width),
            AllpassReverbParam::ReturnLevel => AllpassReverbParamRT::new(AllpassReverbParam::ReturnLevel),
        })
    }
}
