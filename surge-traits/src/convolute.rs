pub struct ConvolutionCfg {
    pub voice: usize, 
    pub stereo: bool, 
    pub fm: bool
}

pub trait Convolute {
    type ConvoluteArgs = ConvolutionCfg;
    fn convolute(&mut self, cfg: Self::ConvoluteArgs);
}
