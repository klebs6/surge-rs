ix!();

use crate::{
    SceneParam,
    SurgeScene,
};

impl SurgeScene {

    pub fn get_non_ultra_release_voices(&self) -> i32 {

        let mut count = 0;

        for item in self.voices.iter() {
            if !item.borrow().state.uberrelease {
                count += 1;
            }
        }

        count
    }

    pub fn get_non_released_voices(&self) -> i32 {

        let mut count = 0;

        for item in self.voices.iter() {
            if item.borrow().state.gate {
                count += 1;
            }
        }

        count
    }

    #[inline] pub fn get_polymode(&self) -> PolyMode {
        let idx1 = pvali![self.params[SceneParam::PolyMode]];
        PolyMode::try_from(idx1 as usize).unwrap()
    }

    #[inline] pub fn get_filtertype(&self, unit: usize) -> FilterType {
        let idx = pvali![self.filterunit[unit].params[FilterParam::Type]];
        FilterType::try_from(idx as usize).unwrap()
    }

    #[inline] pub fn get_filtersubtype(&self, unit: usize) -> FilterSubType {
        let idx = pvali![self.filterunit[unit].params[FilterParam::SubType]];
        FilterSubType::try_from(idx as usize).unwrap()
    }

    #[inline] pub fn get_waveshape_type(&self, unit: usize) -> WaveshapeType {
        let idx = pvali![self.waveshaper_unit[unit].params[WaveshaperParam::Type]];
        WaveshapeType::try_from(idx as usize).unwrap()
    }

    #[inline] pub fn get_fbconfig(&self) -> FilterBlockConfiguration {
        let idx = pvali![self.params[SceneParam::FilterBlockConfiguration]];
        FilterBlockConfiguration::try_from(idx as usize).unwrap()
    }

    #[inline] pub fn get_unused_voice(&mut self) -> Option<&mut Option<SurgeVoice>> 
    {
        for i in 0..MAX_VOICES 
        {
            if self.voices_array[i].is_none() {
                //"this signifies: here is an unused voice" (unused because it is represented by
                //None in our pinned array
                return Some(&mut self.voices_array[i]);

            }
        }

        //this signifies "there is *no*
        //unused_voice"
        None
    }

    #[inline] pub fn keytrack_root(&self) -> f64 {
        pvali![ self.params[SceneParam::KeytrackRoot] ] as f64
    }

    #[inline] pub fn oscillator_level(&self, idx: usize) -> f32 {
        match idx {
            0 => pvalf![self.params[SceneParam::LevelO1]],
            1 => pvalf![self.params[SceneParam::LevelO2]],
            2 => pvalf![self.params[SceneParam::LevelO3]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn noise_level(&self, idx: usize) -> f32 {
        match idx {
            0 => pvalf![self.params[SceneParam::LevelNoise]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn ring_level(&self, idx: usize) -> f32 {
        match idx {
            0 => pvalf![self.params[SceneParam::LevelRing12]],
            1 => pvalf![self.params[SceneParam::LevelRing23]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn enabled(&self, ms: ModSource) -> bool {
        self.modsources[ms].as_ref().unwrap().enabled()
    }

    #[inline] pub fn modulation_voice(&self) -> Rc<Vec<ModulationRouting>> {
        todo!();
        //&self.modulation_voice
    }

    #[inline] pub fn modulation_scene(&self) -> Rc<Vec<ModulationRouting>> {
        todo!();
        //&self.modulation_voice
    }

    #[inline] pub fn pan(&self) -> f32 {
        pvalf![self.params[SceneParam::Pan]]
    }

    #[inline] pub fn width(&self) -> f32 {
        pvalf![self.params[SceneParam::Width]]
    }

    #[inline] pub fn volume(&self) -> f32 {
        pvalf![self.params[SceneParam::Volume]]
    }

    #[inline] pub fn pitch(&self) -> f64 {
        pvalf![self.params[SceneParam::Pitch]] as f64
    }

    #[inline] pub fn octave(&self) -> f64 {
        pvali![self.params[SceneParam::Octave]] as f64
    }

    #[inline] pub fn pitchbend_range_up(&self) -> f64 {
        pvali![self.params[SceneParam::PitchBendRangeUp]]   as f64
    }

    #[inline] pub fn pitchbend_range_down(&self) -> f64 {
        pvali![self.params[SceneParam::PitchBendRangeDown]] as f64
    }

    #[inline] pub fn pitch_extend_range(&self) -> bool {
        self.params[SceneParam::Pitch].extend_range
    }

    #[inline] pub fn do_poly_aftertouch(&self) -> bool {
        self.modsources[ModSource::PolyphonicAfterTouch].as_ref().unwrap().enabled()
    }

    #[inline] pub fn oscillator_route(&self, idx: usize) -> i32 {
        match idx {
            0 => pvali![self.params[SceneParam::RouteO1]],
            1 => pvali![self.params[SceneParam::RouteO2]],
            2 => pvali![self.params[SceneParam::RouteO3]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn ring_route(&self, idx: usize) -> i32 {
        match idx {
            0 => pvali![self.params[SceneParam::RouteRing12]],
            1 => pvali![self.params[SceneParam::RouteRing23]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn noise_route(&self, _idx: usize) -> i32 {
        pvali![self.params[SceneParam::RouteNoise]]
    }

    #[inline] pub fn filterblock_cfg(&self) -> FilterBlockConfiguration {
        FilterBlockConfiguration::try_from(
            pvali![self.params[SceneParam::FilterBlockConfiguration]] as usize
        ).unwrap()
    }

    #[inline] pub fn drift(&self) -> f32 {
        pvalf![self.params[SceneParam::Drift]]
    }

    #[inline] pub fn fm_depth(&self) -> f32 {
        pvalf![self.params[SceneParam::FmDepth]]
    }

    #[inline] pub fn noise_colour(&self) -> f32 {
        pvalf![self.params[SceneParam::NoiseColour]]
    }

    #[inline] pub fn oscillator_octave(&self, idx: usize) -> f32 {
        match idx {
            0 => pvali![self.osc[0].osc_params_const(OscillatorParam::Octave)] as f32,
            1 => pvali![self.osc[1].osc_params_const(OscillatorParam::Octave)] as f32,
            2 => pvali![self.osc[2].osc_params_const(OscillatorParam::Octave)] as f32,
            _ => unreachable!(),
        }
    }

    #[inline] pub fn oscillator_keytrack(&self, idx: usize) -> bool {
        pvalb![self.osc[idx].osc_params_const(OscillatorParam::KeyTrack)]
    }

    #[inline] pub fn portamento(&self) -> f32 {
        pvalf![self.params[SceneParam::Portamento]]
    }

    #[inline] pub fn portamento_min(&self) -> f32 {
        pvalminf![self.params[SceneParam::Portamento]]
    }

    #[inline] pub fn portamento_temposync(&self) -> bool {
        self.params[SceneParam::Portamento].temposync
    }

    #[inline] pub fn polymode(&self) -> PolyMode {
        let idx = pvali![self.params[SceneParam::PolyMode]];
        PolyMode::try_from(idx as usize).unwrap()
    }

    #[inline] pub fn filterunit_cutoff(&self, idx: usize) -> f32 {
        pvalf![self.filterunit[idx].params[FilterParam::Cutoff]]
    }

    #[inline] pub fn filterunit_keytrack(&self, idx: usize) -> f32 {
        pvalf![self.filterunit[idx].params[FilterParam::KeyTrack]]
    }

    #[inline] pub fn filterunit_envelopemode(&self, idx: usize) -> f32 {
        pvalf![self.filterunit[idx].params[FilterParam::EnvelopeMode]]
    }

    #[inline] pub fn filterunit_resonance(&self, idx: usize) -> f32 {
        pvalf![self.filterunit[idx].params[FilterParam::Resonance]]
    }

    #[inline] pub fn filterunit_filtertype(&self, idx: usize) -> FilterType {
        FilterType::try_from(pvali![
            self.filterunit[idx].params[
            FilterParam::Type
            ]
        ] as usize).unwrap()
    }

    #[inline] pub fn filterunit_filtersubtype(&self, idx: usize) -> FilterSubType {
        FilterSubType::try_from(pvali![
            self.filterunit[idx].params[
            FilterParam::SubType
            ]
        ] as usize).unwrap()
    }

    #[inline] pub fn f2_cutoff_is_offset(&self) -> bool {
        pvalb![self.params[SceneParam::F2CutoffIsOffset]]
    }

    #[inline] pub fn oscillator_solo(&self, idx: usize) -> bool {
        match idx {
            0 => pvalb![self.params[SceneParam::SoloO1]],
            1 => pvalb![self.params[SceneParam::SoloO2]],
            2 => pvalb![self.params[SceneParam::SoloO3]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn noise_solo(&self, _idx: usize) -> bool {
        pvalb![self.params[SceneParam::SoloNoise]]
    }

    #[inline] pub fn ring_solo(&self, idx: usize) -> bool {
        match idx {
            0 => pvalb![self.params[SceneParam::SoloRing12]],
            1 => pvalb![self.params[SceneParam::SoloRing23]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn oscillator_mute(&self, idx: usize) -> bool {
        match idx {
            0 => pvalb![self.params[SceneParam::MuteO1]],
            1 => pvalb![self.params[SceneParam::MuteO2]],
            2 => pvalb![self.params[SceneParam::MuteO3]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn noise_mute(&self, _idx: usize) -> bool {
        pvalb![self.params[SceneParam::MuteNoise]]
    }

    #[inline] pub fn ring_mute(&self, idx: usize) -> bool {
        match idx {
            0 => pvalb![self.params[SceneParam::MuteRing12]],
            1 => pvalb![self.params[SceneParam::MuteRing23]],
            _ => unreachable!(),
        }
    }

    #[inline] pub fn fm_cfg(&self) -> FmConfiguration {
        match self.oscillator_solo(0) {
            true => {
                FmConfiguration::try_from(
                    pvali![self.params[SceneParam::FmSwitch]] 
                    as usize
                ).unwrap()
            },
            false => FmConfiguration::Off,
        }
    }

    #[inline] pub fn oscillator_pitch(&self, idx: usize) -> f64 {
        pvalf![self.osc[idx].osc_params_const(OscillatorParam::Pitch)] as f64
    }

    #[inline] pub fn oscillator_pitch_extend_range(&self, idx: usize) -> bool {

        self.osc[idx].osc_params_const( 
            OscillatorParam::Pitch
        ).extend_range()
    }

    #[inline] pub fn oscillator_pitch_absolute(&self, idx: usize) -> bool {

        self.osc[idx].osc_params_const( 
            OscillatorParam::Pitch
        ).absolute
    }

    #[inline] pub fn filter_balance(&self) -> f32 {
        pvalf![self.params[SceneParam::FilterBalance]]
    }

    #[inline] pub fn waveshaper_drive(&self, idx: usize) -> f32 {
        pvalf![self.waveshaper_unit[idx].params[WaveshaperParam::Drive]]
    }

    #[inline] pub fn vca_level(&self) -> f32 {
        pvalf![self.params[SceneParam::VcaLevel]]
    }

    #[inline] pub fn vca_velsense(&self) -> f32 {
        pvalf![self.params[SceneParam::VcaVelsense]]
    }

    #[inline] pub fn feedback(&self) -> f32 {
        pvalf![self.params[SceneParam::Feedback]]
    }

    #[inline] pub fn oscillator_type(&self, idx: usize) -> OscillatorType {

        let osc = &self.osc[idx];

        let ty = pvali![osc.osc_params_const(OscillatorParam::Type)];

        OscillatorType::try_from(
            ty as usize
        ).unwrap()
    }
}
