crate::ix!();

pub trait GetMoverate {

    fn moverate(&self)
        -> f32 { 1.0 }
}

impl<P: ParameterInterface + ?Sized> GetMoverate for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn moverate(&self) -> f32;
        }
    }
}
