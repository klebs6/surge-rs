crate::ix!();

impl WindowOscillator {

    pub fn process_sub_oscs(&mut self, stereo: bool, fm: bool) {

        // SSE2 path
        let morph = self.pvalf(WindowOscillatorParam::Morph);

        let table: i32 = self.get_table(morph);

        let formant     = self.pvalf(WindowOscillatorParam::Formant);
        let formant_mul = self.get_formant_mul(formant as f64);

        let size_mask:     u32 = self.get_size_mask();
        let size_mask_win: u32 = self.get_size_mask_win();

        let window: i8 = self.get_window();

        let num_active_sub_oscs = self.active_sub_oscs as usize;

        let cfg = SubOscProcessCfg{
            table,
            formant_mul,
            size_mask,
            size_mask_win,
            window,
            stereo,
            fm
        };

        for so in (0_usize..num_active_sub_oscs).step_by(1) {
            self.process_sub_osc(so, &cfg);
        }
    }
}
