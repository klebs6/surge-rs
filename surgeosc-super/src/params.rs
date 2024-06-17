crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum SSOParam {
    Shape,
    Width,
    SubWidth,
    SubLevel,
    SyncPitch,
    UniSpread,
    UniCount,
    Character,
}

impl_trait_defaults!{
    SSOParam;
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

impl CheckIfAbsolute for SSOParam {
    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for SSOParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for SSOParam {

    fn control_type(&self) -> ControlType {
        match self {
            SSOParam::Shape     => ControlType::PercentBidirectional,
            SSOParam::Width     => ControlType::Percent,
            SSOParam::SubWidth  => ControlType::Percent,
            SSOParam::SubLevel  => ControlType::Percent,
            SSOParam::SyncPitch => ControlType::SyncPitch,
            SSOParam::UniSpread => ControlType::OscSpread,
            SSOParam::UniCount  => ControlType::OscCount,
            SSOParam::Character => ControlType::Character,
        }

    }
}

impl GetDefaultParameterValue for SSOParam {

    fn default_value(&self) -> PData {
        match self {
            SSOParam::Shape     => PData::Float(0.0),
            SSOParam::Width     => PData::Float(0.5),
            SSOParam::SubWidth  => PData::Float(0.5),
            SSOParam::SubLevel  => PData::Float(0.0),
            SSOParam::SyncPitch => PData::Float(0.0),
            SSOParam::UniSpread => PData::Float(0.2),
            SSOParam::UniCount  => PData::Int(1),
            SSOParam::Character => PData::Int(0),
        }
    }
}

impl CheckIfModulateable for SSOParam {

    fn modulateable(&self) -> bool {
        true
    }
}

impl GetMinParameterValue for SSOParam {

    fn min_value(&self) -> PData {
        match self {
            SSOParam::Shape     => PData::Float(-1.0),
            SSOParam::Width     => PData::Float(0.0), 
            SSOParam::SubWidth  => PData::Float(0.0), 
            SSOParam::SubLevel  => PData::Float(0.0), 
            SSOParam::SyncPitch => PData::Float(0.0), 
            SSOParam::UniSpread => PData::Float(0.0), 
            SSOParam::UniCount  => PData::Int(1),     
            SSOParam::Character => PData::Int(0),
        }
    }
}

impl GetMaxParameterValue for SSOParam {

    fn max_value(&self) -> PData {
        match self {
            SSOParam::Shape     => PData::Float(1.0), 
            SSOParam::Width     => PData::Float(1.0), 
            SSOParam::SubWidth  => PData::Float(1.0), 
            SSOParam::SubLevel  => PData::Float(1.0), 
            SSOParam::SyncPitch => PData::Float(60.0),
            SSOParam::UniSpread => PData::Float(1.0), 
            SSOParam::UniCount  => PData::Int(16),    
            SSOParam::Character => PData::Int(2),
        }
    }
}

impl GetParameterValueType for SSOParam {

    fn value_type(&self) -> ValType {
        match self {
            SSOParam::Shape     => ValType::VtFloat,
            SSOParam::Width     => ValType::VtFloat,
            SSOParam::SubWidth  => ValType::VtFloat,
            SSOParam::SubLevel  => ValType::VtFloat,
            SSOParam::SyncPitch => ValType::VtFloat,
            SSOParam::UniSpread => ValType::VtFloat,
            SSOParam::UniCount  => ValType::VtInt,  
            SSOParam::Character => ValType::VtInt,
        }
    }
}

impl GetMoverate for SSOParam {

    fn moverate(&self) -> f32 {
        match self {
            SSOParam::Shape     => 1.0,
            SSOParam::Width     => 1.0,
            SSOParam::SubWidth  => 1.0,
            SSOParam::SubLevel  => 1.0,
            SSOParam::SyncPitch => 1.0,
            SSOParam::UniSpread => 1.0,
            SSOParam::UniCount  => 1.0,
            SSOParam::Character => 1.0,
        }
    }
}
