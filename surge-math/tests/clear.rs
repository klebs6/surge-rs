use surge_math::*;

#[test]
#[cfg(target_feature = "sse")]
fn test_clear_block() {
    const NQUADS: usize = 4;
    let mut input: [f32; NQUADS * 4] = [1.0; NQUADS * 4];

    unsafe { clear_block::<NQUADS>(input.as_mut_ptr()) };

    for value in input.iter() {
        assert_eq!(*value, 0.0);
    }
}

#[test]
#[cfg(target_feature = "sse")]
fn test_clear_block_antidenormalnoise() {
    const NQUADS: usize = 4;
    let mut input: [f32; NQUADS * 4] = [1.0; NQUADS * 4];

    unsafe { clear_block_antidenormalnoise::<NQUADS>(input.as_mut_ptr()) };

    let expected_values = [
        0.000_000_000_000_001,
        0.000_000_000_000_001,
        -0.000_000_000_000_001,
        -0.000_000_000_000_001,
    ];

    for (i, value) in input.iter().enumerate() {
        assert_eq!(*value, expected_values[i % 4]);
    }
}

