/// hello! could you please help me figure out what HALFRATE_MAX_M is?
/// 
/// In the code you provided, `HALFRATE_MAX_M` is a constant with a value of 6. It is used as
/// a maximum value for the filter order parameter `m` in the `new` method of the
/// `HalfRateFilterSSE` struct.
/// 
/// In this implementation, the `m` parameter represents the filter order, which is related to the
/// number of cascaded integrator-comb (CIC) stages used in the filter. The CIC filter is
/// a decimation filter that reduces the sample rate of a signal by an integer factor, and its
/// filter order determines the amount of decimation performed.
/// 
/// By limiting the maximum value of `m` to 6, this implementation restricts the maximum amount of
/// decimation that can be performed by the filter. This limit is likely based on the trade-off
/// between filter complexity and performance, as increasing the filter order typically improves
/// the filter's attenuation characteristics but also increases its computational complexity.
///
/// Based on the code you provided, `HALFRATE_MAX_M` is a constant variable with the value of 6,
/// represented as a `usize` type. This variable is used as a maximum bound for the variable `m` in
/// the `HalfRateFilterSSE` struct. The value of `m` determines the number of cascaded
/// integrator-comb (CIC) stages used in the half-rate filter, which is implemented in the
/// `HalfRateFilterSSE` struct. The value of `HALFRATE_MAX_M` limits the number of stages to 6 or
/// less, and ensures that the filter does not use more memory than is allocated for it.
///

/// hello! could you please help me figure out better variable names for the HalfRateFilterSSE struct?
/// how would you document them?
/// 
/// Here are some suggestions for better variable names for the `HalfRateFilterSSE` struct:
/// 
/// - `scratch`: `va`, `vx0`, `vx1`, `vx2`, `vy0`, `vy1`, and `vy2` all seem to be temporary variables used for intermediate calculations. You could rename them to something like `scratch_a`, `scratch_x0`, `scratch_x1`, `scratch_x2`, `scratch_y0`, `scratch_y1`, and `scratch_y2`, respectively, to make their purpose clearer.
/// - `oldout`: This variable seems to represent the output of the filter from the previous sample. You could rename it to something like `prev_out` or `last_out` to make its meaning clearer.
/// - `m`: This variable is used to specify the filter order, but its name is not very descriptive. You could rename it to something like `order` or `filter_order`.
/// - `steep`: It is unclear what this variable represents from the code provided. You could add a comment or rename it to something more descriptive based on its intended use.
/// - `oldout_l` and `oldout_r`: These variables represent the left and right output channels of the filter from the previous sample. You could rename them to something like `prev_out_left` and `prev_out_right` to make their meaning clearer.
/// 
/// Here is an example of how you could document these variables:
/// 
/// ```rust
/// #[derive(Debug, Clone)]
/// #[repr(align(16))]
/// pub struct HalfRateFilterSSE {
///     // Scratch space for intermediate calculations
///     pub scratch_a:  A1d::<__m128>,
///     pub scratch_x0: A1d::<__m128>,
///     pub scratch_x1: A1d::<__m128>,
///     pub scratch_x2: A1d::<__m128>,
///     pub scratch_y0: A1d::<__m128>,
///     pub scratch_y1: A1d::<__m128>,
///     pub scratch_y2: A1d::<__m128>,
/// 
///     // The output of the filter from the previous sample
///     pub prev_out:   __m128,
/// 
///     // The order of the filter
///     pub filter_order: usize,
/// 
///     // Flag indicating whether the filter has a steep roll-off
///     pub is_steep:   bool,
/// 
///     // The left and right output channels of the filter from the previous sample
///     pub prev_out_left: f32,
///     pub prev_out_right: f32,
/// }
/// ```


crate::ix!();

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

    /// The `steep` flag affects the behavior of
    /// the filter by adjusting its slopes,
    /// attenuation, and stopband ripple.
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
