ix!();

use crate::HALFRATE_BLOCK_SIZE;

impl ProcessBlockD2 for crate::HalfRateFilterSSE {

    ///process in place, the new block will be half the size
    fn process_block_downsample_by_two(&mut self, 
        l:        *mut f32, 
        r:        *mut f32, 
        nsamples: Option<usize>, 
        out_l:    Option<*mut f32>, 
        out_r:    Option<*mut f32>) {

        let nsamples = nsamples.unwrap_or(64);

        let mut l: *mut __m128 = l as *mut __m128;
        let mut r: *mut __m128 = r as *mut __m128;

        let mut o = A1d::<__m128>::from_elem(HALFRATE_BLOCK_SIZE, unsafe { z128![] });

        // fill the buffer with interleaved stereo samples
        for k in (0..nsamples).step_by(4) {
            //[o3,o2,o1,o0] = [L0,L0,R0,R0]
            unsafe {
                o[k]     = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(0, 0, 0, 0));
                o[k + 1] = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(1, 1, 1, 1));
                o[k + 2] = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(2, 2, 2, 2));
                o[k + 3] = _mm_shuffle_ps(*l.add(k >> 2), *r.add(k >> 2), _MM_SHUFFLE(3, 3, 3, 3));
            }
        }

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

        let mut a_r: __m128 = unsafe { _mm_setzero_ps() };
        let mut c_r: __m128 = unsafe { _mm_setzero_ps() };
        let mut d_r: __m128 = unsafe { _mm_setzero_ps() };

        l = match out_l {
            Some(x) => x as *mut __m128,
            None => l,
        };

        r = match out_r {
            Some(x) => x as *mut __m128,
            None => r,
        };

        // 95
        for k in (0..nsamples).step_by(8) {
            unsafe {

                let mut t_l0: __m128 = _mm_shuffle_ps(o[k], o[k], _MM_SHUFFLE(1, 1, 1, 1));
                let mut t_r0: __m128 = _mm_shuffle_ps(o[k], o[k], _MM_SHUFFLE(3, 3, 3, 3));
                let mut a_l: __m128 = _mm_add_ss(t_l0, o[k + 1]);

                a_r = _mm_movehl_ps(a_r, o[k + 1]);
                a_r = _mm_add_ss(a_r, t_r0);

                t_l0 = _mm_shuffle_ps(o[k + 2], o[k + 2], _MM_SHUFFLE(1, 1, 1, 1));
                t_r0 = _mm_shuffle_ps(o[k + 2], o[k + 2], _MM_SHUFFLE(3, 3, 3, 3));

                let b_l: __m128 = _mm_add_ss(t_l0, o[k + 3]);

                let mut b_r = _mm_movehl_ps(a_r, o[k + 3]);
                b_r = _mm_add_ss(b_r, t_r0);

                t_l0 = _mm_shuffle_ps(o[k + 4], o[k + 4], _MM_SHUFFLE(1, 1, 1, 1));
                t_r0 = _mm_shuffle_ps(o[k + 4], o[k + 4], _MM_SHUFFLE(3, 3, 3, 3));

                let mut c_l: __m128 = _mm_add_ss(t_l0, o[k + 5]);

                c_r = _mm_movehl_ps(c_r, o[k + 5]);
                c_r = _mm_add_ss(c_r, t_r0);

                t_l0 = _mm_shuffle_ps(o[k + 6], o[k + 6], _MM_SHUFFLE(1, 1, 1, 1));
                t_r0 = _mm_shuffle_ps(o[k + 6], o[k + 6], _MM_SHUFFLE(3, 3, 3, 3));

                let d_l: __m128 = _mm_add_ss(t_l0, o[k + 7]);

                d_r = _mm_movehl_ps(d_r, o[k + 7]);
                d_r = _mm_add_ss(d_r, t_r0);

                a_l = _mm_movelh_ps(a_l, b_l);
                c_l = _mm_movelh_ps(c_l, d_l);
                a_r = _mm_movelh_ps(a_r, b_r);
                c_r = _mm_movelh_ps(c_r, d_r);

                *l.add(k >> 3) = _mm_shuffle_ps(a_l, c_l, _MM_SHUFFLE(2, 0, 2, 0));
                *r.add(k >> 3) = _mm_shuffle_ps(a_r, c_r, _MM_SHUFFLE(2, 0, 2, 0));

                // optional: *=0.5;
                *l.add(k >> 3) = _mm_mul_ps(*l.add(k >> 3), m128_half![]);
                *r.add(k >> 3) = _mm_mul_ps(*r.add(k >> 3), m128_half![]);
            }
        }
    }
}
