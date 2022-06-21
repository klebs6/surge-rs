crate::ix!();

enhanced_enum![FreqShiftParam {
    Shift,
    RMult,
    Delay,
    Feedback,
    Mix,
    ReturnLevel,
}];

rt![FreqShiftParam];

impl Param for FreqShiftParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
    fn control_type(&self) -> ControlType {
        match self {
            FreqShiftParam::Shift       => ControlType::FreqShift,
            FreqShiftParam::RMult       => ControlType::PercentBidirectional,
            FreqShiftParam::Delay       => ControlType::EnvTime,
            FreqShiftParam::Feedback    => ControlType::Amplitude,
            FreqShiftParam::Mix         => ControlType::Percent,
            FreqShiftParam::ReturnLevel => ControlType::Percent,
        }
    }
    fn default_value(&self) -> PData {
        match self {
            FreqShiftParam::Shift       => PData::Float(0.0),
            FreqShiftParam::RMult       => PData::Float(1.0),
            FreqShiftParam::Delay       => PData::Float(-8.0),
            FreqShiftParam::Feedback    => PData::Float(0.0),
            FreqShiftParam::Mix         => PData::Float(1.0),
            FreqShiftParam::ReturnLevel => PData::Float(0.5),
        }
    }
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
    fn min_value(&self) -> PData {
        match self {
            FreqShiftParam::Shift       => PData::Float(-10.0),
            FreqShiftParam::RMult       => PData::Float(-1.0),
            FreqShiftParam::Delay       => PData::Float(-8.0),
            FreqShiftParam::Feedback    => PData::Float(0.0),
            FreqShiftParam::Mix         => PData::Float(0.0),
            FreqShiftParam::ReturnLevel => PData::Float(0.0),
        }
    }
    fn max_value(&self) -> PData {
        match self {
            FreqShiftParam::Shift       => PData::Float(10.0),
            FreqShiftParam::RMult       => PData::Float(1.0),
            FreqShiftParam::Delay       => PData::Float(5.0),
            FreqShiftParam::Feedback    => PData::Float(1.0),
            FreqShiftParam::Mix         => PData::Float(1.0),
            FreqShiftParam::ReturnLevel => PData::Float(1.0),
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            FreqShiftParam::Shift       => ValType::VtFloat,
            FreqShiftParam::RMult       => ValType::VtFloat,
            FreqShiftParam::Delay       => ValType::VtFloat,
            FreqShiftParam::Feedback    => ValType::VtFloat,
            FreqShiftParam::Mix         => ValType::VtFloat,
            FreqShiftParam::ReturnLevel => ValType::VtFloat,
        }
    }
    fn moverate(&self) -> f32 {
        match self {
            FreqShiftParam::Shift       => 1.0,
            FreqShiftParam::RMult       => 1.0,
            FreqShiftParam::Delay       => 1.0,
            FreqShiftParam::Feedback    => 1.0,
            FreqShiftParam::Mix         => 1.0,
            FreqShiftParam::ReturnLevel => 1.0,
        }
    }
}

impl FreqShiftParam {
    #[inline] pub fn new_runtime() -> FreqShiftParamArrayRT {
        FreqShiftParamArrayRT::new_with(|x| match x {
            FreqShiftParam::Shift       => FreqShiftParamRT::new(FreqShiftParam::Shift),
            FreqShiftParam::RMult       => FreqShiftParamRT::new(FreqShiftParam::RMult),
            FreqShiftParam::Delay       => FreqShiftParamRT::new(FreqShiftParam::Delay),
            FreqShiftParam::Feedback    => FreqShiftParamRT::new(FreqShiftParam::Feedback),
            FreqShiftParam::Mix         => FreqShiftParamRT::new(FreqShiftParam::Mix),
            FreqShiftParam::ReturnLevel => FreqShiftParamRT::new(FreqShiftParam::ReturnLevel),
        })
    }
}
