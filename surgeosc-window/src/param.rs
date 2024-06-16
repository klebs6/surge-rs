crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum WindowOscillatorParam {
    Morph, //oscdata->p[0].set_user_data(oscdata);
    Formant,
    Window,
    UniSpread,
    UniCount,
}

impl_trait_defaults!{
    WindowOscillatorParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetExtendedValue,
    GetModulation,
    SetModulation,
}

impl GetControlGroup for WindowOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for WindowOscillatorParam {

    fn control_type(&self) -> ControlType {
        match self {
            WindowOscillatorParam::Morph     => ControlType::CountedSetPercent,
            WindowOscillatorParam::Formant   => ControlType::Pitch,
            WindowOscillatorParam::Window    => ControlType::Wt2Window,
            WindowOscillatorParam::UniSpread => ControlType::OscSpread,
            WindowOscillatorParam::UniCount  => ControlType::OscCountWT,
        }
    }
}

impl GetDefaultParameterValue for WindowOscillatorParam {

    fn default_value(&self) -> PData {
        match self {
            WindowOscillatorParam::Morph     => PData::Float(0.0),
            WindowOscillatorParam::Formant   => PData::Float(0.0),
            WindowOscillatorParam::Window    => PData::Int(0),
            WindowOscillatorParam::UniSpread => PData::Float(0.2),
            WindowOscillatorParam::UniCount  => PData::Int(1),
        }
    }
}

impl GetMinParameterValue for WindowOscillatorParam {

    fn min_value(&self) -> PData {
        match self {
            WindowOscillatorParam::Morph     => PData::Float(0.0),  
            WindowOscillatorParam::Formant   => PData::Float(-60.0),
            WindowOscillatorParam::Window    => PData::Int(0),      
            WindowOscillatorParam::UniSpread => PData::Float(0.0),  
            WindowOscillatorParam::UniCount  => PData::Int(1),      
        }
    }
}

impl GetMaxParameterValue for WindowOscillatorParam {

    fn max_value(&self) -> PData {
        match self {
            WindowOscillatorParam::Morph     => PData::Float(1.0), 
            WindowOscillatorParam::Formant   => PData::Float(60.0),
            WindowOscillatorParam::Window    => PData::Int(8),     
            WindowOscillatorParam::UniSpread => PData::Float(1.0), 
            WindowOscillatorParam::UniCount  => PData::Int(16),    
        }
    }
}

impl GetParameterValueType for WindowOscillatorParam {

    fn value_type(&self) -> ValType {
        match self {
            WindowOscillatorParam::Morph     => ValType::VtFloat,
            WindowOscillatorParam::Formant   => ValType::VtFloat,
            WindowOscillatorParam::Window    => ValType::VtInt,  
            WindowOscillatorParam::UniSpread => ValType::VtFloat,
            WindowOscillatorParam::UniCount  => ValType::VtInt,  
        }
    }
}

impl GetMoverate for WindowOscillatorParam {

    fn moverate(&self) -> f32 {
        match self {
            WindowOscillatorParam::Morph     => 1.0,
            WindowOscillatorParam::Formant   => 1.0,
            WindowOscillatorParam::Window    => 1.0,
            WindowOscillatorParam::UniSpread => 1.0,
            WindowOscillatorParam::UniCount  => 1.0,
        }
    }
}

impl CheckIfModulateable for WindowOscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetSnap for WindowOscillatorParam {

    fn snap(&self) -> bool {
        *self != WindowOscillatorParam::Morph
    }
}
