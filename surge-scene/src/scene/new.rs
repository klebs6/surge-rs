ix!();

use crate::{
    SurgeScene,
    SceneParam,
};

pub struct SceneConstructorHandles<'a> {
    pub timeunit:        &'a  TimeUnitHandle,
    pub tables:          &'a  TablesHandle,
    pub tuner:           &'a  TunerHandle,
    pub srunit:          &'a  SampleRateHandle,
    pub hold_pedal_unit: &'a  HoldPedalUnitHandle,
    pub midi_unit:       &'a  MIDIUnitHandle,
    pub mpe_unit:        &'a  MPEUnitHandle,
    pub synth_in:        &'a  SynthInputHandle,
}

impl SurgeScene {
    pub fn new(ctor: SceneConstructorHandles<'a>) -> Self {
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

            //MAX_VOICES = 64
            voices_array:  std::pin::Pin::new(box [ 
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
