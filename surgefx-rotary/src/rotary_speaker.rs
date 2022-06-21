crate::ix!();

 //note, is this the right value?
pub const ROTARY_SPEAKER_MAX_DELAY_LENGTH: usize = 1 << 18;
pub const ROTARY_SPEAKER_RINGOUT:          usize = ROTARY_SPEAKER_MAX_DELAY_LENGTH >> 5;

#[derive(Debug,Clone)]
pub struct RotarySpeaker {
    pub ringout:     Ringout,
    pub params:      RotarySpeakerParamArrayRT,
    pub buffer:      A1d::<f32>,
    pub wpos:        i32,
    pub xover:       BiquadFilter,
    pub lowbass:     BiquadFilter,
    pub lfo:         QuadrOsc,
    pub lf_lfo:      QuadrOsc,
    pub d_l:         LiPol<f32>,
    pub d_r:         LiPol<f32>,
    pub drive:       LiPol<f32>,
    pub hornamp:     [LiPol<f32>; 2],
    pub first_run:   bool,
    pub time_unit:   TimeUnitHandle,
    pub tables:      TablesHandle,
    pub srunit:      SampleRateHandle,
}

effect!        [RotarySpeaker, RotarySpeakerParam ];
has_timeunit!  [RotarySpeaker, RotarySpeakerParam ];
name!          [RotarySpeaker, "rotary"           ];
no_update!     [RotarySpeaker                     ];
