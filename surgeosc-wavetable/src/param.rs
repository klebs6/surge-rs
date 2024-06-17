crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum WTOscillatorParam {
    //oscdata->p[0].set_user_data(oscdata);
    Morph,
    SkewV,
    Saturate,
    Formant,
    SkewH,
    UniSpread,
    UniCount,
}

impl_trait_defaults!{
    WTOscillatorParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetModulation,
    SetModulation,
    GetExtendedValue,
    GetExtendRange,
}

impl CheckIfAbsolute for WTOscillatorParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for WTOscillatorParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Osc } 
}

impl GetControlType for WTOscillatorParam {

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
}

impl GetDefaultParameterValue for WTOscillatorParam {

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
}

impl GetMinParameterValue for WTOscillatorParam {

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
}

impl GetMaxParameterValue for WTOscillatorParam {

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
}

impl GetParameterValueType for WTOscillatorParam {

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
}

impl GetMoverate for WTOscillatorParam {

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
}

impl CheckIfModulateable for WTOscillatorParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetSnap for WTOscillatorParam {

    fn snap(&self) -> bool {
        *self != WTOscillatorParam::Morph
    }
}
