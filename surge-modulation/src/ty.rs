crate::ix!();

//#[enum_dispatch(ModulationSourceControl)]
#[derive(Debug)]
#[modulation_source_control]
pub enum ModulationSource {
    Lfo(Lfo),
    AdsrEnvelope(AdsrEnvelope),
    ControllerModulationSource(ControllerModulationSource),
}

pub type MaybeBoxedModulationSource = Option<Box<ModulationSource>>;
pub type ModulationSourceArray      = ModSourceArray::<MaybeBoxedModulationSource>;

impl GetModulationSourceType for ModulationSource {

    fn modulation_source_type(&self) -> ModSrcType {
        match self {
            ModulationSource::Lfo(inner)                        => inner.modulation_source_type(),
            ModulationSource::AdsrEnvelope(inner)               => inner.modulation_source_type(),
            ModulationSource::ControllerModulationSource(inner) => inner.modulation_source_type(),
        }
    }
}

impl GetModulationSourceOutput for ModulationSource {

    fn get_output(&self) -> f64 {
        match self {
            ModulationSource::Lfo(inner)                        => inner.get_output(),
            ModulationSource::AdsrEnvelope(inner)               => inner.get_output(),
            ModulationSource::ControllerModulationSource(inner) => inner.get_output(),
        }
    }

    fn get_output01(&self) -> f64 {
        match self {
            ModulationSource::Lfo(inner)                        => inner.get_output01(),
            ModulationSource::AdsrEnvelope(inner)               => inner.get_output01(),
            ModulationSource::ControllerModulationSource(inner) => inner.get_output01(),
        }
    }
}

impl SetModulationSourceOutput for ModulationSource {

    fn set_output(&mut self, x: f64) {

        match self {
            ModulationSource::Lfo(inner)                        => inner.set_output(x),
            ModulationSource::AdsrEnvelope(inner)               => inner.set_output(x as f32),
            ModulationSource::ControllerModulationSource(inner) => inner.set_output(x),
        }
    }
}

impl CheckIsModulationSourcePerVoice for ModulationSource {

    fn per_voice(&self) -> bool {
        match self {
            ModulationSource::Lfo(inner)                        => inner.per_voice(),
            ModulationSource::AdsrEnvelope(inner)               => inner.per_voice(),
            ModulationSource::ControllerModulationSource(inner) => inner.per_voice(),
        }
    }
}

impl CheckBipolar for ModulationSource {

    fn is_bipolar(&self) -> bool {
        match self {
            ModulationSource::Lfo(inner)                        => inner.is_bipolar(),
            ModulationSource::AdsrEnvelope(inner)               => inner.is_bipolar(),
            ModulationSource::ControllerModulationSource(inner) => inner.is_bipolar(),
        }
    }
}

impl SetBipolar for ModulationSource {

    fn set_bipolar(&mut self, b: bool) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.set_bipolar(b),
            ModulationSource::AdsrEnvelope(inner)               => inner.set_bipolar(b),
            ModulationSource::ControllerModulationSource(inner) => inner.set_bipolar(b),
        }
    }
}

impl ModulationSourceProcessBlock for ModulationSource {

    fn process_block(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.process_block(),
            ModulationSource::AdsrEnvelope(inner)               => inner.process_block(),
            ModulationSource::ControllerModulationSource(inner) => inner.process_block(),
        }
    }
}

impl Attack for ModulationSource {

    fn attack(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.attack(),
            ModulationSource::AdsrEnvelope(inner)               => inner.attack(),
            ModulationSource::ControllerModulationSource(inner) => inner.attack(),
        }
    }
}

impl Release for ModulationSource {

    fn release(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.release(),
            ModulationSource::AdsrEnvelope(inner)               => inner.release(),
            ModulationSource::ControllerModulationSource(inner) => inner.release(),
        }
    }
}

impl CheckEnabled for ModulationSource {

    fn enabled(&self) -> bool {
        match self {
            ModulationSource::Lfo(inner)                        => inner.enabled(),
            ModulationSource::AdsrEnvelope(inner)               => inner.enabled(),
            ModulationSource::ControllerModulationSource(inner) => inner.enabled(),
        }
    }
}

impl Enable for ModulationSource {

    fn enable(&mut self, v: bool) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.enable(v),
            ModulationSource::AdsrEnvelope(inner)               => inner.enable(v),
            ModulationSource::ControllerModulationSource(inner) => inner.enable(v),
        }
    }
}

impl Reset for ModulationSource {

    fn reset(&mut self) {
        match self {
            ModulationSource::Lfo(inner)                        => inner.reset(),
            ModulationSource::AdsrEnvelope(inner)               => inner.reset(),
            ModulationSource::ControllerModulationSource(inner) => inner.reset(),
        }
    }
}
