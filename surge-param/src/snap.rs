crate::ix!();

pub trait GetSnap {

    fn snap(&self)
        -> bool { true }
}

pub trait SetSnap {
    fn set_snap(&mut self, snap: bool);
}

impl<P: ParameterInterface + ?Sized> GetSnap for ParamRT<P> {
    delegate!{
        to self.delegate {
            fn snap(&self) -> bool;
        }
    }
}

pub trait CheckIfCanSnap
: GetControlType
{
    fn can_snap(&self) -> bool {
        matches!{
            self.control_type(),
            ControlType::CountedSetPercent
        }
    }
}

impl<P: ParameterInterface + ?Sized> CheckIfCanSnap for ParamRT<P> {}
