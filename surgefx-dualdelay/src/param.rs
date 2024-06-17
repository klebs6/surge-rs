crate::ix!();

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
#[synth_parameters]
pub enum DualDelayParam {
    Left,
    Right,
    Feedback,
    CrossFeed,
    LowCut,
    HighCut,
    Rate,
    Depth,
    Pan,
    Mix,
    Width,
    ReturnLevel,
}

impl_trait_defaults!{
    DualDelayParam;
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
    GetExtendRange,
}

impl CheckIfAbsolute for DualDelayParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for DualDelayParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for DualDelayParam {

    fn control_type(&self) -> ControlType {
        match self {
            DualDelayParam::Left        => ControlType::EnvTime,
            DualDelayParam::Right       => ControlType::EnvTime,
            DualDelayParam::Feedback    => ControlType::Amplitude,
            DualDelayParam::CrossFeed   => ControlType::Amplitude,
            DualDelayParam::LowCut      => ControlType::FreqAudible,
            DualDelayParam::HighCut     => ControlType::FreqAudible,
            DualDelayParam::Rate        => ControlType::LfoRate,
            DualDelayParam::Depth       => ControlType::Detuning,
            DualDelayParam::Pan         => ControlType::PercentBidirectional,
            DualDelayParam::Mix         => ControlType::Percent,
            DualDelayParam::Width       => ControlType::DecibelNarrow,
            DualDelayParam::ReturnLevel => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for DualDelayParam {

    fn default_value(&self) -> PData {
        match self {
            DualDelayParam::Left        => PData::Float(-2.0),
            DualDelayParam::Right       => PData::Float(-2.0),
            DualDelayParam::Feedback    => PData::Float(0.0),
            DualDelayParam::CrossFeed   => PData::Float(0.0),
            DualDelayParam::LowCut      => PData::Float(-24.0),
            DualDelayParam::HighCut     => PData::Float(30.0),
            DualDelayParam::Rate        => PData::Float(-2.0),
            DualDelayParam::Depth       => PData::Float(0.0),
            DualDelayParam::Pan         => PData::Float(0.0),
            DualDelayParam::Mix         => PData::Float(1.0),
            DualDelayParam::Width       => PData::Float(0.0),
            DualDelayParam::ReturnLevel => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for DualDelayParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for DualDelayParam {

    fn min_value(&self) -> PData {
        match self {
            DualDelayParam::Left        => PData::Float(-8.0),
            DualDelayParam::Right       => PData::Float(-8.0),
            DualDelayParam::Feedback    => PData::Float(0.0),
            DualDelayParam::CrossFeed   => PData::Float(0.0),
            DualDelayParam::LowCut      => PData::Float(-60.0),
            DualDelayParam::HighCut     => PData::Float(-60.0),
            DualDelayParam::Rate        => PData::Float(-7.0),
            DualDelayParam::Depth       => PData::Float(0.0),
            DualDelayParam::Pan         => PData::Float(-1.0),
            DualDelayParam::Mix         => PData::Float(0.0),
            DualDelayParam::Width       => PData::Float(-24.0),
            DualDelayParam::ReturnLevel => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for DualDelayParam {

    fn max_value(&self) -> PData {
        match self {
            DualDelayParam::Left        => PData::Float(5.0),
            DualDelayParam::Right       => PData::Float(5.0),
            DualDelayParam::Feedback    => PData::Float(1.0),
            DualDelayParam::CrossFeed   => PData::Float(1.0),
            DualDelayParam::LowCut      => PData::Float(70.0),
            DualDelayParam::HighCut     => PData::Float(70.0),
            DualDelayParam::Rate        => PData::Float(9.0),
            DualDelayParam::Depth       => PData::Float(2.0),
            DualDelayParam::Pan         => PData::Float(1.0),
            DualDelayParam::Mix         => PData::Float(1.0),
            DualDelayParam::Width       => PData::Float(24.0),
            DualDelayParam::ReturnLevel => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for DualDelayParam {

    fn value_type(&self) -> ValType {
        match self {
            DualDelayParam::Left        => ValType::VtFloat,
            DualDelayParam::Right       => ValType::VtFloat,
            DualDelayParam::Feedback    => ValType::VtFloat,
            DualDelayParam::CrossFeed   => ValType::VtFloat,
            DualDelayParam::LowCut      => ValType::VtFloat,
            DualDelayParam::HighCut     => ValType::VtFloat,
            DualDelayParam::Rate        => ValType::VtFloat,
            DualDelayParam::Depth       => ValType::VtFloat,
            DualDelayParam::Pan         => ValType::VtFloat,
            DualDelayParam::Mix         => ValType::VtFloat,
            DualDelayParam::Width       => ValType::VtFloat,
            DualDelayParam::ReturnLevel => ValType::VtFloat,
        }
    }
}

impl GetMoverate for DualDelayParam {

    fn moverate(&self) -> f32 {
        match self {
            DualDelayParam::Left        => 1.0,
            DualDelayParam::Right       => 1.0,
            DualDelayParam::Feedback    => 1.0,
            DualDelayParam::CrossFeed   => 1.0,
            DualDelayParam::LowCut      => 1.0,
            DualDelayParam::HighCut     => 1.0,
            DualDelayParam::Rate        => 0.33,
            DualDelayParam::Depth       => 1.0,
            DualDelayParam::Pan         => 1.0,
            DualDelayParam::Mix         => 1.0,
            DualDelayParam::Width       => 1.0,
            DualDelayParam::ReturnLevel => 1.0,
        }
    }
}
