ix!();

use crate::{
    VocoderParam,
    VocoderParamArrayRT,
};

#[derive(Debug,Clone)]
pub struct Vocoder<'sr> {

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
    pub synth_in:     SynthInputHandle<'sr>,
    pub tables:       TablesHandle<'sr>,
    pub srunit:       SampleRateHandle<'sr>,
}

effect!          [Vocoder<'sr>,       VocoderParam];
no_op!           [Vocoder<'sr>, ProcessOnlyControl];
name!            [Vocoder<'sr>,          "vocoder"];
update_on_init!  [Vocoder<'sr>                    ];
init_on_suspend! [Vocoder<'sr>                    ];

