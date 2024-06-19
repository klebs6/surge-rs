// This was suggested by the AI and may be implemented later...
//
crate::ix!();

/// A trait defining two functions, `apply` and
/// `reset`, for applying the filter to an input
/// value and resetting the filter's internal
/// state, respectively. This trait is used for
/// applying filters to input values.
///
pub trait BiquadFilterApply {
    fn apply(&mut self, x: f64) -> f64;
    fn reset(&mut self);
}

pub trait BiquadCascade {

    /// takes a mutable reference to another
    /// filter implementing the
    /// `BiquadFilterApply` trait and returns
    /// a mutable reference to itself. 
    ///
    /// This function is used for cascading two
    /// filters together.
    ///
    fn cascade(&mut self, other: &mut dyn BiquadFilterApply) -> &mut Self;
}
