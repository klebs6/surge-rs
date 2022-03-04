ix!();

use crate::{
    SurgeVoiceState,
    FilterBlockState,
    FilterBlockData,
    FBP,
};

enhanced_enum![
    LagEntry {
        Osc1,
        Osc2,
        Osc3,
        Noise,
        Ring12,
        Ring23,
        Pfg,
    }
];

pub type BoxedModRoutingIter 
= Box<dyn Iterator<Item=Rc<ModulationRouting>>>;

pub type VoiceOscLevels 
= LagEntryArray::<LipolPs>;

pub type VoiceModSourceArray 
= ModSourceArray::<MaybeBoxedModulationSource>;

pub type MaybeVoiceOscillator
= Option<Box<dyn Oscillator + >>;

pub type MaybeVoiceOscillators
= [MaybeVoiceOscillator; 3];

#[derive(Debug)]
pub struct SurgeVoice {

    pub output:             Align16<[[f32; BLOCK_SIZE_OS]; 2]>,
    pub osclevels:          Align16<VoiceOscLevels>,
    pub fmbuffer:           Align16<[f32; BLOCK_SIZE_OS]>,

    // used for the 2>1<3 fm-mode (Needs the pointer earlier)
    pub osctype:            [OscillatorType; N_OSCS],
    pub state:              SurgeVoiceState,
    pub age:                i32,
    pub age_release:        i32,
    pub filterblock_state:  FilterBlockState,
    pub fbp:                FBP,
    pub coeffmaker:         [FilterCoefficientMaker; 2],
    pub route:              [i32; 6],
    pub octave_size:        f32, // 12.0
    pub osc_enable:         [bool; 3],
    pub ring_enable:        [bool; 2],
    pub noise_enable:       bool,
    pub fm_mode:            FmConfiguration,
    pub noisegen_l:         [f32; 2],
    pub noisegen_r:         [f32; 2],
    pub osc:                MaybeVoiceOscillators,
    pub modsources:         VoiceModSourceArray,
    pub filterblock_data:   FilterBlockData,
    pub mpe_enabled:        MpeEnableSwitch,
    pub uuid:               Uuid,
    pub mpe_unit:           MPEUnitHandle,
    pub tables:             TablesHandle,
    pub time_unit:          TimeUnitHandle,
    pub tuner:              TunerHandle,
    pub synth_in:           SynthInputHandle,
    pub srunit:             SampleRateHandle,
}

compare_by![SurgeVoice, uuid];

