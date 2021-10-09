ix!();

use crate::{
    FlangerParam,
    FlangerParamArrayRT,
    InterpDelay,
    OnePoleLPFilterState,
};

#[derive(Debug,Clone)]
pub struct Flanger<'sr> {

    pub ringout:        Ringout,
    pub params:         FlangerParamArrayRT,
    pub idels:          [InterpDelay; 2],
    pub lfophase:       A2d::<f32>,
    pub longphase:      f32,

    ///state for the onepole LP filter
    pub onepole_state:  OnePoleLPFilterState,

    pub lfoval:         A2d::<LiPol::<f32>>,//[[LiPol<f32>; FLANGER_COMBS_PER_CHANNEL]; 2],
    pub delaybase:      A2d::<LiPol::<f32>>,//[[LiPol<f32>; FLANGER_COMBS_PER_CHANNEL]; 2],
    pub depth:          LiPol<f32>,
    pub mix:            LiPol<f32>,
    pub voices:         LiPol<f32>,
    pub voice_detune:   LiPol<f32>,
    pub voice_chord:    LiPol<f32>,
    pub feedback:       LiPol<f32>,
    pub fb_lf_damping:  LiPol<f32>,
    pub stereo_width:   LiPol<f32>,
    pub gain:           LiPol<f32>,
    pub sin_lfo_table:  A1d::<f32>,

    /// don't make it analytic since 
    /// I want to smooth the edges
    pub saw_lfo_table:  A1d::<f32>, 

    pub time_unit:      TimeUnitHandle<'sr>,
    pub tables:         TablesHandle<'sr>,
    pub tuner:          TunerHandle<'sr>,
    pub srunit:         SampleRateHandle<'sr>,
}

no_op!         [Flanger<'sr>, ProcessOnlyControl];
effect!        [Flanger<'sr>,       FlangerParam];
has_timeunit!  [Flanger<'sr>,       FlangerParam];
name!          [Flanger<'sr>,          "flanger"];
no_op!         [Flanger<'sr>,            Suspend];
no_update!     [Flanger<'sr>                    ];
