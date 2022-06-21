crate::ix!();

#[test] fn test_lipol() {

    use crate::LiPol;

    let mut x = LiPol::<f32>::default();
    x.set_blocksize(BLOCK_SIZE);
    x.instantize();
    x.new_value(5.0);
    let val = x.get_target_value();
    assert!(val == 5.0);
    x.reset(BLOCK_SIZE);
    x.process();
}
