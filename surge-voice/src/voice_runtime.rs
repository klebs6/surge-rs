crate::ix!();

/**
  |this structure decouples SurgeVoice from
  |SurgeScene.
  |
  |Aside from breaking the cyclic dependency, we
  |are essentially trading space for time.
  |
  |this struct takes up a modest amount of memory,
  |though access should be fast from each voice.
  |
  |Additionally, we don't need a separate
  |calculation for these parameters for each
  |voice.
  |
  |So far, I have not measured any of these
  |performance tradeoffs.
  |
  |I decided to use a Runtime struct instead of
  |embedding a backreference to the scene (or
  |trait implemented by Scene) in each voice.
  |
  |If we need to use an indirection to access
  |these fields, better to leverage a struct which
  |does not require circular dependency with
  |SurgeScene.
  |
  */
pub struct VoiceRuntime {
    pub filterblock_cfg:          FilterBlockConfiguration,
    pub drift:                    f32,
    pub fm_depth:                 f32,
    pub noise_colour:             f32,
    pub oscillator_octave:       [f32; 3],
    pub oscillator_pitch:        [f32; 3],
    pub oscillator_absolute:     [bool; 3],
    pub oscillator_extend_range: [bool; 3],
    pub oscillator_keytrack2:     bool,
    pub voice_update_qfcs_cfg:    VoiceUpdateQFCSCfg,

    pub update_lfo:               [bool; 6],
    pub modulation_voice:         Rc<Vec<ModulationRouting>>,
    pub modulation_scene:         Rc<Vec<ModulationRouting>>,
    pub portamento:               f32,
    pub portamento_min:           f32,
    pub portamento_temposync:     bool,

    pub pitch:                    f64,
    pub octave:                   f64,
    pub pitch_extend_range:       bool,
    pub pitchbend_cfg:            PitchBendCfg,

    pub oscillator_level0:        f32,
    pub oscillator_level1:        f32,
    pub oscillator_level2:        f32,
    pub noise_level0:             f32,
    pub ring_level0:              f32,
    pub ring_level1:              f32,

    pub oscillator_route0:        i32,
    pub oscillator_route1:        i32,
    pub oscillator_route2:        i32,
    pub ring_route0:              i32,
    pub ring_route1:              i32,
    pub noise_route0:             i32,

    pub keytrack_root:            f64,
    pub do_poly_aftertouch:       bool,

    pub pan:                      f32,
    pub width:                    f32,
    pub volume:                   f32,

    pub filterunit_filtertype:    Vec<FilterType>,
    pub filterunit_filtersubtype: Vec<FilterSubType>,

    pub voice_toggle_solo_cfg:    VoiceToggleSoloCfg,
    pub oscillator_type:          [OscillatorType; N_OSCS],
    pub fm_cfg:                   FmConfiguration,
}

pub type VoiceRuntimeHandle = Rc<RefCell<VoiceRuntime>>;
