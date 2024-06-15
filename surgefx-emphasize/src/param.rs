crate::ix!();

enhanced_enum![EmphasizeParam {
    Time,
    ReturnLevel,
}];

rt![EmphasizeParam];

impl ParameterInterface for EmphasizeParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 

    fn control_type(&self) -> ControlType {
        match self {
            EmphasizeParam::Time => ControlType::DelayModTime,
            EmphasizeParam::ReturnLevel => ControlType::Percent,
        } 
    }

    fn default_value(&self) -> PData {
        match self {
            EmphasizeParam::Time => PData::Float(-6.0),
            EmphasizeParam::ReturnLevel => PData::Float(0.5),
        }
    }

    fn modulateable(&self) -> bool {
        //true for all
        true
    }

    fn min_value(&self) -> PData {
        match self {
            EmphasizeParam::Time => PData::Float(-11.0),
            EmphasizeParam::ReturnLevel => PData::Float(0.0),
        }
    }

    fn max_value(&self) -> PData {
        match self {
            EmphasizeParam::Time => PData::Float(-3.0),
            EmphasizeParam::ReturnLevel => PData::Float(1.0),
        }
    }

    fn value_type(&self) -> ValType {
        match self {
            EmphasizeParam::Time => ValType::VtFloat,
            EmphasizeParam::ReturnLevel => ValType::VtFloat,
        }
    }

    fn moverate(&self) -> f32 {
        match self {
            EmphasizeParam::Time => 1.0,
            EmphasizeParam::ReturnLevel => 1.0,
        }
    }
}

impl EmphasizeParam {
    #[inline] pub fn new_runtime() -> EmphasizeParamArrayRT {
        EmphasizeParamArrayRT::new_with(|x| match x {
            EmphasizeParam::Time        => EmphasizeParamRT::new(EmphasizeParam::Time),
            EmphasizeParam::ReturnLevel => EmphasizeParamRT::new(EmphasizeParam::ReturnLevel),
        })
    }
}
