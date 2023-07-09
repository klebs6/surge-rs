use surge_math::*;

use rand::Rng;
use std::mem;

fn generate_aligned_f32_vec(len: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

fn test_copy_block_func(
    copy_func: fn(*mut f32, *mut f32, usize),
    src: &[f32],
    dst: &mut [f32],
) {
    let src_ptr = src.as_ptr() as *mut f32;
    let dst_ptr = dst.as_mut_ptr();
    copy_func(src_ptr, dst_ptr, src.len() / 4);

    for (i, &val) in dst.iter().enumerate() {
        assert_eq!(src[i], val);
    }
}

#[test]
fn test_copy_block() {
    let src = generate_aligned_f32_vec(32);
    let mut dst = vec![0.0; 32];
    test_copy_block_func(copy_block, &src, &mut dst);
}

#[test]
fn test_copy_block_unaligned_source() {
    let src = generate_aligned_f32_vec(33);
    let mut dst = vec![0.0; 32];
    test_copy_block_func(copy_block_unaligned_source, &src[1..], &mut dst);
}

#[test]
fn test_copy_block_unaligned_destination() {
    let src = generate_aligned_f32_vec(32);
    let mut dst = vec![0.0; 33];
    test_copy_block_func(copy_block_unaligned_destination, &src, &mut dst[1..]);
}

#[test]
fn test_copy_block_unaligned_src_and_dst() {
    let src = generate_aligned_f32_vec(33);
    let mut dst = vec![0.0; 33];
    test_copy_block_func(copy_block_unaligned_src_and_dst, &src[1..], &mut dst[1..]);
}
