crate::ix!();

impl SurgeSynthesizer<'plugin_layer> {

    /**
      | If I am changing my sample rate I will
      | change my internal tables, so this needs
      | to be tuning aware and reapply tuning if
      | needed
      */
    pub fn set_samplerate(&mut self, sr: f64) {

        let scale = self.tuner.current_scale();
        let was_st = self.tuner.current_tuning_is_standard();

        self.srunit.set_samplerate(sr);

        self.tables.init();

        self.hold_pedal_unit.set_quadrosc_rate(
            1000.0 * 
            self.srunit.dsamplerate_inv()
        );

        if ! was_st {

            self.tuner.retune_to_scale(&scale);
        }
    }
}

