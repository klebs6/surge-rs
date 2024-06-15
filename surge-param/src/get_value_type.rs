crate::ix!();

pub trait GetParameterValueType {
    fn value_type(&self)
        -> ValType { ValType::VtFloat }
}

impl<P: ParameterInterface + ?Sized> GetParameterValueType for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn value_type(&self) -> ValType;
        }
    }
}
