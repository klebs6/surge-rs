ix!();

use crate::{
    Chorus,
    ChorusParam
};

impl Chorus<'sr> {

    #[inline] pub fn new_time_lag() -> A1d::<Lag::<f32>> {
        A1d::<Lag::<f32>>::from_elem(CHORUS_DEPTH, Lag::<f32>::default())
    }

    #[inline] pub fn new_voicepan_quad() -> A1d::<__m128> {
        unsafe {
            A1d::<__m128>::from_elem(CHORUS_DEPTH, z128![])
        }
    }

    #[inline] pub fn new_voicepan() -> A2d::<f32> {
        A2d::<f32>::from_elem((CHORUS_DEPTH,2), 0.0)
    }

    #[inline] pub fn new_chorus_buffer() -> A1d::<f32> {
        A1d::<f32>::from_elem(CHORUS_MAX_DELAY_LENGTH + FIR_IPOL_N, 0.0)
    }

    #[inline] pub fn new_lfo_phase() -> A1d::<f64> {
        A1d::<f64>::from_elem(CHORUS_DEPTH, 0.0)
    }

    pub fn new<const N: usize>( 
        tuner:     &'sr TunerHandle<'sr>,
        tables:    &'sr TablesHandle<'sr>,
        srunit:    &'sr SampleRateHandle<'sr>,
        time_unit: &'sr TimeUnitHandle<'sr>) -> Self 
    {
        let lp = BiquadFilter::new(tuner,tables,srunit);
        let hp = BiquadFilter::new(tuner,tables,srunit);

        let mut x = Self {
            feedback:        Align16(Box::new(LipolPs::default())),
            mix:             Align16(Box::new(LipolPs::default())),
            width:           Align16(Box::new(LipolPs::default())),
            voicepan_l4:     Align16(Self::new_voicepan_quad()),
            voicepan_r4:     Align16(Self::new_voicepan_quad()),
            buffer:          Align16(Self::new_chorus_buffer()),
            ringout:         Ringout::blocks(RINGOUT_DEFAULT),
            params:          ChorusParam::new_runtime(),
            time:            Self::new_time_lag(),
            voicepan:        Self::new_voicepan(),
            envf:            0.0,
            wpos:            0,
            lp,
            hp,
            lfophase:        Self::new_lfo_phase(),
            tables:          tables.clone(),
            tuner:           tuner.clone(),
            time_unit:       time_unit.clone(),
            srunit:          srunit.clone(),

        };
        x.mix.set_blocksize(N as i32);
        x.feedback.set_blocksize(N as i32);
        x
    }
}
