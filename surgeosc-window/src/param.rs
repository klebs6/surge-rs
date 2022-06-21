crate::ix!();

enhanced_enum![
    WindowOscillatorParam {
        Morph, //oscdata->p[0].set_user_data(oscdata);
        Formant,
        Window,
        UniSpread,
        UniCount,
    }
];

rt![WindowOscillatorParam];

impl Param for WindowOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 

    fn snap(&self) -> bool {
        *self != WindowOscillatorParam::Morph
    }

    fn control_type(&self) -> ControlType {
        match self {
            WindowOscillatorParam::Morph     => ControlType::CountedSetPercent,
            WindowOscillatorParam::Formant   => ControlType::Pitch,
            WindowOscillatorParam::Window    => ControlType::Wt2Window,
            WindowOscillatorParam::UniSpread => ControlType::OscSpread,
            WindowOscillatorParam::UniCount  => ControlType::OscCountWT,
        }
    }

    fn default_value(&self) -> PData {
        match self {
            WindowOscillatorParam::Morph     => PData::Float(0.0),
            WindowOscillatorParam::Formant   => PData::Float(0.0),
            WindowOscillatorParam::Window    => PData::Int(0),
            WindowOscillatorParam::UniSpread => PData::Float(0.2),
            WindowOscillatorParam::UniCount  => PData::Int(1),
        }
    }

    fn min_value(&self) -> PData {
        match self {
            WindowOscillatorParam::Morph     => PData::Float(0.0),  
            WindowOscillatorParam::Formant   => PData::Float(-60.0),
            WindowOscillatorParam::Window    => PData::Int(0),      
            WindowOscillatorParam::UniSpread => PData::Float(0.0),  
            WindowOscillatorParam::UniCount  => PData::Int(1),      
        }
    }

    fn max_value(&self) -> PData {
        match self {
            WindowOscillatorParam::Morph     => PData::Float(1.0), 
            WindowOscillatorParam::Formant   => PData::Float(60.0),
            WindowOscillatorParam::Window    => PData::Int(8),     
            WindowOscillatorParam::UniSpread => PData::Float(1.0), 
            WindowOscillatorParam::UniCount  => PData::Int(16),    
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            WindowOscillatorParam::Morph     => ValType::VtFloat,
            WindowOscillatorParam::Formant   => ValType::VtFloat,
            WindowOscillatorParam::Window    => ValType::VtInt,  
            WindowOscillatorParam::UniSpread => ValType::VtFloat,
            WindowOscillatorParam::UniCount  => ValType::VtInt,  
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            WindowOscillatorParam::Morph     => 1.0,
            WindowOscillatorParam::Formant   => 1.0,
            WindowOscillatorParam::Window    => 1.0,
            WindowOscillatorParam::UniSpread => 1.0,
            WindowOscillatorParam::UniCount  => 1.0,
        }
    }

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl WindowOscillatorParam {

    #[inline] pub fn new_runtime() -> WindowOscillatorParamArrayRT {
        WindowOscillatorParamArrayRT::new_with(|x| match x {
            WindowOscillatorParam::Morph     => WindowOscillatorParamRT::new(WindowOscillatorParam::Morph),
            WindowOscillatorParam::Formant   => WindowOscillatorParamRT::new(WindowOscillatorParam::Formant),
            WindowOscillatorParam::Window    => WindowOscillatorParamRT::new(WindowOscillatorParam::Window),
            WindowOscillatorParam::UniSpread => WindowOscillatorParamRT::new(WindowOscillatorParam::UniSpread),
            WindowOscillatorParam::UniCount  => WindowOscillatorParamRT::new(WindowOscillatorParam::UniCount),
        })
    }
}
