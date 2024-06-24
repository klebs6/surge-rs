use surge_math::*;
use surge_imports::*;

#[test]
fn test_access_mut() {
    unsafe {
        let size = 16;
        let buffer = create_aligned_buffer(size);
        
        // Fill buffer with some values
        for i in 0..size {
            *buffer.add(i) = i as f32;
        }

        // Access the __m128 values
        for offset in 0..(size / 4) {
            let simd_ptr = crate::access_mut(buffer, offset);
            let expected_ptr = (buffer as *mut __m128).add(offset);
            assert_eq!(simd_ptr, expected_ptr);
            
            // Verify values
            let values: &[f32; 4] = &*(simd_ptr as *const [f32; 4]);
            for j in 0..4 {
                assert_eq!(values[j], (offset * 4 + j) as f32);
            }
        }

        std::alloc::dealloc(buffer as *mut u8, std::alloc::Layout::from_size_align(size * mem::size_of::<f32>(), mem::align_of::<__m128>()).unwrap());
    }
}

#[test]
fn test_access() {
    unsafe {
        let size = 16;
        let buffer = create_aligned_buffer(size);
        
        // Fill buffer with some values
        for i in 0..size {
            *buffer.add(i) = i as f32;
        }

        // Access the __m128 values
        for offset in 0..(size / 4) {
            let simd_ptr = crate::access(buffer, offset);
            let expected_ptr = (buffer as *const __m128).add(offset);
            assert_eq!(simd_ptr, expected_ptr);
            
            // Verify values
            let values: &[f32; 4] = &*(simd_ptr as *const [f32; 4]);
            for j in 0..4 {
                assert_eq!(values[j], (offset * 4 + j) as f32);
            }
        }

        std::alloc::dealloc(buffer as *mut u8, std::alloc::Layout::from_size_align(size * mem::size_of::<f32>(), mem::align_of::<__m128>()).unwrap());
    }
}

