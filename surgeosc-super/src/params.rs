ix!();

enhanced_enum![
    SSOParam {
        Shape,
        Width,
        SubWidth,
        SubLevel,
        SyncPitch,
        UniSpread,
        UniCount,
        Character,
    }
];

rt![SSOParam];

/*
control_type!{SSOParam;
    (Shape,      PercentBidirectional),
    (Width,      Percent),
    (SubWidth,   Percent),
    (SubLevel,   Percent),
    (SyncPitch,  SyncPitch),
    (UniSpread,  OscSpread),
    (UniCount,   OscCount),
    (Character,  Character),
}

value_type![SSOParam;
    (Shape,      VtFloat),
    (Width,      VtFloat),
    (SubWidth,   VtFloat),
    (SubLevel,   VtFloat),
    (SyncPitch,  VtFloat),
    (UniSpread,  VtFloat),
    (UniCount,   VtInt),  
    Character,   VtInt),
];

moverate![SSOParam;
    (Shape,     1.0),
    (Width,     1.0),
    (SubWidth,  1.0),
    (SubLevel,  1.0),
    (SyncPitch, 1.0),
    (UniSpread, 1.0),
    (UniCount,  1.0),
    (Character, 1.0),
];
*/

impl Param for SSOParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

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
    fn modulateable(&self) -> bool {
        true
    }
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


impl SSOParam {
    #[inline] pub fn new_runtime() -> SSOParamArrayRT {
        SSOParamArrayRT::new_with(|x| match x {
            SSOParam::Shape     => SSOParamRT::new(SSOParam::Shape     ),
            SSOParam::Width     => SSOParamRT::new(SSOParam::Width     ),
            SSOParam::SubWidth  => SSOParamRT::new(SSOParam::SubWidth  ),
            SSOParam::SubLevel  => SSOParamRT::new(SSOParam::SubLevel  ),
            SSOParam::SyncPitch => SSOParamRT::new(SSOParam::SyncPitch ),
            SSOParam::UniSpread => SSOParamRT::new(SSOParam::UniSpread ),
            SSOParam::UniCount  => SSOParamRT::new(SSOParam::UniCount  ),
            SSOParam::Character => SSOParamRT::new(SSOParam::Character ),
        })
    }
}
