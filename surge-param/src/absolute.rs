crate::ix!();

pub trait CheckIfAbsolute {

    fn is_absolute(&self) -> bool;
}

pub trait CheckIfCanBeAbsolute: GetControlType {

    fn can_be_absolute(&self) -> bool {
        matches!{
            self.control_type(),
            ControlType::OscSpread | ControlType::PitchSemi7BPAbsolutable
        }
    }
}

impl<P: ParameterInterface + ?Sized> CheckIfCanBeAbsolute for ParamRT<P> {}
