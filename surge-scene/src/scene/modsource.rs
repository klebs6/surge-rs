ix!();

use crate::{
    SurgeScene,
    SceneParam,
};

impl SurgeScene {

    pub fn handle_boolsolo<P: Param + ?Sized>(&mut self, param: &mut ParamRT<P>) 
    {
        if pvalb![param] {
            self.params[SceneParam::SoloO1].val     = PData::Bool(false);
            self.params[SceneParam::SoloO2].val     = PData::Bool(false);
            self.params[SceneParam::SoloO3].val     = PData::Bool(false);
            self.params[SceneParam::SoloRing12].val = PData::Bool(false);
            self.params[SceneParam::SoloRing23].val = PData::Bool(false);
            self.params[SceneParam::SoloNoise].val  = PData::Bool(false);
            param.val = PData::Bool(true);
        }
    }

    pub fn filterunit_cutoffs(&self) -> (f32,f32) {
        (self.filterunit_cutoff(0), self.filterunit_cutoff(1))
    }

    pub fn filterunit_envmodes(&self) -> (f32,f32) {
        (self.filterunit_envelopemode(0), self.filterunit_envelopemode(1))
    }

    pub fn filterunit_keytracks(&self) -> (f32,f32) {
        (self.filterunit_keytrack(0), self.filterunit_keytrack(1))
    }

    pub fn handle_boolrelative_switching<P: Param + ?Sized>(&mut self, 
        param: &mut ParamRT<P>, 
        oldvalb: bool) 
    {
        let down: bool = pvalb![param];

        let mut polarity: f32 = 
            match down { true => -1.0, false => 1.0 };

        if oldvalb == down {
            polarity = 0.0;
        }

        let (c0, c1)   = self.filterunit_cutoffs();
        let (m0, m1)   = self.filterunit_envmodes();
        let (kt0,kt1) = self.filterunit_keytracks();

        let unit = self.filterunit[1];

        unit.params[FilterParam::Cutoff].val        = PData::Float(c1 + polarity * c0);
        unit.params[FilterParam::EnvelopeMode].val  = PData::Float(m1 + polarity * m0);
        unit.params[FilterParam::KeyTrack].val      = PData::Float(kt1 + polarity * kt0);
    }

    pub fn set_channel_aftertouch_target(&mut self, fval: f32) {
        match &mut self.modsources[ModSource::ChannelAfterTouch] {
            Some(Box::new(ModulationSource::ControllerModulationSource(ref mut inner))) => inner.set_target(fval as f64),
            _                                                                           => unreachable!(),
        }
    }

    pub fn set_controler_modsource_target01(&mut self, 
        ctrl_idx: usize, 
        fval: f32) 
    {
        let ctrli = ModSource::ctrl(ctrl_idx);

        match &mut self.modsources[ctrli] 
        {
            Some(Box::new(ModulationSource::ControllerModulationSource(ref mut inner))) 
                => inner.set_target01(fval as f64, false),
            _ => unreachable!(),
        };
    }

    pub fn new_controller_modsource(srunit: SampleRateHandle) -> MaybeBoxedModulationSource {

        let cms = ControllerModulationSource::new(srunit.clone());

        let ms  = ModulationSource::ControllerModulationSource(cms);

        Some(Box::new(ms))
    }

    pub fn new_lfo_modsource(
        timeunit: TimeUnitHandle, 
        tables:   TablesHandle,
    ) -> MaybeBoxedModulationSource {

        let lfo = Lfo::new(timeunit.clone(), tables.clone());

        let ms  = ModulationSource::Lfo(lfo);

        Some(Box::new(ms))
    }

    pub fn new_modsources(
        srunit:   SampleRateHandle,
        timeunit: TimeUnitHandle,
        tables:   TablesHandle
    ) -> ModulationSourceArray {
        ModulationSourceArray::new_with( |x| match x {
            ModSource::Original             => None,
            ModSource::Velocity             => None,
            ModSource::KeyTrack             => None,
            ModSource::PolyphonicAfterTouch => None,
            ModSource::ChannelAfterTouch    => Self::new_controller_modsource(srunit.clone()),
            ModSource::PitchBend            => Self::new_controller_modsource(srunit.clone()),
            ModSource::ModWheel             => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl1                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl2                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl3                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl4                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl5                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl6                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl7                => Self::new_controller_modsource(srunit.clone()),
            ModSource::Ctrl8                => Self::new_controller_modsource(srunit.clone()),
            ModSource::AmpEg                => None,
            ModSource::FilterEg             => None,
            ModSource::VoiceLfo1            => None,
            ModSource::VoiceLfo2            => None,
            ModSource::VoiceLfo3            => None,
            ModSource::VoiceLfo4            => None,
            ModSource::VoiceLfo5            => None,
            ModSource::VoiceLfo6            => None,
            ModSource::SceneLfo1            => Self::new_lfo_modsource(timeunit.clone(),tables.clone()),
            ModSource::SceneLfo2            => Self::new_lfo_modsource(timeunit.clone(),tables.clone()),
            ModSource::SceneLfo3            => Self::new_lfo_modsource(timeunit.clone(),tables.clone()),
            ModSource::SceneLfo4            => Self::new_lfo_modsource(timeunit.clone(),tables.clone()),
            ModSource::SceneLfo5            => Self::new_lfo_modsource(timeunit.clone(),tables.clone()),
            ModSource::SceneLfo6            => Self::new_lfo_modsource(timeunit.clone(),tables.clone()),
            ModSource::Timbre               => None,
            ModSource::ReleaseVelocity      => None,
        })
    }
}
