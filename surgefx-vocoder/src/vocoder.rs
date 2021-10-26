ix!();

use crate::{
    VocoderParam,
    VocoderParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Vocoder {

    pub carrier_l:    Align16<A1d::<VectorizedSvfFilter>>,
    pub carrier_r:    Align16<A1d::<VectorizedSvfFilter>>,
    pub modulator:    Align16<A1d::<VectorizedSvfFilter>>,
    pub env_f:        Align16<A1d::<__m128>>,
    pub gain:         Align16<LipolPs>,

    pub ringout:      Ringout,
    pub params:       VocoderParamArrayRT,

    /// block increment (to keep track of events not occurring every n blocks)
    pub bi:           i32, 
    pub active_bands: i32,
    pub synth_in:     SynthInputHandle,
    pub tables:       TablesHandle,
    pub srunit:       SampleRateHandle,
}

effect!          [Vocoder,       VocoderParam];
no_op!           [Vocoder, ProcessOnlyControl];
name!            [Vocoder,          "vocoder"];
update_on_init!  [Vocoder                    ];
init_on_suspend! [Vocoder                    ];

