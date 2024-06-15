crate::ix!();

pub trait SetModulationVal {
    fn set_modulation_val(&mut self, val: PData);
}

pub trait SetModulation
: GetControlType 
+ GetParameterValueType
+ GetMinParameterValue
+ GetMaxParameterValue
{
    /// used by the gui to set the modulation to match the 
    /// position of the modulated handle
    fn set_modulation_f01(&self, v: f32) -> f32 {

        if self.control_type() == ControlType::Nil 
            || self.value_type() != ValType::VtFloat {
            return 0.0;
        }

        match (self.min_value(), self.max_value()) {
            (PData::Float(min), PData::Float(max)) => 
                v * (max - min),
            _ => unreachable!(),
        }
    }
}

impl<P: ParameterInterface + ?Sized> SetModulation for ParamRT<P> {}
