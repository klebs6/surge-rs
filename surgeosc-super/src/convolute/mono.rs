crate::ix!();

impl SurgeSuperOscillator {

    #[inline] pub fn convolve_mono(&mut self, 
        g:        f32, 
        delay:    usize, 
        m:        usize, 
        lipol128: __m128) 
    {
        unsafe {

            /* This is SSE for the convolution described above */
            let mut g128: __m128 = _mm_load_ss(&g);
            g128 = _mm_shuffle_ps(g128, g128, _MM_SHUFFLE(0, 0, 0, 0));

            for k in (0..FIR_IPOL_N).step_by(4) {

                let idx = (self.blitter.bufpos as usize) + (k as usize) + (delay as usize);
                let sincidx:  usize = m as usize + k;
                let sincidx2: usize = m as usize + k + FIR_IPOL_N;

                // Get buffer[pos + delay + k ]
                let obf = &mut self.blitter.oscbuffer_l[idx];

                let mut ob: __m128 = _mm_loadu_ps(obf);

                // get the synctable for our fractional position
                let mut st: __m128 = _mm_load_ps(self.tables.sinctable_ptr(sincidx));             

                // get the synctable deriv
                let mut so: __m128 = _mm_load_ps(self.tables.sinctable_ptr(sincidx2)); 

                // scale the deriv by the lipol fractional time
                so = _mm_mul_ps(so, lipol128); 

                // this is now st = sinctable + dt * dsinctable
                st = _mm_add_ps(st, so); 

                // so this is now the convolved difference, g * kernel
                st = _mm_mul_ps(st, g128); 

                // which we add back onto the buffer
                ob = _mm_add_ps(ob, st); 

                // and store.
                _mm_storeu_ps(obf, ob); 
            }
        }
    }
}
