ix!();

pub const HUOVILAINEN_GAIN_COMPENSATION:    f32 = 0.5;
pub const HUOVILAINEN_EXTRA_OVERSAMPLE:     i32 = 2;
pub const HUOVILAINEN_EXTRA_OVERSAMPLE_INV: f32 = 0.5;

coeffidx![
    HuovilainenCoeff;
    Cutoff, 
    Res, 
    Fc, 
    GainCompensation
];

coeffidx![
    HuovilainenRegOffsets;
    Stage,
    Unused1,
    Unused2,
    Unused3,
    StageTanh,
    Unused5,
    Unused6,
    Delay
];

pub type C = HuovilainenCoeff;
pub type R = HuovilainenRegOffsets;

/**
** This contains various adaptations of the models found at
**
** https://github.com/ddiakopoulos/MoogLadders/blob/master/src/RKSimulationModel.h
**
** Modifications include
** 1. Modifying to make surge compatible with state mamagenemt
** 2. SSe and so on
** 3. Model specici changes per model
*/
pub struct HuovilainenLadder<'sr> {
    pub tuner:                   TunerHandle<'sr>,
    pub srunit:                  SampleRateHandle<'sr>,
    pub gain_compensation:       Option<f32>,
}

impl HuovilainenLadder<'sr> {

    pub fn clamped_frequency(&self, pitch: f32) -> f32
    {
        let freq = self.tuner.n2p::<f32,true>( pitch + 69.0 ) * (MIDI_0_FREQ as f32);
        limit_range( freq, 5.0, self.srunit.samplerate_os() * 0.3 )
    }
}
