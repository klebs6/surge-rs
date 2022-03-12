ix!();

use crate::*;

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct GainTables {
    pub table_db: Align16<A1d::<f32>>,
}

impl Default for GainTables {
    fn default() -> Self {
        Self {
            table_db: Align16(A1d::<f32>::zeros(512)),
        }
    }
}

impl DbToLinear for GainTables {
    #[inline] fn db_to_linear(&self, mut x: f32) -> f32
    {
        x += 384.0;
        let e: i32 = x as i32;
        let a: f32 = x - (e as f32);

        (1.0 - a) * self.table_db[(e & 0x1ff) as usize] + a * self.table_db[((e + 1) & 0x1ff) as usize]
    }
}

impl ClipScale for GainTables {

    fn clipscale(&self, freq: f32, subtype: FilterSubType) -> f32 
    {
        match subtype {
            FilterSubType::Rough  =>  (1.0 / 64.0) * 
                self.db_to_linear(freq * 0.55),

            // * db_to_linear(freq*0.55f);
            FilterSubType::Smooth => (1.0 / 1024.0), 

            _ => 0.0,
        }
    }
}

impl Init for GainTables {

    fn init(&mut self) {

        for i in 0..512 {
            self.table_db[i] = 10.0_f32.powf(0.05 * (i as f32 - 384.0));
        }
    }
}
