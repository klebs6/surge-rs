crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum FMOscillatorParam {
    M1Amount,
    M1Ratio,
    M2Amount,
    M2Ratio,
    M3Amount,
    M3Freq,
    Feedback,
}

impl_trait_defaults!{
    FMOscillatorParam;
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

impl CheckIfAbsolute for FMOscillatorParam {
    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for FMOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for FMOscillatorParam {
    fn control_type(&self) -> ControlType {
        match self {
            FMOscillatorParam::M1Amount => ControlType::Percent,
            FMOscillatorParam::M1Ratio  => ControlType::FMRatio,
            FMOscillatorParam::M2Amount => ControlType::Percent,
            FMOscillatorParam::M2Ratio  => ControlType::FMRatio,
            FMOscillatorParam::M3Amount => ControlType::Percent,
            FMOscillatorParam::M3Freq   => ControlType::FreqAudible,
            FMOscillatorParam::Feedback => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for FMOscillatorParam {

    fn default_value(&self) -> PData {

        match self {
            FMOscillatorParam::M1Amount => PData::Float(0.0),
            FMOscillatorParam::M1Ratio  => PData::Float(1.0),
            FMOscillatorParam::M2Amount => PData::Float(0.0),
            FMOscillatorParam::M2Ratio  => PData::Float(1.0),
            FMOscillatorParam::M3Amount => PData::Float(0.0),
            FMOscillatorParam::M3Freq   => PData::Float(0.0),
            FMOscillatorParam::Feedback => PData::Float(0.0),
        }
    }
}

impl CheckIfModulateable for FMOscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for FMOscillatorParam {

    fn min_value(&self) -> PData {
        match self {
            FMOscillatorParam::M1Amount => PData::Float(0.0),  
            FMOscillatorParam::M1Ratio  => PData::Float(0.0),  
            FMOscillatorParam::M2Amount => PData::Float(0.0),  
            FMOscillatorParam::M2Ratio  => PData::Float(0.0),  
            FMOscillatorParam::M3Amount => PData::Float(0.0),  
            FMOscillatorParam::M3Freq   => PData::Float(-60.0),
            FMOscillatorParam::Feedback => PData::Float(0.0),  
        }
    }
}

impl GetMaxParameterValue for FMOscillatorParam {

    fn max_value(&self) -> PData {
        match self {
            FMOscillatorParam::M1Amount => PData::Float(1.0), 
            FMOscillatorParam::M1Ratio  => PData::Float(32.0),
            FMOscillatorParam::M2Amount => PData::Float(1.0), 
            FMOscillatorParam::M2Ratio  => PData::Float(32.0),
            FMOscillatorParam::M3Amount => PData::Float(1.0), 
            FMOscillatorParam::M3Freq   => PData::Float(70.0),
            FMOscillatorParam::Feedback => PData::Float(1.0), 
        }
    }
}

impl GetParameterValueType for FMOscillatorParam {

    fn value_type(&self) -> ValType {
        match self {
            FMOscillatorParam::M1Amount => ValType::VtFloat,
            FMOscillatorParam::M1Ratio  => ValType::VtFloat,
            FMOscillatorParam::M2Amount => ValType::VtFloat,
            FMOscillatorParam::M2Ratio  => ValType::VtFloat,
            FMOscillatorParam::M3Amount => ValType::VtFloat,
            FMOscillatorParam::M3Freq   => ValType::VtFloat,
            FMOscillatorParam::Feedback => ValType::VtFloat,
        }
    }
}

impl GetMoverate for FMOscillatorParam {

    fn moverate(&self) -> f32 {
        match self {
            FMOscillatorParam::M1Amount => 1.0, 
            FMOscillatorParam::M1Ratio  => 0.25,
            FMOscillatorParam::M2Amount => 1.0, 
            FMOscillatorParam::M2Ratio  => 0.25,
            FMOscillatorParam::M3Amount => 1.0, 
            FMOscillatorParam::M3Freq   => 1.0, 
            FMOscillatorParam::Feedback => 1.0, 
        }
    }
}
