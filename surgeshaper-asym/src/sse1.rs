ix!();

#[cfg(not(target_arch = "x86_64"))] 
impl Waveshaper for AsymShaper {

    fn shape(&self, input: __m128, drive: __m128) -> __m128 {

        unsafe {

            let one:  __m128 = _mm_set1_ps(1.0);
            let m32:  __m128 = _mm_set1_ps(32.0);
            let m512: __m128 = _mm_set1_ps(512.0);
            let LB:   __m128 = _mm_set1_pi16(0);
            let UB:   __m128 = _mm_set1_pi16(0x3fe);

            let mut x: __m128 = _mm_mul_ps(input, drive);

            x = _mm_add_ps(
                _mm_mul_ps(x, m32), 
                m512
            );

            let e1: __m64 = _mm_cvtps_pi32(x);

            let e2: __m64 = _mm_cvtps_pi32(
                _mm_movehl_ps(x, x)
            );

            let a: __m128 = _mm_sub_ps(
                x, 
                _mm_cvtpi32_ps(
                    _mm_movelh_ps(
                        x, 
                        _mm_cvtpi32_ps(x, e2)), 
                    e1)
            );

            e1 = _mm_packs_pi32(e1, e2);

            e1 = _mm_max_pi16(
                _mm_min_pi16(e1, UB), 
                LB
            );

            let mut e4 = WetBlock1t::<i32,4>::new();
            todo!("need to load values form e1 and e2 somehow -- see comment immediately below for how this can be done in sse2");
            /*
            e4.buf[0] = _mm_cvtsi64_si32(e);
            e4.buf[1] = _mm_cvtsi128_si32(_mm_shufflelo_epi16(e, _MM_SHUFFLE(1, 1, 1, 1)));
            e4.buf[2] = _mm_cvtsi128_si32(_mm_shufflelo_epi16(e, _MM_SHUFFLE(2, 2, 2, 2)));
            e4.buf[3] = _mm_cvtsi128_si32(_mm_shufflelo_epi16(e, _MM_SHUFFLE(3, 3, 3, 3)));
            */

            macro_rules! waveshaper2 {
                ($offset:expr) => {
                    self.tables.get_waveshaper_ptr(2, $offset)
                }
            }

            let mut ws1: __m128 = _mm_load_ss(waveshaper2![e4.buf[0]]);
            let mut ws2: __m128 = _mm_load_ss(waveshaper2![e4.buf[1]]);
            let mut ws3: __m128 = _mm_load_ss(waveshaper2![e4.buf[2]]);
            let mut ws4: __m128 = _mm_load_ss(waveshaper2![e4.buf[3]]);

            __m128 ws = _mm_movelh_ps(
                _mm_unpacklo_ps(ws1, ws2), 
                _mm_unpacklo_ps(ws3, ws4)
            );

            ws1 = _mm_load_ss(waveshaper2![e4.buf[0] + 1]);
            ws2 = _mm_load_ss(waveshaper2![e4.buf[1] + 1]);
            ws3 = _mm_load_ss(waveshaper2![e4.buf[2] + 1]);
            ws4 = _mm_load_ss(waveshaper2![e4.buf[3] + 1]);

            __m128 wsn = _mm_movelh_ps(
                _mm_unpacklo_ps(ws1, ws2), 
                _mm_unpacklo_ps(ws3, ws4)
            );

            x = _mm_add_ps(
                _mm_mul_ps(
                    _mm_sub_ps(one, a), 
                    ws), 
                _mm_mul_ps(a, wsn)
            );

            _mm_empty();

            return x;
        }
    }
}
