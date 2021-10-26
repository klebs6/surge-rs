ix!();

use crate::{
    DualDelay,
    DualDelayParam,
    DUAL_DELAY_MAX_DELAY_LENGTH,
};

impl DualDelay {

    pub fn instantize_all(&mut self) {
        self.time_l.instantize();
        self.time_r.instantize();
        self.feedback.instantize();
        self.crossfeed.instantize();
        self.mix.instantize();
        self.width.instantize();
        self.pan.instantize();
        self.hp.coeff_instantize();
        self.lp.coeff_instantize();
    }

    pub fn clear_buffers(&mut self) {
        self.buffer = Align16(Self::new_delay_buffer());
    }
}

impl Init for DualDelay {

    fn init(&mut self) {

        self.clear_buffers();

        self.wpos          = 0;
        self.lfophase      = 0.0;
        self.envf          = 0.0;
        self.lfo_val       = 0.0;
        self.lfo_direction = true;

        self.lp.suspend();
        self.hp.suspend();

        self.update();
        self.instantize_all();

        self.inithadtempo = true;

        // See issue #1444 and the fix for this stuff
        if self.time_unit.temposyncratio_inv() == 0.0 {
            self.inithadtempo = false;
        }
    }
}

impl DualDelay {

    pub fn new( 
        tuner:     &TunerHandle,
        tables:    &TablesHandle,
        srunit:    &SampleRateHandle,
        time_unit: &TimeUnitHandle) -> Self {

        Self {
            feedback:           Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            crossfeed:          Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            aligpan:            Align16(LipolPs::new()),
            pan:                Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            mix:                Align16(LipolPs::new_with_blocksize(BLOCK_SIZE)),
            width:              Align16(LipolPs::new()),
            buffer:             Align16(Self::new_delay_buffer()),
            ringout:            Ringout::blocks(10000000),
            params:             DualDelayParam::new_runtime(), 
            time_l:             Lag::<f32>::new(0.0001),
            time_r:             Lag::<f32>::new(0.0001),
            inithadtempo:       false,
            envf:               0.0,
            wpos:               0,
            lp:                 BiquadFilter::new(tuner,tables,srunit),
            hp:                 BiquadFilter::new(tuner,tables,srunit),
            lfophase:           0.0,
            lfo_val:            0.0,
            lfo_direction:      false,

            scratch_left:       ScratchChannel::<f32>::new(BLOCK_SIZE),
            scratch_right:      ScratchChannel::<f32>::new(BLOCK_SIZE),
            wetblock:           WetBlock2::<BLOCK_SIZE>::default(),

            time_unit:          time_unit.clone(),
            tables:             tables.clone(),
            tuner:              tuner.clone(),
            srunit:             srunit.clone(),
        }
    }

    #[inline] pub fn new_delay_buffer() -> A2d::<f32> {
        A2d::<f32>::zeros((2, DUAL_DELAY_MAX_DELAY_LENGTH + FIR_IPOL_N))
    }
}
