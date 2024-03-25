/*
use surge_math::*;
use surge_imports::*;

#[test]
fn test_get_absmax() {

    let mut src = align16![[-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]];

    let absmax = unsafe {
        get_absmax(src.as_mut_ptr(), 2)
    };

    assert_eq!(absmax, 8.0);
}

#[test]
fn test_get_absmax_2() {

    let mut src1 = align16![[-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]];
    let mut src2 = align16![[1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -9.0]];

    assert_eq!(
        unsafe {
            get_absmax_2(
                src1.as_mut_ptr(), 
                src2.as_mut_ptr(), 
                2
            )
        },
        9.0
    );
}

#[test]
fn test_uniform_positive() {
    let data = align16![[2.0; 8]];
    unsafe {
        assert_eq!(get_absmax_2(data.as_ptr(), data.as_ptr(), 2), 2.0);
    }
}

#[test]
fn test_uniform_negative() {
    let data = align16![[-2.0; 8]];
    unsafe {
        assert_eq!(get_absmax_2(data.as_ptr(), data.as_ptr(), 2), 2.0);
    }
}

#[test]
fn test_mixed_values() {
    let d1 = align16![[1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0]];
    let d2 = align16![[-1.5, 2.5, -3.5, 4.5, -5.5, 6.5, -7.5, 8.5]];
    unsafe {
        assert_eq!(get_absmax_2(d1.as_ptr(), d2.as_ptr(), 2), 8.5);
    }
}

#[test]
fn test_with_zeros_and_extremes() {
    let d1 = align16![[0.0, -0.0, f32::MAX, -f32::MAX]];
    let d2 = align16![[-0.0, 0.0, f32::MIN, -f32::MIN]];
    unsafe {
        assert_eq!(get_absmax_2(d1.as_ptr(), d2.as_ptr(), 1), f32::MAX);
    }
}

#[test]
fn test_get_absmax_2_typical_scenario() {
    let d1 = align16![[-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0]];
    let d2 = align16![[1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -9.0]];

    let result = unsafe { get_absmax_2(d1.as_ptr(), d2.as_ptr(), 2) };

    assert_eq!(result, 9.0);
}
*/
