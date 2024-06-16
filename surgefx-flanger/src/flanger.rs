crate::ix!();

#[derive(Debug,Clone)]
#[name("flanger")]
pub struct Flanger {

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

    pub time_unit:      TimeUnitHandle,
    pub tables:         TablesHandle,
    pub tuner:          TunerHandle,
    pub srunit:         SampleRateHandle,
}

no_op!         [Flanger, ProcessOnlyControl];
effect!        [Flanger,       FlangerParam];
has_timeunit!  [Flanger,       FlangerParam];
no_op!         [Flanger,            Suspend];
no_update!     [Flanger                    ];
