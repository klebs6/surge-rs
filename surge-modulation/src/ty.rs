ix!();

use crate::{
    ControllerModulationSource,
    ModSourceArray,
};

//#[enum_dispatch(ModulationSourceControl)]
#[derive(Debug)]
pub enum ModulationSource {
    Lfo(Lfo),
    AdsrEnvelope(AdsrEnvelope),
    ControllerModulationSource(ControllerModulationSource),
}

pub type MaybeBoxedModulationSource 
= Option<Box<ModulationSource>>;

pub type ModulationSourceArray 
= ModSourceArray::<MaybeBoxedModulationSource>;

impl ModulationSourceControl for ModulationSource {

    fn get_type(&self) -> ModSrcType {
        match self {
            ModulationSource::Lfo(inner)                        => inner.get_type(),
            ModulationSource::AdsrEnvelope(inner)               => inner.get_type(),
            ModulationSource::ControllerModulationSource(inner) => inner.get_type(),
        }
    }
    fn get_output(&self) -> f64 {
        match self {
            ModulationSource::Lfo(inner)                        => inner.get_output(),
            ModulationSource::AdsrEnvelope(inner)               => inner.get_output(),
            ModulationSource::ControllerModulationSource(inner) => inner.get_output(),
        }
    }
    fn set_output(&mut self, x: f64) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.set_output(x),
            ModulationSource::AdsrEnvelope(inner)               => inner.set_output(x),
            ModulationSource::ControllerModulationSource(inner) => inner.set_output(x),
        }
    }
    fn get_output01(&self) -> f64 {
        match self {
            ModulationSource::Lfo(inner)                        => inner.get_output01(),
            ModulationSource::AdsrEnvelope(inner)               => inner.get_output01(),
            ModulationSource::ControllerModulationSource(inner) => inner.get_output01(),
        }
    }
    fn per_voice(&self) -> bool {
        match self {
            ModulationSource::Lfo(inner)                        => inner.per_voice(),
            ModulationSource::AdsrEnvelope(inner)               => inner.per_voice(),
            ModulationSource::ControllerModulationSource(inner) => inner.per_voice(),
        }
    }
    fn is_bipolar(&self) -> bool {
        match self {
            ModulationSource::Lfo(inner)                        => inner.is_bipolar(),
            ModulationSource::AdsrEnvelope(inner)               => inner.is_bipolar(),
            ModulationSource::ControllerModulationSource(inner) => inner.is_bipolar(),
        }
    }
    fn set_bipolar(&mut self, b: bool) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.set_bipolar(b),
            ModulationSource::AdsrEnvelope(inner)               => inner.set_bipolar(b),
            ModulationSource::ControllerModulationSource(inner) => inner.set_bipolar(b),
        }
    }
    fn process_block(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.process_block(),
            ModulationSource::AdsrEnvelope(inner)               => inner.process_block(),
            ModulationSource::ControllerModulationSource(inner) => inner.process_block(),
        }
    }
    fn attack(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.attack(),
            ModulationSource::AdsrEnvelope(inner)               => inner.attack(),
            ModulationSource::ControllerModulationSource(inner) => inner.attack(),
        }
    }
    fn release(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.release(),
            ModulationSource::AdsrEnvelope(inner)               => inner.release(),
            ModulationSource::ControllerModulationSource(inner) => inner.release(),
        }
    }
    fn enabled(&self) -> bool {
        match self {
            ModulationSource::Lfo(inner)                        => inner.enabled(),
            ModulationSource::AdsrEnvelope(inner)               => inner.enabled(),
            ModulationSource::ControllerModulationSource(inner) => inner.enabled(),
        }
    }
    fn enable(&mut self, v: bool) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.enable(v),
            ModulationSource::AdsrEnvelope(inner)               => inner.enable(v),
            ModulationSource::ControllerModulationSource(inner) => inner.enable(v),
        }
    }
    fn reset(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.reset(),
            ModulationSource::AdsrEnvelope(inner)               => inner.reset(),
            ModulationSource::ControllerModulationSource(inner) => inner.reset(),
        }
    }
}
