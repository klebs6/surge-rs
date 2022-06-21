crate::ix!();

enhanced_enum![
    PhaserParam {
        Base,
        Feedback,
        QualityFactor,
        LFORate,
        LFODepth,
        Stereo,
        Mix,
        ReturnLevel,
    }
];

rt![PhaserParam];

impl Param for PhaserParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 

    fn control_type(&self) -> ControlType {
        match self {
            PhaserParam::Base          => ControlType::PercentBidirectional,
            PhaserParam::Feedback      => ControlType::PercentBidirectional,
            PhaserParam::QualityFactor => ControlType::PercentBidirectional,
            PhaserParam::LFORate       => ControlType::LfoRate,
            PhaserParam::LFODepth      => ControlType::Percent,
            PhaserParam::Stereo        => ControlType::Percent,
            PhaserParam::Mix           => ControlType::Percent,
            PhaserParam::ReturnLevel   => ControlType::Percent,
        }
    }

    fn default_value(&self) -> PData {
        match self {
            PhaserParam::Base          => PData::Float(0.0),
            PhaserParam::Feedback      => PData::Float(0.0),
            PhaserParam::QualityFactor => PData::Float(0.0),
            PhaserParam::LFORate       => PData::Float(0.0),
            PhaserParam::LFODepth      => PData::Float(0.0),
            PhaserParam::Stereo        => PData::Float(0.0),
            PhaserParam::Mix           => PData::Float(0.0),
            PhaserParam::ReturnLevel   => PData::Float(0.5),
        }
    }

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

    fn min_value(&self) -> PData {
        match self {
            PhaserParam::Base          => PData::Float(-1.0),
            PhaserParam::Feedback      => PData::Float(-1.0),
            PhaserParam::QualityFactor => PData::Float(-1.0),
            PhaserParam::LFORate       => PData::Float(-7.0),
            PhaserParam::LFODepth      => PData::Float(0.0),
            PhaserParam::Stereo        => PData::Float(0.0),
            PhaserParam::Mix           => PData::Float(0.0),
            PhaserParam::ReturnLevel   => PData::Float(0.0),
        }
    }

    fn max_value(&self) -> PData {
        match self {
            PhaserParam::Base          => PData::Float(1.0),
            PhaserParam::Feedback      => PData::Float(1.0),
            PhaserParam::QualityFactor => PData::Float(1.0),
            PhaserParam::LFORate       => PData::Float(9.0),
            PhaserParam::LFODepth      => PData::Float(1.0),
            PhaserParam::Stereo        => PData::Float(1.0),
            PhaserParam::Mix           => PData::Float(1.0),
            PhaserParam::ReturnLevel   => PData::Float(1.0),
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            PhaserParam::Base          => ValType::VtFloat,
            PhaserParam::Feedback      => ValType::VtFloat,
            PhaserParam::QualityFactor => ValType::VtFloat,
            PhaserParam::LFORate       => ValType::VtFloat,
            PhaserParam::LFODepth      => ValType::VtFloat,
            PhaserParam::Stereo        => ValType::VtFloat,
            PhaserParam::Mix           => ValType::VtFloat,
            PhaserParam::ReturnLevel   => ValType::VtFloat,
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            PhaserParam::Base          => 1.0,
            PhaserParam::Feedback      => 1.0,
            PhaserParam::QualityFactor => 1.0,
            PhaserParam::LFORate       => 0.33,
            PhaserParam::LFODepth      => 1.0,
            PhaserParam::Stereo        => 1.0,
            PhaserParam::Mix           => 1.0,
            PhaserParam::ReturnLevel   => 1.0,
        }
    }
}

impl PhaserParam {

    #[inline] pub fn new_runtime() -> PhaserParamArrayRT {
        PhaserParamArrayRT::new_with(|x| match x {
            PhaserParam::Base          => PhaserParamRT::new(PhaserParam::Base),
            PhaserParam::Feedback      => PhaserParamRT::new(PhaserParam::Feedback),
            PhaserParam::QualityFactor => PhaserParamRT::new(PhaserParam::QualityFactor),
            PhaserParam::LFORate       => PhaserParamRT::new(PhaserParam::LFORate),
            PhaserParam::LFODepth      => PhaserParamRT::new(PhaserParam::LFODepth),
            PhaserParam::Stereo        => PhaserParamRT::new(PhaserParam::Stereo),
            PhaserParam::Mix           => PhaserParamRT::new(PhaserParam::Mix),
            PhaserParam::ReturnLevel   => PhaserParamRT::new(PhaserParam::ReturnLevel),
        })
    }
}
