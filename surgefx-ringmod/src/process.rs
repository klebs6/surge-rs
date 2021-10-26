ix!();

use crate::{
    RingModulator,
    RingModulatorParam,
    RINGMOD_MAX_UNISON,
    RINGMOD_OVERSAMPLE,
};

//TODO: if possible, clean up/factor out this RINGMOD_OVERSAMPLE funny business
impl Process for RingModulator {

    fn process<const N: usize>(&mut self, data_l: &mut [f32; N], data_r: &mut [f32; N]) 
    {
        let mut dphase = A1d::<f32>::zeros(RINGMOD_MAX_UNISON as usize);

        let mix = pvalf![self.params[RingModulatorParam::Mix]];

        let uni = std::cmp::max(
            1, 
            pvali![self.params[RingModulatorParam::UnisonVoices]]
        );

        // Has unison reset? If so modify settings
        if uni != self.last_unison {
            self.update_unison_settings(uni);
        }

        // Gain Scale based on unison
        let gscale: f32 = 0.4 +  0.6 * ( 1.0 / (uni as f32).sqrt() );

        let mut oversampled = match RINGMOD_OVERSAMPLE {
            true  => Some(WetBlock::new(block_size_oversample![N])),
            false => None,
        };

        if RINGMOD_OVERSAMPLE {
            // Now upsample
            self.halfband_in.process_block_upsample_by_two(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                oversampled.as_mut().unwrap().l.as_mut_ptr(), 
                oversampled.as_mut().unwrap().r.as_mut_ptr(),
                None,
            );
        }

        let sri: f64 = match RINGMOD_OVERSAMPLE {
            true  => self.srunit.dsamplerate_os_inv(),
            false => self.srunit.dsamplerate_inv(),
        };

        for u in 0..uni {

            // need to calc this every time since carierfreq could change
            let carrierfreq     = pvalf![self.params[RingModulatorParam::CarrierFreq]];
            let unison_detune   = pvalf![self.params[RingModulatorParam::UnisonDetune]];

            let detune_extended = self.params[
                RingModulatorParam::UnisonDetune
            ].get_extended(
                unison_detune * self.detune_offset[u as usize]
            );

            let pitch = self.tuner.n2p::<f64,false>( (carrierfreq + detune_extended) as f64 );

            dphase[u as usize]  = (pitch * MIDI_0_FREQ * sri) as f32;
        }

        let ub: usize = match RINGMOD_OVERSAMPLE {
            true  => block_size_oversample![N],
            false => N,
        };

        for i in 0..ub {

            let mut res_l: f32 = 0.0;
            let mut res_r: f32 = 0.0;

            // TODO make faster
            for u in 0..uni {
                let u = u as usize;

                let vc: f32 = SineWaveOscillator::value_from_sin_and_cos( 
                    fastsin( 2.0 * PI_32 * ( self.phase[u] - 0.5 ) ),
                    fastcos( 2.0 * PI_32 * ( self.phase[u] - 0.5 ) ),
                    pvali![self.params[RingModulatorParam::CarrierShape]]
                );

                self.phase[u] += dphase[u];

                if self.phase[u] > 1.0 {

                    self.phase[u] -= self.phase[u];
                }

                for c in 0..2 {

                    let vin = match c == 0 {
                        true  => match RINGMOD_OVERSAMPLE {
                            true  => oversampled.as_ref().unwrap().l[i],
                            false => data_l[i],
                        },
                        false => match RINGMOD_OVERSAMPLE {
                            true  => oversampled.as_ref().unwrap().r[i],
                            false => data_r[i]
                        },
                    };

                    let a = 0.5 * vin + vc;
                    let b = vc - 0.5 * vin;

                    let d_pa: f32 = self.diode_sim(a);
                    let d_ma: f32 = self.diode_sim(-a);
                    let d_pb: f32 = self.diode_sim(b);
                    let d_mb: f32 = self.diode_sim(-b);

                    let res: f32 = d_pa + d_ma - d_pb - d_mb;

                    res_l += res * self.pan_l[u];
                    res_r += res * self.pan_r[u];

                    // std::cout << "RES " << _D(res) << _D(res_l) << _D(res_r) << _D(self.pan_l[u]) << _D(self.pan_r[u]) << _D(u) << _D(uni) << std::endl;
                }
            }

            let (samp_l, samp_r) = match RINGMOD_OVERSAMPLE {
                true  => (
                    oversampled.as_ref().unwrap().l[i], 
                    oversampled.as_ref().unwrap().r[i]
                ),
                false => (data_l[i], data_r[i]),
            };

            let mut outl = gscale * ( mix * res_l + ( 1.0 - mix ) * samp_l );
            let mut outr = gscale * ( mix * res_r + ( 1.0 - mix ) * samp_r );

            outl = 1.5 * outl - 0.5 * outl * outl * outl;
            outr = 1.5 * outr - 0.5 * outr * outr * outr;

            match RINGMOD_OVERSAMPLE {
                true => {
                    let oversampled = oversampled.as_mut().unwrap();
                    oversampled.l[i] = outl;
                    oversampled.r[i] = outr;
                },
                false => {
                    data_l[i] = outl;
                    data_r[i] = outr;
                },
            }
        }

        if RINGMOD_OVERSAMPLE {

            let oversampled = oversampled.as_mut().unwrap();

            self.halfband_out.process_block_downsample_by_two(
                oversampled.l.as_mut_ptr(), 
                oversampled.r.as_mut_ptr(),
                None, None, None
            );

            copy_block(
                oversampled.l.as_mut_ptr(), 
                data_l.as_mut_ptr(), 
                block_size_quad![N]
            );

            copy_block(
                oversampled.r.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                block_size_quad![N]
            );
        }

        let lowcut  = pvalf![self.params[RingModulatorParam::LowCut]]  as f64;
        let highcut = pvalf![self.params[RingModulatorParam::HighCut]] as f64;

        // Apply the filters
        self.hp.coeff_hp(  self.hp.calc_omega(lowcut / 12.0),  0.707);
        self.lp.coeff_lp2b(self.lp.calc_omega(highcut / 12.0), 0.707);

        unsafe {
            self.lp.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None
            );

            self.hp.process_block_stereo(
                data_l.as_mut_ptr(), 
                data_r.as_mut_ptr(), 
                None
            );
        }
    }
}
