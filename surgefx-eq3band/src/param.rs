crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum Eq3BandParam {
    LGain,
    LFreq,
    LBandwidth,
    MGain,
    MFreq,
    MBandwidth,
    HGain,
    HFreq,
    HBandwidth,
    Gain,
    ReturnLevel,
}

impl_trait_defaults!{
    Eq3BandParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetModulation,
    GetSnap,
    SetModulation,
    GetExtendedValue,
    GetExtendRange,
}

impl CheckIfAbsolute for Eq3BandParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for Eq3BandParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for Eq3BandParam {

    fn control_type(&self) -> ControlType {
        match self {
            Eq3BandParam::LGain       => ControlType::Decibel,
            Eq3BandParam::LFreq       => ControlType::FreqAudible,
            Eq3BandParam::LBandwidth  => ControlType::Bandwidth,
            Eq3BandParam::MGain       => ControlType::Decibel,
            Eq3BandParam::MFreq       => ControlType::FreqAudible,
            Eq3BandParam::MBandwidth  => ControlType::Bandwidth,
            Eq3BandParam::HGain       => ControlType::Decibel,
            Eq3BandParam::HFreq       => ControlType::FreqAudible,
            Eq3BandParam::HBandwidth  => ControlType::Bandwidth,
            Eq3BandParam::Gain        => ControlType::Decibel,
            Eq3BandParam::ReturnLevel => ControlType::Percent,
        }
    }
}

impl GetDefaultParameterValue for Eq3BandParam {

    fn default_value(&self) -> PData {
        match self {
            Eq3BandParam::LGain       => PData::Float( 0.0 ),
            Eq3BandParam::LFreq       => PData::Float( -2.5 * 12.0 ),
            Eq3BandParam::LBandwidth  => PData::Float( 2.0 ),
            Eq3BandParam::MGain       => PData::Float( 0.0 ),
            Eq3BandParam::MFreq       => PData::Float( 0.5 * 12.0 ),
            Eq3BandParam::MBandwidth  => PData::Float( 2.0 ),
            Eq3BandParam::HGain       => PData::Float( 0.0 ),
            Eq3BandParam::HFreq       => PData::Float( 4.5 * 12.0 ),
            Eq3BandParam::HBandwidth  => PData::Float( 2.0 ),
            Eq3BandParam::Gain        => PData::Float( 0.0 ),
            Eq3BandParam::ReturnLevel => PData::Float( 0.5 ),
        }
    }
}

impl CheckIfModulateable for Eq3BandParam {
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for Eq3BandParam {
    fn min_value(&self) -> PData {
        match self {
            Eq3BandParam::LGain       => PData::Float(-48.0),
            Eq3BandParam::LFreq       => PData::Float(-60.0),
            Eq3BandParam::LBandwidth  => PData::Float(0.0),
            Eq3BandParam::MGain       => PData::Float(-48.0),
            Eq3BandParam::MFreq       => PData::Float(-60.0),
            Eq3BandParam::MBandwidth  => PData::Float(0.0),
            Eq3BandParam::HGain       => PData::Float(-48.0),
            Eq3BandParam::HFreq       => PData::Float(-60.0),
            Eq3BandParam::HBandwidth  => PData::Float(0.0),
            Eq3BandParam::Gain        => PData::Float(-48.0),
            Eq3BandParam::ReturnLevel => PData::Float( 0.0 ),
        }
    }
}

impl GetMaxParameterValue for Eq3BandParam {
    fn max_value(&self) -> PData {
        match self {
            Eq3BandParam::LGain       => PData::Float(48.0),
            Eq3BandParam::LFreq       => PData::Float(70.0),
            Eq3BandParam::LBandwidth  => PData::Float(5.0),
            Eq3BandParam::MGain       => PData::Float(48.0),
            Eq3BandParam::MFreq       => PData::Float(70.0),
            Eq3BandParam::MBandwidth  => PData::Float(5.0),
            Eq3BandParam::HGain       => PData::Float(48.0),
            Eq3BandParam::HFreq       => PData::Float(70.0),
            Eq3BandParam::HBandwidth  => PData::Float(5.0),
            Eq3BandParam::Gain        => PData::Float(48.0),
            Eq3BandParam::ReturnLevel => PData::Float( 1.0 ),
        }
    }
}

impl GetParameterValueType for Eq3BandParam {
    fn value_type(&self) -> ValType {
        match self {
            Eq3BandParam::LGain       => ValType::VtFloat,
            Eq3BandParam::LFreq       => ValType::VtFloat,
            Eq3BandParam::LBandwidth  => ValType::VtFloat,
            Eq3BandParam::MGain       => ValType::VtFloat,
            Eq3BandParam::MFreq       => ValType::VtFloat,
            Eq3BandParam::MBandwidth  => ValType::VtFloat,
            Eq3BandParam::HGain       => ValType::VtFloat,
            Eq3BandParam::HFreq       => ValType::VtFloat,
            Eq3BandParam::HBandwidth  => ValType::VtFloat,
            Eq3BandParam::Gain        => ValType::VtFloat,
            Eq3BandParam::ReturnLevel => ValType::VtFloat,
        }
    }
}

impl GetMoverate for Eq3BandParam {
    fn moverate(&self) -> f32 {
        match self {
            Eq3BandParam::LGain       => 1.0,
            Eq3BandParam::LFreq       => 1.0,
            Eq3BandParam::LBandwidth  => 1.0,
            Eq3BandParam::MGain       => 1.0,
            Eq3BandParam::MFreq       => 1.0,
            Eq3BandParam::MBandwidth  => 1.0,
            Eq3BandParam::HGain       => 1.0,
            Eq3BandParam::HFreq       => 1.0,
            Eq3BandParam::HBandwidth  => 1.0,
            Eq3BandParam::Gain        => 1.0,
            Eq3BandParam::ReturnLevel => 1.0,
        }
    }
}
