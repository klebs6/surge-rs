crate::ix!();

#[repr(align(16))] 
#[derive(Debug,Clone)]
pub struct WaveshapeTables {
    pub table: Align16<[A1d::<f32>; 8]>,
}

impl Default for WaveshapeTables {

    fn default() -> Self {
        Self {
            table: Align16([
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
                A1d::<f32>::zeros(1024),
            ]),
        }
    }
}

impl NTables for WaveshapeTables {

    fn ntables() -> usize {
        //TODO: how many waveshapers do we need?
        8
    }
}

impl LookupWaveshape for WaveshapeTables {
    fn lookup_waveshape(&self, entry: i32, mut x: f32) -> f32 {

        x *= 32.0;
        x += 512.0;

        let e: i32 = x as i32;
        let a: f32 = x - (e as f32);

        if e > 0x3fd {
            return 1.0;
        }

        if e < 1 {
            return -1.0;
        }

        let idx   = entry as usize;
        let eidx  = (e & 0x3ff) as usize;
        let eidx1 = ((e + 1) & 0x3ff) as usize;

        (1.0 - a) * self.table[idx][[eidx]] + 
            a * self.table[idx][[eidx1]]
    }
}

impl LookupWaveshapeWarp for WaveshapeTables {
    fn lookup_waveshape_warp(&self, entry: i32, mut x: f32) -> f32 {

        x *= 256.0;
        x += 512.0;

        let e: i32 = x as i32;
        let a: f32 = x - (e as f32);

        let idx = entry as usize;
        let eidx = (e & 0x3ff) as usize;
        let eidx1 = ((e + 1) & 0x3ff) as usize;

        (1.0 - a) * self.table[idx][[eidx]] + 
            a * self.table[idx][[eidx1]]
    }
}

impl Initialize for WaveshapeTables {
    fn init(&mut self) {

        let mult: f64 = 1.0 / 32.0;

        for i in 0_usize..1024_usize {

            let t = (i as f64) - 512.0;

            let x: f64 = t * mult;
            let absx   = x.abs();

            self.table[0][[i]] = x.tanh() as f32;                          //wst_tanh
            self.table[1][[i]] = (absx.powf(5.0)).tanh().powf(0.2) as f32; //wst_hard

            if x < 0.0 {
                //can this be included as a function call to sign(x)?
                self.table[1][[i]] = - self.table[1][[i]]; 
            }
            self.table[2][[i]] = 
                (shafted_tanh(x + 0.5) - shafted_tanh(0.5)) as f32; //wst_asym
            self.table[3][[i]] = (t * PI / 512.0).sin() as f32;     //wst_sine
            self.table[4][[i]] = (t * mult).tanh() as f32;          //wst_digi
        }
    }
}
