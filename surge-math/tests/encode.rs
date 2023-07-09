use surge_math::*;

#[test]
fn test_encode_mid_side() {
    let mut l = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut r = [8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let mut m = [0.0; 8];
    let mut s = [0.0; 8];

    unsafe {
        encode_mid_side(l.as_mut_ptr(), r.as_mut_ptr(), m.as_mut_ptr(), s.as_mut_ptr(), 2);
    }

    assert_eq!(m, [4.5, 4.5, 4.5, 4.5, 4.5, 4.5, 4.5, 4.5]);
    assert_eq!(s, [-3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5]);
}

#[test]
fn test_decode_mid_side() {
    let mut l = [0.0; 8];
    let mut r = [0.0; 8];
    let mut m = [4.5, 4.5, 4.5, 4.5, 4.5, 4.5, 4.5, 4.5];
    let mut s = [-3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5];

    unsafe {
        decode_mid_side(m.as_mut_ptr(), s.as_mut_ptr(), l.as_mut_ptr(), r.as_mut_ptr(), 2);
    }

    assert_eq!(l, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
    assert_eq!(r, [8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
}

