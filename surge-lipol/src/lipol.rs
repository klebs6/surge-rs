crate::ix!();

const FIRST_RUN_CHECKS: bool = true;

/// A LiPol (linear interpolator) is a type of
/// audio processing algorithm that linearly
/// interpolates between two values over time. 
///
/// This is useful for smoothly transitioning
/// between audio values, for example when fading
/// in or out.
///
#[derive(Debug,Clone)]
pub struct LiPol<T: Num + From<f32> + Clone + AddAssign> {

    /// the current audio value
    ///
    pub v:     T,

    /// the target audio value (used for
    /// interpolation)
    ///
    pub new_v: T,

    /// the rate of change between `v` and `new_v`
    ///
    pub dv:    T,

    /// the inverse of the block size, used for
    /// calculating `dv`
    ///
    bs_inv:    T,

    /// a boolean flag indicating whether this is
    /// the first time the LiPol has been used
    /// (used for special first-run behavior)
    ///
    first_run: bool,
}

impl<T: Num + From<f32> + Clone + AddAssign > Default for LiPol<T> {
    fn default() -> Self {
        Self::new(BLOCK_SIZE)
    }
}

impl<T: Num + From<f32> + Clone + AddAssign > LiPol<T> {

    /// Create a new LiPol with the given block
    /// size.
    ///
    pub fn new(blocksize: usize) -> Self {
        let mut x = Self {
            v:         T::zero(),
            new_v:     T::zero(),
            dv:        T::zero(),
            bs_inv:    T::zero(),
            first_run: true,
        };
        x.set_blocksize(blocksize);
        x
    }

    /// Set the block size of this LiPol to `n`.
    ///
    #[inline] pub fn set_blocksize(&mut self, n: usize) {
        self.bs_inv = (1.0 / n as f32).into();
    }

    /// Set the current audio value to the target
    /// audio value and reset the rate of change
    /// to zero.
    ///
    #[inline] pub fn instantize(&mut self) {
        self.v  = self.new_v.clone();
        self.dv = T::zero();
    }

    /// Advance the current audio value by the
    /// current rate of change.
    ///
    #[inline] pub fn process(&mut self) {
        self.v += self.dv.clone();
    }

    /// Get the current target audio value.
    ///
    #[inline] pub fn get_target_value(&mut self) -> T {
        self.new_v.clone()
    }

    /// Reset this LiPol with the given block
    /// size.
    ///
    #[inline] pub fn reset(&mut self, blocksize: usize) {

        self.first_run = true;
        self.new_v     = T::zero();
        self.v         = T::zero();
        self.dv        = T::zero();
        self.bs_inv    = T::zero();

        self.set_blocksize(blocksize);
    }

    /// Set the target audio value to `x` and
    /// calculate the new rate of change.
    ///
    #[inline] pub fn new_value(&mut self, x: T) {

        self.v     = self.new_v.clone();
        self.new_v = x.clone();

        if FIRST_RUN_CHECKS && self.first_run {
            self.v = x;
            self.first_run = false;
        }

        self.dv = 
            (self.new_v.clone() - self.v.clone()) * 
            self.bs_inv.clone();
    }
}
