crate::ix!();

#[derive(Debug)]
pub struct FXUnit {

    pub load_fx_needed:     bool,
    pub fx:                 Vec<SurgeEffect>,
    pub fx_suspend_bitmask: i32,
    pub fx_enable:          [bool; 8],

    /*
       if true, reload new effect parameters from
       fxsync
      */
    pub fx_reload:          [bool; 8],
}

impl FXUnit {

    pub fn new_fx(
        tuner:    & TunerHandle,
        tables:   & TablesHandle,
        timeunit: & TimeUnitHandle,
        srunit:   & SampleRateHandle)  -> Vec<SurgeEffect> {
        vec![
            SurgeEffect::Conditioner(Box::new(Conditioner::new(tuner,tables,srunit))),
            SurgeEffect::AllpassVerb(Box::new(AllpassVerb::new(srunit))),
            SurgeEffect::DualDelay(Box::new(DualDelay::new(tuner,tables,srunit,timeunit))),
            SurgeEffect::Flanger(Box::new(Flanger::new(tuner,tables,srunit,timeunit))),
            SurgeEffect::Phaser(Box::new(Phaser::new(tuner,tables,srunit,timeunit))),
            SurgeEffect::Reverb(Box::new(Reverb::new(tuner,tables,srunit,timeunit))),
            SurgeEffect::Distortion(Box::new(Distortion::new(tuner,tables,srunit))),
            SurgeEffect::Eq3Band(Box::new(Eq3Band::new(tuner,tables,srunit))),
        ]
    }

    pub fn new(
        tuner:    & TunerHandle,
        tables:   & TablesHandle,
        timeunit: & TimeUnitHandle,
        srunit:   & SampleRateHandle)  -> Self {

        Self {
            load_fx_needed:     true,
            fx:                 Self::new_fx(tuner,tables,timeunit,srunit),
            fx_reload:          [false; 8],
            fx_enable:          [true; 8],
            fx_suspend_bitmask: 0,
        }
    }

    /*
      |this function was different when we had
      |fx_sync
      |
      |todo: do we still need fx_sync?
      */
    pub fn load_fx(&mut self, initp: bool, force_reload_all: bool) -> bool 
    {
        self.load_fx_needed = false;

        let mut something_changed: bool = false;

        let iter = self.fx.iter_mut();

        for (idx, effect) in iter.enumerate() {

            if self.fx_reload[idx] || force_reload_all 
            {
                effect.init();
                if initp {
                    //effect.set_default_params();
                }
                self.fx_reload[idx] = false;
                something_changed = true;
            }
        }

        something_changed
    }
}
