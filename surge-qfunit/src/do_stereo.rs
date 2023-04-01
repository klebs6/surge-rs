this is another:

crate::ix!();

/**
  |# Safety
  |
  |Need to ensure we can at least access
  |BLOCK_SIZE_OS elements starting from valid
  |out_l and out_r
  */
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn do_stereo<const A_FILTER_ACTIVE: bool, const WAVESHAPER_ACTIVE: bool, const B_FILTER_ACTIVE: bool>(
    mut wss:  &mut WaveshaperState, 
    qfcs:     &mut QuadFilterChainState, 
    fbq:      &mut FbqGlobal, 
    out_l:    *mut f32, 
    out_r:    *mut f32) 
{
    unsafe {
        for k in (0..BLOCK_SIZE_OS).step_by(1) 
        {
            qfcs.feedback = _mm_add_ps(
                qfcs.feedback, 
                qfcs.d_feedback
            );

            let mut fb: __m128 = _mm_mul_ps(
                qfcs.feedback, 
                qfcs.feedback_line_l
            );

            fb = softclip_ps(fb);

            let mut x: __m128 = _mm_add_ps( qfcs.dl[k], fb);

            let mut y: __m128 = _mm_add_ps( qfcs.dr[k], fb);

            if A_FILTER_ACTIVE {
                let filter_a = fbq.fu1ptr.unwrap();
                x = filter_a( &mut qfcs.unit_state[0], x );
            }

            if B_FILTER_ACTIVE {
                let filter_b = fbq.fu2ptr.unwrap();
                y = filter_b( &mut qfcs.unit_state[1], y );
            }

            if WAVESHAPER_ACTIVE {
                let waveshaper = fbq.wsptr.unwrap();

                qfcs.drive = _mm_add_ps(
                    qfcs.drive, qfcs.d_drive
                );
                x = waveshaper(
                    &mut wss, x, qfcs.drive
                );
                y = waveshaper(
                    &mut wss, y, qfcs.drive
                );
            }

            qfcs.mix1 = _mm_add_ps(qfcs.mix1, qfcs.d_mix1);
            qfcs.mix2 = _mm_add_ps(qfcs.mix2, qfcs.d_mix2);

            x = _mm_mul_ps(x, qfcs.mix1);
            y = _mm_mul_ps(y, qfcs.mix2);

            qfcs.gain = _mm_add_ps(qfcs.gain, qfcs.d_gain);

            let mask: __m128 = _mm_load_ps(
                qfcs.unit_state[0].active.as_mut_ptr() as *mut f32
            );

            x = _mm_and_ps(mask, _mm_mul_ps(x, qfcs.gain));
            y = _mm_and_ps(mask, _mm_mul_ps(y, qfcs.gain));

            qfcs.feedback_line_l = _mm_add_ps(x, y);

            // output stage
            qfc_write_outputs_dual(qfcs, x, y, k, out_r, out_l) ;

            qfc_assert_reasonable_audio_float!(*out_l.add(k));
            qfc_assert_reasonable_audio_float!(*out_r.add(k));
        }
    }
}
