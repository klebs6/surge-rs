crate::ix!();

pub const ENVELOPE_TABLE_SIZE_D:  f64 = 512.0;
pub const ENVELOPE_TABLE_SIZE_U:  usize = 512;

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct EnvelopeTables {

    pub table_envrate_lpf:    Align16<A1d::<f32>>,
    pub table_envrate_linear: Align16<A1d::<f32>>,

    srunit: SampleRateHandle,
}

impl EnvelopeTables {

    pub fn new(srunit: &SampleRateHandle) -> Self {
        Self {
            table_envrate_lpf:     Align16(A1d::<f32>::zeros(ENVELOPE_TABLE_SIZE_U)),
            table_envrate_linear:  Align16(A1d::<f32>::zeros(ENVELOPE_TABLE_SIZE_U)),
            srunit:                srunit.clone(),
        }
    }

    pub fn envelope_rate_lpf(&self, mut x: f32) -> f32 {

        x *= 16.0;
        x += 256.0;

        let e: i32 = x as i32;
        let a: f32 = x - (e as f32);

        (1.0 - a) * self.table_envrate_lpf[(e & 0x1ff) as usize] 
            + a * self.table_envrate_lpf[((e + 1) & 0x1ff) as usize]
    }

    pub fn envelope_rate_linear(&self, mut x: f32) -> f32 {

        x *= 16.0;
        x += 256.0;

        let e: i32 = x as i32;
        let a: f32 = x - (e as f32);

        (1.0 - a) * self.table_envrate_linear[(e & 0x1ff) as usize] 
            + a * self.table_envrate_linear[((e + 1) & 0x1ff) as usize]
    }
}

impl Initialize for EnvelopeTables {
    fn init(&mut self) -> Result<(),SurgeError> {

        let sr = self.srunit.dsamplerate_os();

        let db_log10 = db60![].log10();

        for i in 0..512 {

            let v = 2.0_f64.powf((i as f64 - 256.0) / 16.0);

            let k: f64 = sr * v / BLOCK_SIZE_OS as f64;

            self.table_envrate_lpf[i] = 
                (1.0 - (db_log10 / k).exp()) as f32;

            self.table_envrate_linear[i] = 
                (1.0 / k) as f32;
        }

        Ok(())
    }
}
