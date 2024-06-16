crate::ix!();

//TODO can we get enum_dipatch for lifetimes?
#[enum_dispatch]
pub trait ModulationSourceControl
: Debug 
+ Attack
+ CheckBipolar
+ CheckEnabled
+ CheckIsModulationSourcePerVoice
+ Enable
+ GetModulationSourceOutput
+ GetModulationSourceType
+ ModulationSourceProcessBlock
+ Release
+ Reset
+ SetBipolar
+ SetModulationSourceOutput
{ }

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

pub trait CheckIsModulationSourcePerVoice {
    fn per_voice(&self) -> bool { false }
}

pub trait CheckBipolar {
    fn is_bipolar(&self) -> bool { false }
}

pub trait SetBipolar {
    fn set_bipolar(&mut self, b: bool);
}

pub trait ModulationSourceProcessBlock {
    fn process_block(&mut self);
}

pub trait Attack {
    fn attack(&mut self) {}
}

pub trait Release {
    fn release(&mut self) {}
}

pub trait Enable {
    fn enable(&mut self, v: bool);
}

pub trait CheckEnabled {
    fn enabled(&self) -> bool;
}
