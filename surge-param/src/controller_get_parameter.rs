crate::ix!();

pub trait ControllerGetParameter01 {
    fn get_parameter01(&mut self, index: usize) -> f32;
}

pub trait ControllerGetParameterAtIndex {
    fn get_parameter(&mut self, index: usize) -> f32;
}

pub trait ControllerGetParameterDisplay {
    fn get_parameter_display(&mut self, index: usize, text: *mut char);
    fn get_parameter_display_alt1(&mut self, index: usize, text: *mut char, x: f32);
    fn get_parameter_display_alt2(&mut self, index: usize, text: *mut char);
}

pub trait ControllerGetParameterName {
    fn get_parameter_name(&mut self, index: usize, text: *mut char);
}

pub trait ControllerGetParameterMeta {
    fn get_parameter_meta(&mut self, index: usize, pm: &mut crate::ParameterMeta);
}
