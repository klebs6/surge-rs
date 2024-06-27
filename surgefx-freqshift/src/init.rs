crate::ix!();

impl Initialize for FreqShift {

    fn init(&mut self) -> Result<(),SurgeError> {

        self.buffer = Self::new_buffer();

        self.wpos = 0;
        self.fr.reset();
        self.fi.reset();
        self.ringout = Ringout::blocks(10000000);

        self.update();

        self.inithadtempo = true;

        // See issue #1444 and the fix for this stuff
        if self.time_unit.temposyncratio_inv() == 0.0 {
            self.inithadtempo = false;
        }

        Ok(())
    }
}

impl FreqShift {

    pub fn new(
        tuner:     & TunerHandle,
        tables:    & TablesHandle,
        srunit:    & SampleRateHandle,
        time_unit: & TimeUnitHandle) -> Result<Self,SurgeError> {

        let mut x = Self {
            fr:             Align16(HalfRateFilterSSE::new(6, true)),
            fi:             Align16(HalfRateFilterSSE::new(6, true)),
            mix:            Align16(LipolPs::default()),
            ringout:        Ringout::blocks(10000000),
            params:         FreqShiftParam::new_runtime(), 
            feedback:       LiPol::<f32>::new(BLOCK_SIZE),
            time:           Lag::<f32>::new(0.0001),
            shift_l:        Lag::<f32>::new(0.01),
            shift_r:        Lag::<f32>::new(0.01),
            inithadtempo:   false,
            buffer:         Self::new_buffer(),
            wpos:           0,
            o1_l:           QuadrOsc::default(),
            o2_l:           QuadrOsc::default(),
            o1_r:           QuadrOsc::default(),
            o2_r:           QuadrOsc::default(),
            time_unit:      time_unit.clone(),
            tables:         tables.clone(),
            tuner:          tuner.clone(),
            srunit:         srunit.clone(),
        };
        x.init()?;
        Ok(x)
    }

    #[inline] fn new_buffer() -> A2d::<f32> {
        A2d::<f32>::zeros((2,FREQSHIFT_MAX_DELAY_LENGTH))
    }
}

impl Suspend for FreqShift {
    fn suspend(&mut self) -> Result<(),SurgeError> {
        self.init()?;
        self.ringout = Ringout::blocks(10000000);
        Ok(())
    }
}
