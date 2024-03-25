use surge_math::*;

#[test] fn test_trixpan() {

    use crate::trixpan;

    let mut l = 0.1;
    let mut r = 0.2;
    let x = -1.0;
    trixpan(&mut l, &mut r, x);
    println!("l: {:?}", l);
    println!("r: {:?}", r);
    assert!(r == 0.0);
    assert!(l == 0.1 + 0.2);

    let mut l = 0.1;
    let mut r = 0.2;
    let x = 1.0;
    trixpan(&mut l, &mut r, x);
    println!("l: {:?}", l);
    println!("r: {:?}", r);
    assert!(l == 0.0);
    assert!(r == 0.1 + 0.2);
}

