crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum AdsrParam {
    Attack,
    Decay,
    Sustain,
    Release,
    AttackShape,
    DecayShape,
    ReleaseShape,
    Mode,
}

impl_trait_defaults!{
    AdsrParam;
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

impl GetControlGroup for AdsrParam {
    fn control_group(&self) -> ControlGroup { ControlGroup::Env } 
}
