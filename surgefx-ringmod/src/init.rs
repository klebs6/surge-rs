ix!();

use crate::{
    RingModulator,
    RingModulatorParam,
};

impl Init for RingModulator<'sr> {

    fn init(&mut self) 
    {
        let lowcut  = pvalf![self.params[RingModulatorParam::LowCut]]  as f64;
        let highcut = pvalf![self.params[RingModulatorParam::HighCut]] as f64;

        self.last_unison = -1;

        self.halfband_out.reset();
        self.halfband_in.reset();

        self.lp.suspend();
        self.hp.suspend();

        self.hp.coeff_hp(self.hp.calc_omega(lowcut / 12.0), 0.707);
        self.hp.coeff_instantize();

        self.lp.coeff_lp2b(self.lp.calc_omega(highcut / 12.0), 0.707);
        self.lp.coeff_instantize();
    }
}
