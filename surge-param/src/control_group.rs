crate::ix!();

pub type ControlGroupEntry = i16;

pub trait GetControlGroup {

    fn control_group(&self)
        -> ControlGroup { ControlGroup::Nil }
}

impl<P: ParameterInterface + ?Sized> GetControlGroup for ParamRT<P> {

    delegate!{
        to self.delegate {
            fn control_group(&self) -> ControlGroup;
        }
    }
}

enhanced_enum!{
    ControlGroup {
        Nil,
        Global,
        Osc,
        Mix,
        Filter,
        Env,
        Lfo,
        Fx,
    }
}
