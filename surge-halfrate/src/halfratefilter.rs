crate::ix!();

/// In the code you provided, `HALFRATE_MAX_M` is
/// a constant with a value of 6. It is used as
/// a maximum value for the filter order parameter
/// `m` in the `new` method of the
/// `HalfRateFilterSSE` struct.
/// 
/// In this implementation, the `m` parameter
/// represents the filter order, which is related
/// to the number of cascaded integrator-comb
/// (CIC) stages used in the filter. The CIC
/// filter is a decimation filter that reduces the
/// sample rate of a signal by an integer factor,
/// and its filter order determines the amount of
/// decimation performed.
/// 
/// By limiting the maximum value of `m` to 6,
/// this implementation restricts the maximum
/// amount of decimation that can be performed by
/// the filter. This limit is likely based on the
/// trade-off between filter complexity and
/// performance, as increasing the filter order
/// typically improves the filter's attenuation
/// characteristics but also increases its
/// computational complexity.
///
pub const HALFRATE_MAX_M: usize = 6;

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct HalfRateFilterSSE {

    /// Scratch space for intermediate
    /// calculations
    ///
    pub va:       A1d::<__m128>,
    pub vx0:      A1d::<__m128>,
    pub vx1:      A1d::<__m128>,
    pub vx2:      A1d::<__m128>,
    pub vy0:      A1d::<__m128>,
    pub vy1:      A1d::<__m128>,
    pub vy2:      A1d::<__m128>,

    /// The output of the filter from the previous
    /// sample
    ///
    pub oldout:   __m128,

    /// The order of the filter
    pub m:        usize,

    /// The `steep` flag affects the behavior of
    /// the filter by adjusting its slopes,
    /// attenuation, and stopband ripple.
    pub steep:    bool,

    /// The left and right output channels of the
    /// filter from the previous sample
    ///
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
            oldout:   z128(),
            m:        0,
            steep:    false,
            oldout_l: 0.0,
            oldout_r: 0.0
        }
    }
}

impl HalfRateFilterSSE {

    #[inline] pub fn scratch_zero() -> A1d::<__m128> {
        A1d::<__m128>::from_elem(HALFRATE_MAX_M, z128())
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
        for i in 0..self.m {
            self.vx0[i] = z128();
            self.vx1[i] = z128();
            self.vx2[i] = z128();
            self.vy0[i] = z128();
            self.vy1[i] = z128();
            self.vy2[i] = z128();
        }
        self.oldout = z128();
    }
}
