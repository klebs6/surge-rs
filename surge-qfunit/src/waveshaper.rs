crate::ix!();

/// This is implemented as a bunch of function
/// pointers which could potentially be passed
/// back up to the caller
/// 
/// TODO: eventually make a type of param which is
/// just a selector for one of several enum
/// options
///
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum WaveshaperParam {
    Type,
    Drive,
}

impl_trait_defaults!{
    WaveshaperParam;
    CheckIfAffectsOtherParameters,
    CheckIfCanBeAbsolute,
    CheckIfCanExtendRange,
    CheckIfCanSnap,
    CheckIfCanTemposync,
    CheckIfModulateable,
    GetControlStyle,
    GetControlType,
    GetDefaultParameterValue,
    GetDefaultValueF01,
    GetExtendRange,
    GetExtendedValue,
    GetMaxParameterValue,
    GetMinParameterValue,
    GetModulation,
    GetMoverate,
    GetParameterValueType,
    GetSnap,
    SetModulation,
}

impl GetControlGroup for WaveshaperParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Filter } 
}

#[derive(Debug)]
pub struct WaveshaperUnit {
    pub params: WaveshaperParamArray::<ParamRT::<WaveshaperParam>>,
}

impl Default for WaveshaperUnit {
    fn default() -> Self {
        todo!();
    }
}

#[derive(Debug)]
pub struct WaveshaperState {
    pub tables: TablesHandle,
}
