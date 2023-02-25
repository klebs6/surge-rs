/// This code appears to be a part of a larger project
/// involving audio processing, possibly in the context of
/// digital signal processing. The specific purpose of the
/// code is to create a half-rate output buffer from
/// a stereo input buffer and then apply a HalfRateFilterSSE
/// to the output buffer.
///
/// Note: In this code, "__m128" is a type representing
/// a 128-bit packed floating-point vector.
///
crate::ix!();

impl ProcessBlockD2 for HalfRateFilterSSE {

    /// process in place, the new block will be half the size
    ///
    fn process_block_downsample_by_two(&mut self, 
        l:        *mut f32, 
        r:        *mut f32, 
        nsamples: Option<usize>, 
        out_l:    Option<*mut f32>, 
        out_r:    Option<*mut f32>) {

        let nsamples = nsamples.unwrap_or(64);

        let mut l: *mut __m128 = l as *mut __m128;
        let mut r: *mut __m128 = r as *mut __m128;

        let mut o = create_halfrate_scratch_buffer(nsamples,l,r);

        self.downsample_2x_process_filters(nsamples);

        if let Some(x) = out_l {
            l = x as *mut __m128;
        }

        if let Some(x) = out_r {
            r = x as *mut __m128;
        }

        self.downsample_2x_apply(l, r, nsamples);
    }
}

impl HalfRateFilterSSE {

    // The method applies a HalfRateFilterSSE to the output
    // buffer by processing each set of eight interleaved
    // stereo samples in `o` and writing the resulting four
    // interleaved stereo samples to the output buffer
    // pointed to by `l` and `r`. 
    //
    // The HalfRateFilterSSE is a low-pass filter which
    // downsamples the input signal by a factor of two
    // while eliminating high-frequency noise.
    //
    // Overall, this code appears to be heavily
    // optimized for performance, as evidenced by the
    // use of low-level SIMD (Single Instruction
    // Multiple Data) instructions and unsafe pointer
    // operations. It would likely be difficult to
    // understand the code fully without knowledge of
    // low-level computer architecture and audio
    // processing algorithms.
    //
    fn downsample_2x_apply(&mut self, 
        l:        *mut __m128, 
        r:        *mut __m128, 
        nsamples: usize) {

        let ctx = detail::Downsample2xApplyContext::default();

        // Loop over the input samples in steps of eight,
        // processing each group of eight interleaved
        // stereo samples.
        //
        for k in (0..nsamples).step_by(8) {

            unsafe {
                ctx.downsample_2x_apply_block(k, l, r, nsamples)
            }
        }
    }

    fn downsample_2x_process_filters(&mut self, nsamples: usize) {

        // process filters
        for j in 0..self.m {

            let mut tx0: __m128 = self.vx0[j];
            let mut tx1: __m128 = self.vx1[j];
            let mut tx2: __m128 = self.vx2[j];
            let mut ty0: __m128 = self.vy0[j];
            let mut ty1: __m128 = self.vy1[j];
            let mut ty2: __m128 = self.vy2[j];

            let ta: __m128 = self.va[j];

            for k in (0..nsamples).step_by(2) {

                let k = k as usize;

                // shuffle inputs
                tx2 = tx1;
                tx1 = tx0;
                tx0 = o[k];

                // shuffle outputs
                ty2 = ty1;
                ty1 = ty0;

                // allpass filter 1
                unsafe {
                    ty0 = _mm_add_ps(tx2, _mm_mul_ps(_mm_sub_ps(tx0, ty2), ta));
                }

                o[k] = ty0;

                // shuffle inputs
                tx2 = tx1;
                tx1 = tx0;
                tx0 = o[k + 1];

                // shuffle outputs
                ty2 = ty1;
                ty1 = ty0;

                // allpass filter 1
                unsafe {
                    ty0 = _mm_add_ps(tx2, _mm_mul_ps(_mm_sub_ps(tx0, ty2), ta));
                }

                o[k + 1] = ty0;
            }

            self.vx0[j] = tx0;
            self.vx1[j] = tx1;
            self.vx2[j] = tx2;
            self.vy0[j] = ty0;
            self.vy1[j] = ty1;
            self.vy2[j] = ty2;
        }
    }
}

mod detail {

    use super::*;

