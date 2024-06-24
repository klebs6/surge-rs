use surge_math::*;
use surge_imports::*;

#[traced_test]
fn test_clear_block() 
    -> Result<(), AlignmentError> 
{
    const NQUADS: usize = 4;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        for i in 0..size {
            *ptr.add(i) = 1.0;
        }

        debug!("Before clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block::<NQUADS>(ptr)?;

        debug!("After clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
            assert_eq!(*ptr.add(i), 0.0);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_antidenormalnoise() 
    -> Result<(), AlignmentError> 
{
    const NQUADS: usize = 4;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        for i in 0..size {
            *ptr.add(i) = 1.0;
        }

        debug!("Before clear_block_antidenormalnoise:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block_antidenormalnoise::<NQUADS>(ptr)?;

        debug!("After clear_block_antidenormalnoise:");

        for i in (0..size).step_by(8) {

            debug!("value at {}: {}", i, *ptr.add(i));
            debug!("value at {}: {}", i + 1, *ptr.add(i + 1));
            debug!("value at {}: {}", i + 2, *ptr.add(i + 2));
            debug!("value at {}: {}", i + 3, *ptr.add(i + 3));
            debug!("value at {}: {}", i + 4, *ptr.add(i + 4));
            debug!("value at {}: {}", i + 5, *ptr.add(i + 5));
            debug!("value at {}: {}", i + 6, *ptr.add(i + 6));
            debug!("value at {}: {}", i + 7, *ptr.add(i + 7));

            assert_eq!(*ptr.add(i), 0.000000000000001);
            assert_eq!(*ptr.add(i + 1), 0.000000000000001);
            assert_eq!(*ptr.add(i + 2), -0.000000000000001);
            assert_eq!(*ptr.add(i + 3), -0.000000000000001);
            assert_eq!(*ptr.add(i + 4), -0.000000000000001);
            assert_eq!(*ptr.add(i + 5), -0.000000000000001);
            assert_eq!(*ptr.add(i + 6), 0.000000000000001);
            assert_eq!(*ptr.add(i + 7), 0.000000000000001);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_smallest_nquads() 
    -> Result<(), AlignmentError> 
{
    const NQUADS: usize = 1;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr, size);

        debug!("Before clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block::<NQUADS>(ptr)?;

        debug!("After clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
            assert_eq!(*ptr.add(i), 0.0);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_antidenormalnoise_smallest_nquads() 
    -> Result<(), AlignmentError> 
{
    println!("running minimal clear_block_antidenormalnoise");

    const NQUADS: usize = 1;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr, size);

        println!("Before clear_block_antidenormalnoise:");

        for i in 0..size {
            println!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block_antidenormalnoise::<NQUADS>(ptr)?;

        println!("After clear_block_antidenormalnoise:");

        println!("value at {}: {}", 0, *ptr.add(0));
        println!("value at {}: {}", 1, *ptr.add(1));
        println!("value at {}: {}", 2, *ptr.add(2));
        println!("value at {}: {}", 3, *ptr.add(3));
        println!("value at {}: {}", 4, *ptr.add(4));
        println!("value at {}: {}", 5, *ptr.add(5));
        println!("value at {}: {}", 6, *ptr.add(6));
        println!("value at {}: {}", 7, *ptr.add(7));

        assert_eq!(*ptr.add(0),  0.000000000000001);
        assert_eq!(*ptr.add(1),  0.000000000000001);
        assert_eq!(*ptr.add(2), -0.000000000000001);
        assert_eq!(*ptr.add(3), -0.000000000000001);
        assert_eq!(*ptr.add(4), -0.000000000000001);
        assert_eq!(*ptr.add(5), -0.000000000000001);
        assert_eq!(*ptr.add(6),  0.000000000000001);
        assert_eq!(*ptr.add(7),  0.000000000000001);

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_random_values() 
    -> Result<(), AlignmentError> 
{
    const NQUADS: usize = 4;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr, size);

        debug!("Before clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block::<NQUADS>(ptr)?;

        debug!("After clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
            assert_eq!(*ptr.add(i), 0.0);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_antidenormalnoise_random_values() 
    -> Result<(), AlignmentError> 
{
    const NQUADS: usize = 4;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr, size);

        debug!("Before clear_block_antidenormalnoise:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block_antidenormalnoise::<NQUADS>(ptr)?;

        debug!("After clear_block_antidenormalnoise:");

        for i in (0..size).step_by(8) {
            debug!("value at {}: {}", i, *ptr.add(i));
            debug!("value at {}: {}", i + 1, *ptr.add(i + 1));
            debug!("value at {}: {}", i + 2, *ptr.add(i + 2));
            debug!("value at {}: {}", i + 3, *ptr.add(i + 3));
            debug!("value at {}: {}", i + 4, *ptr.add(i + 4));
            debug!("value at {}: {}", i + 5, *ptr.add(i + 5));
            debug!("value at {}: {}", i + 6, *ptr.add(i + 6));
            debug!("value at {}: {}", i + 7, *ptr.add(i + 7));

            assert_eq!(*ptr.add(i), 0.000000000000001);
            assert_eq!(*ptr.add(i + 1), 0.000000000000001);
            assert_eq!(*ptr.add(i + 2), -0.000000000000001);
            assert_eq!(*ptr.add(i + 3), -0.000000000000001);
            assert_eq!(*ptr.add(i + 4), -0.000000000000001);
            assert_eq!(*ptr.add(i + 5), -0.000000000000001);
            assert_eq!(*ptr.add(i + 6), 0.000000000000001);
            assert_eq!(*ptr.add(i + 7), 0.000000000000001);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_large_nquads() -> Result<(), AlignmentError> {

    const NQUADS: usize = 1024;

    unsafe {
        let size = NQUADS * 4;
        let ptr = allocate_aligned_memory(size);
        fill_memory_with_random_values(ptr, size);

        debug!("Before clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block::<NQUADS>(ptr)?;

        debug!("After clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
            assert_eq!(*ptr.add(i), 0.0);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_antidenormalnoise_large_nquads() -> Result<(), AlignmentError> {

    const NQUADS: usize = 1024;

    unsafe {

        let size = NQUADS * 4;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr, size);

        debug!("Before clear_block_antidenormalnoise:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        clear_block_antidenormalnoise::<NQUADS>(ptr)?;

        debug!("After clear_block_antidenormalnoise:");

        for i in (0..size).step_by(8) {
            debug!("value at {}: {}", i + 0, *ptr.add(i + 0));
            debug!("value at {}: {}", i + 1, *ptr.add(i + 1));
            debug!("value at {}: {}", i + 2, *ptr.add(i + 2));
            debug!("value at {}: {}", i + 3, *ptr.add(i + 3));
            debug!("value at {}: {}", i + 4, *ptr.add(i + 4));
            debug!("value at {}: {}", i + 5, *ptr.add(i + 5));
            debug!("value at {}: {}", i + 6, *ptr.add(i + 6));
            debug!("value at {}: {}", i + 7, *ptr.add(i + 7));

            assert_eq!(*ptr.add(i + 0),  0.000000000000001);
            assert_eq!(*ptr.add(i + 1),  0.000000000000001);
            assert_eq!(*ptr.add(i + 2), -0.000000000000001);
            assert_eq!(*ptr.add(i + 3), -0.000000000000001);
            assert_eq!(*ptr.add(i + 4), -0.000000000000001);
            assert_eq!(*ptr.add(i + 5), -0.000000000000001);
            assert_eq!(*ptr.add(i + 6),  0.000000000000001);
            assert_eq!(*ptr.add(i + 7),  0.000000000000001);
        }

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_unaligned_memory() -> Result<(), AlignmentError> {

    const NQUADS: usize = 4;

    unsafe {

        let size = NQUADS * 4 + 1;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr.add(1), size - 1);

        debug!("Before clear_block:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        let result = clear_block::<NQUADS>(ptr.add(1));

        assert!(result.is_err());

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}

#[traced_test]
fn test_clear_block_antidenormalnoise_unaligned_memory() 
    -> Result<(), AlignmentError> 
{
    const NQUADS: usize = 4;

    unsafe {

        let size = NQUADS * 4 + 1;
        let ptr  = allocate_aligned_memory(size);

        fill_memory_with_random_values(ptr.add(1), size - 1);

        debug!("Before clear_block_antidenormalnoise:");

        for i in 0..size {
            debug!("value at {}: {}", i, *ptr.add(i));
        }

        let result = clear_block_antidenormalnoise::<NQUADS>(ptr.add(1));

        assert!(result.is_err());

        deallocate_aligned_memory(ptr, size);
    }

    Ok(())
}
