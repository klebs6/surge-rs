ix!();

use crate::*;

impl SurgeScene {

    #[allow(unreachable_code)]
    pub fn play_voice(&mut self, 
        _channel: i32, 
        _key: u8, 
        _velocity: u8, 
        _detune: u8) 
    {

        let non_released_voices = self.get_non_released_voices();

        if non_released_voices == 0 {

            for l in 0..N_LFOS_PER_SCENE {

                let ms = ModSource::scene_lfo(l);

                self.modsources[ms].as_mut().unwrap().attack();
            }
        }

        let polylimit = pvali![self.params[SceneParam::PolyLimit]];

        let excess_voices = surge_math::maxi(
            0, 
            self.get_non_ultra_release_voices() - polylimit + 1
        );

        for _i in 0..excess_voices {
            self.softkill_voice();
        }

        self.enforce_polyphony_limit(polylimit, 3);

        let polymode = self.get_polymode();

        let _channel: u8 = _channel.try_into().unwrap();

        match polymode {
            PolyMode::Poly => {
                self.play_voice_poly(
                    _channel,
                    _key,
                    _velocity,
                    _detune)
            },
            PolyMode::Mono | PolyMode::MonoFingeredPortamento | PolyMode::LatchMonophonic => {
                self.play_voice_mono(
                    _channel,
                    _key,
                    _velocity,
                    _detune)
            },

            PolyMode::MonoSingleTriggerEG | PolyMode::MonoSingleTriggerFingeredPortamento => {
                self.play_voice_portamento(
                    _channel,
                    _key,
                    _velocity,
                    _detune)
            },
        }
    }
}
