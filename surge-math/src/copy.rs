ix!();

fn access_f32<T>(x: *mut T) -> *mut f32 {
    x as *mut f32
}

// copy block (requires aligned data)
pub fn copy_block<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: Debug, 
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

// copy block (unaligned source)
pub fn copy_block_unaligned_source<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: Debug, 
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

// copy block (unaligned destination)
pub fn copy_block_unaligned_destination<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: Debug, 
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

// copy block (unaligned source + destination)
pub fn copy_block_unaligned_src_and_dst<NQ>(
    src:    *mut f32, 
    dst:    *mut f32, 
    nquads: NQ) 

    where 
    <NQ as TryInto<usize>>::Error: Debug, 
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
