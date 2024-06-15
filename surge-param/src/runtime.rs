crate::ix!();

pub trait RuntimeDataFactory {

    type ParamArrayRT;

    fn new_runtime() -> Self::ParamArrayRT;
}
