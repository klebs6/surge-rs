pub trait GetVuFalloff {
    fn vu_falloff(&self) -> f32;
}

pub trait Ms2Samples {
    fn ms_2_samples(&self, ms: f32, scale: f32) -> usize;
}

pub trait GetSampleRate {

    fn dsamplerate_os(&self)      -> f64;
    fn dsamplerate(&self)         -> f64;
    fn samplerate_os(&self)       -> f32;
    fn samplerate(&self)          -> f32;
    fn dsamplerate_os_inv(&self)  -> f64;
    fn dsamplerate_inv(&self)     -> f64;
    fn samplerate_os_inv(&self)   -> f32;
    fn samplerate_inv(&self)      -> f32;
}

pub trait SetSampleRate {
    fn set_samplerate(&self, sr: f64);
}
