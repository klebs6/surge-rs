ix!();

enhanced_enum![
    WTOscillatorParam {
        //oscdata->p[0].set_user_data(oscdata);
        Morph,
        SkewV,
        Saturate,
        Formant,
        SkewH,
        UniSpread,
        UniCount,
    }
];

rt![WTOscillatorParam];

impl Param for WTOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

    fn snap(&self) -> bool {
        *self != WTOscillatorParam::Morph
    }
    fn control_type(&self) -> ControlType {
        match self {
            WTOscillatorParam::Morph     => ControlType::CountedSetPercent,
            WTOscillatorParam::SkewV     => ControlType::PercentBidirectional,
            WTOscillatorParam::Saturate  => ControlType::Percent,
            WTOscillatorParam::Formant   => ControlType::SyncPitch,
            WTOscillatorParam::SkewH     => ControlType::PercentBidirectional,
            WTOscillatorParam::UniSpread => ControlType::OscSpread,
            WTOscillatorParam::UniCount  => ControlType::OscCountWT,
        }
    }
    fn default_value(&self) -> PData {
        match self {
            WTOscillatorParam::Morph     => PData::Float(0.0),
            WTOscillatorParam::SkewV     => PData::Float(0.0),
            WTOscillatorParam::Saturate  => PData::Float(0.0),
            WTOscillatorParam::Formant   => PData::Float(0.0),
            WTOscillatorParam::SkewH     => PData::Float(0.0),
            WTOscillatorParam::UniSpread => PData::Float(0.2),
            WTOscillatorParam::UniCount  => PData::Float(1.0),
        }
    }
    fn min_value(&self) -> PData {
        match self {
            WTOscillatorParam::Morph     => PData::Float(0.0), 
            WTOscillatorParam::SkewV     => PData::Float(-1.0),
            WTOscillatorParam::Saturate  => PData::Float(0.0), 
            WTOscillatorParam::Formant   => PData::Float(0.0), 
            WTOscillatorParam::SkewH     => PData::Float(-1.0),
            WTOscillatorParam::UniSpread => PData::Float(0.0), 
            WTOscillatorParam::UniCount  => PData::Int(1),     
        }
    }
    fn max_value(&self) -> PData {
        match self {
            WTOscillatorParam::Morph     => PData::Float(1.0), 
            WTOscillatorParam::SkewV     => PData::Float(1.0), 
            WTOscillatorParam::Saturate  => PData::Float(1.0), 
            WTOscillatorParam::Formant   => PData::Float(60.0),
            WTOscillatorParam::SkewH     => PData::Float(1.0), 
            WTOscillatorParam::UniSpread => PData::Float(1.0), 
            WTOscillatorParam::UniCount  => PData::Int(16),    
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            WTOscillatorParam::Morph     => ValType::VtFloat,
            WTOscillatorParam::SkewV     => ValType::VtFloat,
            WTOscillatorParam::Saturate  => ValType::VtFloat,
            WTOscillatorParam::Formant   => ValType::VtFloat,
            WTOscillatorParam::SkewH     => ValType::VtFloat,
            WTOscillatorParam::UniSpread => ValType::VtFloat,
            WTOscillatorParam::UniCount  => ValType::VtInt,  
        }
    }
    fn moverate(&self) -> f32 {
        match self {
            WTOscillatorParam::Morph     => 1.0,
            WTOscillatorParam::SkewV     => 1.0,
            WTOscillatorParam::Saturate  => 1.0,
            WTOscillatorParam::Formant   => 1.0,
            WTOscillatorParam::SkewH     => 1.0,
            WTOscillatorParam::UniSpread => 1.0,
            WTOscillatorParam::UniCount  => 1.0,
        }
    }
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl WTOscillatorParam {
    #[inline] pub fn new_runtime() -> WTOscillatorParamArrayRT {
        WTOscillatorParamArrayRT::new_with(|x| match x {
            WTOscillatorParam::Morph     => WTOscillatorParamRT::new(WTOscillatorParam::Morph),
            WTOscillatorParam::SkewV     => WTOscillatorParamRT::new(WTOscillatorParam::SkewV),
            WTOscillatorParam::Saturate  => WTOscillatorParamRT::new(WTOscillatorParam::Saturate),
            WTOscillatorParam::Formant   => WTOscillatorParamRT::new(WTOscillatorParam::Formant),
            WTOscillatorParam::SkewH     => WTOscillatorParamRT::new(WTOscillatorParam::SkewH),
            WTOscillatorParam::UniSpread => WTOscillatorParamRT::new(WTOscillatorParam::UniSpread),
            WTOscillatorParam::UniCount  => WTOscillatorParamRT::new(WTOscillatorParam::UniCount),
        })
    }
}
