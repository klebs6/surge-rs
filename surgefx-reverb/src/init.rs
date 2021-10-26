ix!();

use crate::{
    Reverb,
    ReverbParam,
    ReverbPreset,
    REVERB_TAPS,
};

impl Reverb {

    fn init_taps(&mut self) {

        let taps     = REVERB_TAPS as usize;
        let taps_f32 = REVERB_TAPS as f32;

        for t in 0..taps {

            let x: f32 = (t as f32) / (taps_f32 - 1.0);
            let xbp: f32 = -1.0 + 2.0 * x;

            self.out_tap[t] = 0.0;
            self.delay_pan_l[t] = (0.5 - 0.495 * xbp).sqrt();
            self.delay_pan_r[t] = (0.5 + 0.495 * xbp).sqrt();
        }
    }
}

impl Init for Reverb {

    fn init(&mut self) {

        let f1: f64  = self.pvalf(ReverbParam::Band1Freq).into();
        let g1: f64  = self.pvalf(ReverbParam::Band1Gain).into();
        let lc: f64  = self.pvalf(ReverbParam::LowCut).into();
        let hc: f64  = self.pvalf(ReverbParam::HighCut).into();

        macro_rules! omega {
            ($band:ident, $freq:ident) => {
                self.$band.calc_omega($freq / 12.0)
            }
        }

        self.band1.coeff_peak_eq(
            omega![band1, f1], 
            2.0, 
            g1
        );

        self.locut.coeff_hp(
            omega![locut, lc], 
            0.5
        );

        self.hicut.coeff_lp2b(
            omega![hicut, hc], 
            0.5
        );

        self.band1.coeff_instantize();
        self.locut.coeff_instantize();
        self.hicut.coeff_instantize();

        self.band1.suspend();
        self.locut.suspend();
        self.hicut.suspend();

        self.ringout = Ringout::blocks(10000000);

        self.b = 0;

        self.load_preset(ReverbPreset::A);

        self.modphase = 0.0;

        self.update_rsize();

        // Should be the smoothest
        self.mix.set_target(1.0); 
        self.width.set_target(1.0);

        self.mix.instantize();
        self.width.instantize();

        self.init_taps();

        self.delay_pos = 0;
    }
}