    /// private: just an implementation detail -- should be
    /// kept inside module for now...
    ///
    pub struct Downsample2xApplyContext {
        a_r: __m128,
        c_r: __m128,
        d_r: __m128,
    }

    impl Default for Downsample2xApplyContext {

        fn default() -> Self {
            Self {
                a_r: unsafe { _mm_setzero_ps() },
                c_r: unsafe { _mm_setzero_ps() },
                d_r: unsafe { _mm_setzero_ps() },
            }
        }
    }

    impl Downsample2xApplyContext {

        pub unsafe fn downsample_2x_apply_block(
            &mut self, 
            k:        usize,
            l:        *mut __m128, 
            r:        *mut __m128, 
            nsamples: usize) {

            // We use `_mm_shuffle_ps()` to extract the
            // left and right channel data for the first
            // input sample and add them together.
            //
            let mut t_l0: __m128 = _mm_shuffle_ps(o[k], o[k], _MM_SHUFFLE(1, 1, 1, 1));
            let mut t_r0: __m128 = _mm_shuffle_ps(o[k], o[k], _MM_SHUFFLE(3, 3, 3, 3));
            let mut a_l:  __m128 = _mm_add_ss(t_l0, o[k + 1]);

            // We use `_mm_movehl_ps()` to extract the
            // high-order two single-precision
            // floating-point values from `a_r` and move
            // them to the low-order positions of the
            // new vector.
            //
            // We then add the right channel data for
            // the first input sample to this vector.
            //
            self.a_r = _mm_movehl_ps(self.a_r, o[k + 1]);
            self.a_r = _mm_add_ss(self.a_r, t_r0);

            t_l0 = _mm_shuffle_ps(o[k + 2], o[k + 2], _MM_SHUFFLE(1, 1, 1, 1));
            t_r0 = _mm_shuffle_ps(o[k + 2], o[k + 2], _MM_SHUFFLE(3, 3, 3, 3));

            let b_l: __m128 = _mm_add_ss(t_l0, o[k + 3]);

            let mut b_r = _mm_movehl_ps(self.a_r, o[k + 3]);
            b_r = _mm_add_ss(b_r, t_r0);

            t_l0 = _mm_shuffle_ps(o[k + 4], o[k + 4], _MM_SHUFFLE(1, 1, 1, 1));
            t_r0 = _mm_shuffle_ps(o[k + 4], o[k + 4], _MM_SHUFFLE(3, 3, 3, 3));

            let mut c_l: __m128 = _mm_add_ss(t_l0, o[k + 5]);

            self.c_r = _mm_movehl_ps(self.c_r, o[k + 5]);
            self.c_r = _mm_add_ss(self.c_r, t_r0);

            t_l0 = _mm_shuffle_ps(o[k + 6], o[k + 6], _MM_SHUFFLE(1, 1, 1, 1));
            t_r0 = _mm_shuffle_ps(o[k + 6], o[k + 6], _MM_SHUFFLE(3, 3, 3, 3));

            let d_l: __m128 = _mm_add_ss(t_l0, o[k + 7]);

            self.d_r = _mm_movehl_ps(self.d_r, o[k + 7]);
            self.d_r = _mm_add_ss(self.d_r, t_r0);

            // Combine the left and right samples obtained
            // above to form two interleaved stereo samples
            // using the `_mm_movelh_ps` function.
            //
            a_l = _mm_movelh_ps(a_l, b_l);
            c_l = _mm_movelh_ps(c_l, d_l);

            self.a_r = _mm_movelh_ps(self.a_r, b_r);
            self.c_r = _mm_movelh_ps(self.c_r, self.d_r);

            // - Write the resulting four interleaved
            // stereo samples to the output buffer
            // pointed to by `l` and `r`. 
            //
            // These operations are performed using the
            // `_mm_shuffle_ps` function and the
            // pointers `l` and `r`.
            //
            *l.add(k >> 3) = _mm_shuffle_ps(a_l, c_l, _MM_SHUFFLE(2, 0, 2, 0));
            *r.add(k >> 3) = _mm_shuffle_ps(self.a_r, self.c_r, _MM_SHUFFLE(2, 0, 2, 0));

            // optional: *=0.5;
            *l.add(k >> 3) = _mm_mul_ps(*l.add(k >> 3), m128_half![]);
            *r.add(k >> 3) = _mm_mul_ps(*r.add(k >> 3), m128_half![]);
        }
    }
}
