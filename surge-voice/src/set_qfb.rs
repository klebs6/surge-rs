ix!();

use crate::{
    SurgeVoice,
};

pub struct VoiceUpdateQFCSCfg {
    pub f2_cutoff_is_offset:      bool,
    pub feedback:                 f32,
    pub filterbalance:            f32,
    pub filterblock_cfg:          FilterBlockConfiguration,
    pub filterunit_cutoff:        Vec<f32>,
    pub filterunit_envelopemode:  Vec<f32>,
    pub filterunit_keytrack:      Vec<f32>,
    pub filterunit_resonance:     Vec<f32>,
    pub filterunit_subtype:       Vec<FilterSubType>,
    pub filterunit_type:          Vec<FilterType>,
    pub keytrack_root:            f32,
    pub vca_level:                f32,
    pub vca_velsense:             f32,
    pub waveshaper0_drive_db:     f32,
}

impl SurgeVoice<'sr> {

    /// Set the parameters & registers
    pub fn set_quad_filterblock(&mut self, 
        cfg: &VoiceUpdateQFCSCfg,
        mut qfcs: Option<&mut QuadFilterChainState<'sr>>, 
        e: i32) 
    {
        // qfcs == None means init(ialise)

        if let Some(ref mut qfcs) = qfcs {
            self.filterblock_state.fbq = *qfcs;
            self.filterblock_state.fbqi = e;
        }

        let filterbalance = cfg.filterbalance;

        let drive_db = cfg.waveshaper0_drive_db;

        let drive = self.tables.db_to_linear(drive_db);

        let fbc = cfg.filterblock_cfg;

        let (fmix1, fmix2) = match fbc.is_dual() || fbc.is_stereo() {
            true =>  {
                let fmix1 = 0.5 - 0.5 * filterbalance;
                let fmix2 = 0.5 + 0.5 * filterbalance;
                (fmix1, fmix2)
            },
            false => {
                let fmix1 = minf(1.0, 1.0 - filterbalance);
                let fmix2 = minf(1.0, 1.0 + filterbalance);
                (fmix1, fmix2)
            }
        };

        let vca_level = cfg.vca_level;

        let vca_velsense = cfg.vca_velsense;

        let amp_eg_output = self.modsources[ModSource::AmpEg].as_ref().unwrap().get_output() as f32;

        let gain = self.tables.db_to_linear(
            vca_level + vca_velsense * ((1.0 - self.state.fvel) as f32)
        ) * amp_eg_output;

        let feedback = cfg.feedback;

        if let Some(ref mut qfcs) = qfcs {
            unsafe {
                set1f(&mut qfcs.gain,          e, self.fbp.gain);
                set1f(&mut qfcs.d_gain,        e, (gain - self.fbp.gain) * BLOCK_SIZE_OS_INV);
                set1f(&mut qfcs.drive,         e, self.fbp.drive);
                set1f(&mut qfcs.d_drive,       e, (drive - self.fbp.drive) * BLOCK_SIZE_OS_INV);
                set1f(&mut qfcs.feedback,      e, self.fbp.fb);
                set1f(&mut qfcs.d_feedback,    e, (feedback - self.fbp.fb) * BLOCK_SIZE_OS_INV);
                set1f(&mut qfcs.mix1,          e, self.fbp.mix1);
                set1f(&mut qfcs.d_mix1,        e, (fmix1 - self.fbp.mix1) * BLOCK_SIZE_OS_INV);
                set1f(&mut qfcs.mix2,          e, self.fbp.mix2);
                set1f(&mut qfcs.d_mix2,        e, (fmix2 - self.fbp.mix2) * BLOCK_SIZE_OS_INV);
            }
        }

        self.fbp.gain   = gain;
        self.fbp.drive  = drive;
        self.fbp.fb     = feedback;
        self.fbp.mix1   = fmix1;
        self.fbp.mix2   = fmix2;

        if let Some(ref mut qfcs) = qfcs {

            // filterunits
            unsafe {
                set1f(&mut qfcs.ws_lpf,    e, self.fbp.ws_lpf); // remember state
                set1f(&mut qfcs.feedback_line_l, e, self.fbp.feedback_line_l);
                set1f(&mut qfcs.feedback_line_r, e, self.fbp.feedback_line_r);
            }

            qfcs.unit_state[0].active[e as usize] = 0xffffffff;
            qfcs.unit_state[1].active[e as usize] = 0xffffffff;
            qfcs.unit_state[2].active[e as usize] = 0xffffffff;
            qfcs.unit_state[3].active[e as usize] = 0xffffffff;

            let (cutoff_a, cutoff_b) = self.get_cutoff_pair(cfg);
            let (reso_a, reso_b)     = self.get_reso_pair(cfg);
            let (gen_a, gen_b)       = self.get_generator_pair();

            self.coeffmaker[0].make_coeffs( cutoff_a, reso_a, gen_a );

            self.coeffmaker[1].make_coeffs( cutoff_b, reso_b, gen_b );

            let fbc = cfg.filterblock_cfg;

            for u in 0..2 {

                let ty    = cfg.filterunit_type[u];
                let subty = cfg.filterunit_subtype[u];

                if ty as usize != 0 {

                    unsafe {
                        for coeff_idx in 0..N_COEFFMAKER_COEFFS {
                            set1f(&mut qfcs.unit_state[u].coeff[coeff_idx],  e, self.coeffmaker[u].coeff[coeff_idx]);
                            set1f(&mut qfcs.unit_state[u].dcoeff[coeff_idx], e, self.coeffmaker[u].dcoeff[coeff_idx]);
                        }

                        for reg_idx in 0..N_FILTER_REGISTERS {
                            set1f(&mut qfcs.unit_state[u].reg[reg_idx], e, self.fbp.fu[u].reg[reg_idx]);
                        }
                    }

                    qfcs.unit_state[u].delay_buffer[e as usize] = self.fbp.delay[u].as_mut_ptr();
                    qfcs.unit_state[u].comb_write_position[e as usize] = self.fbp.fu[u].comb_write_position as i32;

                    if ty == FilterType::MoogLadder {
                        // LPMoog's output stage index is 
                        // stored in comb_write_position[0] for the entire quad
                        qfcs.unit_state[u].comb_write_position[0] = subty as i32;
                    }

                    if fbc.is_wide() {
                        unsafe {
                            for i in 0..N_COEFFMAKER_COEFFS {
                                set1f(&mut qfcs.unit_state[u + 2].coeff[i],  e, self.coeffmaker[u].coeff[i]);
                                set1f(&mut qfcs.unit_state[u + 2].dcoeff[i], e, self.coeffmaker[u].dcoeff[i]);
                            }

                            for i in 0..N_FILTER_REGISTERS {
                                set1f(&mut qfcs.unit_state[u + 2].reg[i], e, self.fbp.fu[u + 2].reg[i]);
                            }
                        }

                        qfcs.unit_state[u + 2].delay_buffer[e as usize] = self.fbp.delay[u + 2].as_mut_ptr();
                        qfcs.unit_state[u + 2].comb_write_position[e as usize] = self.fbp.fu[u].comb_write_position as i32;

                        if ty == FilterType::MoogLadder {
                            qfcs.unit_state[u + 2].comb_write_position[0] = subty as i32;
                        }
                    }
                }
            }
        }
    }
}
