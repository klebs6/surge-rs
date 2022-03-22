ix!();

use crate::*;

impl Init for Reverb {

    fn init(&mut self) {

        self.init_band1();
        self.init_lowcut();
        self.init_hicut();

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

macro_rules! omega {
    ($band:expr, $freq:ident) => {
        $band.calc_omega($freq / 12.0)
    }
}

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

    fn init_band1(&mut self) {

        let f1: f64 = self.pvalf(ReverbParam::Band1Freq).into();
        let g1: f64 = self.pvalf(ReverbParam::Band1Gain).into();

        self.band1.coeff_peak_eq(
            omega![self.band1, f1], 
            2.0, 
            g1
        );

        self.band1.coeff_instantize();
        self.band1.suspend();
    }

    fn init_lowcut(&mut self) {

        let lc: f64  = self.pvalf(ReverbParam::LowCut).into();

        self.locut.coeff_hp(
            omega![self.locut, lc], 
            0.5
        );

        self.locut.coeff_instantize();
        self.locut.suspend();
    }

    fn init_hicut(&mut self) {

        let hc: f64  = self.pvalf(ReverbParam::HighCut).into();

        self.hicut.coeff_lp2b(
            omega![self.hicut, hc], 
            0.5
        );

        self.hicut.coeff_instantize();

        self.hicut.suspend();
    }
}
