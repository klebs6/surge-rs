use surge_imports::*;
use surge_lipol::*;

/// In these tests, we create a `LipolPs`
/// instance, a buffer of data, and then call
/// either `add_block` or `subtract_block` on
/// the `LipolPs` instance, passing in the
/// buffer and the number of quadruplets to
/// add or subtract. 
///
/// We then check that the values in the
/// buffer have been modified as expected.
///
#[test]
fn test_add_block() {

    // create a LipolPs instance
    let mut lp = LipolPs::new();

    // create a buffer with some data to add
    let mut buffer = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

    // call the add_block function
    unsafe {
        lp.add_block(buffer.as_mut_ptr(), 4);
    }

    // verify that the values in the buffer have been incremented by 1
    assert_eq!(buffer, [2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
}

#[test]
fn test_subtract_block() {

    // create a LipolPs instance
    let mut lp = LipolPs::new();

    // create a buffer with some data to subtract
    let mut buffer = align16![[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]];

    // call the subtract_block function
    unsafe {
        lp.subtract_block(buffer.as_mut_ptr(), 4);
    }

    // verify that the values in the buffer have been decremented by 1
    assert_eq!(buffer, align16![[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
}

/// Adding a block of positive and negative values.
///
#[test]
fn test_add_block() {
    let mut lipol_ps = LipolPs::new(2);

    let mut src = vec![1.0, 2.0, 3.0, 4.0];
    let nquads = src.len() / 4;

    unsafe {
        lipol_ps.add_block(src.as_mut_ptr(), nquads);
    }

    assert_eq!(src, vec![1.0, 2.0, 4.0, 6.0]);

    let mut src = vec![-1.0, -2.0, -3.0, -4.0];
    let nquads = src.len() / 4;

    unsafe {
        lipol_ps.add_block(src.as_mut_ptr(), nquads);
    }

    assert_eq!(src, vec![0.0, 0.0, 1.0, 2.0]);
}

/// Subtracting a block of positive and negative values.
///
#[test]
fn test_subtract_block() {
    let mut lipol_ps = LipolPs::new(2);

    let mut src = vec![1.0, 2.0, 3.0, 4.0];
    let nquads = src.len() / 4;

    unsafe {
        lipol_ps.subtract_block(src.as_mut_ptr(), nquads);
    }

    assert_eq!(src, vec![1.0, 2.0, 2.0, 2.0]);

    let mut src = vec![-1.0, -2.0, -3.0, -4.0];
    let nquads = src.len() / 4;

    unsafe {
        lipol_ps.subtract_block(src.as_mut_ptr(), nquads);
    }

    assert_eq!(src, vec![-1.0, -2.0, -4.0, -6.0]);
}

/// Panicking when `nquads` is not a multiple of 4.
///
#[test]
#[should_panic]
fn test_add_block_panic() {
    let mut lipol_ps = LipolPs::new(2);

    let src = vec![1.0, 2.0, 3.0];
    let nquads = src.len() / 4;

    unsafe {
        lipol_ps.add_block(src.as_mut_ptr(), nquads);
    }
}

/// Testing both `add_block` and `subtract_block`.
///
#[test]
#[should_panic]
fn test_subtract_block_panic() {
    let mut lipol_ps = LipolPs::new(2);

    let src = vec![1.0, 2.0, 3.0];
    let nquads = src.len() / 4;

    unsafe {
        lipol_ps.subtract_block(src.as_mut_ptr(), nquads);
    }
}

#[test]
fn test_add_block() {
    let mut lipol_ps = LipolPs::new(2);
    let mut src = [1.0, 2.0, 3.0, 4.0];
    let nquads = 2;

    unsafe {
        lipol_ps.add_block(src.as_mut_ptr(), nquads);
    }

    let expected = [1.0, 2.0, 4.0, 6.0];
    assert_eq!(src, expected);
}

#[test]
fn test_subtract_block() {
    let mut lipol_ps = LipolPs::new(2);
    let mut src = align16![[1.0, 2.0, 3.0, 4.0]];
    let nquads = 2;

    unsafe {
        lipol_ps.subtract_block(src.as_mut_ptr(), nquads);
    }

    let expected = [-1.0, 0.0, 2.0, 2.0];
    assert_eq!(src, expected);
}

#[test]
fn test_set_params() {
    let mut lipol_ps = LipolPs::new(2);
    let a = [1.0, 2.0];
    let b = [3.0, 4.0];
    let c = [5.0, 6.0];
    let d = [7.0, 8.0];

    lipol_ps.set_params(&a, &b, &c, &d);

    assert_eq!(lipol_ps.a, a);
    assert_eq!(lipol_ps.b, b);
    assert_eq!(lipol_ps.c, c);
    assert_eq!(lipol_ps.d, d);
}

#[test]
fn test_lipol_ps_process() {
    let mut lipol_ps = LipolPs::new(2);
    let src = [1.0, 2.0, 3.0, 4.0];
    let mut dst = [0.0, 0.0, 0.0, 0.0];
    let nquads = 2;

    lipol_ps.set_params(
        &[0.5, 0.5],
        &[0.5, 0.5],
        &[0.5, 0.5],
        &[0.5, 0.5]
    );

    lipol_ps.process(src.as_ptr(), dst.as_mut_ptr(), nquads);

    let expected = [2.0, 3.0, 4.0, 5.0];
    assert_eq!(dst, expected);
}

#[test]
fn test_lipol_ps_process_zero_nquads() {
    let mut lipol_ps = LipolPs::new(2);
    let src = [1.0, 2.0, 3.0, 4.0];
    let mut dst = [0.0, 0.0, 0.0, 0.0];
    let nquads = 0;

    lipol_ps.set_params(
        &[0.5, 0.5],
        &[0.5, 0.5],
        &[0.5, 0.5],
        &[0.5, 0.5]
    );
}

#[test]
fn test_lipol_ps_process_less_than_blocksize() {
    let mut filter = LipolPs::new(2, 2);
    let mut input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let expected_output = vec![
        2.0, 4.0,  // input[0] + 1.0, input[1] + 2.0
        6.0, 8.0,  // input[2] + 1.0, input[3] + 2.0
        5.0, 7.0,  // input[4] - 1.0, input[5] - 2.0
        3.0, 5.0,  // input[6] - 1.0, input[7] - 2.0
    ];
    let nquads = 4;

    unsafe {
        filter.add_block(input.as_mut_ptr(), nquads);
        filter.subtract_block(input.as_mut_ptr(), nquads);
    }

    assert_eq!(input, expected_output);
}

#[test]
fn test_lipol_ps_process_single_quad() {

    let mut filter = LipolPs::new(2, 2);
    let mut input  = vec![1.0, 2.0, 3.0, 4.0];

    let expected_output = vec![
        2.0, 4.0,  // input[0] + 1.0, input[1] + 2.0
        3.0, 4.0,  // input[2] - 1.0, input[3] - 2.0
    ];
    let nquads = 1;

    unsafe {
        filter.add_block(input.as_mut_ptr(), nquads);
        filter.subtract_block(input.as_mut_ptr(), nquads);
    }

    assert_eq!(input, expected_output);
}

#[test]
fn test_lipol_ps_process_odd_nquads() {
    let mut filter = LipolPs::new(2, 2);
    let mut input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let expected_output = vec![
        2.0, 4.0,  // input[0] + 1.0, input[1] + 2.0
        6.0, 8.0,  // input[2] + 1.0, input[3] + 2.0
        4.0, 6.0,  // input[4] - 1.0, input[5] - 2.0
    ];
    let nquads = 3;

    unsafe {
        filter.add_block(input.as_mut_ptr(), nquads);
        filter.subtract_block(input.as_mut_ptr(), nquads);
    }

    assert_eq!(input, expected_output);
}

/// The `test_lipol_ps_process_large_nquads`
/// test processes 100 blocks of 16 elements
/// each, adding each block and then
/// subtracting it to ensure that the LipolPs
/// instance returns the same values. 
///
/// This test verifies that the implementation
/// can handle a large number of blocks.
///
#[test]
fn test_lipol_ps_process_large_nquads() {
    let mut lipol_ps = LipolPs::new(16, 4);
    let mut src = vec![0.0f32; 16];
    let mut expected = vec![0.0f32; 16];

    // Process 100 blocks of 16 elements each
    for i in 0..100 {
        // Set src values to the current block index
        for j in 0..16 {
            src[j] = (i * 16 + j) as f32;
            expected[j] = (i * 16 + j) as f32;
        }

        // Add the block to the LipolPs instance
        unsafe {
            lipol_ps.add_block(src.as_mut_ptr(), 16);
        }

        // Subtract the block from the LipolPs instance
        unsafe {
            lipol_ps.subtract_block(src.as_mut_ptr(), 16);
        }

        // Verify that the LipolPs instance returns the same values
        for j in 0..16 {
            let actual = src[j];
            assert_eq!(actual, expected[j], "Error at block {} element {}", i, j);
        }
    }
}

/// The `test_lipol_ps_process_out_of_bounds`
/// test attempts to add and subtract blocks
/// with `nquads` larger than the buffer size,
/// which should result in a panic. 
///
/// This test verifies that the safety checks
/// in the `add_block` and `subtract_block`
/// methods are working correctly.
///
#[test]
fn test_lipol_ps_process_out_of_bounds() {
    let mut lipol_ps = LipolPs::new(16, 4);
    let mut src = vec![0.0f32; 16];

    // Attempt to add and subtract blocks with nquads larger than 16
    assert_panics!(
        unsafe { lipol_ps.add_block(src.as_mut_ptr(), 17); },
        "Should panic when attempting to add a block with nquads larger than buffer size"
    );

    assert_panics!(
        unsafe { lipol_ps.subtract_block(src.as_mut_ptr(), 17); },
        "Should panic when attempting to subtract a block with nquads larger than buffer size"
    );
}

#[test]
fn test_lipol_ps_process_negative_nquads() {
    let mut lipol = LipolPs::new();

    let src: [f32; 4] = [0.0; 4];
    let nquads = -1;

    assert!(unsafe {
        lipol.subtract_block(src.as_ptr() as *mut f32, nquads)
    }.is_err());

    assert!(unsafe {
        lipol.add_block(src.as_ptr() as *mut f32, nquads)
    }.is_err());
}

#[test]
fn test_lipol_ps_process_unaligned_src_pointer() {
    let mut lipol = LipolPs::new();

    let src = vec![0.0; 16];
    let nquads = 4;

    assert!(unsafe {
        lipol.add_block(src[1..].as_ptr() as *mut f32, nquads)
    }.is_err());

    assert!(unsafe {
        lipol.subtract_block(src[1..].as_ptr() as *mut f32, nquads)
    }.is_err());
}

#[test]
fn test_lipol_ps_process_unaligned_src_pointer_and_nquads() {
    let mut lipol = LipolPs::new();

    let src = vec![0.0; 16];
    let nquads = 3;

    assert!(unsafe {
        lipol.add_block(src[1..].as_ptr() as *mut f32, nquads)
    }.is_err());

    assert!(unsafe {
        lipol.subtract_block(src[1..].as_ptr() as *mut f32, nquads)
    }.is_err());
}

#[test]
fn test_lipol_ps_process_small_block() {
    let mut lipol = LipolPs::new();

    let src: [f32; 2] = [1.0, 2.0];

    unsafe {
        lipol.add_block(src.as_ptr() as *mut f32, 1).unwrap();

        assert_eq!(src[0], 2.0);
        assert_eq!(src[1], 4.0);

        lipol.subtract_block(src.as_ptr() as *mut f32, 1).unwrap();

        assert_eq!(src[0], 1.0);
        assert_eq!(src[1], 2.0);
    }
}

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

const SEED: [u8; 32] = [5; 32];

#[test]
fn test_lipol_ps_add_block_random() {
    const NQUADS: usize = 1024;
    const BLOCK_SIZE: usize = 64;
    let mut rng = StdRng::from_seed(SEED);

    let mut src = vec![0.0; NQUADS * 4];
    rng.fill(&mut src[..]);

    let mut expected = src.clone();
    let mut ps = LipolPs::new();
    for i in (0..NQUADS).step_by(BLOCK_SIZE) {
        unsafe {
            ps.add_block(
                src.as_mut_ptr().add(i * 4),
                BLOCK_SIZE
            );
        }
        for j in 0..BLOCK_SIZE {
            let idx = (i + j) * 4;
            expected[idx] += 1.0;
            expected[idx + 1] += 1.0;
            expected[idx + 2] += 1.0;
            expected[idx + 3] += 1.0;
        }
    }

    for i in 0..src.len() {
        assert_approx_eq!(src[i], expected[i], 1e-5);
    }
}

#[test]
fn test_lipol_ps_subtract_block_random() {
    const NQUADS: usize = 1024;
    const BLOCK_SIZE: usize = 64;
    let mut rng = StdRng::from_seed(SEED);

    let mut src = vec![0.0; NQUADS * 4];
    rng.fill(&mut src[..]);

    let mut expected = src.clone();
    let mut ps = LipolPs::new();
    for i in (0..NQUADS).step_by(BLOCK_SIZE) {
        unsafe {
            ps.subtract_block(
                src.as_mut_ptr().add(i * 4),
                BLOCK_SIZE
            );
        }
        for j in 0..BLOCK_SIZE {
            let idx = (i + j) * 4;
            expected[idx] -= 1.0;
            expected[idx + 1] -= 1.0;
            expected[idx + 2] -= 1.0;
            expected[idx + 3] -= 1.0;
        }
    }

    for i in 0..src.len() {
        assert_approx_eq!(src[i], expected[i], 1e-5);
    }
}

/// अन्तरिक्षतः तृतीयचक्षुषा पृथिवीं दृष्ट्वा
/// सृष्टिशक्त्या स्पष्टीकृतं ज्ञानं अस्माकं समीपं वहन्
/// वयं भयं विना संस्कृतिं विकसयामः।
///
/// काञ्चनपक्षा तेजस्वरूपम् ।
/// अथ वह्निज्ञानेन जनाः विस्मिताः भवन्ति
///
#[test]
fn test_lipol_ps_add_subtract_block_random() {

    let mut rng = rand::thread_rng();

    for nquads in &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20] {
        let mut src = vec![0.0f32; nquads * 2];

        let mut expected = vec![0.0f32; nquads * 2];
        let mut expected2 = vec![0.0f32; nquads * 2];

        let mut lipol_ps = LipolPs::default();
        let mut lipol_ps2 = LipolPs::default();

        for _ in 0..100 {

            for i in 0..(nquads * 2) {
                src[i] = rng.gen::<f32>();
            }

            let offset = rng.gen_range(0, 100);
            let nquads = nquads + offset;

            let add_dy = rng.gen::<[f32; 4]>();
            let add_y1 = rng.gen::<[f32; 4]>();
            let add_y2 = rng.gen::<[f32; 4]>();

            let subtract_dy = rng.gen::<[f32; 4]>();
            let subtract_y1 = rng.gen::<[f32; 4]>();
            let subtract_y2 = rng.gen::<[f32; 4]>();

            // compute expected output using reference implementation
            for i in 0..nquads {
                let idx = i * 2;

                expected[idx] = src[idx] + add_y1;
                expected[idx + 1] = src[idx + 1] + add_y2;

                expected2[idx] = expected[idx] - subtract_y1;
                expected2[idx + 1] = expected[idx + 1] - subtract_y2;

                add_y1 = add_y1 + add_dy;
                add_y2 = add_y2 + add_dy;

                subtract_y1 = subtract_y1 + subtract_dy;
                subtract_y2 = subtract_y2 + subtract_dy;
            }

            // perform the computation using the LipolPs implementation
            unsafe {
                lipol_ps.add_block(src.as_mut_ptr(), nquads);
                lipol_ps2.subtract_block(src.as_mut_ptr(), nquads);

                // check the result against the expected output
                for i in 0..(nquads * 2) {
                    assert_approx_eq!(
                        expected[i],
                        src[i],
                        0.00001f32,
                        "add_block failed for nquads={}, iteration={}",
                        nquads,
                        i
                    );

                    assert_approx_eq!(
                        expected2[i],
                        src[i],
                        0.00001f32,
                        "subtract_block failed for nquads={}, iteration={}",
                        nquads,
                        i
                    );
                }
            }
        }
    }
}
