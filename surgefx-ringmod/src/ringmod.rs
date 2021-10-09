ix!();

use crate::{
    RingModulatorParamArrayRT,
    RingModulatorParam
};

pub const RINGMOD_MAX_UNISON: i32 = 16;
pub const RINGMOD_OVERSAMPLE: bool = true;

/// http://recherche.ircam.fr/pub/dafx11/Papers/66_e.pdf
#[derive(Debug,Clone)]
pub struct RingModulator<'sr> {
    pub ringout:       Ringout,
    pub params:        RingModulatorParamArrayRT,
    pub lp:            BiquadFilter<'sr>,
    pub hp:            BiquadFilter<'sr>,
    pub halfband_out:  HalfRateFilterSSE,
    pub halfband_in:   HalfRateFilterSSE,
    pub phase:         A1d::<f32>,
    pub detune_offset: A1d::<f32>,
    pub pan_l:         A1d::<f32>,
    pub pan_r:         A1d::<f32>,
    pub last_unison:   i32,
    pub tuner:         TunerHandle<'sr>,
    pub srunit:        SampleRateHandle<'sr>,
}

effect!         [RingModulator<'sr>, RingModulatorParam];
no_op!          [RingModulator<'sr>, ProcessOnlyControl];
name!           [RingModulator<'sr>,    "RingModulator"];
no_op!          [RingModulator<'sr>,             Update];
init_on_suspend![RingModulator<'sr>                    ];
