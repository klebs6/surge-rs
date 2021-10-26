ix!();

use crate::SSOParam;

impl crate::SurgeSuperOscillator {

    pub fn update_lagvals(&mut self, is_init: bool) {

        macro_rules! sso_param {
            ($x:ident) => {
                self.params[SSOParam::$x]
            }
        }

        let shape  = pvalf![sso_param![Shape]];
        let pw     = pvalf![sso_param![Width]];
        let pw2    = pvalf![sso_param![SubWidth]];
        let sub    = pvalf![sso_param![SubLevel]];
        let sync   = pvalf![sso_param![SyncPitch]];

        self.l_sync.new_value(maxf(0.0, sync));
        self.l_pw.new_value(limit_range(pw, 0.001, 0.999));
        self.l_pw2.new_value(limit_range(pw2, 0.001, 0.999));
        self.l_shape.new_value(limit_range(shape, -1.0, 1.0));
        self.l_sub.new_value(limit_range(sub, 0.0, 1.0));

        let pp:   f64 = self.tuner.n2p_tuningctr( (self.pitch + self.l_sync.v) as f64 );
        let invt: f32 = 4.0 * (
            mind(1.0_f64, 
                8.175798915_f64 * pp * self.srunit.dsamplerate_os_inv()
            ) as f32
        );

        // TODO ACHTUNG/WARNING! Make a lookup table
        let hpf2: f32 = minf(SSO_INTEGRATOR_HPF, SSO_HPF_CYCLE_LOSS.powf(invt)); 

        self.li_hpf.set_target(hpf2);

        if is_init {
            self.l_pw.instantize();
            self.l_pw2.instantize();
            self.l_shape.instantize();
            self.l_sub.instantize();
            self.l_sync.instantize();
            self.li_dc.instantize();
            self.li_hpf.instantize();
            self.li_integratormult.instantize();
        }
    }
}

