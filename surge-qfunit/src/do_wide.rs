ix!();

use crate::qfc_write_outputs_dual;

///# Safety
///
///Need to ensure we can at least access BLOCK_SIZE_OS elements starting
///from valid out_l and out_r
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn do_wide<const A_FILTER_ACTIVE: bool, const WAVESHAPER_ACTIVE: bool, const B_FILTER_ACTIVE: bool>(
    mut wss:  &mut crate::WaveshaperState<'tables>, 
    qfcs:     &mut crate::QuadFilterChainState<'tables>, 
    fbq:      &mut crate::FbqGlobal<'tables>, 
    out_l:    *mut f32, 
    out_r:    *mut f32) 
{
    unsafe {
        let one:  __m128 = _mm_set1_ps(1.0);

        for k in (0..BLOCK_SIZE_OS).step_by(1) {
            qfcs.feedback = _mm_add_ps(qfcs.feedback, qfcs.d_feedback);

            let fb_l: __m128 = _mm_mul_ps(qfcs.feedback, qfcs.feedback_line_l);
            let fb_r: __m128 = _mm_mul_ps(qfcs.feedback, qfcs.feedback_line_r);
            let xin:  __m128 = _mm_add_ps(qfcs.dl[k], softclip_ps(fb_l));
            let yin:  __m128 = _mm_add_ps(qfcs.dr[k], softclip_ps(fb_r));

            let mut x: __m128 = xin;
            let mut y: __m128 = yin;

            if A_FILTER_ACTIVE {
                let filter_a = fbq.fu1ptr.unwrap();
                x = filter_a(&mut qfcs.unit_state[0], x);
                y = filter_a(&mut qfcs.unit_state[2], y);
            }

            if WAVESHAPER_ACTIVE {
                let waveshaper = fbq.wsptr.unwrap();
                qfcs.drive = _mm_add_ps(qfcs.drive, qfcs.d_drive);
                x = waveshaper(&mut wss, x, qfcs.drive);
                y = waveshaper(&mut wss, y, qfcs.drive);
            }

            if A_FILTER_ACTIVE || WAVESHAPER_ACTIVE {

                qfcs.mix1 = _mm_add_ps(qfcs.mix1, qfcs.d_mix1);

                let one_minus_mix1: __m128 = _mm_sub_ps(one, qfcs.mix1);

                x = _mm_add_ps(
                    _mm_mul_ps(xin, one_minus_mix1), 
                    _mm_mul_ps(x, qfcs.mix1)
                );

                y = _mm_add_ps(
                    _mm_mul_ps(yin, one_minus_mix1), 
                    _mm_mul_ps(y, qfcs.mix1)
                );
            }

            if B_FILTER_ACTIVE {
                let filter_b = fbq.fu2ptr.unwrap();

                let z: __m128 = filter_b(&mut qfcs.unit_state[1], x);
                let w: __m128 = filter_b(&mut qfcs.unit_state[3], y);

                qfcs.mix2 = _mm_add_ps(qfcs.mix2, qfcs.d_mix2);

                let one_minus_mix2: __m128 = _mm_sub_ps(one, qfcs.mix2);

                x = _mm_add_ps(
                    _mm_mul_ps(x, one_minus_mix2), 
                    _mm_mul_ps(z, qfcs.mix2)
                );

                y = _mm_add_ps(
                    _mm_mul_ps(y, one_minus_mix2), 
                    _mm_mul_ps(w, qfcs.mix2)
                );
            }

            qfcs.gain = _mm_add_ps(qfcs.gain, qfcs.d_gain);

            let mask: __m128 = _mm_load_ps(
                qfcs.unit_state[0].active.as_mut_ptr() as *mut f32
            );

            x = _mm_and_ps(mask, _mm_mul_ps(x, qfcs.gain));
            y = _mm_and_ps(mask, _mm_mul_ps(y, qfcs.gain));

            qfcs.feedback_line_l = x;
            qfcs.feedback_line_r = y;

            // output stage
            qfc_write_outputs_dual(qfcs, x, y, k, out_r, out_l) ;

            qfc_assert_reasonable_audio_float!(*out_l.add(k));
            qfc_assert_reasonable_audio_float!(*out_r.add(k));
        }
    }
}
