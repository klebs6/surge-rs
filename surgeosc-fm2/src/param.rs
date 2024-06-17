crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum FM2OscillatorParam {
    M1Amount,
    M1Ratio,
    M2Amount,
    M2Ratio,
    MxShift,
    MxStartPhase,
    Feedback,
}

impl_trait_defaults!{
    FM2OscillatorParam;
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

impl CheckIfAbsolute for FM2OscillatorParam {
    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for FM2OscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for FM2OscillatorParam {

    fn control_type(&self) -> ControlType {
        match self {
            FM2OscillatorParam::M1Amount      => ControlType::Percent,
            FM2OscillatorParam::M1Ratio       => ControlType::FMRatio,
            FM2OscillatorParam::M2Amount      => ControlType::Percent,
            FM2OscillatorParam::M2Ratio       => ControlType::FMRatio,
            FM2OscillatorParam::MxShift       => ControlType::FreqShift,
            FM2OscillatorParam::MxStartPhase  => ControlType::Percent,
            FM2OscillatorParam::Feedback      => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for FM2OscillatorParam {

    fn default_value(&self) -> PData {
        match self {
            FM2OscillatorParam::M1Amount     => PData::Float(0.0),
            FM2OscillatorParam::M1Ratio      => PData::Int(1),
            FM2OscillatorParam::M2Amount     => PData::Float(0.0),
            FM2OscillatorParam::M2Ratio      => PData::Int(1),
            FM2OscillatorParam::MxShift      => PData::Float(0.0),
            FM2OscillatorParam::MxStartPhase => PData::Float(0.0),
            FM2OscillatorParam::Feedback     => PData::Float(0.0),
        }
    }
}

impl GetMinParameterValue for FM2OscillatorParam {

    fn min_value(&self) -> PData {
        match self {
            FM2OscillatorParam::M1Amount     => PData::Float(0.0),  
            FM2OscillatorParam::M1Ratio      => PData::Float(0.0),  
            FM2OscillatorParam::M2Amount     => PData::Float(0.0),  
            FM2OscillatorParam::M2Ratio      => PData::Float(0.0),  
            FM2OscillatorParam::MxShift      => PData::Float(-10.0),
            FM2OscillatorParam::MxStartPhase => PData::Float(0.0),  
            FM2OscillatorParam::Feedback     => PData::Float(0.0),  
        }
    }
}

impl GetMaxParameterValue for FM2OscillatorParam {

    fn max_value(&self) -> PData {
        match self {
            FM2OscillatorParam::M1Amount     => PData::Float(1.0), 
            FM2OscillatorParam::M1Ratio      => PData::Float(32.0),
            FM2OscillatorParam::M2Amount     => PData::Float(1.0), 
            FM2OscillatorParam::M2Ratio      => PData::Float(32.0),
            FM2OscillatorParam::MxShift      => PData::Float(10.0),
            FM2OscillatorParam::MxStartPhase => PData::Float(1.0), 
            FM2OscillatorParam::Feedback     => PData::Float(1.0), 
        }
    }
}

impl GetParameterValueType for FM2OscillatorParam {

    fn value_type(&self) -> ValType {
        match self {
            FM2OscillatorParam::M1Amount     => ValType::VtFloat,
            FM2OscillatorParam::M1Ratio      => ValType::VtFloat,
            FM2OscillatorParam::M2Amount     => ValType::VtFloat,
            FM2OscillatorParam::M2Ratio      => ValType::VtFloat,
            FM2OscillatorParam::MxShift      => ValType::VtFloat,
            FM2OscillatorParam::MxStartPhase => ValType::VtFloat,
            FM2OscillatorParam::Feedback     => ValType::VtFloat,
        }
    }
}

impl GetMoverate for FM2OscillatorParam {

    fn moverate(&self) -> f32 {
        match self {
            FM2OscillatorParam::M1Amount     => 1.0, 
            FM2OscillatorParam::M1Ratio      => 0.25,
            FM2OscillatorParam::M2Amount     => 1.0, 
            FM2OscillatorParam::M2Ratio      => 0.25,
            FM2OscillatorParam::MxShift      => 1.0, 
            FM2OscillatorParam::MxStartPhase => 1.0, 
            FM2OscillatorParam::Feedback     => 1.0, 
        }
    }
}

impl CheckIfModulateable for FM2OscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}
