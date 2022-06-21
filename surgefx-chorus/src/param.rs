crate::ix!();

enhanced_enum![
    ChorusParam {
        Time,
        Rate,
        Depth,
        Feedback,
        LowCut,
        HighCut,
        Mix,
        Width,
        ReturnLevel,
    }
];

rt![ChorusParam];

impl Param for ChorusParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 

    fn control_type(&self) -> ControlType {
        match self {
            ChorusParam::Time        => ControlType::DelayModTime,
            ChorusParam::Rate        => ControlType::LfoRate,
            ChorusParam::Depth       => ControlType::Percent,
            ChorusParam::Feedback    => ControlType::Amplitude,
            ChorusParam::LowCut      => ControlType::FreqAudible,
            ChorusParam::HighCut     => ControlType::FreqAudible,
            ChorusParam::Mix         => ControlType::Percent,
            ChorusParam::Width       => ControlType::DecibelNarrow,
            ChorusParam::ReturnLevel => ControlType::Percent,
        } 
    }

    fn default_value(&self) -> PData {
        match self {
            ChorusParam::Time        => PData::Float(-6.0),
            ChorusParam::Rate        => PData::Float(-2.0),
            ChorusParam::Depth       => PData::Float(0.30),
            ChorusParam::Feedback    => PData::Float(0.50),
            ChorusParam::LowCut      => PData::Float(-3.0 * 12.0),
            ChorusParam::HighCut     => PData::Float(3.0 * 12.0),
            ChorusParam::Mix         => PData::Float(1.0),
            ChorusParam::Width       => PData::Float(0.0),
            ChorusParam::ReturnLevel => PData::Float(1.0),
        }
    }

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

    fn min_value(&self) -> PData {
        match self {
            ChorusParam::Time        => PData::Float(-11.0),
            ChorusParam::Rate        => PData::Float(-7.0),
            ChorusParam::Depth       => PData::Float(0.0),
            ChorusParam::Feedback    => PData::Float(0.0),
            ChorusParam::LowCut      => PData::Float(-60.0),
            ChorusParam::HighCut     => PData::Float(-60.0),
            ChorusParam::Mix         => PData::Float(0.0),
            ChorusParam::Width       => PData::Float(-24.0),
            ChorusParam::ReturnLevel => PData::Float(0.0),
        }
    }

    fn max_value(&self) -> PData {
        match self {
            ChorusParam::Time        => PData::Float(-3.0),
            ChorusParam::Rate        => PData::Float(9.0),
            ChorusParam::Depth       => PData::Float(1.0),
            ChorusParam::Feedback    => PData::Float(1.0),
            ChorusParam::LowCut      => PData::Float(70.0),
            ChorusParam::HighCut     => PData::Float(70.0),
            ChorusParam::Mix         => PData::Float(1.0),
            ChorusParam::Width       => PData::Float(24.0),
            ChorusParam::ReturnLevel => PData::Float(1.0),
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            ChorusParam::Time        => ValType::VtFloat,
            ChorusParam::Rate        => ValType::VtFloat,
            ChorusParam::Depth       => ValType::VtFloat,
            ChorusParam::Feedback    => ValType::VtFloat,
            ChorusParam::LowCut      => ValType::VtFloat,
            ChorusParam::HighCut     => ValType::VtFloat,
            ChorusParam::Mix         => ValType::VtFloat,
            ChorusParam::Width       => ValType::VtFloat,
            ChorusParam::ReturnLevel => ValType::VtFloat,
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            ChorusParam::Time        => 1.0,
            ChorusParam::Rate        => 0.33,
            ChorusParam::Depth       => 1.0,
            ChorusParam::Feedback    => 1.0,
            ChorusParam::LowCut      => 1.0,
            ChorusParam::HighCut     => 1.0,
            ChorusParam::Mix         => 1.0,
            ChorusParam::Width       => 1.0,
            ChorusParam::ReturnLevel => 1.0,
        }
    }
}

impl ChorusParam {

    #[inline] pub fn new_runtime() -> ChorusParamArrayRT {
        ChorusParamArrayRT::new_with(|x| match x {
            ChorusParam::Time        => ChorusParamRT::new(ChorusParam::Time),
            ChorusParam::Rate        => ChorusParamRT::new(ChorusParam::Rate),
            ChorusParam::Depth       => ChorusParamRT::new(ChorusParam::Depth),
            ChorusParam::Feedback    => ChorusParamRT::new(ChorusParam::Feedback),
            ChorusParam::LowCut      => ChorusParamRT::new(ChorusParam::LowCut),
            ChorusParam::HighCut     => ChorusParamRT::new(ChorusParam::HighCut),
            ChorusParam::Mix         => ChorusParamRT::new(ChorusParam::Mix),
            ChorusParam::Width       => ChorusParamRT::new(ChorusParam::Width),
            ChorusParam::ReturnLevel => ChorusParamRT::new(ChorusParam::ReturnLevel),
        })
    }
}
