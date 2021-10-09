ix!();

use crate::{
    SceneParamArrayRT,
};

pub const N_MODSOURCES_PER_SCENE: usize = 32;

#[derive(Debug)] 
pub struct SurgeScene<'sr> {
    pub osc:                 Vec<Box<dyn Oscillator + 'sr>>,
    pub params:              SceneParamArrayRT,
    pub filterunit:          Vec<FilterUnit>,
    pub waveshaper_unit:     Vec<WaveshaperUnit>,
    pub asdr:                Vec<AdsrEnvelope<'sr>>,
    pub lfo:                 Vec<Lfo<'sr>>,
    pub modulation_scene:    Vec<ModulationRouting<'sr>>,
    pub modulation_voice:    Vec<ModulationRouting<'sr>>,
    pub modsources:          ModulationSourceArray<'sr>,
    pub stepsequences:       Vec<StepSequencer>,

    //TODO: rework this -- somewhat related to "fanout"
    //the issue is that we need not know where or how many 
    //sends we will use, but the toplevel is hardcoded at 2
    pub send:                 [LipolPs; 2],

    pub returnfx:             LipolPs,
    pub halfband:             HalfRateFilterSSE,
    pub highpass:             BiquadFilter<'sr>,
    pub fbq:                  Box<QuadFilterChain<'sr>>,

    ///voices are ephemeral, voices_array is not
    pub voices:               Vec<&'sr mut SurgeVoice<'sr>>,
    pub voices_array:         Pin<Box<[Option<SurgeVoice<'sr>>; MAX_VOICES ]>>,

    pub out:                  WetBlock1Dual::<BLOCK_SIZE_OS>,

    pub tables:               TablesHandle<'sr>,
    pub timeunit:             TimeUnitHandle<'sr>,
    pub tuner:                TunerHandle<'sr>,
    pub srunit:               SampleRateHandle<'sr>,
    pub synth_in:             SynthInputHandle<'sr>,
    pub midi_unit:            MIDIUnitHandle<'sr>,
    pub mpe_unit:             MPEUnitHandle<'sr>,
    pub hold_pedal_unit:      HoldPedalUnitHandle<'sr>,

    pub release_if_latched:    bool,
    pub release_anyway:        bool,
    pub switch_toggled_queued: bool,

}

