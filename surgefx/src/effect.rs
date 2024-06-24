crate::ix!();

///probably better to use something like the crate
///enum_dispatch for this, but how?
#[derive(Debug)]
pub enum SurgeEffect {
    Eq3Band(Box<Eq3Band>),
    Distortion(Box<Distortion>),
    Conditioner(Box<Conditioner>),
    AllpassVerb(Box<AllpassVerb>),
    DualDelay(Box<DualDelay>),
    Flanger(Box<Flanger>),
    Phaser(Box<Phaser>),
    Reverb(Box<Reverb>),
    Chorus(Box<Chorus>),
    Emphasize(Box<Emphasize>),
    FreqShift(Box<FreqShift>),
    RingModulator(Box<RingModulator>),
    RotarySpeaker(Box<RotarySpeaker>),
    Vocoder(Box<Vocoder>),
}

impl Reset for SurgeEffect { 

    fn reset(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.reset(),
            SurgeEffect::Distortion(x)    => x.reset(),
            SurgeEffect::Conditioner(x)   => x.reset(),
            SurgeEffect::AllpassVerb(x)   => x.reset(),
            SurgeEffect::DualDelay(x)     => x.reset(),
            SurgeEffect::Flanger(x)       => x.reset(),
            SurgeEffect::Phaser(x)        => x.reset(),
            SurgeEffect::Reverb(x)        => x.reset(),
            SurgeEffect::Chorus(x)        => x.reset(),
            SurgeEffect::Emphasize(x)     => x.reset(),
            SurgeEffect::FreqShift(x)     => x.reset(),
            SurgeEffect::RingModulator(x) => x.reset(),
            SurgeEffect::RotarySpeaker(x) => x.reset(),
            SurgeEffect::Vocoder(x)       => x.reset(),
        }
    }
}

impl Effect for SurgeEffect { }

impl StereoProcess for SurgeEffect { 

    fn stereo_process<const N: usize>(
        &mut self, 
        data_l: &mut [f32; N], 
        data_r: &mut [f32; N]

    ) -> Result<(),AlignmentError> {

        match self {
            SurgeEffect::Eq3Band(x)       => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Distortion(x)    => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Conditioner(x)   => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::AllpassVerb(x)   => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::DualDelay(x)     => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Flanger(x)       => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Phaser(x)        => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Reverb(x)        => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Chorus(x)        => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Emphasize(x)     => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::FreqShift(x)     => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::RingModulator(x) => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::RotarySpeaker(x) => x.stereo_process::<N>(data_l, data_r)?,
            SurgeEffect::Vocoder(x)       => x.stereo_process::<N>(data_l, data_r)?,
        }

        Ok(())
    }
}

impl ProcessOnlyControl for SurgeEffect { 
    fn process_only_control<const N: usize>(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.process_only_control::<N>(),
            SurgeEffect::Distortion(x)    => x.process_only_control::<N>(),
            SurgeEffect::Conditioner(x)   => x.process_only_control::<N>(),
            SurgeEffect::AllpassVerb(x)   => x.process_only_control::<N>(),
            SurgeEffect::DualDelay(x)     => x.process_only_control::<N>(),
            SurgeEffect::Flanger(x)       => x.process_only_control::<N>(),
            SurgeEffect::Phaser(x)        => x.process_only_control::<N>(),
            SurgeEffect::Reverb(x)        => x.process_only_control::<N>(),
            SurgeEffect::Chorus(x)        => x.process_only_control::<N>(),
            SurgeEffect::Emphasize(x)     => x.process_only_control::<N>(),
            SurgeEffect::FreqShift(x)     => x.process_only_control::<N>(),
            SurgeEffect::RingModulator(x) => x.process_only_control::<N>(),
            SurgeEffect::RotarySpeaker(x) => x.process_only_control::<N>(),
            SurgeEffect::Vocoder(x)       => x.process_only_control::<N>(),
        }
    }
}

impl GetRingout for SurgeEffect { 

    fn get_ringout(&self)         -> Ringout {
        match self {
            SurgeEffect::Eq3Band(x)       => x.get_ringout(),
            SurgeEffect::Distortion(x)    => x.get_ringout(),
            SurgeEffect::Conditioner(x)   => x.get_ringout(),
            SurgeEffect::AllpassVerb(x)   => x.get_ringout(),
            SurgeEffect::DualDelay(x)     => x.get_ringout(),
            SurgeEffect::Flanger(x)       => x.get_ringout(),
            SurgeEffect::Phaser(x)        => x.get_ringout(),
            SurgeEffect::Reverb(x)        => x.get_ringout(),
            SurgeEffect::Chorus(x)        => x.get_ringout(),
            SurgeEffect::Emphasize(x)     => x.get_ringout(),
            SurgeEffect::FreqShift(x)     => x.get_ringout(),
            SurgeEffect::RingModulator(x) => x.get_ringout(),
            SurgeEffect::RotarySpeaker(x) => x.get_ringout(),
            SurgeEffect::Vocoder(x)       => x.get_ringout(),
        }
    }

