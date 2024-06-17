crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum SampleAndHoldOscillatorParam {
    Correlation,
    Width,
    Smooth,
    Sub,
    Sync_,
    UniSpread,
    UniCount,
}

impl_trait_defaults!{
    SampleAndHoldOscillatorParam;
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

impl CheckIfAbsolute for SampleAndHoldOscillatorParam {
    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for SampleAndHoldOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for SampleAndHoldOscillatorParam {

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
}

impl GetDefaultParameterValue for SampleAndHoldOscillatorParam {

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
}

impl CheckIfModulateable for SampleAndHoldOscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for SampleAndHoldOscillatorParam {

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
}

impl GetMaxParameterValue for SampleAndHoldOscillatorParam {

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
}

impl GetParameterValueType for SampleAndHoldOscillatorParam {

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
}

impl GetMoverate for SampleAndHoldOscillatorParam {

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
