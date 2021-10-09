ix!();

pub trait SynthProcess {
    fn process(&mut self);
    fn process_control(&mut self);
    fn process_threadunsafe_operations(&mut self);
}

pub trait LfoProcess {
    fn process(&mut self);
}

#[enum_dispatch]
pub trait Process {
    fn process<const N: usize>(&mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]);
}

#[enum_dispatch]
pub trait ProcessOnlyControl {
    fn process_only_control<const N: usize>(&mut self) { }
}

pub trait ProcessBlockD2 {
    ///process in place, the new block will be half the size
    fn process_block_downsample_by_two(&mut self, 
        l: *mut f32, 
        r: *mut f32, 
        nsamples: Option<usize>, 
        out_l: Option<*mut f32>, 
        out_r: Option<*mut f32>);

}

pub trait ProcessBlockU2 {
    fn process_block_upsample_by_two(&mut self, 
        l_in: *mut f32, 
        r_in: *mut f32, 
        l: *mut f32, 
        r: *mut f32, 
        nsamples: Option<usize>
    );
}

pub trait ProcessBlock {
    fn process_block(&mut self, 
        l: *mut f32, 
        r: *mut f32, 
        nsamples: Option<usize>);
}

