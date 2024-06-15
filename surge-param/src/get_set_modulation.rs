crate::ix!();

pub trait GetSetModulation
: Debug 
+ SetModulationVal
+ GetModulationVal
+ SetParamVal
+ GetParamVal
+ GetValueF01
+ SetValueF01
{ }

impl<P: ParameterInterface + ?Sized> GetSetModulation for ParamRT<P> {}
