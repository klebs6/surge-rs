ix!();

enhanced_enum![
    Eq3BandParam {
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
];

rt![Eq3BandParam];

impl Param for Eq3BandParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 

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

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

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

impl Eq3BandParam {
    pub fn new_runtime() -> Eq3BandParamArrayRT {
        Eq3BandParamArrayRT::new_with(|x| match x {
            Eq3BandParam::LGain       => Eq3BandParamRT::new(Eq3BandParam::LGain),
            Eq3BandParam::LFreq       => Eq3BandParamRT::new(Eq3BandParam::LFreq),
            Eq3BandParam::LBandwidth  => Eq3BandParamRT::new(Eq3BandParam::LBandwidth),
            Eq3BandParam::MGain       => Eq3BandParamRT::new(Eq3BandParam::MGain),
            Eq3BandParam::MFreq       => Eq3BandParamRT::new(Eq3BandParam::MFreq),
            Eq3BandParam::MBandwidth  => Eq3BandParamRT::new(Eq3BandParam::MBandwidth),
            Eq3BandParam::HGain       => Eq3BandParamRT::new(Eq3BandParam::HGain),
            Eq3BandParam::HFreq       => Eq3BandParamRT::new(Eq3BandParam::HFreq),
            Eq3BandParam::HBandwidth  => Eq3BandParamRT::new(Eq3BandParam::HBandwidth),
            Eq3BandParam::Gain        => Eq3BandParamRT::new(Eq3BandParam::Gain),
            Eq3BandParam::ReturnLevel => Eq3BandParamRT::new(Eq3BandParam::ReturnLevel),
        })
    }
}