    fn get_ringout_counter(&self) -> NumberOfBlocks {
        match self {
            SurgeEffect::Eq3Band(x)       => x.get_ringout_counter(),
            SurgeEffect::Distortion(x)    => x.get_ringout_counter(),
            SurgeEffect::Conditioner(x)   => x.get_ringout_counter(),
            SurgeEffect::AllpassVerb(x)   => x.get_ringout_counter(),
            SurgeEffect::DualDelay(x)     => x.get_ringout_counter(),
            SurgeEffect::Flanger(x)       => x.get_ringout_counter(),
            SurgeEffect::Phaser(x)        => x.get_ringout_counter(),
            SurgeEffect::Reverb(x)        => x.get_ringout_counter(),
            SurgeEffect::Chorus(x)        => x.get_ringout_counter(),
            SurgeEffect::Emphasize(x)     => x.get_ringout_counter(),
            SurgeEffect::FreqShift(x)     => x.get_ringout_counter(),
            SurgeEffect::RingModulator(x) => x.get_ringout_counter(),
            SurgeEffect::RotarySpeaker(x) => x.get_ringout_counter(),
            SurgeEffect::Vocoder(x)       => x.get_ringout_counter(),
        }
    }
}

impl SetRingout for SurgeEffect { 

    fn ringout_counter_incr(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.ringout_counter_incr(),
            SurgeEffect::Distortion(x)    => x.ringout_counter_incr(),
            SurgeEffect::Conditioner(x)   => x.ringout_counter_incr(),
            SurgeEffect::AllpassVerb(x)   => x.ringout_counter_incr(),
            SurgeEffect::DualDelay(x)     => x.ringout_counter_incr(),
            SurgeEffect::Flanger(x)       => x.ringout_counter_incr(),
            SurgeEffect::Phaser(x)        => x.ringout_counter_incr(),
            SurgeEffect::Reverb(x)        => x.ringout_counter_incr(),
            SurgeEffect::Chorus(x)        => x.ringout_counter_incr(),
            SurgeEffect::Emphasize(x)     => x.ringout_counter_incr(),
            SurgeEffect::FreqShift(x)     => x.ringout_counter_incr(),
            SurgeEffect::RingModulator(x) => x.ringout_counter_incr(),
            SurgeEffect::RotarySpeaker(x) => x.ringout_counter_incr(),
            SurgeEffect::Vocoder(x)       => x.ringout_counter_incr(),
        }
    }

    fn ringout_counter_reset(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.ringout_counter_reset(),
            SurgeEffect::Distortion(x)    => x.ringout_counter_reset(),
            SurgeEffect::Conditioner(x)   => x.ringout_counter_reset(),
            SurgeEffect::AllpassVerb(x)   => x.ringout_counter_reset(),
            SurgeEffect::DualDelay(x)     => x.ringout_counter_reset(),
            SurgeEffect::Flanger(x)       => x.ringout_counter_reset(),
            SurgeEffect::Phaser(x)        => x.ringout_counter_reset(),
            SurgeEffect::Reverb(x)        => x.ringout_counter_reset(),
            SurgeEffect::Chorus(x)        => x.ringout_counter_reset(),
            SurgeEffect::Emphasize(x)     => x.ringout_counter_reset(),
            SurgeEffect::FreqShift(x)     => x.ringout_counter_reset(),
            SurgeEffect::RingModulator(x) => x.ringout_counter_reset(),
            SurgeEffect::RotarySpeaker(x) => x.ringout_counter_reset(),
            SurgeEffect::Vocoder(x)       => x.ringout_counter_reset(),
        }
    }
}

impl ProcessRingout for SurgeEffect { 

    unsafe fn process_ringout<const N: usize>(
        &mut self, 
        data_l: *mut f32, 
        data_r: *mut f32, 
        indata_present: bool

    ) -> Result<OutputDataPresent,AlignmentError> {

        match self {
            SurgeEffect::Eq3Band(x)       => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Distortion(x)    => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Conditioner(x)   => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::AllpassVerb(x)   => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::DualDelay(x)     => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Flanger(x)       => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Phaser(x)        => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Reverb(x)        => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Chorus(x)        => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Emphasize(x)     => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::FreqShift(x)     => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::RingModulator(x) => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::RotarySpeaker(x) => x.process_ringout::<N>(data_l, data_r, indata_present),
            SurgeEffect::Vocoder(x)       => x.process_ringout::<N>(data_l, data_r, indata_present),
        }
    }
}

