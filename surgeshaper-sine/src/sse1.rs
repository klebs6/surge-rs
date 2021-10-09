ix!();

use crate::SineShaper;

impl Waveshaper for SineShaper {
    pub fn shape(&mut self, input: __m128, drive: __m128) -> __m128 {

        unsafe {

            let one:  __m128 = _mm_set1_ps(1.0);
            let m256: __m128 = _mm_set1_ps(256.0);
            let m512: __m128 = _mm_set1_ps(512.0);

            let x: __m128 = _mm_mul_ps(input, drive);

            x = _mm_add_ps(
                _mm_mul_ps(x, m256), 
                m512
            );

            let e1: __m64 = _mm_cvtps_pi32(x);
            let e2: __m64 = _mm_cvtps_pi32(_mm_movehl_ps(x, x));

            let a: __m128 = _mm_sub_ps(
                x, 
                _mm_cvtpi32_ps(
                    _mm_movelh_ps(
                        x, 
                        _mm_cvtpi32_ps(x, e2)), 
                    e1)
            );

            let mut e4 = WetBlock1t::<i32,4>::new();
            todo!("need to load values form e1 and e2 somehow");

            macro_rules! waveshaper3 {
                ($offset:expr) => {
                    self.tables.get_waveshaper_ptr(3, $offset)
                }
            }

            let mut ws1: __m128 = _mm_load_ss(waveshaper3![e4.buf[0]]);
            let mut ws2: __m128 = _mm_load_ss(waveshaper3![e4.buf[1]]);
            let mut ws3: __m128 = _mm_load_ss(waveshaper3![e4.buf[2]]);
            let mut ws4: __m128 = _mm_load_ss(waveshaper3![e4.buf[3]]);

            let ws: __m128  = _mm_movelh_ps(
                _mm_unpacklo_ps(ws1, ws2), 
                _mm_unpacklo_ps(ws3, ws4)
            );

            ws1 = _mm_load_ss(waveshaper3![e4.buf[0] + 1]);
            ws2 = _mm_load_ss(waveshaper3![e4.buf[1] + 1]);
            ws3 = _mm_load_ss(waveshaper3![e4.buf[2] + 1]);
            ws4 = _mm_load_ss(waveshaper3![e4.buf[3] + 1]);

            let wsn: __m128 = _mm_movelh_ps(
                _mm_unpacklo_ps(ws1, ws2), 
                _mm_unpacklo_ps(ws3, ws4)
            );

            x = _mm_add_ps(_mm_mul_ps(_mm_sub_ps(one, a), ws), _mm_mul_ps(a, wsn));

            _mm_empty();

            x
        }
    }
}
