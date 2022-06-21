crate::ix!();

impl Reverb {

    pub fn new( 
        tuner:     & TunerHandle,
        tables:    & TablesHandle,
        srunit:    & SampleRateHandle,
        time_unit: & TimeUnitHandle) -> Self {

        Self {
            delay_pan_l:     Align16(Self::new_1d::<f32>(REVERB_TAPS)),
            delay_pan_r:     Align16(Self::new_1d::<f32>(REVERB_TAPS)),
            delay_fb:        Align16(Self::new_1d::<f32>(REVERB_TAPS)),
            delay:           Align16(Self::new_1d::<f32>(REVERB_TAPS * REVERB_MAX_DELAY)),
            out_tap:         Align16(Self::new_1d::<f32>(REVERB_TAPS)),
            predelay:        Align16(Self::new_1d::<f32>(REVERB_MAX_DELAY)),
            delay_time:      Align16(Self::new_1d::<usize>(REVERB_TAPS)),
            mix:             Align16(LipolPs::default()),
            width:           Align16(LipolPs::default()),
            ringout:         Ringout::blocks(10000000),
            params:          ReverbParam::new_runtime(),
            delay_pos:       0,
            modphase:        0.0,
            preset:          ReverbPreset::A,
            lastf:           Self::new_1d::<f32>(N_FX_PARAMS),
            band1:           BiquadFilter::new(tuner,tables,srunit),
            locut:           BiquadFilter::new(tuner,tables,srunit),
            hicut:           BiquadFilter::new(tuner,tables,srunit),
            b:               0,
            time_unit:       time_unit.clone(),
            tables:          tables.clone(),
            tuner:           tuner.clone(),
            srunit:          srunit.clone(),
        }
    }

    #[inline] pub fn new_1d<T: Zero + Clone>(len: usize) -> A1d::<T> {
        A1d::<T>::zeros(len)
    }
}
