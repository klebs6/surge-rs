crate::ix!();

enhanced_enum![
    ConditionerParam {
        Bass,
        Treble,
        Width,
        Balance,
        Threshold,
        AttackRate,
        ReleaseRate,
        Gain,
        ReturnLevel,
    }
];

rt![ConditionerParam];

impl Param for ConditionerParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
    fn control_type(&self) -> ControlType {
        match self {
            ConditionerParam::Bass        => ControlType::DecibelExtraNarrow,
            ConditionerParam::Treble      => ControlType::DecibelExtraNarrow,
            ConditionerParam::Width       => ControlType::PercentBidirectional,
            ConditionerParam::Balance     => ControlType::PercentBidirectional,
            ConditionerParam::Threshold   => ControlType::DecibelAttenuation,
            ConditionerParam::AttackRate  => ControlType::PercentBidirectional,
            ConditionerParam::ReleaseRate => ControlType::PercentBidirectional,
            ConditionerParam::Gain        => ControlType::DecibelAttenuation,
            ConditionerParam::ReturnLevel => ControlType::Percent,
        }
    }
    fn default_value(&self) -> PData {
        match self {
            ConditionerParam::Bass        => PData::Float(0.0),
            ConditionerParam::Treble      => PData::Float(0.0),
            ConditionerParam::Width       => PData::Float(0.0),
            ConditionerParam::Balance     => PData::Float(0.0),
            ConditionerParam::Threshold   => PData::Float(0.0),
            ConditionerParam::AttackRate  => PData::Float(0.0),
            ConditionerParam::ReleaseRate => PData::Float(0.0),
            ConditionerParam::Gain        => PData::Float(0.0),
            ConditionerParam::ReturnLevel => PData::Float(0.5),
        }
    }
    fn modulateable(&self) -> bool {
        //true for all
        true
    }
    fn min_value(&self) -> PData {
        match self {
            ConditionerParam::Bass        => PData::Float(-12.0),
            ConditionerParam::Treble      => PData::Float(-12.0),
            ConditionerParam::Width       => PData::Float(-1.0), 
            ConditionerParam::Balance     => PData::Float(-1.0), 
            ConditionerParam::Threshold   => PData::Float(-48.0),
            ConditionerParam::AttackRate  => PData::Float(-1.0), 
            ConditionerParam::ReleaseRate => PData::Float(-1.0), 
            ConditionerParam::Gain        => PData::Float(-48.0),
            ConditionerParam::ReturnLevel => PData::Float(0.0),
        }
    }
    fn max_value(&self) -> PData {
        match self {
            ConditionerParam::Bass        => PData::Float(12.0),
            ConditionerParam::Treble      => PData::Float(12.0),
            ConditionerParam::Width       => PData::Float(1.0), 
            ConditionerParam::Balance     => PData::Float(1.0), 
            ConditionerParam::Threshold   => PData::Float(0.0), 
            ConditionerParam::AttackRate  => PData::Float(1.0), 
            ConditionerParam::ReleaseRate => PData::Float(1.0), 
            ConditionerParam::Gain        => PData::Float(0.0), 
            ConditionerParam::ReturnLevel => PData::Float(1.0),
        }
    }
    fn value_type(&self) -> ValType {
        match self {
            ConditionerParam::Bass        => ValType::VtFloat,
            ConditionerParam::Treble      => ValType::VtFloat,
            ConditionerParam::Width       => ValType::VtFloat,
            ConditionerParam::Balance     => ValType::VtFloat,
            ConditionerParam::Threshold   => ValType::VtFloat,
            ConditionerParam::AttackRate  => ValType::VtFloat,
            ConditionerParam::ReleaseRate => ValType::VtFloat,
            ConditionerParam::Gain        => ValType::VtFloat,
            ConditionerParam::ReturnLevel => ValType::VtFloat,
        }
    }
    fn moverate(&self) -> f32 {
        match self {
            ConditionerParam::Bass        => 1.0,
            ConditionerParam::Treble      => 1.0,
            ConditionerParam::Width       => 1.0,
            ConditionerParam::Balance     => 1.0,
            ConditionerParam::Threshold   => 1.0,
            ConditionerParam::AttackRate  => 1.0,
            ConditionerParam::ReleaseRate => 1.0,
            ConditionerParam::Gain        => 1.0,
            ConditionerParam::ReturnLevel => 1.0,
        }
    }
}

impl ConditionerParam {
    #[inline] pub fn new_runtime() -> ConditionerParamArrayRT {
        ConditionerParamArrayRT::new_with(|x| match x {
            ConditionerParam::Bass        => ConditionerParamRT::new(ConditionerParam::Bass        ),
            ConditionerParam::Treble      => ConditionerParamRT::new(ConditionerParam::Treble      ),
            ConditionerParam::Width       => ConditionerParamRT::new(ConditionerParam::Width       ),
            ConditionerParam::Balance     => ConditionerParamRT::new(ConditionerParam::Balance     ),
            ConditionerParam::Threshold   => ConditionerParamRT::new(ConditionerParam::Threshold   ),
            ConditionerParam::AttackRate  => ConditionerParamRT::new(ConditionerParam::AttackRate  ),
            ConditionerParam::ReleaseRate => ConditionerParamRT::new(ConditionerParam::ReleaseRate ),
            ConditionerParam::Gain        => ConditionerParamRT::new(ConditionerParam::Gain        ),
            ConditionerParam::ReturnLevel => ConditionerParamRT::new(ConditionerParam::ReturnLevel ),
        })
    }
}
