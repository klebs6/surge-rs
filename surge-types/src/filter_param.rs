crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum FilterParam {
    Type,
    SubType,
    Cutoff,
    Resonance,
    EnvelopeMode,
    KeyTrack,
}

impl CheckIfAbsolute for FilterParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl_trait_defaults!{
    FilterParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetExtendedValue,
    GetExtendRange,
    GetModulation,
    GetSnap,
    SetModulation
}

impl GetControlGroup for FilterParam {

    //TODO: check these defaults, perhaps there are better ones
    //what type is KeyTrack?
    //how do we select one of the cases in an enum switch?
    fn control_group(&self) -> ControlGroup { ControlGroup::Filter } 
}

impl GetControlType for FilterParam {

    fn control_type(&self) -> ControlType {
        match self {
            FilterParam::Type         => ControlType::FilterType,
            FilterParam::SubType      => ControlType::FilterSubType,
            FilterParam::Cutoff       => ControlType::FreqAudible,
            FilterParam::Resonance    => ControlType::Unknown,
            FilterParam::EnvelopeMode => ControlType::EnvelopeMode,
            FilterParam::KeyTrack     => ControlType::Unknown,
        }
    }
}

impl GetDefaultParameterValue for FilterParam {

    fn default_value(&self) -> PData {
        match self {
            FilterParam::Type         => PData::Int(0),
            FilterParam::SubType      => PData::Int(0),
            FilterParam::Cutoff       => PData::Float(0.5),
            FilterParam::Resonance    => PData::Float(0.5),
            FilterParam::EnvelopeMode => PData::Float(0.5),
            FilterParam::KeyTrack     => PData::Float(0.5),
        }
    }
}

impl GetMinParameterValue for FilterParam {

    fn min_value(&self) -> PData {
        match self {
            FilterParam::Type         => PData::Int(0),
            FilterParam::SubType      => PData::Int(0),
            FilterParam::Cutoff       => PData::Float(0.0),
            FilterParam::Resonance    => PData::Float(0.0),
            FilterParam::EnvelopeMode => PData::Float(0.0),
            FilterParam::KeyTrack     => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for FilterParam {

    fn max_value(&self) -> PData {
        match self {
            FilterParam::Type         => PData::Int(1),
            FilterParam::SubType      => PData::Int(1),
            FilterParam::Cutoff       => PData::Float(1.0),
            FilterParam::Resonance    => PData::Float(1.0),
            FilterParam::EnvelopeMode => PData::Float(1.0),
            FilterParam::KeyTrack     => PData::Float(1.0),
        }
    }
}

impl CheckIfModulateable for FilterParam {

    fn modulateable(&self) -> bool {
        match self {
            FilterParam::Type         => false,
            FilterParam::SubType      => false,
            FilterParam::Cutoff       => true,
            FilterParam::Resonance    => true,
            FilterParam::EnvelopeMode => false,
            FilterParam::KeyTrack     => false,
        }
    }
}

impl GetParameterValueType for FilterParam {

    fn value_type(&self) -> ValType {
        match self {
            FilterParam::Type         => ValType::VtFloat,
            FilterParam::SubType      => ValType::VtFloat,
            FilterParam::Cutoff       => ValType::VtFloat,
            FilterParam::Resonance    => ValType::VtFloat,
            FilterParam::EnvelopeMode => ValType::VtFloat,
            FilterParam::KeyTrack     => ValType::VtFloat,
        }
    }
}

impl GetMoverate for FilterParam {

    fn moverate(&self) -> f32 {
        match self {
            FilterParam::Type         => 1.0,
            FilterParam::SubType      => 1.0,
            FilterParam::Cutoff       => 1.0,
            FilterParam::Resonance    => 1.0,
            FilterParam::EnvelopeMode => 1.0,
            FilterParam::KeyTrack     => 1.0,
        }
    }
}
