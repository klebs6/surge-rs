crate::ix!();

pub trait CheckIfAffectsOtherParameters {

    fn affect_other_parameters(&self)
        -> bool { false }
}

impl<P: ParameterInterface + ?Sized> CheckIfAffectsOtherParameters for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn affect_other_parameters(&self) -> bool;
        }
    }
}
