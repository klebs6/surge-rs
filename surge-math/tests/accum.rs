use surge_math::*;
use surge_imports::*;

#[test]
fn test_accumulate_block() {
    unsafe {
        let size = 16;
        let src = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src and dst with some values
        for i in 0..size {
            *src.add(i) = i as f32;
            *dst.add(i) = (i * 2) as f32;
        }

        // Call the function
        accumulate_block(src, dst, size / 4);

        // Verify the results
        for i in 0..size {
            assert_eq!(*dst.add(i), (i * 3) as f32);
        }

        std::alloc::dealloc(src as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_accumulate_block_edge_case() {
    unsafe {
        let size = 4;
        let src = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src and dst with some values
        for i in 0..size {
            *src.add(i) = 1.0;
            *dst.add(i) = 2.0;
        }

        // Call the function
        accumulate_block(src, dst, size / 4);

        // Verify the results
        for i in 0..size {
            assert_eq!(*dst.add(i), 3.0);
        }

        std::alloc::dealloc(src as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_accumulate_block_zero_quads() {
    unsafe {
        let size = 16;
        let src = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src and dst with some values
        for i in 0..size {
            *src.add(i) = 1.0;
            *dst.add(i) = 2.0;
        }

        // Call the function with zero quads
        accumulate_block(src, dst, 0);

        // Verify the results (should remain unchanged)
        for i in 0..size {
            assert_eq!(*dst.add(i), 2.0);
        }

        std::alloc::dealloc(src as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_accumulate_block_large_input() {
    unsafe {
        let size = 1024;
        let src = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src and dst with some values
        for i in 0..size {
            *src.add(i) = (i % 10) as f32;
            *dst.add(i) = ((i % 10) * 2) as f32;
        }

        // Call the function
        accumulate_block(src, dst, size / 4);

        // Verify the results
        for i in 0..size {
            assert_eq!(*dst.add(i), ((i % 10) * 3) as f32);
        }

        std::alloc::dealloc(src as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_accumulate_block_misaligned() {
    // Ensure the function panics on misaligned pointers
    let result = std::panic::catch_unwind(|| {
        unsafe {
            let size = 16;
            let src = create_aligned_buffer(size);
            let dst = create_aligned_buffer(size);

            // Call the function with misaligned pointers
            accumulate_block(src.add(1), dst, size / 4);
        }
    });

    assert!(result.is_err(), "Function did not panic on misaligned src pointer");

    let result = std::panic::catch_unwind(|| {
        unsafe {
            let size = 16;
            let src = create_aligned_buffer(size);
            let dst = create_aligned_buffer(size);

            // Call the function with misaligned pointers
            accumulate_block(src, dst.add(1), size / 4);
        }
    });

    assert!(result.is_err(), "Function did not panic on misaligned dst pointer");
}

