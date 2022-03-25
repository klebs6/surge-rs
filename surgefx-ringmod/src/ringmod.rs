ix!();

use crate::*;

pub const RINGMOD_MAX_UNISON: i32 = 16;
pub const RINGMOD_OVERSAMPLE: bool = true;

/// http://recherche.ircam.fr/pub/dafx11/Papers/66_e.pdf
#[derive(Debug,Clone)]
pub struct RingModulator {
    pub ringout:       Ringout,
    pub params:        RingModulatorParamArrayRT,
    pub lp:            BiquadFilter,
    pub hp:            BiquadFilter,
    pub halfband_out:  HalfRateFilterSSE,
    pub halfband_in:   HalfRateFilterSSE,
    pub phase:         A1d::<f32>,
    pub detune_offset: A1d::<f32>,
    pub pan_l:         A1d::<f32>,
    pub pan_r:         A1d::<f32>,
    pub last_unison:   i32,
    pub tuner:         TunerHandle,
    pub srunit:        SampleRateHandle,
}

effect!         [RingModulator, RingModulatorParam];
no_op!          [RingModulator, ProcessOnlyControl];
name!           [RingModulator,    "RingModulator"];
no_op!          [RingModulator,             Update];
init_on_suspend![RingModulator                    ];
