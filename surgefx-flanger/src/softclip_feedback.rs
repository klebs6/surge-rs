crate::ix!();

impl Flanger {

    pub fn softclip_the_feedback_to_avoid_explosions(
        &mut self,
        block_idx: usize, 
        combs:     &mut WetBlock1Dual::<BLOCK_SIZE>) -> (f32, f32)
    {
        let mut fbl: f32 = 0.0;
        let mut fbr: f32 = 0.0;

        if self.feedback.v > 0.0 {

            fbl = limit_range( self.feedback.v * combs.buf[0][block_idx], -1.0, 1.0 );
            fbr = limit_range( self.feedback.v * combs.buf[1][block_idx], -1.0, 1.0 );

            fbl = 1.5 * fbl - 0.5 * fbl * fbl * fbl;
            fbr = 1.5 * fbr - 0.5 * fbr * fbr * fbr;

            // and now we have clipped, apply the damping. 
            //
            // FIXME - move to one mul form
            self.onepole_state.lpa_l = 
                self.onepole_state.lpa_l * ( 1.0 - self.fb_lf_damping.v ) + 
                fbl * self.fb_lf_damping.v;

            fbl -= self.onepole_state.lpa_l;

            self.onepole_state.lpa_r = 
                self.onepole_state.lpa_r * ( 1.0 - self.fb_lf_damping.v ) + 
                fbr * self.fb_lf_damping.v;

            fbr -= self.onepole_state.lpa_r;
        }

        (fbl, fbr)
    }
}
