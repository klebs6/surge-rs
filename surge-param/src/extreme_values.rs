crate::ix!();

pub trait GetMinParameterValue {
    fn min_value(&self)
        -> PData { PData::Float(0.0) }
}

pub trait GetMaxParameterValue {
    fn max_value(&self)
        -> PData { PData::Float(1.0) }
}

impl<P: ParameterInterface + ?Sized> GetMinParameterValue for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn min_value(&self) -> PData;
        }
    }
}

impl<P: ParameterInterface + ?Sized> GetMaxParameterValue for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn max_value(&self) -> PData;
        }
    }
}
