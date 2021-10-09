ix!();

pub trait Redo {
    fn redo(&mut self);
}

pub trait Named {
    const NAME: &'static str;
}

pub trait MemcpyFromOther {
    fn memcpy_from_other(&mut self, 
        other: *mut Self) ;
}

pub trait UpdateDisplay {
    fn update_display(&mut self);
}

pub trait AllowDisplay {
    fn allow_display(&self) 
        -> bool { true }
}

pub trait PlotMagnitude {
    fn plot_magnitude(&self, 
        f: f32) 
        -> f32;
}

pub trait SetBlockSize {
    fn set_blocksize(&mut self, 
        bs: i32); 
}

pub trait SetSampleRate {
    fn set_samplerate(&mut self, 
        sr: f32);
}

pub trait SetPitch {
    fn set_pitch(&mut self, _pitch: f32, _is_display: bool) {}
}

pub trait CoefficientLoadStore {
    fn load_coefficients(&mut self);

    ///# Safety
    ///
    ///need to be able to safely access N contiguous elements from both a and b, however
    ///far your implementation reads
    unsafe fn store_coefficients(&mut self, 
        coefficient_a: *mut f64, 
        coefficient_b: *mut f64);
}

