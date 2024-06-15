crate::ix!();

pub trait SetParamVal {
    fn set_param_val(&mut self, val: PData);
}

pub trait SetValueF01 {
    fn set_value_f01(&mut self, v: f32, is_force_integer: bool);
}

impl<P: ParameterInterface + ?Sized> SetParamVal for ParamRT<P> {

    fn set_param_val(&mut self, val: PData) {
        match val {
            PData::Float(x) => self.set_value_f01(x, false),
            PData::Int(x) => self.set_value_f01(x as f32, true),
            PData::Bool(x) => match x {
                true  => self.set_value_f01(1.0, false),
                false => self.set_value_f01(0.0, false),
            },
        }
    }
}

impl<P: ParameterInterface + ?Sized> SetValueF01 for ParamRT<P> {

    fn set_value_f01(&mut self, v: f32, force_integer: bool) {

        match (self.get_value(), self.min_value(), self.max_value()) {

            (PData::Float(_val), PData::Float(min), PData::Float(max)) => {
                self.set_value(PData::Float(v * (max - min) + min));
            },

            (PData::Int(val), PData::Int(min), PData::Int(max)) => {
                self.set_value(
                    PData::Int(((
                                (1.0 / 0.99) 
                                * (val as f32 - 0.005) 
                                * ((max - min) as f32) + 0.5
                    ) as i32) + min)
                );
            },

            (PData::Bool(_val), _, _) => {
                self.set_value(PData::Bool(v > 0.5));
            },
            _ => unreachable!(),
        }
        self.bound_value(force_integer);

    }
}
