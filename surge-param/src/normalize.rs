ix!();

use crate::{PData,ParamRT,Param};

impl<P: Param> crate::ConvertValueToFromNormalized for ParamRT<P> {

    fn value2normalized(&self, value: f32) -> f32 {
        match (self.val, 
            self.delegate.min_value(), 
            self.delegate.max_value()) 
        {
            (_, PData::Float(min), PData::Float(max)) => {
                (value - min) / (max - min)
            },
            (_, PData::Int(min), PData::Int(max)) => {
                (value - (min as f32)) / ((max as f32) - (min as f32))
            },
            (PData::Bool(val), _, _) => {
                match val { true => 1.0, false => 0.0 }
            },
            _ => 0.0,
        }
    }

    fn normalized2value(&self, value: f32) -> f32 {
        match (self.delegate.min_value(), 
            self.delegate.max_value()) 
        {
            (PData::Float(min), PData::Float(max)) => {
                value * (max - min) + min
            },
            (PData::Int(min), PData::Int(max)) => {
                value * ((max as f32) - (min as f32)) + (min as f32)
            },
            (PData::Bool(_min), PData::Bool(_max)) => {
                match value > 0.5 { true => 1.0, false => 0.0 }
            },
            _ => 0.0,
        }
    }
}
