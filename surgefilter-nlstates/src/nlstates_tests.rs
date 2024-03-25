/*
crate::ix!();

/// These tests verify that the `NLSFType`, `NLSFCoeff`,
/// `NLSFState`, and `NLSFSaturator` structures are
/// correctly defined and that the `process_quad` function
/// works as expected for a specific input and filter state. 
///
/// You could add more tests to cover other cases and edge
/// cases as well.
///
/// Here are some additional test cases that could be added:
/// 
/// 1. Test edge cases for the `NLSFSaturator::SoftClip`
/// option by passing in very small and very large input
/// values and ensuring that the output is clipped to the
/// appropriate range.
/// 
/// 2. Test the behavior of the filter when `stages` is set
/// to 0 (i.e., no stages). In this case, the output should
/// be the same as the input, and none of the coefficient or
/// state variables should be modified.
/// 
/// 3. Test the behavior of the filter when `stages` is set
/// to its maximum value (i.e., all four stages). In this
/// case, the output should be the result of processing the
/// input through all four stages, and all of the
/// coefficient and state variables should be updated
/// accordingly.
/// 
/// 4. Test the behavior of the filter when the `qfu`
/// argument has been initialized with non-zero values for
/// the `z1` and `z2` state variables. In this case, the
/// filter should pick up where it left off from the
/// previous call to `process_quad`, and the output should
/// reflect the state of the filter after processing the new
/// input.
/// 
/// 5. Test the behavior of the filter when the `qfu`
/// argument has been initialized with non-zero values for
/// the `coeff` array. In this case, the filter coefficients
/// should be updated before processing the input, and the
/// output should reflect the effect of these updated
/// coefficients on the filtering operation.
/// 
/// 6. Test the performance of the filter by running it on
///    a large input signal and measuring the processing
///    time. This can be useful for determining whether the
///    filter is suitable for real-time applications or
///    whether optimizations are needed to improve its
///    performance.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_nlsf_type() {
        let _ = NLSFType::LowPass;
        let _ = NLSFType::HighPass;
        let _ = NLSFType::BandPass;
        let _ = NLSFType::Notch;
        let _ = NLSFType::Allpass;
    }

    #[test]
    fn test_nlsf_coeff() {
        let _ = NLSFCoeff::A1;
        let _ = NLSFCoeff::A2;
        let _ = NLSFCoeff::B0;
        let _ = NLSFCoeff::B1;
        let _ = NLSFCoeff::B2;
    }

    #[test]
    fn test_nlsf_state() {
        let _ = NLSFState::Z1;
        let _ = NLSFState::Z2;
        let _ = NLSFState::Z3;
        let _ = NLSFState::Z4;
        let _ = NLSFState::Z5;
        let _ = NLSFState::Z6;
        let _ = NLSFState::Z7;
        let _ = NLSFState::Z8;
    }

    #[test]
    fn test_nlsf_saturator() {
        let _ = NLSFSaturator::Tanh;
        let _ = NLSFSaturator::SoftClip;
    }

    #[test]
    fn test_process_quad() {
        let mut qfu = QuadFilterUnitState::default();
        qfu.comb_write_position[0] = 3;
        qfu.coeff[C::B0] = _mm_set1_ps(1.0);
        qfu.coeff[C::B1] = _mm_set1_ps(2.0);
        qfu.coeff[C::B2] = _mm_set1_ps(1.0);
        qfu.coeff[C::A1] = _mm_set1_ps(-1.1430);
        qfu.coeff[C::A2] = _mm_set1_ps(0.4128);
        let input = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
        let filter = NonlinearStatesFilter {};
        let output = filter.process_quad(&mut qfu, input);
        assert_eq!(output, _mm_setr_ps(-0.2316865, -0.75838494, -1.3734908, -1.9278406));
    }

    #[test]
    fn test_process_quad_allpass() {
        let mut qfu = QuadFilterUnitState::new();
        qfu.coeff[C::A1] = _mm_set1_ps(-1.7248939);
        qfu.coeff[C::A2] = _mm_set1_ps(0.79024154);
        qfu.coeff[C::B0] = _mm_set1_ps(1.0);
        qfu.coeff[C::B1] = _mm_set1_ps(1.7344192);
        qfu.coeff[C::B2] = _mm_set1_ps(0.79024154);
        qfu.reg[R::Z1 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z2 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z3 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z4 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z5 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z6 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z7 as usize] = _mm_set1_ps(0.0);
        qfu.reg[R::Z8 as usize] = _mm_set1_ps(0.0);
        let nlsf = crate::NonlinearStatesFilter::new(NLSFType::Allpass);
        let result = nlsf.process_quad(&mut qfu, _mm_set1_ps(1.0));
        assert_eq!(_mm_extract_ps(result, 0), -1.7248939);
        assert_eq!(_mm_extract_ps(result, 1), 1.7344192);
        assert_eq!(_mm_extract_ps(result, 2), 0.0);
        assert_eq!(_mm_extract_ps(result, 3), 0.0);
    }

    #[test]
    fn test_process_quad_allpass2() {
        let mut qfu = QuadFilterUnitState::new();
        qfu.coeff[C::B0] = _mm_set1_ps(0.5);
        qfu.coeff[C::B1] = _mm_set1_ps(-1.0);
        qfu.coeff[C::B2] = _mm_set1_ps(0.5);
        qfu.coeff[C::A1] = _mm_set1_ps(-0.9);
        qfu.coeff[C::A2] = _mm_set1_ps(0.0);

        let filter = NonlinearStatesFilter::new(NLSFType::Allpass);

        let input = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
        let output = filter.process_quad(&mut qfu, input);

        let expected_output = _mm_setr_ps(
            1.5,
            -0.5,
            -2.5,
            -3.5
        );

        assert_eq_m128(expected_output, output);
    }

    #[test]
    fn test_process_quad_highpass() {
        let mut qfu = QuadFilterUnitState::new();
        qfu.coeff[C::B0] = _mm_set1_ps(0.5);
        qfu.coeff[C::B1] = _mm_set1_ps(-1.0);
        qfu.coeff[C::B2] = _mm_set1_ps(0.5);
        qfu.coeff[C::A1] = _mm_set1_ps(-0.9);
        qfu.coeff[C::A2] = _mm_set1_ps(0.0);

        let filter = NonlinearStatesFilter::new(NLSFType::HighPass);

        let input = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
        let output = filter.process_quad(&mut qfu, input);

        let expected_output = _mm_setr_ps(
            -0.8099999,
            -0.8099999,
            -0.8099999,
            -0.8099999
        );

        assert_eq_m128(expected_output, output);
    }

    #[test]
    fn test_process_quad_bandpass() {
        let mut qfu = QuadFilterUnitState::new();
        qfu.coeff[C::B0] = _mm_set1_ps(0.5);
        qfu.coeff[C::B1] = _mm_set1_ps(-1.0);
        qfu.coeff[C::B2] = _mm_set1_ps(0.5);
        qfu.coeff[C::A1] = _mm_set1_ps(-0.9);
        qfu.coeff[C::A2] = _mm_set1_ps(0.0);

        let filter = NonlinearStatesFilter::new(NLSFType::BandPass);

        let input = _mm_setr_ps(1.0, 2.0, 3.0, 4.0);
        let output = filter.process_quad(&mut qfu, input);

        let expected_output = _mm_setr_ps(
            1.0,
            -2.0,
            3.0,
            -4.0
        );

        assert_eq_m128(expected_output, output);
    }

    /// This test case sets up a `QuadFilterUnitState` and a `NonlinearStatesFilter` for
    /// a second-order all-pass filter with a Tanh saturator, with an initial coefficient of 0.2. 
    ///
    /// The test then sets up a complex input signal, and calls the `process_quad` function on the
    /// filter with the input signal and the `QuadFilterUnitState`.
    /// 
    /// The test verifies that the output of the `process_quad` function matches the expected
    /// output, and that the `z1` and `z2` values of the `QuadFilterUnitState` are as expected
    /// after the function call.
    ///
    #[test]
    fn test_process_quad_allpass_complex() {
        let mut qfu = QuadFilterUnitState::new(NLSFType::Allpass, NLSFSaturator::Tanh);
        let mut filter = NonlinearStatesFilter::new(NLSFType::Allpass, NLSFSaturator::Tanh);

        // Set initial filter coefficients
        let mut coeffs = NLSFCoeffs::default();
        coeffs.set_allpass(0.2);
        qfu.set_coefficients(&coeffs);
        filter.set_coefficients(&coeffs);

        let input_real: [f32; 4] = [1.0, -2.0, 3.0, -4.0];
        let input_imag: [f32; 4] = [4.0, -3.0, 2.0, -1.0];
        let mut input = unsafe { _mm_set_ps(input_imag[0], input_real[0], input_imag[1], input_real[1]) };
        input = unsafe { _mm_insert_ps(input, _mm_set_ps(input_imag[3], input_real[3], input_imag[2], input_real[2]), 0b0010) };

        let output = filter.process_quad(&mut qfu, input);

        let expected_output_real: [f32; 4] = [0.48, -0.96, 1.44, -1.92];
        let expected_output_imag: [f32; 4] = [3.52, -2.64, 1.76, -0.88];
        let expected_output = unsafe { _mm_set_ps(expected_output_imag[0], expected_output_real[0], expected_output_imag[1], expected_output_real[1]) };
        let expected_output = unsafe { _mm_insert_ps(expected_output, _mm_set_ps(expected_output_imag[3], expected_output_real[3], expected_output_imag[2], expected_output_real[2]), 0b0010) };

        unsafe {
            assert_m128_eq(expected_output, output, 1e-6);
            assert_m128_eq(qfu.z1(0), _mm_set_ps(0.80477, -0.40267, -0.80477, 0.40267), 1e-6);
            assert_m128_eq(qfu.z2(0), _mm_set_ps(-0.93556, 0.65698, 0.93556, -0.65698), 1e-6);
        }
    }
}
*/
