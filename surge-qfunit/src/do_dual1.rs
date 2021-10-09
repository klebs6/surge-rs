ix!();

use crate::{
    WaveshaperState,
    QuadFilterChainState,
    FbqGlobal,
    qfc_write_outputs,
};

///# Safety
///
///Need to ensure we can at least access BLOCK_SIZE_OS elements starting
///from valid out_l and out_r
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn do_dual1<const A_FILTER_ACTIVE: bool, const WAVESHAPER_ACTIVE: bool, const B_FILTER_ACTIVE: bool>(
    mut wss: &mut WaveshaperState<'tables>, 
    qfcs:    &mut QuadFilterChainState<'tables>, 
    fbq:     &mut FbqGlobal<'tables>, 
    out_l:   *mut f32, 
    out_r:   *mut f32) 
{
    unsafe {
        let hb_c: __m128 = _mm_set1_ps(0.50); 

        for k in (0..BLOCK_SIZE_OS).step_by(1) 
        {
            qfcs.feedback = _mm_add_ps(qfcs.feedback, qfcs.d_feedback);

            let mut feedback: __m128 = _mm_mul_ps(
                qfcs.feedback, 
                qfcs.feedback_line_l
            );

            feedback = softclip_ps(feedback);

            let mut x: __m128 = _mm_add_ps(qfcs.dl[k], feedback);
            let mut y: __m128 = _mm_add_ps(qfcs.dr[k], feedback);

            if A_FILTER_ACTIVE {
                let filter_a = fbq.fu1ptr.unwrap();
                x = filter_a(&mut qfcs.unit_state[0], x);
            }

            if B_FILTER_ACTIVE {
                let filter_b = fbq.fu2ptr.unwrap();
                y = filter_b(&mut qfcs.unit_state[1], y);
            }

            qfcs.mix1 = _mm_add_ps(qfcs.mix1, qfcs.d_mix1);
            qfcs.mix2 = _mm_add_ps(qfcs.mix2, qfcs.d_mix2);

            x = _mm_add_ps(
                _mm_mul_ps(x, qfcs.mix1), 
                _mm_mul_ps(y, qfcs.mix2)
            );

            if WAVESHAPER_ACTIVE {
                let waveshaper = fbq.wsptr.unwrap();

                qfcs.ws_lpf = _mm_mul_ps(
                    hb_c, 
                    _mm_add_ps(qfcs.ws_lpf, x)
                );

                qfcs.drive  = _mm_add_ps(
                    qfcs.drive, 
                    qfcs.d_drive
                );

                x = waveshaper(&mut wss, 
                    qfcs.ws_lpf, 
                    qfcs.drive
                );
            }

            qfcs.gain = _mm_add_ps(qfcs.gain, qfcs.d_gain);

            let mask: __m128 = _mm_load_ps(
                qfcs.unit_state[0].active.as_mut_ptr() as *mut f32
            );

            let out:  __m128 = _mm_and_ps(
                mask, 
                _mm_mul_ps(x, qfcs.gain)
            );

            qfcs.feedback_line_l = out;

            // output stage
            qfc_write_outputs(qfcs, out, k, out_r, out_l);
        }
    }
}
