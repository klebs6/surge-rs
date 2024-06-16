/*!
  | Overall, this code defines the different
  | parameters that can be controlled for an LFO
  | and provides functionality to create runtime
  | representations of these parameters.
  |
  */

crate::ix!();

/// This code defines an enhanced enum called `LfoParam`, which represents the
/// different parameters of an LFO (low-frequency oscillator) that can be
/// controlled. 
///
/// I have used the ordering here in CLFOGui to iterate.
/// 
/// Be careful if rate or release move from first/last position.
///
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
#[synth_parameters]
pub enum LfoParam {
    Rate,
    Shape,
    StartPhase,
    Magnitude,
    Deform,
    Trigmode,
    Unipolar,
    Delay,
    Hold,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl_trait_defaults!{
    LfoParam;
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

/// The `ParameterInterface` trait is implemented for
/// `LfoParam`, indicating that it is a parameter
/// that can be controlled. 
///
/// The `control_group` method returns
/// `ControlGroup::Lfo`, which specifies that
/// these parameters are part of the LFO control
/// group.
///
impl GetControlGroup for LfoParam {

    fn control_group(&self) -> ControlGroup { ControlGroup::Lfo } 
}
