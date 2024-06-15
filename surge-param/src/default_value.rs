crate::ix!();

pub trait GetDefaultParameterValue {
    fn default_value(&self)
        -> PData { PData::Float(0.5) }
}

impl<P: ParameterInterface + ?Sized> GetDefaultParameterValue for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn default_value(&self) -> PData;
        }
    }
}

pub trait GetDefaultValueF01
: GetControlType 
+ GetDefaultParameterValue
+ GetMinParameterValue
+ GetMaxParameterValue
{

    fn get_default_value_f01(&self) -> f32 {

        if self.control_type() == ControlType::Nil {
            return 0.0
        }

        match ( self.default_value(), self.min_value(), self.max_value() ) {

            (PData::Float(default), PData::Float(min), PData::Float(max)) => {
              (default - min) / (max - min)
            },
            (PData::Int(default), PData::Int(min), PData::Int(max)) => {
              0.005 + 0.99 * ((default - min) as f32) / ((max - min) as f32)
            },
            (PData::Bool(default), _, _) => {
                match default { true => 1.0, false => 0.0 }
            },
            _ => 0.0,
        }
    }
}

impl<P: ParameterInterface + ?Sized> GetDefaultValueF01 for ParamRT<P> {}
