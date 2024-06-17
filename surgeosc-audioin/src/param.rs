crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum AudioInputOscillatorParam {
    Input,
    Gain,
}

impl CheckIfAbsolute for AudioInputOscillatorParam {
    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl_trait_defaults!{
    AudioInputOscillatorParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetExtendRange,
    GetExtendedValue,
    GetModulation,
    GetSnap,
    SetModulation,
}

impl GetControlGroup for AudioInputOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for AudioInputOscillatorParam {

    fn control_type(&self) -> ControlType {
        match self {
            AudioInputOscillatorParam::Input => ControlType::PercentBidirectional,
            AudioInputOscillatorParam::Gain => ControlType::Decibel,
        }
    }
}

impl GetDefaultParameterValue for AudioInputOscillatorParam {

    fn default_value(&self) -> PData {
        match self {
            AudioInputOscillatorParam::Input => PData::Float(0.0),
            AudioInputOscillatorParam::Gain  => PData::Float(0.0),
        }
    }
}

impl CheckIfModulateable for AudioInputOscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for AudioInputOscillatorParam {

    fn min_value(&self) -> PData {
        match self {
            AudioInputOscillatorParam::Input => PData::Float(-1.0), 
            AudioInputOscillatorParam::Gain  => PData::Float(-48.0),
        }
    }
}

impl GetMaxParameterValue for AudioInputOscillatorParam {

    fn max_value(&self) -> PData {
        match self {
            AudioInputOscillatorParam::Input => PData::Float(1.0), 
            AudioInputOscillatorParam::Gain  => PData::Float(48.0),
        }
    }
}

impl GetParameterValueType for AudioInputOscillatorParam {

    fn value_type(&self) -> ValType {
        match self {
            AudioInputOscillatorParam::Input => ValType::VtFloat,
            AudioInputOscillatorParam::Gain  => ValType::VtFloat,
        }
    }
}

impl GetMoverate for AudioInputOscillatorParam {

    fn moverate(&self) -> f32 {
        match self {
            AudioInputOscillatorParam::Input => 1.0,
            AudioInputOscillatorParam::Gain  => 1.0,
        }
    }
}
