crate::ix!();

/// This constant value represents the block size
/// used in the processing of the half-rate filter
///
pub const HALFRATE_BLOCK_SIZE: usize = 256;

impl ProcessBlock for HalfRateFilterSSE {

    /// This function processes a block of audio
    /// samples
    ///
    /// inputs:
    /// ```ignore
    /// l:        *mut f32      // A pointer to the left channel samples
    /// r:        *mut f32      // A pointer to the right channel samples
    /// nsamples: Option<usize> // An optional number of samples to process
    /// ```
    ///
    fn process_block(
        &mut self, 
        l:        *mut f32, 
        r:        *mut f32, 
        nsamples: Option<usize>) {

        // If the number of samples is not
        // specified, use a default value of 64
        //
        let nsamples = nsamples.unwrap_or(64);

        // Cast the pointers to pointers of
        // 128-bit packed single-precision
        // floating-point values
        //
        let l: *mut __m128 = l as *mut __m128;
        let r: *mut __m128 = r as *mut __m128;

        let mut o = create_halfrate_scratch_buffer(nsamples,l,r);

        self.process_filters(&mut o, nsamples);

        // The last block of code performs some
        // final processing on the samples. 
        //
        // It first creates pointers to the left
        // and right channels of the output buffer
        // as `f_l` and `f_r`, respectively. 
        //
        // It then initializes two temporary SSE
        // variables `fa_r` and `fb_r` to zero
        // using `_mm_setzero_ps()`.
        //
        let mut ctx = unsafe { 
            process_block_detail::ProcessBlockApplyContext::new(l, r) 
        };

        // It then enters a loop over the samples
        // `k`. 
        //
        // It then uses unsafe code to perform SSE
        // operations on the interleaved stereo
        // samples in `o`. 
        //
        for k in 0..nsamples {

            self.oldout = unsafe {
                ctx.process_block(k, self.oldout, &o)
            };
        }
    }
}

impl HalfRateFilterSSE {

    fn process_filters(
        &mut self, 
        o:        &mut A1d<__m128>,
        nsamples: usize)
    {
        // Process the filters
        for j in 0..self.m {

            let mut tx0: __m128 = self.vx0[j];
            let mut tx1: __m128 = self.vx1[j];
            let mut tx2: __m128 = self.vx2[j];
            let mut ty0: __m128 = self.vy0[j];
            let mut ty1: __m128 = self.vy1[j];
            let mut ty2: __m128 = self.vy2[j];
            let ta:      __m128 = self.va[j];

            for k in (0_usize..nsamples).step_by(2) {

                // shuffle inputs
                tx2 = tx1;
                tx1 = tx0;
                tx0 = o[k];

                // shuffle outputs
                ty2 = ty1;
                ty1 = ty0;

                // Apply the first allpass filter
                ty0 = unsafe {

                    _mm_add_ps(
                        tx2, 
                        _mm_mul_ps(_mm_sub_ps(tx0, ty2), ta)
                    )
                };

                o[k] = ty0;

                // shuffle inputs
                tx2 = tx1;
                tx1 = tx0;
                tx0 = o[k + 1];

                // shuffle outputs
                ty2 = ty1;
                ty1 = ty0;

                // allpass filter 1
                ty0 = unsafe {

                    _mm_add_ps(
                        tx2, 
                        _mm_mul_ps(_mm_sub_ps(tx0, ty2), ta)
                    )
                };

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

mod process_block_detail {

    use super::*;

    pub struct ProcessBlockApplyContext {

        f_l: *mut f32,
        f_r: *mut f32,

        fa_r: __m128,
        fb_r: __m128,
    }

    impl ProcessBlockApplyContext {

        pub unsafe fn new(l: *mut __m128, r: *mut __m128) -> Self {

            Self {
                f_l:  l as *mut f32,
                f_r:  r as *mut f32,
                fa_r: _mm_setzero_ps(),
                fb_r: _mm_setzero_ps(),
            }
        }

        pub unsafe fn process_block(
            &mut self, 
            k:      usize, 
            oldout: __m128, 
            o:      &A1d<__m128>) -> __m128 {

            _mm_store_ss(self.f_l.add(k), {

                // add the current sample
                // `o[k]` to the previous
                // sample `self.oldout` 
                //
                let mut v_l: __m128 =  _mm_add_ss(
                    o[k], 
                    oldout
                );

                // multiply by a constant
                // `m128_half![]` and store
                // the result in the left
                // channel of the output
                // buffer
                v_l = _mm_mul_ss(v_l, m128_half![]);

                v_l

            });

            // use `_mm_movehl_ps` to extract
            // the high-order half of the
            // current sample in `o[k]` and
            // the previous sample in
            // `oldout` and store them in
            // `self.fa_r` and `self.fb_r`,
            // respectively. 
            //
            self.fa_r = _mm_movehl_ps(self.fa_r, o[k]);
            self.fb_r = _mm_movehl_ps(self.fb_r, oldout);

            _mm_store_ss(self.f_r.add(k), {

                // add `self.fa_r` and `self.fb_r`,
                //
                let mut v_r: __m128 = _mm_add_ss(self.fa_r, self.fb_r);

                // multiply the result by
                // `m128_half![]` 
                v_r = _mm_mul_ss(v_r, m128_half![]);

                // store the result in the right
                // channel of the output buffer
                v_r
            });

            // Finally, set `self.oldout` to
            // `o[k]` shuffled according to
            // the constant 
            //
            // `_MM_SHUFFLE(3, 3, 1, 1)`. 
            //
            // This completes the processing
            // of the block of samples.
            //
            _mm_shuffle_ps(
                o[k], 
                o[k], 
                _MM_SHUFFLE(3, 3, 1, 1)
            )
        }
    }
}
