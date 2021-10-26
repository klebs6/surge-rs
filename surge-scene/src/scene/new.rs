ix!();

use crate::{
    SurgeScene,
    SceneParam,
};

pub struct SceneConstructorHandles {
    pub timeunit:        &  TimeUnitHandle,
    pub tables:          &  TablesHandle,
    pub tuner:           &  TunerHandle,
    pub srunit:          &  SampleRateHandle,
    pub hold_pedal_unit: &  HoldPedalUnitHandle,
    pub midi_unit:       &  MIDIUnitHandle,
    pub mpe_unit:        &  MPEUnitHandle,
    pub synth_in:        &  SynthInputHandle,
}

impl SurgeScene {
    pub fn new(ctor: SceneConstructorHandles) -> Self {
        Self {
            osc:                 vec![
                box SurgeSuperOscillator::new(ctor.tuner.clone(),ctor.tables.clone(),ctor.srunit.clone()),
                box SurgeSuperOscillator::new(ctor.tuner.clone(),ctor.tables.clone(),ctor.srunit.clone()),
                box SurgeSuperOscillator::new(ctor.tuner.clone(),ctor.tables.clone(),ctor.srunit.clone()),
            ],
            params:              SceneParam::new_runtime(),
            filterunit:          vec![
                FilterUnit::default(),
                FilterUnit::default()
            ],
            waveshaper_unit:     vec![
                WaveshaperUnit::default(),
            ],
            asdr:                vec![
                AdsrEnvelope::new(ctor.timeunit.clone(),ctor.tables.clone(),ctor.srunit.clone()),
                AdsrEnvelope::new(ctor.timeunit.clone(),ctor.tables.clone(),ctor.srunit.clone()),
            ],
            lfo:                 vec![
                Lfo::new(ctor.timeunit.clone(),ctor.tables.clone()),
                Lfo::new(ctor.timeunit.clone(),ctor.tables.clone()),
                Lfo::new(ctor.timeunit.clone(),ctor.tables.clone()),
                Lfo::new(ctor.timeunit.clone(),ctor.tables.clone()),
                Lfo::new(ctor.timeunit.clone(),ctor.tables.clone()),
                Lfo::new(ctor.timeunit.clone(),ctor.tables.clone()),
            ],
            modulation_scene:    vec![],
            modulation_voice:    vec![],
            modsources:          Self::new_modsources(
                ctor.srunit.clone(),
                ctor.timeunit.clone(),
                ctor.tables.clone()
            ),
            stepsequences:       vec![
                StepSequencer::default(),
                StepSequencer::default(),
                StepSequencer::default(),
                StepSequencer::default(),
                StepSequencer::default(),
                StepSequencer::default(),
            ],
            send: [
                LipolPs::new_with_blocksize(BLOCK_SIZE),
                LipolPs::new_with_blocksize(BLOCK_SIZE),
            ],
            returnfx:      LipolPs::new_with_blocksize(BLOCK_SIZE),
            halfband:      HalfRateFilterSSE::default(),
            highpass:      BiquadFilter::new(ctor.tuner,ctor.tables,ctor.srunit),
            fbq:           box QuadFilterChain::new(MAX_VOICES >> 2, ctor.tables),
            voices:        vec![], 
            voices_array:  std::pin::Pin::new(box [ //MAX_VOICES = 64
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ]),
            out:                    WetBlock1Dual::<BLOCK_SIZE_OS>::default(),
            tables:                 ctor.tables.clone(),
            tuner:                  ctor.tuner.clone(),
            release_if_latched:     true,
            release_anyway:         false,
            hold_pedal_unit:        ctor.hold_pedal_unit.clone(),
            midi_unit:              ctor.midi_unit.clone(),
            mpe_unit:               ctor.mpe_unit.clone(),
            srunit:                 ctor.srunit.clone(),
            timeunit:               ctor.timeunit.clone(),
            synth_in:               ctor.synth_in.clone(),
            switch_toggled_queued:   false,
        }
    }
}
