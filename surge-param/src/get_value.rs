crate::ix!();

pub trait GetParamVal {
    fn get_param_val(&self) -> PData;
}

pub trait GetValueF01 {
    fn get_value_f01(&self) -> f32;
}

impl<P: ParameterInterface + ?Sized> GetParamVal for ParamRT<P> {

    fn get_param_val(&self) -> PData {

        if self.control_type() == ControlType::Nil {
            return PData::Float(0.0);
        } 

        match (self.get_value(), self.max_value(), self.min_value()) {

            (PData::Float(v), PData::Float(max), PData::Float(min)) => { 
                PData::Float((v - min) / (max - min))
            },

            (PData::Int(v), PData::Int(max), PData::Int(min)) => { 
                PData::Int((0.005 + 0.99 * ((v - min) as f32) / ((max - min) as f32)) as i32)
            },

            (PData::Bool(v), _, _) =>  {
                //match v { true => 1.0, false => 0.0 } 
                PData::Bool(v)
            },

            _ => unreachable!(),
        }
    }
}

impl<P: ParameterInterface + ?Sized> GetValueF01 for ParamRT<P> {

    fn get_value_f01(&self) -> f32 {

        if self.control_type() == ControlType::Nil {
            return 0.0;
        }

        match (self.get_value(), self.max_value(), self.min_value()) {

            (PData::Float(v), PData::Float(max), PData::Float(min)) => { 
                (v - min) / (max - min) 
            },

            (PData::Int(v), PData::Int(max), PData::Int(min)) => { 
                0.005 + 0.99 * ((v - min) as f32) / ((max - min) as f32) 
            },

            (PData::Bool(v), _, _) =>  {
                match v { true => 1.0, false => 0.0 } 
            },

            _ => 0.0,
        }
    }
}
