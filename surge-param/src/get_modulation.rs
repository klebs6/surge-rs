crate::ix!();

pub trait GetModulationVal {
    fn get_modulation_val(&self) -> PData;
}

//---------------------------------
pub trait GetModulation
: GetControlType 
+ GetParameterValueType
+ GetMinParameterValue
+ GetMaxParameterValue
{
    /// used by the gui to get the position of the modulated 
    /// handle
    fn get_modulation_f01(&self, modulation: f32) -> f32 {

        if self.control_type() == ControlType::Nil 
            || self.value_type() != ValType::VtFloat {
            return 0.0;
        }

        match (self.min_value(), self.max_value()) {
            (PData::Float(min), PData::Float(max)) => {
                let v = modulation / (max - min);
                limit_range(v, -1.0, 1.0)
            },
            _ => unreachable!(),
        }
    }
}

impl<P: ParameterInterface + ?Sized> GetModulation for ParamRT<P> {}
