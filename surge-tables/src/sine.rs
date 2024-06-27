crate::ix!();

pub const SIN_TABLE_SIZE_D: f64 = 512.0;
pub const SIN_TABLE_SIZE_U: usize = 512;

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct SineTables {
    pub table_sin:        Align16<A1d::<f32>>,
    pub table_sin_offset: Align16<A1d::<f32>>,
}

impl Default for SineTables {
    fn default() -> Self {
        Self {
            table_sin:         Align16(A1d::<f32>::zeros(SIN_TABLE_SIZE_U)),
            table_sin_offset:  Align16(A1d::<f32>::zeros(SIN_TABLE_SIZE_U)),
        }
    }
}
impl Initialize for SineTables {
    fn init(&mut self) -> Result<(),SurgeError> {

        for i in 0_usize..SIN_TABLE_SIZE_U {

            let t: f64 = 2.0 * PI * (i as f64) / SIN_TABLE_SIZE_D;

            self.table_sin[i] = t.sin() as f32;

            self.table_sin_offset[i] = 
                ((t + (2.0 * PI / SIN_TABLE_SIZE_D)).sin() 
                 - t.sin()) as f32;
        }

        Ok(())
    }
}
