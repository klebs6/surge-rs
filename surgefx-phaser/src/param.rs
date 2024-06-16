crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum PhaserParam {
    Base,
    Feedback,
    QualityFactor,
    LFORate,
    LFODepth,
    Stereo,
    Mix,
    ReturnLevel,
}

impl_trait_defaults!{
    PhaserParam;
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

impl GetControlGroup for PhaserParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for PhaserParam {

    fn control_type(&self) -> ControlType {
        match self {
            PhaserParam::Base          => ControlType::PercentBidirectional,
            PhaserParam::Feedback      => ControlType::PercentBidirectional,
            PhaserParam::QualityFactor => ControlType::PercentBidirectional,
            PhaserParam::LFORate       => ControlType::LfoRate,
            PhaserParam::LFODepth      => ControlType::Percent,
            PhaserParam::Stereo        => ControlType::Percent,
            PhaserParam::Mix           => ControlType::Percent,
            PhaserParam::ReturnLevel   => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for PhaserParam {

    fn default_value(&self) -> PData {
        match self {
            PhaserParam::Base          => PData::Float(0.0),
            PhaserParam::Feedback      => PData::Float(0.0),
            PhaserParam::QualityFactor => PData::Float(0.0),
            PhaserParam::LFORate       => PData::Float(0.0),
            PhaserParam::LFODepth      => PData::Float(0.0),
            PhaserParam::Stereo        => PData::Float(0.0),
            PhaserParam::Mix           => PData::Float(0.0),
            PhaserParam::ReturnLevel   => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for PhaserParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for PhaserParam {

    fn min_value(&self) -> PData {
        match self {
            PhaserParam::Base          => PData::Float(-1.0),
            PhaserParam::Feedback      => PData::Float(-1.0),
            PhaserParam::QualityFactor => PData::Float(-1.0),
            PhaserParam::LFORate       => PData::Float(-7.0),
            PhaserParam::LFODepth      => PData::Float(0.0),
            PhaserParam::Stereo        => PData::Float(0.0),
            PhaserParam::Mix           => PData::Float(0.0),
            PhaserParam::ReturnLevel   => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for PhaserParam {

    fn max_value(&self) -> PData {
        match self {
            PhaserParam::Base          => PData::Float(1.0),
            PhaserParam::Feedback      => PData::Float(1.0),
            PhaserParam::QualityFactor => PData::Float(1.0),
            PhaserParam::LFORate       => PData::Float(9.0),
            PhaserParam::LFODepth      => PData::Float(1.0),
            PhaserParam::Stereo        => PData::Float(1.0),
            PhaserParam::Mix           => PData::Float(1.0),
            PhaserParam::ReturnLevel   => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for PhaserParam {

    fn value_type(&self) -> ValType {
        match self {
            PhaserParam::Base          => ValType::VtFloat,
            PhaserParam::Feedback      => ValType::VtFloat,
            PhaserParam::QualityFactor => ValType::VtFloat,
            PhaserParam::LFORate       => ValType::VtFloat,
            PhaserParam::LFODepth      => ValType::VtFloat,
            PhaserParam::Stereo        => ValType::VtFloat,
            PhaserParam::Mix           => ValType::VtFloat,
            PhaserParam::ReturnLevel   => ValType::VtFloat,
        }
    }
}

impl GetMoverate for PhaserParam {

    fn moverate(&self) -> f32 {
        match self {
            PhaserParam::Base          => 1.0,
            PhaserParam::Feedback      => 1.0,
            PhaserParam::QualityFactor => 1.0,
            PhaserParam::LFORate       => 0.33,
            PhaserParam::LFODepth      => 1.0,
            PhaserParam::Stereo        => 1.0,
            PhaserParam::Mix           => 1.0,
            PhaserParam::ReturnLevel   => 1.0,
        }
    }
}
