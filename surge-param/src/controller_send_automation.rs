crate::ix!();

pub trait ControllerSendParameterAutomation {

    fn send_parameter_automation(&mut self, index: usize, value: f32);
}
