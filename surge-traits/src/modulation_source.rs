crate::ix!();

//TODO can we get enum_dipatch for lifetimes?
#[enum_dispatch]
pub trait ModulationSourceControl: Debug {
    fn get_type(&self)      -> ModSrcType;
    fn get_output(&self)    -> f64;
    fn get_output01(&self)  -> f64;
    fn per_voice(&self)     -> bool { false }
    fn is_bipolar(&self)    -> bool { false }
    fn set_bipolar(&mut self, b: bool);
    fn process_block(&mut self);
    fn attack(&mut self) {}
    fn release(&mut self) {}
    fn reset(&mut self) {}
    fn set_output(&mut self, x: f64);
    fn enable(&mut self, v: bool);
    fn enabled(&self) -> bool;
}
