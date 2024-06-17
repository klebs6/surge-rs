crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum SineWaveOscillatorParam {
    Shape,
    Feedback,
    FMBehavior,
}

impl_trait_defaults!{
    SineWaveOscillatorParam;
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

impl CheckIfAbsolute for SineWaveOscillatorParam {
    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for SineWaveOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for SineWaveOscillatorParam {

    fn control_type(&self) -> ControlType {
        match self {
            SineWaveOscillatorParam::Shape      => ControlType::SineOscMode,
            SineWaveOscillatorParam::Feedback   => ControlType::Percent,
            SineWaveOscillatorParam::FMBehavior => ControlType::SineFMLegacy,
        }
    }
}

impl GetDefaultParameterValue for SineWaveOscillatorParam {

    fn default_value(&self) -> PData {
        match self {
            SineWaveOscillatorParam::Shape      => PData::Int(0),
            SineWaveOscillatorParam::Feedback   => PData::Float(0.0),
            SineWaveOscillatorParam::FMBehavior => PData::Int(1),
        }
    }
}

impl CheckIfModulateable for SineWaveOscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for SineWaveOscillatorParam {

    fn min_value(&self) -> PData {
        match self {
            SineWaveOscillatorParam::Shape      => PData::Int(0),    
            SineWaveOscillatorParam::Feedback   => PData::Float(0.0),
            SineWaveOscillatorParam::FMBehavior => PData::Int(0),    
        }
    }
}

impl GetMaxParameterValue for SineWaveOscillatorParam {

    fn max_value(&self) -> PData {
        match self {
            SineWaveOscillatorParam::Shape      => PData::Int(8),    
            SineWaveOscillatorParam::Feedback   => PData::Float(1.0),
            SineWaveOscillatorParam::FMBehavior => PData::Int(1),    
        }
    }
}

impl GetParameterValueType for SineWaveOscillatorParam {

    fn value_type(&self) -> ValType {
        match self {
            SineWaveOscillatorParam::Shape      => ValType::VtInt,  
            SineWaveOscillatorParam::Feedback   => ValType::VtFloat,
            SineWaveOscillatorParam::FMBehavior => ValType::VtInt,  
        }
    }
}

impl GetMoverate for SineWaveOscillatorParam {

    fn moverate(&self) -> f32 {
        match self {
            SineWaveOscillatorParam::Shape      => 1.0,
            SineWaveOscillatorParam::Feedback   => 1.0,
            SineWaveOscillatorParam::FMBehavior => 1.0,
        }
    }
}
