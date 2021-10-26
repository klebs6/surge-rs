ix!();

use crate::{
    SampleAndHoldOscillator,
    SampleAndHoldOscillatorParam,
};

impl SampleAndHoldOscillator {

    pub fn update_lagvals<const IS_INIT: bool>(&mut self)
    {
        let sync        = pvalf![self.params[SampleAndHoldOscillatorParam::Sync_]]       as f64;
        let correlation = pvalf![self.params[SampleAndHoldOscillatorParam::Correlation]] as f64;
        let smooth      = pvalf![self.params[SampleAndHoldOscillatorParam::Smooth]]      as f64;
        let sub         = pvalf![self.params[SampleAndHoldOscillatorParam::Sub]]         as f64;
        let width       = pvalf![self.params[SampleAndHoldOscillatorParam::Width]];

        self.l_sync.new_value(maxd(0.0, sync));
        self.l_pw.new_value(limit_range(width, 0.001, 0.999) as f64);
        self.l_shape.new_value(correlation);
        self.l_smooth.new_value(smooth);
        self.l_sub.new_value(sub);

        let pp:   f32 = self.tuner.n2p_tuningctr(
            self.pitch as f64 + self.l_sync.v
        ) as f32;

        let invt: f32 = 4.0 * minf(
            1.0, 
            (8.175798915_f64 as f32) * pp * self.srunit.samplerate_os_inv() 
        );

        let hpf2: f32 = minf(
            SNH_INTEGRATOR_HPF, 
            SNH_HPF_CYCLE_LOSS.powf(invt)
        );

        // TODO Make a lookup-table

        self.li_hpf.set_target(hpf2);

        if IS_INIT {
            self.hpf_coeff.instantize();
            self.integrator_mult.instantize();
            self.l_pw.instantize();
            self.l_shape.instantize();
            self.l_smooth.instantize();
            self.l_sub.instantize();
            self.l_sync.instantize();
        }
    }
}
