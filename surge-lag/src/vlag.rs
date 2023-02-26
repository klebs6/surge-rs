crate::ix!();

/// a `f64` constant representing the minimum
/// bandwidth of the `VLag` struct.
///
pub const VLAG_MIN_BW:   f64   = 0.0001;

/// a `f64` constant representing the LP (low
/// pass) filter coefficient for the `VLag`
/// struct.
///
pub const VLAG_D_LP:     f64   = 0.004;

/// a `f64` constant representing the inverse of
/// the LP (low pass) filter coefficient for the
/// `VLag` struct.
///
pub const VLAG_D_LPINV:  f64   = 1.0 - 0.004;

pub type VDouble  = [f64; 2];

/// A struct representing a variable lag filter
/// with double precision floating-point numbers.
///
/// This filter is used to smooth out a stream of
/// data over time, by updating the current value
/// (`v`) based on the target value (`target_v`)
/// and a fixed lag coefficient (`VLAG_D_LP`),
/// which determines how quickly the filter
/// responds to changes in the target value. 
///
/// The lag coefficient and the inverse of the lag
/// coefficient (`VLAG_D_LPINV`) are pre-defined
/// constants.
///
/// The filter has two double-precision
/// floating-point values (`v` and `target_v`)
/// aligned on a 16-byte boundary, to facilitate
/// SSE/AVX vectorization.
///
/// # Example
///
/// ```rust
/// use crate::VLag;
///
/// let mut filter = VLag::new_x87();
/// filter.start_value(1.0);
/// filter.new_value(2.0);
/// filter.process();
/// assert_eq!(filter.v[0], 1.004);
/// ```
///
#[derive(Debug,Clone)]
pub struct VLag {

    /// The current value of the filter.
    pub v:        Align16<VDouble>,

    /// The target value of the filter.
    pub target_v: Align16<VDouble>,
}

impl Default for VLag {
    fn default() -> Self {
        Self::new_x87()
    }
}

impl VLag {

    /// Creates a new `VLag` filter with initial values of 0.0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::VLag;
    ///
    /// let filter = VLag::new_x87();
    /// assert_eq!(filter.v[0], 0.0);
    /// assert_eq!(filter.target_v[0], 0.0);
    /// ```
    ///
    #[inline] pub fn new_x87() -> Self {
        Self { 
            v:        Align16([0.0; 2]),
            target_v: Align16([0.0; 2])
        }
    }

    /// Initializes the filter's current and target values to 0.0.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::VLag;
    ///
    /// let mut filter = VLag::new_x87();
    /// filter.start_value(1.0);
    /// filter.init_x87();
    /// assert_eq!(filter.v[0], 0.0);
    /// assert_eq!(filter.target_v[0], 0.0);
    /// ```
    ///
    #[inline] pub fn init_x87(&mut self) {
        self.v[0] = 0.0;
        self.v[1] = 0.0;
        self.target_v[0] = 0.0;
        self.target_v[1] = 0.0;
    }

    /// Processes the filter, updating the current
    /// value based on the target value and lag
    /// coefficient.
    ///
    /// # Example
    ///
    /// ```rust
    /// use crate::VLag;
    ///
    /// let mut filter = VLag::new_x87();
    /// filter.start_value(1.0);
    /// filter.new_value(2.0);
    /// filter.process();
    /// assert_eq!(filter.v[0], 1.004);
    /// ```
    ///
    #[inline] pub fn process(&mut self) {
        self.v[0] = self.v[0] * VLAG_D_LPINV + self.target_v[0] * VLAG_D_LP;
    }

    /// This method sets the value of the target
    /// variable `target_v[0]` to the specified
    /// `f64` value `f`.
    ///
    #[inline] pub fn new_value(&mut self, f: f64) {
        self.target_v[0] = f;
    }

    /// This method sets the current value `v` to
    /// the same value as the target value
    /// `target_v`.
    ///
    #[inline] pub fn instantize(&mut self) {
        self.v = self.target_v.clone();
    }

    /// This method sets the current value `v` and
    /// the target value `target_v` to the
    /// specified `f64` value `f`.
    ///
    #[inline] pub fn start_value(&mut self, f: f64) {
        self.target_v[0] = f;
        self.v[0] = f;
    }
}

