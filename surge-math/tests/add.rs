use surge_math::*;
use surge_imports::*;

#[test]
fn test_add_block() {
    unsafe {
        let size = 16;
        let src1 = create_aligned_buffer(size);
        let src2 = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src1 and src2 with some values
        for i in 0..size {
            *src1.add(i) = i as f32;
            *src2.add(i) = (i * 2) as f32;
        }

        // Call the function
        assert!(add_block(src1, src2, dst, size / 4).is_ok());

        // Verify the results
        for i in 0..size {
            assert_eq!(*dst.add(i), (i * 3) as f32);
        }

        std::alloc::dealloc(src1 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(src2 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_add_block_edge_case() {
    unsafe {
        let size = 4;
        let src1 = create_aligned_buffer(size);
        let src2 = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src1 and src2 with some values
        for i in 0..size {
            *src1.add(i) = 1.0;
            *src2.add(i) = 2.0;
        }

        // Call the function
        assert!(add_block(src1, src2, dst, size / 4).is_ok());

        // Verify the results
        for i in 0..size {
            assert_eq!(*dst.add(i), 3.0);
        }

        std::alloc::dealloc(src1 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(src2 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_add_block_zero_quads() {
    unsafe {
        let size = 16;
        let src1 = create_aligned_buffer(size);
        let src2 = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src1 and src2 with some values
        for i in 0..size {
            *src1.add(i) = 1.0;
            *src2.add(i) = 2.0;
        }

        // Call the function with zero quads
        assert!(add_block(src1, src2, dst, 0).is_ok());

        // Verify the results (should remain unchanged)
        for i in 0..size {
            assert_eq!(*dst.add(i), 0.0);
        }

        std::alloc::dealloc(src1 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(src2 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_add_block_large_input() {
    unsafe {
        let size = 1024;
        let src1 = create_aligned_buffer(size);
        let src2 = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Fill src1 and src2 with some values
        for i in 0..size {
            *src1.add(i) = (i % 10) as f32;
            *src2.add(i) = ((i % 10) * 2) as f32;
        }

        // Call the function
        assert!(add_block(src1, src2, dst, size / 4).is_ok());

        // Verify the results
        for i in 0..size {
            assert_eq!(*dst.add(i), ((i % 10) * 3) as f32);
        }

        std::alloc::dealloc(src1 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(src2 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_add_block_misaligned() {
    // Ensure the function returns an error on misaligned pointers
    unsafe {
        let size = 16;
        let src1 = create_aligned_buffer(size);
        let src2 = create_aligned_buffer(size);
        let dst = create_aligned_buffer(size);

        // Call the function with misaligned src1 pointer
        let result = add_block(src1.add(1), src2, dst, size / 4);
        assert!(matches!(result, Err(AlignmentError::SrcPtr { idx: 0, required_align: 16 })));

        // Call the function with misaligned src2 pointer
        let result = add_block(src1, src2.add(1), dst, size / 4);
        assert!(matches!(result, Err(AlignmentError::SrcPtr { idx: 1, required_align: 16 })));

        // Call the function with misaligned dst pointer
        let result = add_block(src1, src2, dst.add(1), size / 4);
        assert!(matches!(result, Err(AlignmentError::DstPtr { idx: 0, required_align: 16 })));

        std::alloc::dealloc(src1 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(src2 as *mut u8, std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
        std::alloc::dealloc(dst as *mut u8,  std::alloc::Layout::from_size_align(size * std::mem::size_of::<f32>(), std::mem::align_of::<__m128>()).unwrap());
    }
}
