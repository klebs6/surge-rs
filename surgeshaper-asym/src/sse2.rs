crate::ix!();

#[cfg(target_arch = "x86_64")] 
impl Waveshaper for AsymShaper {

    fn shape(&self, input: __m128, drive: __m128) -> __m128 {

        unsafe {

            let one:  __m128 = _mm_set1_ps(1.0);
            let m32:  __m128 = _mm_set1_ps(32.0);
            let m512: __m128 = _mm_set1_ps(512.0);
            let ub:  __m128i = _mm_set1_epi16(0x3fe);

            let mut x: __m128 = _mm_mul_ps(input, drive);
            x = _mm_add_ps(_mm_mul_ps(x, m32), m512);

            let mut e: __m128i = _mm_cvtps_epi32(x);
            let a: __m128 = _mm_sub_ps(x, _mm_cvtepi32_ps(e));

            e = _mm_packs_epi32(e, e);
            e = _mm_max_epi16(_mm_min_epi16(e, ub), _mm_setzero_si128());

            if !cfg![target_os="macos"] {
                println!("on PC write to memory & back as XMM -> GPR is slow on K8");
                /*let mut e4 = WetBlock1t::<i16,8>::new();
                  _mm_store_si128((__m128i*)&e4, e);*/
            } else {
                // this should be very fast on C2D/C1D (and there are no macs with K8's)
            }

            let mut e4 = WetBlock1t::<i32,4>::default();

            e4.buf[0] = _mm_cvtsi128_si32(e);
            e4.buf[1] = _mm_cvtsi128_si32(_mm_shufflelo_epi16(e, _MM_SHUFFLE(1, 1, 1, 1)));
            e4.buf[2] = _mm_cvtsi128_si32(_mm_shufflelo_epi16(e, _MM_SHUFFLE(2, 2, 2, 2)));
            e4.buf[3] = _mm_cvtsi128_si32(_mm_shufflelo_epi16(e, _MM_SHUFFLE(3, 3, 3, 3)));

            macro_rules! waveshaper2 {
                ($offset:expr) => {
                    self.tables.get_waveshaper_ptr(2, $offset)
                }
            }

            let mut ws1: __m128 = _mm_load_ss(waveshaper2![e4.buf[0]]);
            let mut ws2: __m128 = _mm_load_ss(waveshaper2![e4.buf[1]]);
            let mut ws3: __m128 = _mm_load_ss(waveshaper2![e4.buf[2]]);
            let mut ws4: __m128 = _mm_load_ss(waveshaper2![e4.buf[3]]);

            let ws: __m128 = _mm_movelh_ps(_mm_unpacklo_ps(ws1, ws2), _mm_unpacklo_ps(ws3, ws4));

            ws1 = _mm_load_ss(waveshaper2![e4.buf[0] + 1]);
            ws2 = _mm_load_ss(waveshaper2![e4.buf[1] + 1]);
            ws3 = _mm_load_ss(waveshaper2![e4.buf[2] + 1]);
            ws4 = _mm_load_ss(waveshaper2![e4.buf[3] + 1]);

            let wsn: __m128 = _mm_movelh_ps(_mm_unpacklo_ps(ws1, ws2), _mm_unpacklo_ps(ws3, ws4));

            x = _mm_add_ps(_mm_mul_ps(_mm_sub_ps(one, a), ws), _mm_mul_ps(a, wsn));
            x
        }
    }
}
