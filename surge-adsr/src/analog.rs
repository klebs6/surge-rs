crate::ix!();

impl AdsrEnvelope {

    /**
      | This is the "analog" mode of the
      | envelope. If you are unclear what it is
      | doing because of the SSE the algo is
      | pretty simple; charge up and discharge
      | a capacitor with a gate. charge until you
      | hit 1, discharge while the gate is open
      | floored at the Sustain; then release.
      |
      | There is, in src/headless/UnitTests.cpp in
      | the "Clone the Analog" section, a non-SSE
      | implementation of this which makes it much
      | easier to understand.
      */
    pub fn process_block_analog(&mut self) {

        let v_cc: f32 = 1.5;

        let lc_a: f32 = pvalf![self.params[AdsrParam::Attack]];
        let lc_d: f32 = pvalf![self.params[AdsrParam::Decay]];
        let lc_s: f32 = pvalf![self.params[AdsrParam::Sustain]];
        let lc_r: f32 = pvalf![self.params[AdsrParam::Release]];

        let gate: bool = 
            (self.envstate == AdsrState::Attack) || 
            (self.envstate == AdsrState::Decay);

        unsafe {
            let mut v_c1:         __m128 = _mm_load_ss(&self._v_c1);
            let mut v_c1_delayed: __m128 = _mm_load_ss(&self._v_c1_delayed);
            let mut discharge:    __m128 = _mm_load_ss(&self._discharge);

            // attack->decay switch at 1 volt
            let one:      __m128 = _mm_set_ss(1.0); 
            let v_cc_vec: __m128 = _mm_set_ss(v_cc);

            let v_gate: __m128 = match gate { 
                true  => _mm_set_ss(v_cc),
                false => _mm_set_ss(0.0)
            };

            let v_is_gate: __m128 = _mm_cmpgt_ss(
                v_gate,
                _mm_set_ss(0.0)
            );

            // The original code here was
            //
            // _mm_and_ps(_mm_or_ps(_mm_cmpgt_ss(v_c1_delayed, one), discharge), v_gate);
            //
            // which ored in the v_gate value as
            // opposed to the boolean
            //
            discharge = _mm_and_ps( 
                _mm_or_ps(
                    _mm_cmpgt_ss(v_c1_delayed, one),
                    discharge
                ),
                v_is_gate 
            );

            v_c1_delayed = v_c1;

            let mut sustain:     __m128 = _mm_load_ss(&lc_s);
            sustain = _mm_mul_ss(sustain, sustain);

            let v_attack:  __m128 = _mm_andnot_ps(discharge, v_gate);

            let v_decay:   __m128 = _mm_or_ps(
                _mm_andnot_ps(discharge, v_cc_vec), 
                _mm_and_ps(discharge, sustain)
            );

            let v_release: __m128 = v_gate;

            let diff_v_a:  __m128 = _mm_max_ss(_mm_setzero_ps(), _mm_sub_ss(v_attack, v_c1));

            // This change from a straight min allows sustain swells
            let diff_vd_kernel:     __m128 = _mm_sub_ss(v_decay, v_c1);
            let diff_vd_kernel_min: __m128 = _mm_min_ss(_mm_setzero_ps(), diff_vd_kernel );
            let dis_and_gate:       __m128 = _mm_and_ps(discharge, v_is_gate);
            let diff_v_d:           __m128 = _mm_or_ps(
                _mm_and_ps(dis_and_gate, diff_vd_kernel), 
                _mm_andnot_ps(dis_and_gate, diff_vd_kernel_min)
            );

            let diff_v_r: __m128 = _mm_min_ss(_mm_setzero_ps(), _mm_sub_ss(v_release, v_c1));

            // calculate coefficients for envelope
            let _shortest:    f32 = 6.0;
            let _longest:     f32 = -2.0;
            let coeff_offset: f32 = 2.0 - (self.srunit.samplerate() / BLOCK_SIZE as f32).log2();

            let temposyncratio_a:  f32 = tsyncratio![self,Attack];
            let temposyncratio_d:  f32 = tsyncratio![self,Decay];
            let _temposyncratio_s: f32 = tsyncratio![self,Sustain];
            let temposyncratio_r:  f32 = tsyncratio![self,Release];

            let coef_a: f32 = 2.0_f32.powf(
                std::cmp::min(
                    FloatOrd(0.0), 
                    FloatOrd(coeff_offset - lc_a * temposyncratio_a)
                ).0
            );

            let coef_d: f32 = 2.0_f32.powf(std::cmp::min(FloatOrd(0.0), 
                    FloatOrd(coeff_offset - lc_d * temposyncratio_d)).0);

            let coef_r: f32 = match self.envstate == AdsrState::UberRelease {
                true => 6.0,
                false => { 
                    let z = FloatOrd(0.0);
                    let x = FloatOrd(coeff_offset - lc_r * temposyncratio_r);
                    2.0_f32.powf(std::cmp::min(z, x).0)
                }
            };

            v_c1 = _mm_add_ss(v_c1, _mm_mul_ss(diff_v_a, _mm_load_ss(&coef_a)));
            v_c1 = _mm_add_ss(v_c1, _mm_mul_ss(diff_v_d, _mm_load_ss(&coef_d)));
            v_c1 = _mm_add_ss(v_c1, _mm_mul_ss(diff_v_r, _mm_load_ss(&coef_r)));

            _mm_store_ss(&mut self._v_c1, v_c1);
            _mm_store_ss(&mut self._v_c1_delayed, v_c1_delayed);
            _mm_store_ss(&mut self._discharge, discharge);

            _mm_store_ss(&mut self.output, v_c1);
        }

        const SILENCE_THRESHOLD: f32 = 1e-6;

        if !gate && 
            self._discharge == 0.0 && 
                self._v_c1 < SILENCE_THRESHOLD
        {
            self.envstate = AdsrState::Idle;
            self.output = 0.0;
            self.idlecount += 1;
        }
    }
}
