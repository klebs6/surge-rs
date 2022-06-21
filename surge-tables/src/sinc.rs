crate::ix!();

pub const SINCTABLE1X_SIZE:  usize = (FIR_IPOL_M + 1) * FIR_IPOL_N;
pub const SINCTABLEI16_SIZE: usize = (FIR_IPOL_M + 1) * FIR_IPOL_I16_N;
pub const SINCTABLE_SIZE:    usize = (FIR_IPOL_M + 1) * FIR_IPOL_N * 2 ;

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct SincTables {

    pub table:     Align16<A1d::<f32>>,
    pub table_1x:  Align16<A1d::<f32>>,
    pub table_i16: Align16<A1d::<i16>>,
}

impl Default for SincTables {
    fn default() -> Self {
        Self {
            table:     Align16(A1d::<f32>::zeros(SINCTABLE_SIZE)),
            table_1x:  Align16(A1d::<f32>::zeros(SINCTABLE1X_SIZE)),
            table_i16: Align16(A1d::<i16>::zeros(SINCTABLEI16_SIZE)),
        }
    }
}

impl Init for SincTables {
    fn init(&mut self) {

        fn get_table_t(i: usize, j: usize, rows: usize, cols: usize) -> f64 {
            let i    = i as f64;
            let j    = j as f64;
            let rows = rows as f64;
            let cols = cols as f64;
            -i + (cols / 2.0) + j / rows - 1.0
        }

        fn windowed_sinc(t: f64, cutoff: f32, len: usize) -> f64 {
            let cutoff = cutoff as f64;
            let len = len as i32;
            symmetric_blackman(t, len) * cutoff * sincf(cutoff * t)
        }

        let cutoff:     f32 = 0.455;
        let cutoff1x:   f32 = 0.85;
        let cutoff_i16: f32 = 1.0;

        for j in 0_usize..(FIR_IPOL_M as usize + 1) {
            for i in 0_usize..(FIR_IPOL_N as usize) {
                let t:     f64 = get_table_t(i,j,FIR_IPOL_M,FIR_IPOL_N);
                let val:   f64 = windowed_sinc(t, cutoff, FIR_IPOL_N);
                let val1x: f64 = windowed_sinc(t, cutoff1x, FIR_IPOL_N);
                let idx1 = j * FIR_IPOL_N * 2 + i;
                let idx2 = j * FIR_IPOL_N + i;
                self.table[idx1]   = val as f32;
                self.table_1x[idx2] = val1x as f32;
            }
        }

        for j in 0_usize..(FIR_IPOL_M as usize) {
            for i in 0_usize..(FIR_IPOL_N as usize) {
                let widx:  usize = j * FIR_IPOL_N * 2 + FIR_IPOL_N + i;
                let ridx1: usize = (j + 1) * FIR_IPOL_N * 2 + i;
                let ridx2: usize = j * FIR_IPOL_N * 2 + i;
                self.table[widx] = (self.table[ridx1] - self.table[ridx2]) / 65536.0
            }
        }

        for j in 0_usize..(FIR_IPOL_M as usize + 1) {
            for i in 0_usize..(FIR_IPOL_I16_N as usize) {
                let t:      f64 = get_table_t(i,j,FIR_IPOL_M,FIR_IPOL_I16_N);
                let val:    f64 = windowed_sinc(t,cutoff_i16,FIR_IPOL_I16_N);
                let widx: usize = j * FIR_IPOL_I16_N + i;
                self.table_i16[widx] = ((val + 16384.0 ) as f32) as i16;
            }
        }
    }
}