impl Suspend for SurgeEffect { 
    fn suspend(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.suspend(),
            SurgeEffect::Distortion(x)    => x.suspend(),
            SurgeEffect::Conditioner(x)   => x.suspend(),
            SurgeEffect::AllpassVerb(x)   => x.suspend(),
            SurgeEffect::DualDelay(x)     => x.suspend(),
            SurgeEffect::Flanger(x)       => x.suspend(),
            SurgeEffect::Phaser(x)        => x.suspend(),
            SurgeEffect::Reverb(x)        => x.suspend(),
            SurgeEffect::Chorus(x)        => x.suspend(),
            SurgeEffect::Emphasize(x)     => x.suspend(),
            SurgeEffect::FreqShift(x)     => x.suspend(),
            SurgeEffect::RingModulator(x) => x.suspend(),
            SurgeEffect::RotarySpeaker(x) => x.suspend(),
            SurgeEffect::Vocoder(x)       => x.suspend(),
        }
    }
}

impl Initialize for SurgeEffect {
    fn init(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.init(),
            SurgeEffect::Distortion(x)    => x.init(),
            SurgeEffect::Conditioner(x)   => x.init(),
            SurgeEffect::AllpassVerb(x)   => x.init(),
            SurgeEffect::DualDelay(x)     => x.init(),
            SurgeEffect::Flanger(x)       => x.init(),
            SurgeEffect::Phaser(x)        => x.init(),
            SurgeEffect::Reverb(x)        => x.init(),
            SurgeEffect::Chorus(x)        => x.init(),
            SurgeEffect::Emphasize(x)     => x.init(),
            SurgeEffect::FreqShift(x)     => x.init(),
            SurgeEffect::RingModulator(x) => x.init(),
            SurgeEffect::RotarySpeaker(x) => x.init(),
            SurgeEffect::Vocoder(x)       => x.init(),
        }
    }
}

impl Update for SurgeEffect {
    fn update(&mut self) {
        match self {
            SurgeEffect::Eq3Band(x)       => x.update(),
            SurgeEffect::Distortion(x)    => x.update(),
            SurgeEffect::Conditioner(x)   => x.update(),
            SurgeEffect::AllpassVerb(x)   => x.update(),
            SurgeEffect::DualDelay(x)     => x.update(),
            SurgeEffect::Flanger(x)       => x.update(),
            SurgeEffect::Phaser(x)        => x.update(),
            SurgeEffect::Reverb(x)        => x.update(),
            SurgeEffect::Chorus(x)        => x.update(),
            SurgeEffect::Emphasize(x)     => x.update(),
            SurgeEffect::FreqShift(x)     => x.update(),
            SurgeEffect::RingModulator(x) => x.update(),
            SurgeEffect::RotarySpeaker(x) => x.update(),
            SurgeEffect::Vocoder(x)       => x.update(),
        }
    }
}

impl ClearBuffers for SurgeEffect {
    fn clear_buffers(&mut self) {
        match self {
            SurgeEffect::Eq3Band(_x)       => {},
            SurgeEffect::Distortion(_x)    => {},
            SurgeEffect::Conditioner(_x)   => {},
            SurgeEffect::AllpassVerb(_x)   => {},
            SurgeEffect::DualDelay(_x)     => {},
            SurgeEffect::Flanger(_x)       => {},
            SurgeEffect::Phaser(_x)        => {},
            SurgeEffect::Reverb(_x)        => {},
            SurgeEffect::Chorus(_x)        => {},
            SurgeEffect::Emphasize(_x)     => {},
            SurgeEffect::FreqShift(_x)     => {},
            SurgeEffect::RingModulator(_x) => {},
            SurgeEffect::RotarySpeaker(_x) => {},
            SurgeEffect::Vocoder(_x)       => {},
        }
    }
}

impl GetReturnLevel for SurgeEffect {
    fn returnlevel(&self) -> f32 {
        match self {
            SurgeEffect::Eq3Band(x)       => x.returnlevel(),
            SurgeEffect::Distortion(x)    => x.returnlevel(),
            SurgeEffect::Conditioner(x)   => x.returnlevel(),
            SurgeEffect::AllpassVerb(x)   => x.returnlevel(),
            SurgeEffect::DualDelay(x)     => x.returnlevel(),
            SurgeEffect::Flanger(x)       => x.returnlevel(),
            SurgeEffect::Phaser(x)        => x.returnlevel(),
            SurgeEffect::Reverb(x)        => x.returnlevel(),
            SurgeEffect::Chorus(x)        => x.returnlevel(),
            SurgeEffect::Emphasize(x)     => x.returnlevel(),
            SurgeEffect::FreqShift(x)     => x.returnlevel(),
            SurgeEffect::RingModulator(x) => x.returnlevel(),
            SurgeEffect::RotarySpeaker(x) => x.returnlevel(),
            SurgeEffect::Vocoder(x)       => x.returnlevel(),
        }
    }
}
