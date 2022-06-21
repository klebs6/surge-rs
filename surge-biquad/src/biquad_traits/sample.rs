pub trait ProcessSample {
    fn process_sample(
        &mut self, 
        input: f32) -> f32;
}

pub trait ProcessSampleStereo {
    fn process_sample_stereo(
        &mut self, 
        l: f32, 
        r: f32, 
        l_out: &mut f32, 
        r_out: &mut f32);
}

pub trait ProcessSampleNolag {
    fn process_sample_nolag(
        &mut self, 
        l: &mut f32, 
        r: &mut f32);
}

pub trait ProcessSampleStereoNolag {
    fn process_sample_stereo_nolag(
        &mut self, 
        l: &mut f32, 
        r: &mut f32, 
        l_out: &mut f32, 
        r_out: &mut f32);
}

pub trait ProcessSampleNolagNoinput {
    fn process_sample_nolag_noinput(
        &mut self, 
        l_out: &mut f32, 
        r_out: &mut f32);
}
