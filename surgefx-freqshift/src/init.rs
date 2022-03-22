ix!();

use crate::{
    FreqShift,
    FreqShiftParam,
};

impl Init for FreqShift {

    fn init(&mut self) {

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
    }
}

impl FreqShift {

    pub fn new(
        tuner:     & TunerHandle,
        tables:    & TablesHandle,
        srunit:    & SampleRateHandle,
        time_unit: & TimeUnitHandle) -> Self {

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
            o1_l:           QuadrOsc::new(),
            o2_l:           QuadrOsc::new(),
            o1_r:           QuadrOsc::new(),
            o2_r:           QuadrOsc::new(),
            time_unit:      time_unit.clone(),
            tables:         tables.clone(),
            tuner:          tuner.clone(),
            srunit:         srunit.clone(),
        };
        x.init();
        x
    }

    #[inline] fn new_buffer() -> A2d::<f32> {
        A2d::<f32>::zeros((2,FREQSHIFT_MAX_DELAY_LENGTH))
    }
}

impl Suspend for FreqShift {
    fn suspend(&mut self) {
        self.init();
        self.ringout = Ringout::blocks(10000000);
    }
}
