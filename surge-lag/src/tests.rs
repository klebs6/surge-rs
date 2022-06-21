crate::ix!();

#[test] fn test_lag() {
    let mut x = Lag::default();
    x.instantize();
    x.new_value(5.0);
    let val = x.get_target_value();
    x.process();
}
