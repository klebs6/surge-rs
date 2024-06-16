crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum ReverbParam {
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

impl_trait_defaults!{
    ReverbParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetModulation,
    GetSnap,
    SetModulation,
    GetExtendedValue,
}

impl GetControlGroup for ReverbParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for ReverbParam {

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
}

impl GetDefaultParameterValue for ReverbParam {

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
}

impl CheckIfModulateable for ReverbParam {

    fn modulateable(&self) -> bool {
        !matches![self, ReverbParam::PreDelay]
    }
}

impl GetMinParameterValue for ReverbParam {

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
}

impl GetMaxParameterValue for ReverbParam {

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
}

impl GetParameterValueType for ReverbParam {

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
}

impl GetMoverate for ReverbParam {

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
