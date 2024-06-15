crate::ix!();

enhanced_enum![
    FM2OscillatorParam {
        M1Amount,
        M1Ratio,
        M2Amount,
        M2Ratio,
        MxShift,
        MxStartPhase,
        Feedback,
    }
];

rt![FM2OscillatorParam];

impl ParameterInterface for FM2OscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

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
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl FM2OscillatorParam {

    #[inline] pub fn new_runtime() -> FM2OscillatorParamArrayRT {
        FM2OscillatorParamArrayRT::new_with(|x| match x {
            FM2OscillatorParam::M1Amount     => FM2OscillatorParamRT::new(FM2OscillatorParam::M1Amount),
            FM2OscillatorParam::M1Ratio      => FM2OscillatorParamRT::new(FM2OscillatorParam::M1Ratio),
            FM2OscillatorParam::M2Amount     => FM2OscillatorParamRT::new(FM2OscillatorParam::M2Amount),
            FM2OscillatorParam::M2Ratio      => FM2OscillatorParamRT::new(FM2OscillatorParam::M2Ratio),
            FM2OscillatorParam::MxShift      => FM2OscillatorParamRT::new(FM2OscillatorParam::MxShift),
            FM2OscillatorParam::MxStartPhase => FM2OscillatorParamRT::new(FM2OscillatorParam::MxStartPhase),
            FM2OscillatorParam::Feedback     => FM2OscillatorParamRT::new(FM2OscillatorParam::Feedback),
        })
    }
}
