crate::ix!();

enhanced_enum![
    SineWaveOscillatorParam {
        Shape,
        Feedback,
        FMBehavior,
    }
];

rt![SineWaveOscillatorParam];

impl ParameterInterface for SineWaveOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

    fn control_type(&self) -> ControlType {
        match self {
            SineWaveOscillatorParam::Shape      => ControlType::SineOscMode,
            SineWaveOscillatorParam::Feedback   => ControlType::Percent,
            SineWaveOscillatorParam::FMBehavior => ControlType::SineFMLegacy,
        }
    }
    fn default_value(&self) -> PData {
        match self {
            SineWaveOscillatorParam::Shape      => PData::Int(0),
            SineWaveOscillatorParam::Feedback   => PData::Float(0.0),
            SineWaveOscillatorParam::FMBehavior => PData::Int(1),
        }
    }
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
    fn min_value(&self) -> PData {
        match self {
            SineWaveOscillatorParam::Shape      => PData::Int(0),    
            SineWaveOscillatorParam::Feedback   => PData::Float(0.0),
            SineWaveOscillatorParam::FMBehavior => PData::Int(0),    
        }
    }
    fn max_value(&self) -> PData {
        match self {
            SineWaveOscillatorParam::Shape      => PData::Int(8),    
            SineWaveOscillatorParam::Feedback   => PData::Float(1.0),
            SineWaveOscillatorParam::FMBehavior => PData::Int(1),    
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            SineWaveOscillatorParam::Shape      => ValType::VtInt,  
            SineWaveOscillatorParam::Feedback   => ValType::VtFloat,
            SineWaveOscillatorParam::FMBehavior => ValType::VtInt,  
        }
    }
    fn moverate(&self) -> f32 {
        match self {
            SineWaveOscillatorParam::Shape      => 1.0,
            SineWaveOscillatorParam::Feedback   => 1.0,
            SineWaveOscillatorParam::FMBehavior => 1.0,
        }
    }
}

impl SineWaveOscillatorParam {
    pub fn new_runtime() -> SineWaveOscillatorParamArrayRT {
        SineWaveOscillatorParamArrayRT::new_with(|x| match x {
            SineWaveOscillatorParam::Shape      => SineWaveOscillatorParamRT::new(SineWaveOscillatorParam::Shape),
            SineWaveOscillatorParam::Feedback   => SineWaveOscillatorParamRT::new(SineWaveOscillatorParam::Feedback),
            SineWaveOscillatorParam::FMBehavior => SineWaveOscillatorParamRT::new(SineWaveOscillatorParam::FMBehavior),
        })
    }
}
