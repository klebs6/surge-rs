ix!();

use crate::{
    Reverb,
    ReverbParam,
    REVERB_TAPS,
};

impl Reverb<'sr> {

    pub fn need_recalc_coefficients(&mut self) -> bool {
        // periodically true
        let x = matches![self.b, 0];
        self.b = ( self.b + 1) & 31;
        x
    }

    pub fn maybe_recalc_coefficients(&mut self) {

        if self.need_recalc_coefficients() {

            let f_freq1: f64 = self.pvalf(ReverbParam::Band1Freq).into();
            let f_locut: f64 = self.pvalf(ReverbParam::LowCut).into();
            let f_hicut      = self.pvalf(ReverbParam::HighCut);
            let f_gain1: f64 = self.pvalf(ReverbParam::Band1Gain).into();

            self.band1.coeff_peak_eq(
                self.band1.calc_omega(f_freq1 * (1.0 / 12.0)), 2.0, f_gain1);

            self.locut.coeff_hp(
                self.locut.calc_omega(f_locut * (1.0 / 12.0)), 0.5);

            self.hicut.coeff_lp2b(self.hicut.calc_omega(f_hicut as f64 * (1.0 / 12.0)), 0.5);
        }
    }

    pub fn update_rtime(&mut self) {

        let mut max_dt: i32 = 0;
        let decaytime:  f32 = self.pvalf(ReverbParam::DecayTime);
        let sr_f32:     f32 = self.srunit.samplerate() as f32;

        for t in 0..REVERB_TAPS {

            self.delay_fb[t] = (db60![] as f32).powf(
                (self.delay_time[t] as f32) / (256.0 * sr_f32 * 2.0_f32.powf(decaytime))
            );

            max_dt = maxf(max_dt as f32, self.delay_time[t] as f32) as i32;
        }

        self.lastf[ReverbParam::DecayTime as usize] = self.pvalf(ReverbParam::DecayTime);

        let t: f32 = BLOCK_SIZE_INV * (
            ((max_dt >> 8) as f32) + 
            sr_f32 * 2.0_f32.powf(decaytime) * 2.0  // *2 is to get the db120 time
        );

        self.ringout = Ringout::blocks(t as NumberOfBlocks);
    }

    pub fn update_rsize(&mut self) {
        self.load_preset(self.preset);
    }
}

