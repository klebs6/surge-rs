crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum FreqShiftParam {
    Shift,
    RMult,
    Delay,
    Feedback,
    Mix,
    ReturnLevel,
}

impl_trait_defaults!{
    FreqShiftParam;
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

impl CheckIfAbsolute for FreqShiftParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for FreqShiftParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for FreqShiftParam {

    fn control_type(&self) -> ControlType {
        match self {
            FreqShiftParam::Shift       => ControlType::FreqShift,
            FreqShiftParam::RMult       => ControlType::PercentBidirectional,
            FreqShiftParam::Delay       => ControlType::EnvTime,
            FreqShiftParam::Feedback    => ControlType::Amplitude,
            FreqShiftParam::Mix         => ControlType::Percent,
            FreqShiftParam::ReturnLevel => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for FreqShiftParam {

    fn default_value(&self) -> PData {

        match self {
            FreqShiftParam::Shift       => PData::Float(0.0),
            FreqShiftParam::RMult       => PData::Float(1.0),
            FreqShiftParam::Delay       => PData::Float(-8.0),
            FreqShiftParam::Feedback    => PData::Float(0.0),
            FreqShiftParam::Mix         => PData::Float(1.0),
            FreqShiftParam::ReturnLevel => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for FreqShiftParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for FreqShiftParam {

    fn min_value(&self) -> PData {
        match self {
            FreqShiftParam::Shift       => PData::Float(-10.0),
            FreqShiftParam::RMult       => PData::Float(-1.0),
            FreqShiftParam::Delay       => PData::Float(-8.0),
            FreqShiftParam::Feedback    => PData::Float(0.0),
            FreqShiftParam::Mix         => PData::Float(0.0),
            FreqShiftParam::ReturnLevel => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for FreqShiftParam {
    fn max_value(&self) -> PData {
        match self {
            FreqShiftParam::Shift       => PData::Float(10.0),
            FreqShiftParam::RMult       => PData::Float(1.0),
            FreqShiftParam::Delay       => PData::Float(5.0),
            FreqShiftParam::Feedback    => PData::Float(1.0),
            FreqShiftParam::Mix         => PData::Float(1.0),
            FreqShiftParam::ReturnLevel => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for FreqShiftParam {
    fn value_type(&self) -> ValType {
        match self {
            FreqShiftParam::Shift       => ValType::VtFloat,
            FreqShiftParam::RMult       => ValType::VtFloat,
            FreqShiftParam::Delay       => ValType::VtFloat,
            FreqShiftParam::Feedback    => ValType::VtFloat,
            FreqShiftParam::Mix         => ValType::VtFloat,
            FreqShiftParam::ReturnLevel => ValType::VtFloat,
        }
    }
}

impl GetMoverate for FreqShiftParam {
    fn moverate(&self) -> f32 {
        match self {
            FreqShiftParam::Shift       => 1.0,
            FreqShiftParam::RMult       => 1.0,
            FreqShiftParam::Delay       => 1.0,
            FreqShiftParam::Feedback    => 1.0,
            FreqShiftParam::Mix         => 1.0,
            FreqShiftParam::ReturnLevel => 1.0,
        }
    }
}
