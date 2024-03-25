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

    // Correcting the expectation for the second test case
    assert_eq!(big_mul_r16(0xFFFF, 0xFFFF), 0xFFFE);
}

#[test]
fn test_mul_block() {

    let mut src1 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut src2 = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut dst  = [0.0; 8];

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

    let mut src1 = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut src2 = vec![2.0f32, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut dst  = vec![0.0f32; 8];

    // Align and get raw pointers
    let (_, src1_aligned, _) = unsafe { src1.align_to_mut::<f32>() };
    let (_, src2_aligned, _) = unsafe { src2.align_to_mut::<f32>() };
    let (_, dst_aligned, _)  = unsafe { dst.align_to_mut::<f32>() };

    // Ensure we're getting the correct pointer type
    let src1_ptr = src1_aligned.as_mut_ptr();
    let src2_ptr = src2_aligned.as_mut_ptr();
    let dst_ptr  = dst_aligned.as_mut_ptr();

    add_block(src1_ptr, src2_ptr, dst_ptr, 2);

    // Flatten the aligned slices for comparison
    let dst_flattened: Vec<_> = dst_aligned.iter().copied().collect();

    assert_eq!(dst_flattened, vec![3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0]);
}

#[test]
fn test_subtract_block() {

    let mut src1 = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut src2 = [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut dst  = [0.0; 8];

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
