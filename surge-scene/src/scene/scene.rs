ix!();

use crate::*;

pub const N_MODSOURCES_PER_SCENE: usize = 32;

#[derive(Debug)] 
pub struct SurgeScene {
    pub osc:                 Vec<Box<dyn Oscillator>>,
    pub params:              SceneParamArrayRT,
    pub filterunit:          Vec<FilterUnit>,
    pub waveshaper_unit:     Vec<WaveshaperUnit>,
    pub asdr:                Vec<AdsrEnvelope>,
    pub lfo:                 Vec<Lfo>,
    pub modulation_scene:    Vec<ModulationRouting>,
    pub modulation_voice:    Vec<ModulationRouting>,
    pub modsources:          ModulationSourceArray,
    pub stepsequences:       Vec<StepSequencer>,

    //TODO: rework this -- somewhat related to "fanout"
    //the issue is that we need not know where or how many 
    //sends we will use, but the toplevel is hardcoded at 2
    pub send:                 [LipolPs; 2],

    pub returnfx:             LipolPs,
    pub halfband:             HalfRateFilterSSE,
    pub highpass:             BiquadFilter,
    pub fbq:                  Box<QuadFilterChain>,

    ///voices are ephemeral, voices_array is not
    pub voices:               Vec<Rc<RefCell<SurgeVoice>>>,
    pub voices_array:         Pin<Box<[Option<SurgeVoice>; MAX_VOICES ]>>,

    pub out:                  WetBlock1Dual::<BLOCK_SIZE_OS>,

    pub tables:               TablesHandle,
    pub timeunit:             TimeUnitHandle,
    pub tuner:                TunerHandle,
    pub srunit:               SampleRateHandle,
    pub synth_in:             SynthInputHandle,
    pub midi_unit:            MIDIUnitHandle,
    pub mpe_unit:             MPEUnitHandle,
    pub hold_pedal_unit:      HoldPedalUnitHandle,

    pub release_if_latched:    bool,
    pub release_anyway:        bool,
    pub switch_toggled_queued: bool,
}
