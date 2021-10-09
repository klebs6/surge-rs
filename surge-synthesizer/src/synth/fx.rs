ix!();

#[derive(Debug)]
pub struct FXUnit<'sr> {

    pub load_fx_needed:     bool,
    pub fx:                 Vec<SurgeEffect<'sr>>,
    pub fx_suspend_bitmask: i32,
    pub fx_enable:          [bool; 8],

    /// if true, reload new effect parameters from fxsync
    pub fx_reload:          [bool; 8],
}

impl FXUnit<'sr> {

    pub fn new(
        tuner:    &'sr TunerHandle<'sr>,
        tables:   &'sr TablesHandle<'sr>,
        timeunit: &'sr TimeUnitHandle<'sr>,
        srunit:   &'sr SampleRateHandle<'sr>)  -> Self {

        Self {
            load_fx_needed: true,
            fx: vec![
                SurgeEffect::Conditioner(box Conditioner::new(tuner,tables,srunit)),
                SurgeEffect::AllpassVerb(box AllpassVerb::new(srunit)),
                SurgeEffect::DualDelay(box DualDelay::new(tuner,tables,srunit,timeunit)),
                SurgeEffect::Flanger(box Flanger::new(tuner,tables,srunit,timeunit)),
                SurgeEffect::Phaser(box Phaser::new(tuner,tables,srunit,timeunit)),
                SurgeEffect::Reverb(box Reverb::new(tuner,tables,srunit,timeunit)),
                SurgeEffect::Distortion(box Distortion::new(tuner,tables,srunit)),
                SurgeEffect::Eq3Band(box Eq3Band::new(tuner,tables,srunit)),
            ],
            fx_reload:          [false; 8],
            fx_enable:          [true; 8],
            fx_suspend_bitmask: 0,
        }
    }

    //this function was different when we had fx_sync
    //todo: do we still need fx_sync?
    pub fn load_fx(&mut self, initp: bool, force_reload_all: bool) -> bool 
    {
        self.load_fx_needed = false;

        let mut something_changed: bool = false;

        for (idx, effect) in self.fx.iter_mut().enumerate() {
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
