ix!();

#[test] fn test_lipol() {
    let mut x = LiPol::new();
    x.set_blocksize(BLOCK_SIZE);
    x.instantize();
    x.new_value(5.0);
    let val = x.get_target_value();
    assert!(val == 5.0);
    x.reset();
    x.process();
}

