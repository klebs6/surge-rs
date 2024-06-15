crate::ix!();

pub trait ParameterController
: ControllerSetParameter01 
+ ControllerSendParameterAutomation
+ ControllerGetParameter01
+ ControllerGetParameterAtIndex
+ ControllerGetParameterDisplay
+ ControllerGetParameterName
+ ControllerGetParameterMeta
+ ControllerSetParameterSmoothed
+ ControllerConvertNormalizedToValue
+ ControllerConvertValueToNormalized
{ }
