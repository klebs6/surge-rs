crate::ix!();

pub fn fill_memory_with_random_values(ptr: *mut f32, size: usize) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for i in 0..size {
        unsafe {
            *ptr.add(i) = rng.gen_range(-1000.0..1000.0);
        }
    }
}
