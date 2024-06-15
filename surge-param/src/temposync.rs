crate::ix!();

pub trait CheckIfCanTemposync
: GetControlType 
{
    fn can_temposync(&self) -> bool {
        matches!{
            self.control_type(), 
            ControlType::PortaTime 
                | ControlType::LfoRate 
                | ControlType::EnvTime 
                | ControlType::EnvTimeLfoDecay 
        }
    }
}

impl<P: ParameterInterface + ?Sized> CheckIfCanTemposync for ParamRT<P> {}
