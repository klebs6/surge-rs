/*
crate::ix!();

/// These tests cover the `instantize`,
/// `get_target_value`, `reset`, `new_value`, and
/// `default` methods. 
///
/// They use different values for `blocksize` and
/// test various scenarios, such as setting a new
/// value and resetting the `LiPol` instance.
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lipol = LiPol::new(128);
        assert_eq!(lipol.v, 0.0.into());
        assert_eq!(lipol.new_v, 0.0.into());
        assert_eq!(lipol.dv, 0.0.into());
        assert_eq!(lipol.bs_inv, 0.0078125.into());
        assert_eq!(lipol.first_run, true);
    }

    #[test]
    fn test_set_blocksize() {
        let mut lipol = LiPol::new(128);
        lipol.set_blocksize(64);
        assert_eq!(lipol.bs_inv, 0.015625.into());
    }

    #[test]
    fn test_instantize() {
        let mut lipol = LiPol::new(4);
        lipol.new_value(2.0);
        lipol.process();
        lipol.instantize();
        assert_eq!(lipol.v, 2.0);
        assert_eq!(lipol.dv, 0.0);
    }

    #[test]
    fn test_get_target_value() {
        let mut lipol = LiPol::new(4);
        lipol.new_value(2.0);
        assert_eq!(lipol.get_target_value(), 2.0);
    }

    #[test]
    fn test_reset() {
        let mut lipol = LiPol::new(4);
        lipol.new_value(2.0);
        lipol.process();
        lipol.reset(8);
        assert_eq!(lipol.bs_inv, 1.0 / 8.0);
        assert_eq!(lipol.v, 0.0);
        assert_eq!(lipol.new_v, 0.0);
        assert_eq!(lipol.dv, 0.0);
        assert_eq!(lipol.first_run, true);
    }

    #[test]
    fn test_new_value() {
        let mut lipol = LiPol::new(4);
        lipol.new_value(2.0);
        lipol.process();
        lipol.new_value(4.0);
        assert_eq!(lipol.v, 2.0);
        assert_eq!(lipol.new_v, 4.0);
        assert_eq!(lipol.dv, 0.5);
        lipol.new_value(8.0);
        assert_eq!(lipol.v, 4.0);
        assert_eq!(lipol.new_v, 8.0);
        assert_eq!(lipol.dv, 0.5);
    }

    #[test]
    fn test_default() {
        let lipol = LiPol::<f32>::default();
        assert_eq!(lipol.bs_inv, 0.0);
        assert_eq!(lipol.v, 0.0);
        assert_eq!(lipol.new_v, 0.0);
        assert_eq!(lipol.dv, 0.0);
        assert_eq!(lipol.first_run, true);
    }
}
*/
