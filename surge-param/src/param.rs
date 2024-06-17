crate::ix!();

pub trait AssocParam {
    type ParamType;
}

pub trait ParameterInterface
: Debug 
+ GetControlType
+ GetControlGroup
+ GetControlStyle
+ GetDefaultParameterValue
+ CheckIfModulateable
+ GetMinParameterValue
+ GetMaxParameterValue
+ GetParameterValueType
+ GetMoverate
+ GetSnap
+ GetExtendRange
+ CheckIfAffectsOtherParameters
+ CheckIfCanTemposync
+ CheckIfCanExtendRange
+ CheckIfCanBeAbsolute
+ CheckIfAbsolute
+ CheckIfCanSnap
+ GetModulation
+ SetModulation
+ GetExtendedValue
+ GetDefaultValueF01
{ }
