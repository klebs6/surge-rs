crate::ix!();

#[derive(Debug,Copy,Clone)]
pub struct VectorizedSvfFilter {

   // Registers
   reg_l1: __m128,
   reg_b1: __m128,
   reg_l2: __m128,
   reg_b2: __m128,

   // Coefficients
   coeff_f1: __m128,
   coeff_f2: __m128,

   quality_factor:  __m128,
}

impl Reset for VectorizedSvfFilter {
    fn reset(&mut self) {
        unsafe {
            self.reg_l1         = z128![];
            self.reg_l2         = z128![];
            self.reg_b1         = z128![];
            self.reg_b2         = z128![];
            self.coeff_f1       = z128![];
            self.coeff_f2       = z128![];
            self.quality_factor = z128![];
        }
    }
}

impl Default for VectorizedSvfFilter {
    fn default() -> Self {
        unsafe {
            Self {
                reg_l1:          z128![],
                reg_b1:          z128![],
                reg_l2:          z128![],
                reg_b2:          z128![],

                coeff_f1:        z128![],
                coeff_f2:        z128![],
                quality_factor:  z128![],
            }
        }
    }
}

impl VectorizedSvfFilter {

    #[inline] pub fn calc_f(omega: f32) -> f32 
    { 
        2.0 * ((PI as f32) * omega).sin() 
    }

    #[inline] pub fn calc_q(quality: f32) -> f32 
    { 
        1.0 / quality 
    }

    pub fn set_coeff(&mut self, 
        omega: [f32; 4], 
        mut quality_factor: f32, 
        spread: f32) 
    {
        let mut freq1   = WetBlock1::<4>::default();
        let mut freq2   = WetBlock1::<4>::default();
        let mut quality = WetBlock1::<4>::default();

        quality_factor = Self::calc_q(quality_factor);

        for (idx, item) in omega.iter().enumerate() {
            freq1.buf[idx]   = Self::calc_f(item * (1.0 - spread));
            freq2.buf[idx]   = Self::calc_f(item * (1.0 + spread));
            quality.buf[idx] = quality_factor;
        }

        unsafe {
            self.coeff_f1        = _mm_load_ps(freq1.buf.as_mut_ptr());
            self.coeff_f2        = _mm_load_ps(freq2.buf.as_mut_ptr());
            self.quality_factor  = _mm_load_ps(quality.buf.as_mut_ptr());
        }
    }

    pub fn copy_coeff(&mut self, other: &mut Self) {
        self.coeff_f1        = other.coeff_f1;
        self.coeff_f2        = other.coeff_f2;
        self.quality_factor  = other.quality_factor;
    }

    #[inline] pub fn calc_bpf(&mut self, x: __m128) -> __m128 {

        unsafe {

            self.reg_l1 = v_madd(
                self.coeff_f1, 
                self.reg_b1, 
                self.reg_l1
            );

            let h1: __m128 = {

                let p0 = _mm_mul_ps(
                    x, 
                    self.quality_factor
                );

                let p1 = _mm_sub_ps(
                    p0, 
                    self.reg_l1
                );

                v_nmsub(
                    self.quality_factor, 
                    self.reg_b1, 
                    p1 
                )
            };

            self.reg_b1 = v_madd(
                self.coeff_f1, 
                h1, 
                self.reg_b1
            );

            self.reg_l2 = v_madd(
                self.coeff_f2, 
                self.reg_b2, 
                self.reg_l2
            );

            let h2: __m128 = {

                let p0 = _mm_mul_ps(
                    self.reg_b1, 
                    self.quality_factor
                );

                let p1 = _mm_sub_ps(
                    p0, 
                    self.reg_l2
                );

                v_nmsub(
                    self.quality_factor, 
                    self.reg_b2, 
                    p1
                )
            };

            self.reg_b2 = v_madd(
                self.coeff_f2, 
                h2, 
                self.reg_b2
            );

            self.reg_b2
        }
    }
}
