crate::ix!();

impl WTOscillator {

    #[inline] pub fn distort_level(&mut self, mut x: f32) -> f32 
    {
        let a:    f32 = self.l_vskew.v * 0.5;
        let clip: f32 = self.l_clip.v;
        x = x - a * x * x + a;
        x = limit_range(x * (1.0 - clip) + clip * x * x * x, -1.0, 1.0);
        x
    }

    pub fn update_lagvals<const IS_INIT: bool>(&mut self)
    {
        let hskew = self.pvalf(WTOscillatorParam::SkewH);
        let vskew = self.pvalf(WTOscillatorParam::SkewV);

        let clip = {
            let mut clip = self.pvalf(WTOscillatorParam::Saturate);
            clip = limit_range(clip, 0.0, 1.0);
            clip
        };

        let formant = self.pvalf(WTOscillatorParam::Formant);
        let shape   = self.pvalf(WTOscillatorParam::Morph);

        self.l_vskew.new_value(limit_range(vskew, -1.0, 1.0));
        self.l_hskew.new_value(limit_range(hskew, -1.0, 1.0));
        self.l_clip.new_value(-8.0 * clip * clip * clip);
        self.l_shape.new_value(limit_range(shape, 0.0, 1.0));
        self.formant_t = maxf(0.0, formant);

        let invt: f32 = mind(
            1.0, 
            (8.175798915 * self.tuner.n2p_tuningctr(self.pitch_t as f64)) * 
            self.srunit.dsamplerate_os_inv()
        ) as f32;

        let hpf2: f32 = mind(
            self.blitter.integrator_hpf as f64, 
            HPF_CYCLE_LOSS.powf(4.0 * invt) as f64
        ) as f32; // TODO Make a lookup table

        self.hpf_coeff.new_value(hpf2);
        self.integrator_mult.new_value(invt);

        self.li_hpf.set_target(hpf2);

        if IS_INIT {

            self.hpf_coeff.instantize();
            self.integrator_mult.instantize();
            self.l_shape.instantize();
            self.l_vskew.instantize();
            self.l_hskew.instantize();
            self.l_clip.instantize();

            self.formant_last = self.formant_t;
        }
    }

    pub fn clear_last_level(&mut self) {

        for x in self.last_level.iter_mut() {
            *x = 0.0;
        }
    }
}
