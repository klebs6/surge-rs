ix!();

pub use crate::coeff::*;

pub const HALFRATE_MAX_M: usize = 6;

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct HalfRateFilterSSE {
    pub va:       A1d::<__m128>,
    pub vx0:      A1d::<__m128>,
    pub vx1:      A1d::<__m128>,
    pub vx2:      A1d::<__m128>,
    pub vy0:      A1d::<__m128>,
    pub vy1:      A1d::<__m128>,
    pub vy2:      A1d::<__m128>,
    pub oldout:   __m128,
    pub m:        usize,
    pub steep:    bool,
    pub oldout_l: f32,
    pub oldout_r: f32,
}

impl Default for HalfRateFilterSSE {
    fn default() -> Self {
        Self {
            va:       Self::scratch_zero(),
            vx0:      Self::scratch_zero(),
            vx1:      Self::scratch_zero(),
            vx2:      Self::scratch_zero(),
            vy0:      Self::scratch_zero(),
            vy1:      Self::scratch_zero(),
            vy2:      Self::scratch_zero(),
            oldout:   unsafe { z128![] },
            m:        0,
            steep:    false,
            oldout_l: 0.0,
            oldout_r: 0.0
        }
    }
}

impl HalfRateFilterSSE {

    #[inline] pub fn scratch_zero() -> A1d::<__m128> {
        unsafe {
            A1d::<__m128>::from_elem(HALFRATE_MAX_M, z128![])
        }
    }

    pub fn new(m: usize, steep: bool) -> Self {
        assert!(!(m > HALFRATE_MAX_M));
        let mut result = Self {
            m,
            steep,
            .. Default::default()
        };
        result.load_coefficients();
        result.reset();
        result
    }
}

impl Reset for HalfRateFilterSSE {
    fn reset(&mut self) {
        unsafe {
            for i in 0..self.m {
                self.vx0[i] = z128![];
                self.vx1[i] = z128![];
                self.vx2[i] = z128![];
                self.vy0[i] = z128![];
                self.vy1[i] = z128![];
                self.vy2[i] = z128![];
            }
            self.oldout = z128![];
        }
    }
}
