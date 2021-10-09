ix!();

impl CoefficientLoadStore for crate::HalfRateFilterSSE {

    /**
       when steep is false, we use softer slopes,
       more attenuation and less stopband ripple
      */
    fn load_coefficients(&mut self) {

        for i in 0..self.m {
            self.va[i] = unsafe{ _mm_setzero_ps() };
        }

        let order: usize = self.m << 1;

        match self.steep {
            true  => self.load_steep_coefficients(order),
            false => self.load_softer_coefficients(order),
        }
    }

    unsafe fn store_coefficients(&mut self, coefficient_a: *mut f64, coefficient_b: *mut f64) {
        for idx in 0..self.m {
            self.va[idx] = _mm_set_ps(
                *coefficient_b.add(idx) as f32, 
                *coefficient_a.add(idx) as f32, 
                *coefficient_b.add(idx) as f32, 
                *coefficient_a.add(idx) as f32);
        }
    }
}
