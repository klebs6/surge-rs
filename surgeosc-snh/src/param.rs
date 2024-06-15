crate::ix!();

enhanced_enum![
    SampleAndHoldOscillatorParam {
        Correlation,
        Width,
        Smooth,
        Sub,
        Sync_,
        UniSpread,
        UniCount,
    }
];

rt![SampleAndHoldOscillatorParam];

impl ParameterInterface for SampleAndHoldOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

    fn control_type(&self) -> ControlType {
        match self {
            SampleAndHoldOscillatorParam::Correlation => ControlType::PercentBidirectional,
            SampleAndHoldOscillatorParam::Width       => ControlType::Percent,
            SampleAndHoldOscillatorParam::Smooth      => ControlType::Nil,
            SampleAndHoldOscillatorParam::Sub         => ControlType::Nil,
            SampleAndHoldOscillatorParam::Sync_       => ControlType::SyncPitch,
            SampleAndHoldOscillatorParam::UniSpread   => ControlType::OscSpread,
            SampleAndHoldOscillatorParam::UniCount    => ControlType::OscCount,
        }
    }

    fn default_value(&self) -> PData {
        match self {
            SampleAndHoldOscillatorParam::Correlation => PData::Float(0.0),
            SampleAndHoldOscillatorParam::Width       => PData::Float(0.5),
            SampleAndHoldOscillatorParam::Smooth      => PData::Float(0.0),
            SampleAndHoldOscillatorParam::Sub         => PData::Float(0.0),
            SampleAndHoldOscillatorParam::Sync_       => PData::Float(0.0),
            SampleAndHoldOscillatorParam::UniSpread   => PData::Float(0.2),
            SampleAndHoldOscillatorParam::UniCount    => PData::Float(1.0),
        }
    }

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

    fn min_value(&self) -> PData {
        match self {
            SampleAndHoldOscillatorParam::Correlation => PData::Float(-1.0),     
            SampleAndHoldOscillatorParam::Width       => PData::Float(0.0),      
            SampleAndHoldOscillatorParam::Smooth      => PData::Int(i32::MIN),
            SampleAndHoldOscillatorParam::Sub         => PData::Int(i32::MIN),
            SampleAndHoldOscillatorParam::Sync_       => PData::Float(0.0),      
            SampleAndHoldOscillatorParam::UniSpread   => PData::Float(0.0),      
            SampleAndHoldOscillatorParam::UniCount    => PData::Int(1),          
        }
    }

    fn max_value(&self) -> PData {
        match self {
            SampleAndHoldOscillatorParam::Correlation => PData::Float(1.0),     
            SampleAndHoldOscillatorParam::Width       => PData::Float(1.0),     
            SampleAndHoldOscillatorParam::Smooth      => PData::Int(i32::MAX),
            SampleAndHoldOscillatorParam::Sub         => PData::Int(i32::MAX),
            SampleAndHoldOscillatorParam::Sync_       => PData::Float(60.0),    
            SampleAndHoldOscillatorParam::UniSpread   => PData::Float(1.0),     
            SampleAndHoldOscillatorParam::UniCount    => PData::Int(16),        
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            SampleAndHoldOscillatorParam::Correlation => ValType::VtFloat,
            SampleAndHoldOscillatorParam::Width       => ValType::VtFloat,
            SampleAndHoldOscillatorParam::Smooth      => ValType::VtInt,  
            SampleAndHoldOscillatorParam::Sub         => ValType::VtInt,  
            SampleAndHoldOscillatorParam::Sync_       => ValType::VtFloat,
            SampleAndHoldOscillatorParam::UniSpread   => ValType::VtFloat,
            SampleAndHoldOscillatorParam::UniCount    => ValType::VtInt,  
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            SampleAndHoldOscillatorParam::Correlation => 1.0,
            SampleAndHoldOscillatorParam::Width       => 1.0,
            SampleAndHoldOscillatorParam::Smooth      => 1.0,
            SampleAndHoldOscillatorParam::Sub         => 1.0,
            SampleAndHoldOscillatorParam::Sync_       => 1.0,
            SampleAndHoldOscillatorParam::UniSpread   => 1.0,
            SampleAndHoldOscillatorParam::UniCount    => 1.0,
        }
    }
}

impl SampleAndHoldOscillatorParam {

    pub fn new_runtime() -> SampleAndHoldOscillatorParamArrayRT {
        SampleAndHoldOscillatorParamArrayRT::new_with(|x| match x {
            SampleAndHoldOscillatorParam::Correlation => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::Correlation),
            SampleAndHoldOscillatorParam::Width       => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::Width),
            SampleAndHoldOscillatorParam::Smooth      => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::Smooth),
            SampleAndHoldOscillatorParam::Sub         => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::Sub),
            SampleAndHoldOscillatorParam::Sync_       => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::Sync_),
            SampleAndHoldOscillatorParam::UniSpread   => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::UniSpread),
            SampleAndHoldOscillatorParam::UniCount    => SampleAndHoldOscillatorParamRT::new(SampleAndHoldOscillatorParam::UniCount),
        })
    }
}
