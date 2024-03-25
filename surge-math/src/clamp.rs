crate::ix!();

/// Clamps the given value `x` within the range [0, 1].
///
/// # Example
///
/// ```no_run
/// use surge_math::clamp01;
///
/// let value = clamp01(1.5);
/// assert_eq!(value, 1.0);
/// ```
#[inline]
pub fn clamp01<T: LimitRange + From<f32>>(x: T) -> T {
    x.limit_range(T::from(0.0), T::from(1.0))
}

/// Clamps the given value `x` within the range [-1, 1].
///
/// # Example
///
/// ```no_run
/// use surge_math::clamp1_bipolar;
///
/// let value = clamp1_bipolar(1.5);
/// assert_eq!(value, 1.0);
/// ```
#[inline]
pub fn clamp1_bipolar<T: LimitRange + From<f32>>(x: T) -> T {
    x.limit_range(T::from(-1.0), T::from(1.0))
}

/// Checks if the given value `val` is within the range [`low`, `high`].
///
/// # Example
///
/// ```no_run
/// use surge_math::within_range;
///
/// assert!(within_range(1, 2, 3));
/// assert!(!within_range(1, 0, 3));
/// ```
pub fn within_range<T: PartialOrd>(low: T, val: T, high: T) -> bool {
    (val >= low) && (val <= high)
}
