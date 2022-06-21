crate::ix!();

enhanced_enum![
    ReverbParam {
        PreDelay,
        RoomShape,
        RoomSize,
        DecayTime,
        Damping,
        LowCut,
        Band1Freq,
        Band1Gain,
        HighCut,
        Mix,
        Width,
        ReturnLevel,
    }
];

rt![ReverbParam];

impl Param for ReverbParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 

    fn control_type(&self) -> ControlType {
        match self {
            ReverbParam::PreDelay    => ControlType::EnvTime,
            ReverbParam::RoomShape   => ControlType::ReverbShape,
            ReverbParam::RoomSize    => ControlType::Percent,
            ReverbParam::DecayTime   => ControlType::ReverbTime,
            ReverbParam::Damping     => ControlType::Percent,
            ReverbParam::LowCut      => ControlType::FreqAudible,
            ReverbParam::Band1Freq   => ControlType::FreqAudible,
            ReverbParam::Band1Gain   => ControlType::Decibel,
            ReverbParam::HighCut     => ControlType::FreqAudible,
            ReverbParam::Mix         => ControlType::Percent,
            ReverbParam::Width       => ControlType::DecibelNarrow,
            ReverbParam::ReturnLevel => ControlType::Percent,
        }
    }

    fn default_value(&self) -> PData {
        match self {
            ReverbParam::PreDelay    => PData::Float(-4.0),
            ReverbParam::RoomShape   => PData::Int(0),
            ReverbParam::RoomSize    => PData::Float(0.5),
            ReverbParam::DecayTime   => PData::Float(1.0),
            ReverbParam::Damping     => PData::Float(0.2),
            ReverbParam::LowCut      => PData::Float(-24.0),
            ReverbParam::Band1Freq   => PData::Float(0.0),
            ReverbParam::Band1Gain   => PData::Float(0.0),
            ReverbParam::HighCut     => PData::Float(72.0),
            ReverbParam::Mix         => PData::Float(1.0),
            ReverbParam::Width       => PData::Float(0.0),
            ReverbParam::ReturnLevel => PData::Float(0.5),
        }
    }

    fn modulateable(&self) -> bool {
        !matches![self, ReverbParam::PreDelay]
    }

    fn min_value(&self) -> PData {
        match self {
            ReverbParam::PreDelay    => PData::Float(-8.0),
            ReverbParam::RoomShape   => PData::Int(0),
            ReverbParam::RoomSize    => PData::Float(0.0),
            ReverbParam::DecayTime   => PData::Float(-4.0),
            ReverbParam::Damping     => PData::Float(0.0),
            ReverbParam::LowCut      => PData::Float(-60.0),
            ReverbParam::Band1Freq   => PData::Float(-60.0),
            ReverbParam::Band1Gain   => PData::Float(-48.0),
            ReverbParam::HighCut     => PData::Float(-60.0),
            ReverbParam::Mix         => PData::Float(0.0),
            ReverbParam::Width       => PData::Float(-24.0),
            ReverbParam::ReturnLevel => PData::Float(0.0),
        }
    }

    fn max_value(&self) -> PData {
        match self {
            ReverbParam::PreDelay    => PData::Float(5.0),
            ReverbParam::RoomShape   => PData::Int(3),
            ReverbParam::RoomSize    => PData::Float(1.0),
            ReverbParam::DecayTime   => PData::Float(6.0),
            ReverbParam::Damping     => PData::Float(1.0),
            ReverbParam::LowCut      => PData::Float(70.0),
            ReverbParam::Band1Freq   => PData::Float(70.0),
            ReverbParam::Band1Gain   => PData::Float(48.0),
            ReverbParam::HighCut     => PData::Float(70.0),
            ReverbParam::Mix         => PData::Float(1.0),
            ReverbParam::Width       => PData::Float(24.0),
            ReverbParam::ReturnLevel => PData::Float(1.0),
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            ReverbParam::PreDelay    => ValType::VtFloat,
            ReverbParam::RoomShape   => ValType::VtInt,
            ReverbParam::RoomSize    => ValType::VtFloat,
            ReverbParam::DecayTime   => ValType::VtFloat,
            ReverbParam::Damping     => ValType::VtFloat,
            ReverbParam::LowCut      => ValType::VtFloat,
            ReverbParam::Band1Freq   => ValType::VtFloat,
            ReverbParam::Band1Gain   => ValType::VtFloat,
            ReverbParam::HighCut     => ValType::VtFloat,
            ReverbParam::Mix         => ValType::VtFloat,
            ReverbParam::Width       => ValType::VtFloat,
            ReverbParam::ReturnLevel => ValType::VtFloat,
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            ReverbParam::PreDelay    => 1.0,
            ReverbParam::RoomShape   => 1.0,
            ReverbParam::RoomSize    => 1.0,
            ReverbParam::DecayTime   => 1.0,
            ReverbParam::Damping     => 1.0,
            ReverbParam::LowCut      => 1.0,
            ReverbParam::Band1Freq   => 1.0,
            ReverbParam::Band1Gain   => 1.0,
            ReverbParam::HighCut     => 1.0,
            ReverbParam::Mix         => 1.0,
            ReverbParam::Width       => 1.0,
            ReverbParam::ReturnLevel => 1.0,
        }
    }
}

impl ReverbParam {
    #[inline] pub fn new_runtime() -> ReverbParamArrayRT {
        ReverbParamArrayRT::new_with(|x| match x {
            ReverbParam::PreDelay    => ReverbParamRT::new(ReverbParam::PreDelay),
            ReverbParam::RoomShape   => ReverbParamRT::new(ReverbParam::RoomShape),
            ReverbParam::RoomSize    => ReverbParamRT::new(ReverbParam::RoomSize),
            ReverbParam::DecayTime   => ReverbParamRT::new(ReverbParam::DecayTime),
            ReverbParam::Damping     => ReverbParamRT::new(ReverbParam::Damping),
            ReverbParam::LowCut      => ReverbParamRT::new(ReverbParam::LowCut),
            ReverbParam::Band1Freq   => ReverbParamRT::new(ReverbParam::Band1Freq),
            ReverbParam::Band1Gain   => ReverbParamRT::new(ReverbParam::Band1Gain),
            ReverbParam::HighCut     => ReverbParamRT::new(ReverbParam::HighCut),
            ReverbParam::Mix         => ReverbParamRT::new(ReverbParam::Mix),
            ReverbParam::Width       => ReverbParamRT::new(ReverbParam::Width),
            ReverbParam::ReturnLevel => ReverbParamRT::new(ReverbParam::ReturnLevel),
        })
    }
}
