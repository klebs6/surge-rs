crate::ix!();

impl AdsrEnvelope {

    pub fn new( 
        time_unit: TimeUnitHandle,
        tables:    TablesHandle,
        srunit:    SampleRateHandle) -> Self {

        let mut x = Self {
            time_unit:     time_unit,
            tables:        tables,
            srunit:        srunit,
            params:        AdsrParam::new_runtime(),
            output:        0.0,
            phase:         0.0,
            sustain:       0.0,
            scalestage:    0.0,
            idlecount:     0,
            envstate:      AdsrState::Idle,
            _v_c1:         0.0,
            _v_c1_delayed: 0.0,
            _discharge:    0.0,
            enabled:       true
        };
        x.init();
        x
    }
}
