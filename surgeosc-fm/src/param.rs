crate::ix!();

enhanced_enum![
    FMOscillatorParam {
        M1Amount,
        M1Ratio,
        M2Amount,
        M2Ratio,
        M3Amount,
        M3Freq,
        Feedback,
    }
];

rt![FMOscillatorParam];

impl ParameterInterface for FMOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

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
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
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

impl FMOscillatorParam {
    #[inline] pub fn new_runtime() -> FMOscillatorParamArrayRT {
        FMOscillatorParamArrayRT::new_with(|x| match x {
            FMOscillatorParam::M1Amount => FMOscillatorParamRT::new(FMOscillatorParam::M1Amount),
            FMOscillatorParam::M1Ratio  => FMOscillatorParamRT::new(FMOscillatorParam::M1Ratio),
            FMOscillatorParam::M2Amount => FMOscillatorParamRT::new(FMOscillatorParam::M2Amount),
            FMOscillatorParam::M2Ratio  => FMOscillatorParamRT::new(FMOscillatorParam::M2Ratio),
            FMOscillatorParam::M3Amount => FMOscillatorParamRT::new(FMOscillatorParam::M3Amount),
            FMOscillatorParam::M3Freq   => FMOscillatorParamRT::new(FMOscillatorParam::M3Freq),
            FMOscillatorParam::Feedback => FMOscillatorParamRT::new(FMOscillatorParam::Feedback),
        })
    }
}
