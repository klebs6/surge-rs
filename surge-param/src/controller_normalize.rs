crate::ix!();

pub trait ControllerConvertNormalizedToValue {
    fn normalized_to_value(&mut self, parameter_index: usize, value: f32) -> f32;
}

pub trait ControllerConvertValueToNormalized {
    fn value_to_normalized(&mut self, parameter_index: usize, value: f32) -> f32;
}
