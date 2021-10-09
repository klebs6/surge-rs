ix!();

use crate::{ClearFlags,GetSetModulation,PData,ControlType,ParamRT,Param,BoundValue};

impl<P: Param + ?Sized> GetSetModulation for ParamRT<P> {

    fn set_modulation_val(&mut self, val: PData) {
        self.modulation_delta = val;
    }
    fn get_modulation_val(&self) -> PData {
        self.modulation_delta
    }

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

    fn get_param_val(&self) -> PData {
        if self.control_type() == ControlType::Nil {
            PData::Float(0.0)
        } else {
            match (self.val, self.max_value(), self.min_value()) {

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

    fn get_value_f01(&self) -> f32 {
        if self.control_type() == ControlType::Nil {
            0.0
        } else {
            match (self.val, self.max_value(), self.min_value()) {

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


    fn set_value_f01(&mut self, v: f32, force_integer: bool) {

        match (self.val, self.min_value(), self.max_value()) {

            (PData::Float(_val), PData::Float(min), PData::Float(max)) => {
                self.val = PData::Float(v * (max - min) + min);
            },

            (PData::Int(val), PData::Int(min), PData::Int(max)) => {
                self.val = PData::Int(((
                            (1.0 / 0.99) 
                            * (val as f32 - 0.005) 
                            * ((max - min) as f32) + 0.5
                ) as i32) + min);
            },

            (PData::Bool(_val), _, _) => {
                self.val = PData::Bool(v > 0.5);
            },
            _ => unreachable!(),
        }
        self.bound_value(force_integer);

    }
}

impl<P: Param> ClearFlags for ParamRT<P> {
    fn clear_flags(&mut self) {
        self.temposync    = false;
        self.extend_range = false;
        self.absolute     = false;
        self.snap         = false;
    }
}
