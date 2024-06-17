crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum RotarySpeakerParam {
    HornRate,
    Doppler,
    AmpMod,
    ReturnLevel,
}

impl_trait_defaults!{
    RotarySpeakerParam;
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
    SetModulation,
}

impl CheckIfAbsolute for RotarySpeakerParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for RotarySpeakerParam {

    fn control_group(&self) -> ControlGroup { 
        ControlGroup::Fx 
    }
}

impl GetControlType for RotarySpeakerParam {

    fn control_type(&self) -> ControlType {
        match self {
            RotarySpeakerParam::HornRate    => ControlType::LfoRate,
            RotarySpeakerParam::Doppler     => ControlType::Percent,
            RotarySpeakerParam::AmpMod      => ControlType::Percent,
            RotarySpeakerParam::ReturnLevel => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for RotarySpeakerParam {

    fn default_value(&self) -> PData {
        match self {
            RotarySpeakerParam::HornRate    => PData::Float(1.0),
            RotarySpeakerParam::Doppler     => PData::Float(0.25),
            RotarySpeakerParam::AmpMod      => PData::Float(0.5),
            RotarySpeakerParam::ReturnLevel => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for RotarySpeakerParam {

    fn modulateable(&self) -> bool {
        true
    }
}

impl GetMinParameterValue for RotarySpeakerParam {

    fn min_value(&self) -> PData {
        match self {
            RotarySpeakerParam::HornRate    => PData::Float(-7.0),
            RotarySpeakerParam::Doppler     => PData::Float(0.0),
            RotarySpeakerParam::AmpMod      => PData::Float(0.0),
            RotarySpeakerParam::ReturnLevel => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for RotarySpeakerParam {

    fn max_value(&self) -> PData {
        match self {
            RotarySpeakerParam::HornRate    => PData::Float(9.0),
            RotarySpeakerParam::Doppler     => PData::Float(1.0),
            RotarySpeakerParam::AmpMod      => PData::Float(1.0),
            RotarySpeakerParam::ReturnLevel => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for RotarySpeakerParam {

    fn value_type(&self) -> ValType {
        match self {
            RotarySpeakerParam::HornRate    => ValType::VtFloat,
            RotarySpeakerParam::Doppler     => ValType::VtFloat,
            RotarySpeakerParam::AmpMod      => ValType::VtFloat,
            RotarySpeakerParam::ReturnLevel => ValType::VtFloat,
        }
    }
}

impl GetMoverate for RotarySpeakerParam {

    fn moverate(&self) -> f32 {
        match self {
            RotarySpeakerParam::HornRate    => 0.33,
            RotarySpeakerParam::Doppler     => 1.0,
            RotarySpeakerParam::AmpMod      => 1.0,
            RotarySpeakerParam::ReturnLevel => 1.0,
        }
    }
}

impl RotarySpeakerParam {
    #[inline] pub fn new_runtime() -> RotarySpeakerParamArrayRT {
        RotarySpeakerParamArrayRT::new_with(|x| match x {
            RotarySpeakerParam::HornRate    => RotarySpeakerParamRT::new(RotarySpeakerParam::HornRate),
            RotarySpeakerParam::Doppler     => RotarySpeakerParamRT::new(RotarySpeakerParam::Doppler),
            RotarySpeakerParam::AmpMod      => RotarySpeakerParamRT::new(RotarySpeakerParam::AmpMod),
            RotarySpeakerParam::ReturnLevel => RotarySpeakerParamRT::new(RotarySpeakerParam::ReturnLevel),
        })
    }
}
