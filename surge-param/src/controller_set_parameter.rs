crate::ix!();

pub trait ControllerSetParameter01 {

    fn set_parameter01<P: ParameterInterface + ?Sized>(
        &mut self,
        param:            &mut P,
        value:            f32,
        is_external:      bool,
        is_force_integer: bool

    ) -> bool;
}

pub trait ControllerSetParameterSmoothed {
    fn set_parameter_smoothed(&mut self, index: usize, value: f32);
}
