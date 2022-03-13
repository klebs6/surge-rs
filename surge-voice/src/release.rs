ix!();

use crate::*;

impl SurgeVoice {

    pub fn release(&mut self) {

        macro_rules! release {
            ($key:ident,$variant:tt) => {
                match &mut self.modsources[ModSource::$key] {
                    Some(Box::new(ModulationSource::$variant(modsource))) => {
                        modsource.release();
                    },
                    _ => panic!("bug in release function"),
                }
            }
        }

        release![AmpEg,   AdsrEnvelope];
        release![FilterEg,AdsrEnvelope];

        release![VoiceLfo1,Lfo];
        release![VoiceLfo2,Lfo];
        release![VoiceLfo3,Lfo];
        release![VoiceLfo4,Lfo];
        release![VoiceLfo5,Lfo];
        release![VoiceLfo6,Lfo];

        self.state.gate = false;

        match &mut self.modsources[ModSource::ReleaseVelocity] {
            Some(Box::new(ModulationSource::ControllerModulationSource(rv))) => {
                rv.output = self.state.releasevelocity as f64 / 127.0;
            },
            _ => panic!("bug in release function"),
        }
    }

    pub fn uber_release(&mut self) {
        macro_rules! uber_release {
            ($key:ident,$variant:tt) => {
                match &mut self.modsources[ModSource::$key] {
                    Some(Box::new(ModulationSource::$variant(modsource))) => {
                        modsource.uber_release();
                    },
                    _ => panic!("bug in release function"),
                }
            }
        }
        uber_release![AmpEg,AdsrEnvelope];

        self.state.gate = false;
        self.state.uberrelease = true;
    }

    pub fn get_temposyncratio(&mut self) -> f32 {
        todo!();
    }
}
