crate::ix!();

pub trait GetModulationSourceType {
    fn modulation_source_type(&self) -> ModSrcType;
}

pub trait SetModulationSourceOutput {
    fn set_output(&mut self, x: f64);
}

pub trait GetModulationSourceOutput {
    fn get_output(&self)    -> f64;
    fn get_output01(&self)  -> f64;
}

pub const trait CheckIsModulationSourcePerVoice {
    const fn per_voice(&self) -> bool { false }
}

pub const trait CheckIsModulationSourceBipolar {
    const fn is_bipolar(&self) -> bool { false }
}

pub trait SetBipolar {
    fn set_bipolar(&mut self, b: bool);
}

pub trait ProcessBlock {
    fn process_block(&mut self);
}

pub trait Attack {
    fn attack(&mut self) {}
}

pub trait Release {
    fn release(&mut self) {}
}

pub trait Reset {
    fn reset(&mut self) {}
}

pub trait Enable {
    fn enable(&mut self, v: bool);
}

pub trait CheckIsModulationSourceEnabled {
    fn enabled(&self) -> bool;
}

//TODO can we get enum_dipatch for lifetimes?
#[enum_dispatch]
pub trait ModulationSourceControl
: Debug 
+ Attack
+ CheckIsModulationSourceBipolar
+ CheckIsModulationSourceEnabled
+ CheckIsModulationSourcePerVoice
+ Enable
+ GetModulationSourceOutput
+ GetModulationSourceType
+ ModulationSourceControl
+ ProcessBlock
+ Release
+ Reset
+ SetBipolar
+ SetModulationSourceOutput
{ }
