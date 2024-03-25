crate::ix!();

pub unsafe fn access_mut(ptr: *mut f32, offset: usize) -> *mut __m128 {
    (ptr as *mut __m128).add(offset) 
}

pub unsafe fn access(ptr: *const f32, offset: usize) -> *const __m128 {
    (ptr as *const __m128).add(offset) 
}


