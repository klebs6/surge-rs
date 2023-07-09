crate::ix!();

/// Accesses the given pointer as a mutable f32 pointer.
///
/// # Arguments
///
/// * `x` - A pointer to be cast to a mutable f32 pointer.
fn access_f32<T>(x: *mut T) -> *mut f32 {
    x as *mut f32
}

/// Copies a block of memory between two aligned f32 arrays.
///
/// # Safety
///
/// Both `src` and `dst` must be valid and point to aligned contiguous memory regions.
///
/// # Arguments
///
/// * `src` - A pointer to the source f32 array.
/// * `dst` - A pointer to the destination f32 array.
/// * `nquads` - The number of quads (groups of four f32 values) to copy.
///
/// # Panics
///
/// Panics if `nquads` cannot be converted into a usize.
pub fn copy_block<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: fmt::Debug, 
    NQ: TryInto<usize>
{
   let nquads: usize = nquads.try_into().unwrap();

   let fdst = access_f32(dst);
   let fsrc = access_f32(src);

    let store = |idx: usize| {
        unsafe {
            _mm_store_ps(fdst.add(idx), _mm_load_ps(fsrc.add(idx)));
        }
    };

   for i in (0..(nquads << 2)).step_by(8 << 2) 
   {
        store(i);
        store(i + 4);
        store(i + 8);
        store(i + 12);
        store(i + 16);
        store(i + 20);
        store(i + 24);
        store(i + 28);
   }
}

/// Copies a block of memory between two f32 arrays with an unaligned source.
///
/// # Safety
///
/// Both `src` and `dst` must be valid and point to contiguous memory regions.
///
/// # Arguments
///
/// * `src` - A pointer to the unaligned source f32 array.
/// * `dst` - A pointer to the destination f32 array.
/// * `nquads` - The number of quads (groups of four f32 values) to copy.
///
/// # Panics
///
/// Panics if `nquads` cannot be converted into a usize.
pub fn copy_block_unaligned_source<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: fmt::Debug, 
    NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let fdst = access_f32(dst);
    let fsrc = access_f32(src);

    let storeu = |idx: usize| {
        unsafe {
            _mm_store_ps(fdst.add(idx), _mm_loadu_ps(fsrc.add(idx)));
        }
    };

    for i in (0..(nquads << 2)).step_by(8 << 2) 
        {
            storeu(i);
            storeu(i + 4);
            storeu(i + 8);
            storeu(i + 12);
            storeu(i + 16);
            storeu(i + 20);
            storeu(i + 24);
            storeu(i + 28);
        }
}

/// Copies a block of memory between two f32 arrays with an unaligned destination.
///
/// # Safety
///
/// Both `src` and `dst` must be valid and point to contiguous memory regions.
///
/// # Arguments
///
/// * `src` - A pointer to the source f32 array.
/// * `dst` - A pointer to the unaligned destination f32 array.
/// * `nquads` - The number of quads (groups of four f32 values) to copy.
///
/// # Panics
///
/// Panics if `nquads` cannot be converted into a usize.
pub fn copy_block_unaligned_destination<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: fmt::Debug, 
    NQ: TryInto<usize>
{
    let nquads: usize = nquads.try_into().unwrap();

    let fdst = access_f32(dst);
    let fsrc = access_f32(src);

    let ustore = |idx: usize| {
        unsafe {
            _mm_storeu_ps(fdst.add(idx), _mm_load_ps(fsrc.add(idx)));
        }
    };

    for i in (0..(nquads << 2)).step_by(8 << 2) 
    {
        ustore(i);
        ustore(i + 4);
        ustore(i + 8);
        ustore(i + 12);
        ustore(i + 16);
        ustore(i + 20);
        ustore(i + 24);
        ustore(i + 28);
    }
}

/// Copies a block of memory between two unaligned f32 arrays.
///
/// # Safety
///
/// Both `src` and `dst` must be valid and point to contiguous memory regions.
///
/// # Arguments
///
/// * `src` - A pointer to the unaligned source f32 array.
/// * `dst` - A pointer to the unaligned destination f32 array.
/// * `nquads` - The number of quads (groups of four f32 values) to copy.
///
/// # Panics
///
/// Panics if `nquads` cannot be converted into a usize.
pub fn copy_block_unaligned_src_and_dst<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: fmt::Debug, 
    NQ: TryInto<usize>

{
    let nquads: usize = nquads.try_into().unwrap();

    let fdst = access_f32(dst);
    let fsrc = access_f32(src);

    let ustoreu = |idx: usize| {
        unsafe {
            _mm_storeu_ps(fdst.add(idx), _mm_loadu_ps(fsrc.add(idx)));
        }
    };

    for i in (0..(nquads << 2)).step_by(8 << 2) 
    {
        ustoreu(i);
        ustoreu(i + 4);
        ustoreu(i + 8);
        ustoreu(i + 12);
        ustoreu(i + 16);
        ustoreu(i + 20);
        ustoreu(i + 24);
        ustoreu(i + 28);
    }
}
