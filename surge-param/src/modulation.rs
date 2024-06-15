crate::ix!();

pub trait CheckIfModulateable {

    fn modulateable(&self)
        -> bool { false }
}

impl<P: ParameterInterface + ?Sized> CheckIfModulateable for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn modulateable(&self) -> bool;
        }
    }
}
