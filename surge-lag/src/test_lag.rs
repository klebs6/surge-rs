crate::ix!();

/// These tests cover each method of the `Lag`
/// struct and should help ensure that it works as
/// expected. 
///
/// Note that the expected value for the
/// `test_process` test may need to be adjusted if
/// the `lp` value is changed.
///
#[cfg(test)]
mod tests {

    use super::*;

    #[test] fn smoke() {

        let mut x = Lag::default();

        x.instantize();
        x.new_value(5.0);

        let _val = x.get_target_value();

        x.process();
    }

    #[test]
    fn test_default() {
        let lag = Lag::<f32>::default();
        assert_eq!(lag.v, 0.0);
        assert_eq!(lag.target_v, 0.0);
        assert_eq!(lag.first_run, true);
        assert_eq!(lag.lp, 0.004);
        assert_eq!(lag.lpinv, 0.996);
    }

    #[test]
    fn test_new() {
        let lag = Lag::<f32>::new(0.002);
        assert_eq!(lag.v, 0.0);
        assert_eq!(lag.target_v, 0.0);
        assert_eq!(lag.first_run, true);
        assert_eq!(lag.lp, 0.002);
        assert_eq!(lag.lpinv, 0.998);
    }

    #[test]
    fn test_set_rate() {
        let mut lag = Lag::<f32>::default();
        lag.set_rate(0.003);
        assert_eq!(lag.lp, 0.003);
        assert_eq!(lag.lpinv, 0.997);
    }

    #[test]
    fn test_new_value() {
        let mut lag = Lag::<f32>::default();
        lag.new_value(10.0);
        assert_eq!(lag.target_v, 10.0);
        assert_eq!(lag.v, 10.0);
        assert_eq!(lag.first_run, false);
    }

    #[test]
    fn test_start_value() {
        let mut lag = Lag::<f32>::default();
        lag.start_value(20.0);
        assert_eq!(lag.target_v, 20.0);
        assert_eq!(lag.v, 20.0);
        assert_eq!(lag.first_run, false);
    }

    #[test]
    fn test_instantize() {
        let mut lag = Lag::<f32>::default();
        lag.new_value(10.0);
        lag.instantize();
        assert_eq!(lag.v, 10.0);
    }

    #[test]
    fn test_process() {
        let mut lag = Lag::<f32>::default();
        lag.new_value(10.0);
        lag.process();
        assert_eq!(lag.v, 0.04);  // expected value for lp = 0.004
    }

    #[test]
    fn test_get_target_value() {
        let mut lag = Lag::<f32>::default();
        lag.new_value(10.0);
        assert_eq!(lag.get_target_value(), 10.0);
    }
}
