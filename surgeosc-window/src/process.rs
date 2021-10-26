ix!();

use crate::{
    WindowOscillator,
    WindowOscillatorParam,
};

impl OscillatorProcess for WindowOscillator {

    fn process_block(&mut self, 
        cfg: OscillatorProcessBlockCfg)
    {
        let fmdepth = cfg.fm_depth;
        let fm      = cfg.fm;
        let pitch   = cfg.pitch;
        let drift   = cfg.drift;
        let stereo  = cfg.stereo;

        self.out.clear();

        let detune: f32 = match self.params[WindowOscillatorParam::UniSpread].absolute {
            // See comment in SurgeSuperOscillator
            true => {
                self.pvalf_extended(WindowOscillatorParam::UniSpread) *
                    self.tuner.n2pinv::<f32,true>( minf( 148.0, pitch ) ) * 16.0 / 0.9443
            },
            false => {
                self.pvalf_extended(WindowOscillatorParam::UniSpread) 
            }
        };

        let fmstrength: f32 = 32.0 * PI_32 * fmdepth * fmdepth * fmdepth;

        for l in (0_usize..self.active_sub_oscs as usize).step_by(1) 
        {
            self.drift_lfo[[l, 0]] = drift_noise(self.drift_lfo[[l,1]]);

            /* This original code uses note 57 as a center point with a frequency of 220.  */

            let f: f32 = self.tuner.n2p::<f32,false>(pitch + 
                drift * 
                self.drift_lfo[[l,0]] +
                detune * 
                (self.detune_offset + self.detune_bias * (l as f32))
            );

            // (65536.0*0.50), 0.5 for oversampling
            let ratio: i32 = {
                ((8.1757989150_f64 as f32) * 32768.0 * f * ((self.window_wavetable.num_samples_per_table()) as f32) *
                 self.srunit.samplerate_inv()) as i32
            };

            self.ratio[l] = ratio as u32;

            if fm {
                self.fm_depth[l].new_value( fmstrength.into() );

                for i in 0..BLOCK_SIZE_OS {

                    let fmadj: f32 = ( 
                        1.0_f64 + 
                        self.fm_depth[l].v * (master_osc![self,i]  as f64)
                    ) as f32;

                    let f: f32 = self.tuner.n2p::<f32,false>(
                        pitch + 
                        drift * self.drift_lfo[[l,0]] +
                        detune * 
                        (
                            self.detune_offset + self.detune_bias * (l as f32)
                        )
                    );

                    let ratio: i32 = ((8.1757989150_f64 as f32) * 32768.0 * f * fmadj * 
                        (
                            (
                                self.window_wavetable.num_samples_per_table() as f32
                            ) *
                           self.srunit.samplerate_inv() 
                        )
                    ) as i32 ; // (65536.0*0.50), 0.5 for oversampling


                    self.fm_ratio[[l,i]] = ratio;

                    self.fm_depth[l].process();

                }
            }
        }

        self.process_sub_oscs(stereo, fm);

        unsafe {
            // int32 -> float conversion
            let scale: __m128 = _mm_load1_ps(&self.out_attenuation);

            if stereo {
                for i in (0..BLOCK_SIZE_OS).step_by(4) {

                    _mm_store_ps(
                        &mut self.out.l[i], 
                        _mm_mul_ps(
                            _mm_cvtepi32_ps(*(self.out.l.as_mut_ptr() as *mut __m128i)), 
                            scale)
                    );

                    _mm_store_ps(
                        &mut self.out.r[i], 
                        _mm_mul_ps(
                            _mm_cvtepi32_ps(*(self.out.r.as_mut_ptr() as *mut __m128i)), 
                            scale)
                    );
                }

            } else {

                for i in (0..BLOCK_SIZE_OS).step_by(4) 
                {
                    _mm_store_ps(&mut self.out.l[i], 
                        _mm_mul_ps(
                            _mm_cvtepi32_ps(
                                *(self.out.l.as_mut_ptr() as *mut __m128i)), 
                            scale)
                    );
                }
            }
        }
    }
}
