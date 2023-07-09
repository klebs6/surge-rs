crate::ix!();

/// Test function to verify the alignment of the generated
/// structs
///
#[test] fn test_align() {

    #[derive(Clone,Default)]
    struct TestStruct { }

    let x0 = Align16::<TestStruct>(TestStruct{});
    let x1 = Align32::<TestStruct>(TestStruct{});
    let x2 = Align64::<TestStruct>(TestStruct{});
    let x3 = Align128::<TestStruct>(TestStruct{});
    let x4 = Align256::<TestStruct>(TestStruct{});

    assert!(std::mem::align_of_val(&x0) == 16);
    assert!(std::mem::align_of_val(&x1) == 32);
    assert!(std::mem::align_of_val(&x2) == 64);
    assert!(std::mem::align_of_val(&x3) == 128);
    assert!(std::mem::align_of_val(&x4) == 256);
}

