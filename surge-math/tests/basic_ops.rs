use surge_math::*;

#[test]
fn test_sign() {
    assert_eq!(sign(-5), -1);
    assert_eq!(sign(0), 1);
    assert_eq!(sign(42), 1);
}

#[test]
fn test_big_mul_r16() {
    assert_eq!(big_mul_r16(0x10000, 0x10000), 0x10000);
    assert_eq!(big_mul_r16(0xFFFF, 0xFFFF), 0xFFFE0001);
}

#[test]
fn test_mul_block() {
    let mut src1 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut src2 = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut dst = [0.0; 8];

    mul_block(src1.as_mut_ptr(), src2.as_mut_ptr(), dst.as_mut_ptr(), 2);

    assert_eq!(dst, [2.0, 6.0, 12.0, 20.0, 30.0, 42.0, 56.0, 72.0]);
}

#[test]
fn test_accumulate_block() {
    let mut src = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut dst = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];

    accumulate_block(src.as_mut_ptr(), dst.as_mut_ptr(), 2);

    assert_eq!(dst, [3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0]);
}

#[test]
fn test_add_block() {
    let mut src1 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut src2 = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut dst = [0.0; 8];

    add_block(src1.as_mut_ptr(), src2.as_mut_ptr(), dst.as_mut_ptr(), 2);

    assert_eq!(dst, [3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0]);
}

#[test]
fn test_subtract_block() {
    let mut src1 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut src2 = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut dst = [0.0; 8];

    subtract_block(src1.as_mut_ptr(), src2.as_mut_ptr(), dst.as_mut_ptr(), 2);

    assert_eq!(dst, [-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0]);
}

#[test]
fn test_get_absmax() {

    let mut src = [-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0];

    assert_eq!(get_absmax(src.as_mut_ptr(), 2), 8.0);
}

#[test]
fn test_get_absmax_2() {

    let mut src1 = [-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0];
    let mut src2 = [1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -9.0];

    assert_eq!(get_absmax_2(src1.as_mut_ptr(), src2.as_mut_ptr(), 2), 9.0);
}
