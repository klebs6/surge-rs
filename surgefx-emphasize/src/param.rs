crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum EmphasizeParam {
    Time,
    ReturnLevel,
}

impl_trait_defaults!{
    EmphasizeParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    GetControlStyle,
    GetDefaultValueF01,
    GetExtendedValue,
    GetExtendRange,
    GetModulation,
    GetSnap,
    SetModulation,
}

impl CheckIfAbsolute for EmphasizeParam {

    fn is_absolute(&self) -> bool {
        todo!();
    }
}

impl GetControlGroup for EmphasizeParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Fx } 
}

impl GetControlType for EmphasizeParam {

    fn control_type(&self) -> ControlType {
        match self {
            EmphasizeParam::Time => ControlType::DelayModTime,
            EmphasizeParam::ReturnLevel => ControlType::Percent,
        } 
    }
}

impl GetDefaultParameterValue for EmphasizeParam {

    fn default_value(&self) -> PData {
        match self {
            EmphasizeParam::Time => PData::Float(-6.0),
            EmphasizeParam::ReturnLevel => PData::Float(0.5),
        }
    }
}

impl CheckIfModulateable for EmphasizeParam {

    fn modulateable(&self) -> bool {
        //true for all
        true
    }
}

impl GetMinParameterValue for EmphasizeParam {

    fn min_value(&self) -> PData {
        match self {
            EmphasizeParam::Time => PData::Float(-11.0),
            EmphasizeParam::ReturnLevel => PData::Float(0.0),
        }
    }
}

impl GetMaxParameterValue for EmphasizeParam {

    fn max_value(&self) -> PData {
        match self {
            EmphasizeParam::Time => PData::Float(-3.0),
            EmphasizeParam::ReturnLevel => PData::Float(1.0),
        }
    }
}

impl GetParameterValueType for EmphasizeParam {

    fn value_type(&self) -> ValType {
        match self {
            EmphasizeParam::Time => ValType::VtFloat,
            EmphasizeParam::ReturnLevel => ValType::VtFloat,
        }
    }
}

impl GetMoverate for EmphasizeParam {
    fn moverate(&self) -> f32 {
        match self {
            EmphasizeParam::Time => 1.0,
            EmphasizeParam::ReturnLevel => 1.0,
        }
    }
}
