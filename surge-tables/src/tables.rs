crate::ix!();

#[enum_dispatch]
pub trait NTables {
    fn ntables() -> usize;
}

#[enum_dispatch]
pub trait DbToLinear {
    fn db_to_linear(&self, x: f32) -> f32;
}

#[enum_dispatch]
pub trait ClipScale {
    fn clipscale(&self, freq: f32, subtype: FilterSubType) -> f32;
}

#[enum_dispatch]
pub trait LookupWaveshape {
    fn lookup_waveshape(&self, entry: i32, x: f32) -> f32;
}

#[enum_dispatch]
pub trait LookupWaveshapeWarp {
    fn lookup_waveshape_warp(&self, entry: i32, x: f32) -> f32;
}

#[enum_dispatch]
pub trait EnvelopeRateLpf {
    fn envelope_rate_lpf(&self, x: f32) -> f32;
}

#[enum_dispatch]
pub trait EnvelopeRateLinear {
    fn envelope_rate_linear(&self, x: f32) -> f32;
}

//TODO: is is possible that we can clean up these
//SincTable* traits somehow?
#[enum_dispatch]
pub trait SincTable1X {

    fn _sinctable_1x(&self, idx: usize) -> f32;

    fn sinctable_1x<T: TryInto<usize>>(&self, idx: T) -> f32  
    where 
        <T as TryInto<usize>>::Error: std::fmt::Debug {
            let idx: usize = idx.try_into().unwrap(); 
            self._sinctable_1x(idx)

    }
}

#[enum_dispatch]
pub trait SincTable {

    fn _sinctable(&self, idx: usize) -> f32;

    fn sinctable<T: TryInto<usize>>(&self, idx: T) -> f32  
    where 
        <T as TryInto<usize>>::Error: std::fmt::Debug {
            let idx: usize = idx.try_into().unwrap(); 
            self._sinctable(idx)
    }
}

#[enum_dispatch]
pub trait SincTableI16 {

    fn _sinctable_i16(&self, idx: usize) -> i16;

    fn sinctable_i16<T: TryInto<usize>>(&self, idx: T) -> i16  
    where 
        <T as TryInto<usize>>::Error: std::fmt::Debug {
            let idx: usize = idx.try_into().unwrap(); 
            self._sinctable_i16(idx)

    }
}

#[enum_dispatch]
pub trait SincTable1XPtr {

    fn _sinctable_1x_ptr(&self, idx: usize) -> *const f32;

    fn sinctable_1x_ptr<T: TryInto<usize>>(&self, idx: T) -> *const f32  
    where 
        <T as TryInto<usize>>::Error: std::fmt::Debug {
            let idx: usize = idx.try_into().unwrap(); 
            self._sinctable_1x_ptr(idx)
    }
}

#[enum_dispatch]
pub trait SincTablePtr {

    fn _sinctable_ptr(&self, idx: usize) -> *const f32;

    fn sinctable_ptr<T: TryInto<usize>>(&self, idx: T) -> *const f32  
    where 
        <T as TryInto<usize>>::Error: std::fmt::Debug {
            let idx: usize = idx.try_into().unwrap(); 
            self._sinctable_ptr(idx)
    }
}

#[enum_dispatch]
pub trait SincTableI16Ptr {

    fn _sinctable_i16_ptr(&self, idx: usize) -> *const i16;

    fn sinctable_i16_ptr<T: TryInto<usize>>(&self, idx: T) -> *const i16  
    where 
        <T as TryInto<usize>>::Error: std::fmt::Debug {
            let idx: usize = idx.try_into().unwrap(); 
            self._sinctable_i16_ptr(idx)
    }
}

#[enum_dispatch]
pub trait GetWaveshaperPtr {

    fn _get_waveshaper_ptr(&self, idx: usize, offset: isize) -> *const f32;

    fn get_waveshaper_ptr<T: TryInto<isize>>(&self, idx: usize, _offset: T) -> *const f32  
    where 
        <T as TryInto<isize>>::Error: std::fmt::Debug 
    {
            assert!(idx < WaveshapeTables::ntables());
            let offset: isize = idx.try_into().unwrap(); 
            assert!(offset & 0x3ff == offset);

            self._get_waveshaper_ptr(idx,offset)
    }
}

