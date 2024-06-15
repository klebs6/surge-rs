crate::ix!();

pub trait LimitParameterRange: ParameterInterface {

    fn limit_parameter_range(&mut self);
}

impl<P: ParameterInterface + ?Sized> LimitParameterRange for ParamRT<P> {

    fn limit_parameter_range(&mut self) {

        /*
           |clamp value between parameter minimum
           |and maximum should not need to be
           |called from the outside, because
           |a limited range is and invariant which
           |should be upheld internally
           */
        match (self.get_value(), self.min_value(), self.max_value()) {
            (PData::Float(f), PData::Float(min), PData::Float(max)) => {
                self.set_value(PData::Float(limit_range(f,min,max)));
            },
            (PData::Int(i), PData::Int(min), PData::Int(max)) => {
                self.set_value(PData::Int(limit_range(i,min,max)));
            },
            _ => { /*noop*/ }
        }
    }
}
