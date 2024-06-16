crate::ix!();

#[derive(Debug,Clone)] 
#[repr(align(16))]
pub struct TuningTables {
    pub table_pitch:                        Align16<A1d::<f64>>,
    pub table_pitch_inv:                    Align16<A1d::<f64>>,
    pub table_note_omega:                   Align16<A2d::<f64>>,
    pub table_pitch_ignoring_tuning:        Align16<A1d::<f64>>,
    pub table_pitch_inv_ignoring_tuning:    Align16<A1d::<f64>>,
    pub table_note_omega_ignoring_tuning:   Align16<A2d::<f64>>,
    srunit: SampleRateHandle,
}

impl TuningTables {
    pub fn new(srunit: &SampleRateHandle) -> Self {
        Self {
            table_pitch:                      Align16(A1d::<f64>::zeros(512)),
            table_pitch_inv:                  Align16(A1d::<f64>::zeros(512)),
            table_note_omega:                 Align16(A2d::<f64>::zeros((2,512))),
            table_pitch_ignoring_tuning:      Align16(A1d::<f64>::zeros(512)),
            table_pitch_inv_ignoring_tuning:  Align16(A1d::<f64>::zeros(512)),
            table_note_omega_ignoring_tuning: Align16(A2d::<f64>::zeros((2,512))),
            srunit: srunit.clone(),
        }
    }
}

impl Initialize for TuningTables {
    fn init(&mut self) {

        for i in 0..512 {

            self.table_pitch[i] = 2.0_f64.powf((i as f64 - 256.0) * (1.0 / 12.0));
            self.table_pitch_ignoring_tuning[i] = self.table_pitch[i];

            self.table_pitch_inv[i] = 1.0 / self.table_pitch[i];
            self.table_pitch_inv_ignoring_tuning[i] = self.table_pitch_inv[i];

            self.table_note_omega[[0, i]] =
                (2.0 * PI * std::cmp::min(
                        FloatOrd(0.5), 
                        FloatOrd(
                            CONCERT_A_HZ * 
                            (self.table_pitch[i] as f64) * 
                            self.srunit.dsamplerate_os_inv()
                        )
                ).0).sin() as f64;

            self.table_note_omega[[1,i]] =
                (2.0 * PI * std::cmp::min(
                        FloatOrd(0.5), 
                        FloatOrd(
                            CONCERT_A_HZ * 
                            (self.table_pitch[i] as f64) * 
                            self.srunit.dsamplerate_os_inv()
                        )
                ).0).cos() as f64;

            self.table_note_omega_ignoring_tuning[[0,i]] = self.table_note_omega[[0,i]];
            self.table_note_omega_ignoring_tuning[[1,i]] = self.table_note_omega[[1,i]];
        }
    }
}
