crate::ix!();

/// A constant boolean value that determines
/// whether to run first-time checks.
///
const FIRST_RUN_CHECKS: bool = true;

/// The code defines a `Lag` struct that is used
/// for smoothing values over time. It uses
/// a simple low-pass filter to achieve this. 
///
/// The `Lag` struct is generic over a type
/// parameter `T`, which must implement the `Num`,
/// `From<f32>`, `Clone`, and `AddAssign` traits. 
///
/// This allows the `Lag` struct to be used with
/// a variety of numeric types.
///
#[derive(Debug,Clone)]
pub struct Lag<T: Num + From<f32> + Clone + AddAssign > {

    /// The current smoothed value.
    ///
    pub v:        T,

    /// The target value to be smoothed towards.
    pub target_v: T,

    /// A boolean that tracks whether the `Lag`
    /// struct is being used for the first time.
    ///
    first_run:    bool,

    /// The low-pass filter coefficient.
    lp:           T,

    /// The inverse of the low-pass filter
    /// coefficient.
    ///
    lpinv:        T,
}


impl<T: Num + From<f32> + Clone + AddAssign > Default for Lag<T> {

    /// The `Default` implementation for `Lag` can
    /// be used to create a `Lag` instance with
    /// a default low-pass filter coefficient of
    /// `0.004`.
    ///
    fn default() -> Self {

        let lp: T = 0.004.into();

        Self {
            v:         T::zero(),
            target_v:  T::zero(),
            first_run: true,
            lp:        lp.clone(),
            lpinv:     T::from(1.0) - lp
        }
    }
}

impl<T: Num + From<f32> + Clone + AddAssign > Lag<T> {

    /// The `Lag` struct can be created using the
    /// `new` method, which takes a `lp` parameter
    /// that is used to set the low-pass filter
    /// coefficient. 
    ///
    pub fn new(lp: T) -> Self {
        Self {
            lp,
            .. Default::default()
        }
    }

    /// This method sets the low-pass filter
    /// coefficient to a new value.
    ///
    #[inline] pub fn set_rate(&mut self, lp: T) {
        self.lp    = lp.clone();
        self.lpinv = T::from(1.0) - lp;
    }

    /// This method sets the `target_v` field to
    /// a new value to be smoothed towards. If
    /// `FIRST_RUN_CHECKS` is set to true and it's
    /// the first time `new_value` is called, `v`
    /// is also set to the new value.
    ///
    #[inline] pub fn new_value(&mut self, x: T) {

        self.target_v = x;

        if FIRST_RUN_CHECKS && self.first_run {
            self.v = self.target_v.clone();
            self.first_run = false;
        }
    }

    /// This method sets both `v` and `target_v`
    /// fields to a new value to be smoothed
    /// towards. If `FIRST_RUN_CHECKS` is set to
    /// true and it's the first time `start_value`
    /// is called, `v` is set to the new value and
    /// `first_run` is set to false.
    ///
    #[inline] pub fn start_value(&mut self, x: T) {

        self.target_v = x.clone();
        self.v        = x;

        if FIRST_RUN_CHECKS && self.first_run {
            self.first_run = false;
        }
    }

    /// This method sets `v` to the current value
    /// of `target_v`.
    ///
    #[inline] pub fn instantize(&mut self) {
        self.v  = self.target_v.clone();
    }

    /// This method updates `v` by applying the
    /// low-pass filter to the current values of
    /// `v` and `target_v`.
    ///
    #[inline] pub fn process(&mut self) {
        self.v = self.v.clone() * self.lpinv.clone() + 
            self.target_v.clone() * self.lp.clone();
    }

    /// This method returns the current value of
    /// `target_v`.
    ///
    #[inline] pub fn get_target_value(&mut self) -> T {
        self.target_v.clone()
    }
}

