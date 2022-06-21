crate::ix!();

impl WindowOscillator {

    pub unsafe fn create_wetblock(&self, input: __m128i) -> WetBlock1::<4> {

        let mut wetblock  = WetBlock1::<4>::default();

        if cfg![macos] {

            // this should be very fast on C2D/C1D (and there are no macs with K8's)
            wetblock.buf[0]  = _mm_cvtsi128_si32(input) as f32;
            wetblock.buf[1]  = _mm_cvtsi128_si32(_mm_shuffle_epi32(input, _MM_SHUFFLE(1, 1, 1, 1))) as f32;
            wetblock.buf[2]  = _mm_cvtsi128_si32(_mm_shuffle_epi32(input, _MM_SHUFFLE(2, 2, 2, 2))) as f32;
            wetblock.buf[3]  = _mm_cvtsi128_si32(_mm_shuffle_epi32(input, _MM_SHUFFLE(3, 3, 3, 3))) as f32;

        } else {

            _mm_store_si128(wetblock.buf.as_mut_ptr() as *mut __m128i,  input);
        }

        wetblock.buf[0]  = {

            let b0 = wetblock.buf[0];
            let b1 = wetblock.buf[1];
            let b2 = wetblock.buf[2];
            let b3 = wetblock.buf[3];

            let b4 = b0 + b1 + b2 + b3;

            ((b4 as i32) >> 13) as f32
        };

        wetblock
    }
}
