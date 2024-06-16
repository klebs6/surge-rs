crate::ix!();

pub trait ProcessBlockAnalog {

    fn process_block_analog(&mut self);
}

/// This function implements an analog-mode ADSR (Attack, Decay, Sustain,
/// Release) envelope generator, which is a commonly used sound synthesis
/// technique to shape the amplitude of an audio signal over time.
///
impl ProcessBlockAnalog for AdsrEnvelope {

    /// This is the "analog" mode of the envelope. If you are unclear what it is
    /// doing because of the SSE the algo is pretty simple; charge up and
    /// discharge a capacitor with a gate. charge until you hit 1, discharge
    /// while the gate is open floored at the Sustain; then release.
    ///
    /// There is, in src/headless/UnitTests.cpp in the "Clone the Analog"
    /// section, a non-SSE implementation of this which makes it much easier to
    /// understand.
    ///
    fn process_block_analog(&mut self) {

        let (lc_a, lc_d, lc_s, lc_r) = self.get_adsr();

        // `gate` is a boolean variable that indicates whether the envelope is
        // in Attack or Decay state.
        //
        let gate: bool = self.state_machine_is_attack_or_decay();

        unsafe {

            // Next, the code initializes four variables `v_c1`, `v_c1_delayed`,
            // `discharge`, and `one` as 128-bit vectors using the SSE
            // (Streaming SIMD Extensions) instructions. 
            //
            // These variables represent the charge on a capacitor, the delayed
            // charge on the capacitor, the discharge flag, and the constant
            // value 1.0, respectively. 
            //
            // The `_mm_load_ss` function loads a scalar value into a vector
            // variable, and the `_mm_set_ss` function sets a scalar value as
            // the only element of a vector variable.
            //
            let mut v_c1:         __m128 = self.load_v_c1();
            let mut v_c1_delayed: __m128 = self.load_v_c1_delayed();
            let mut discharge:    __m128 = self.load_discharge();

            // attack->decay switch at 1 volt
            let one:      __m128 = _mm_set_ss(1.0); 
            let v_cc_vec: __m128 = _mm_set_ss(V_CC);

            // Then, the code computes the `v_gate` and `v_is_gate` variables,
            // which represent the gate voltage and a boolean indicating whether
            // the gate is open or not, respectively. 
            //
            // These variables are calculated based on the `gate` boolean
            // variable and the `V_CC` constant voltage level.
            //
            let v_gate: __m128 = match gate { 
                true  => _mm_set_ss(V_CC),
                false => _mm_set_ss(0.0)
            };

            let v_is_gate: __m128 = _mm_cmpgt_ss(
                v_gate,
                _mm_set_ss(0.0)
            );

            // Next, the code computes the discharge flag using the
            // `v_c1_delayed` and `discharge` variables. 
            //
            // The discharge flag is set to 1 if the delayed charge on the
            // capacitor is greater than
            // 1 or if the discharge flag is already set to 1. 
            //
            // The `v_is_gate` variable is used to AND the discharge flag with
            // the gate voltage, effectively disabling discharge when the gate
            // is closed.
            //
            // The original code here was
            //
            // _mm_and_ps(_mm_or_ps(_mm_cmpgt_ss(v_c1_delayed, one), discharge),
            // v_gate);
            //
            // which ored in the v_gate value as opposed to the boolean
            //
            discharge = {

                let t0 = _mm_cmpgt_ss(v_c1_delayed, one);

                let t1 = _mm_or_ps(t0, discharge);

                _mm_and_ps(t1, v_is_gate)
            };

            v_c1_delayed = v_c1;

            let sustain:     __m128 = {
                let s = _mm_load_ss(&lc_s);
                _mm_mul_ss(s, s)
            };

            // The code then calculates the `v_attack`, `v_decay`, and
            // `v_release` variables, which represent the output voltage levels
            // during the Attack, Decay, and Release phases of the envelope,
            // respectively. 
            //
            // The `v_attack` variable is set to the gate voltage when the
            // discharge flag is not set, and it is set to
            // 0 otherwise. 
            //
            // The `v_decay` variable is a combination of the constant voltage
            // level `V_CC`, the sustain level `lc_s`, and the discharge flag. 
            //
            // If the discharge flag is not set, the output voltage is `V_CC`. 
            //
            // Otherwise, it is the sustain level `lc_s` scaled by itself, to
            // make it between 0 and 1. 
            //
            // The `v_release` variable is simply the gate voltage.
            //
            let v_attack:  __m128 = _mm_andnot_ps(discharge, v_gate);

            let v_decay:   __m128 = _mm_or_ps(
                _mm_andnot_ps(discharge, v_cc_vec), 
                _mm_and_ps(discharge, sustain)
            );

            let v_release: __m128 = v_gate;

            // The code then calculates the voltage differences `diff_v_a`,
            // `diff_v_d`, and `diff_v_r`, which represent the voltage
            // differences between the current capacitor charge and the target
            // output voltage during the Attack, Decay, and Release phases of
            // the envelope, respectively. 
            //
            // The `_mm_sub_ss` function subtracts the second argument from the
            // first argument, and `_mm_max_ss` and `_mm_min_ss` functions
            // return the maximum and minimum values of two arguments,
            // respectively.
            //
            let diff_v_a:  __m128 = _mm_max_ss(_mm_setzero_ps(), _mm_sub_ss(v_attack, v_c1));

            // This change from a straight min allows sustain swells
            //
            let diff_vd_kernel:     __m128 = _mm_sub_ss(v_decay, v_c1);
            let diff_vd_kernel_min: __m128 = _mm_min_ss(_mm_setzero_ps(), diff_vd_kernel );
            let dis_and_gate:       __m128 = _mm_and_ps(discharge, v_is_gate);

            let diff_v_d:           __m128 = {

                let x0 = _mm_and_ps(dis_and_gate, diff_vd_kernel);
                let x1 = _mm_andnot_ps(dis_and_gate, diff_vd_kernel_min);

                _mm_or_ps(x0, x1)
            };

            let diff_v_r: __m128 = {

                let x0   = _mm_sub_ss(v_release, v_c1);
                let zero = _mm_setzero_ps();

                _mm_min_ss(zero, x0)
            };

            // calculate coefficients for envelope
            //
            let _shortest:    f32 = 6.0;
            let _longest:     f32 = -2.0;
            let coeff_offset: f32 = 2.0 - (self.get_samplerate() / BLOCK_SIZE as f32).log2();

            let temposyncratio_a:  f32 = self.tsyncratio(AdsrParam::Attack);
            let temposyncratio_d:  f32 = self.tsyncratio(AdsrParam::Decay);
            let temposyncratio_r:  f32 = self.tsyncratio(AdsrParam::Release);

            // The code then calculates the envelope coefficients `coef_a`,
            // `coef_d`, and `coef_r` using the `lc_a`, `lc_d`, `lc_r` values
            // and the `temposyncratio_a`, `temposyncratio_d`,
            // `temposyncratio_r` tempo-sync ratios. 
            //
            // These coefficients determine the speed of the envelope
            // transitions in each phase. 
            //
            // The tempo-sync ratios allow the envelope to synchronize with the
            // tempo of the music being played.
            //
            let coef_a: f32 = 2.0_f32.powf(
                std::cmp::min(
                    FloatOrd(0.0), 
                    FloatOrd(coeff_offset - lc_a * temposyncratio_a)
                ).0
            );

            let coef_d: f32 = 2.0_f32.powf(std::cmp::min(FloatOrd(0.0), 
                    FloatOrd(coeff_offset - lc_d * temposyncratio_d)).0);

            let coef_r: f32 = match self.state_machine_is_uberrelease() {
                true => 6.0,
                false => { 
                    let z = FloatOrd(0.0);
                    let x = FloatOrd(coeff_offset - lc_r * temposyncratio_r);
                    2.0_f32.powf(std::cmp::min(z, x).0)
                }
            };

            // Finally, the code updates the capacitor charge `v_c1` by adding
            // the appropriate voltage difference based on the current envelope
            // state and the computed
            //
            v_c1 = _mm_add_ss(v_c1, _mm_mul_ss(diff_v_a, _mm_load_ss(&coef_a)));
            v_c1 = _mm_add_ss(v_c1, _mm_mul_ss(diff_v_d, _mm_load_ss(&coef_d)));
            v_c1 = _mm_add_ss(v_c1, _mm_mul_ss(diff_v_r, _mm_load_ss(&coef_r)));

            _mm_store_ss(self.v_c1_ref_mut(),         v_c1);
            _mm_store_ss(self.v_c1_delayed_ref_mut(), v_c1_delayed);
            _mm_store_ss(self.discharge_ref_mut(),    discharge);

            _mm_store_ss(self.output_ref_mut(), v_c1);
        }

        // This code is checking whether the gate signal is off (i.e., the note
        // has been released) and the capacitor discharge of the ADSR envelope
        // generator has completed, and the voltage across the capacitor
        // (self._v_c1) is below a certain threshold (SILENCE_THRESHOLD).
        //
        // If all of these conditions are met, it means that the envelope has
        // reached its minimum value (i.e., the note has fully decayed and is
        // silent). In this case, the code sets the ADSR state to "Idle", sets
        // the output to 0.0 (silence), and increments the idlecount variable to
        // keep track of the number of idle samples.
        //
        // This code is essentially responsible for stopping the sound output
        // when a note is released and the ADSR envelope has completed its decay
        // phase.

        if !gate && self.discharged() && 
                self.capacitor_voltage_is_below_silence_threshold()
        {
            self.set_envstate(AdsrState::Idle);
            self.clear_output();
            self.increment_idlecount();
        }
    }
}
