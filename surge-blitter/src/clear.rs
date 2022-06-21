crate::ix!();

impl crate::AbstractBlitter {

    pub fn clear_all_oscout(&mut self) {
        unsafe {
            self.osc_out_l  = _mm_set1_ps(0.0);
            self.osc_out_2l = _mm_set1_ps(0.0);
            self.osc_out_r  = _mm_set1_ps(0.0);
            self.osc_out_2r = _mm_set1_ps(0.0);
        }
    }

    pub fn clear_buffers(&mut self) {

        macro_rules! clear {
            ($buf:ident) => {
                for x in self.$buf.iter_mut() {
                    *x = 0.0;
                }
            }
        }

        clear!{oscbuffer_l}
        clear!{oscbuffer_r}
        clear!{dcbuffer}
    }
}
