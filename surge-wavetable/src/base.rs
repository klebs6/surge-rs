ix!();

use crate::{
    WaveTableHeader,
    WaveTableProperties,
    WaveTableDim,
    hardcoded_wavetables::BASIC_SINE_WAVE,
};

pub trait WaveTableData = 
Zero 
+ Debug 
+ Copy 
+ Clone 
+ hound::Sample 
+ MaybeSaturatingMul 
+ Mul<Output = Self>;

#[derive(Debug,Clone)]
pub struct WaveTableBase<T: WaveTableData> {
    pub header: WaveTableHeader,
    pub data:   A3d::<T>,
}

impl<T: WaveTableData> Default for WaveTableBase<T> {
    fn default() -> Self {
        let dim = WaveTableDim::default();
        Self::new_zero(dim)
    }
}

impl<T: WaveTableData> WaveTableBase<T> {

    pub fn new_zero(dim: WaveTableDim) -> Self {
        Self {
            header: WaveTableHeader {
                dim,
                flags: Default::default(),
            },
            data: A3d::<T>::zeros(
                (dim.mipmap_levels, 
                 dim.num_tables, 
                 dim.table_len)
            ),
        }
    }

    pub fn load_initial_wavetable(&mut self) {
        println!("{:?}", BASIC_SINE_WAVE);
        todo!();

    }
}

impl<T: WaveTableData> WaveTableProperties for WaveTableBase<T> {
    #[inline] fn num_tables(&self) -> usize {
        assert!(self.header.dim.num_tables == self.data.dim().1);
        self.header.dim.num_tables
    }

    #[inline] fn num_mipmap_levels(&self) -> usize {
        assert!(self.header.dim.mipmap_levels == self.data.dim().0);
        self.header.dim.mipmap_levels
    }

    #[inline] fn num_samples_per_table(&self) -> usize {
        assert!(self.header.dim.table_len == self.data.dim().2);
        self.header.dim.table_len
    }

    #[inline] fn num_samples_per_table_po2(&self) -> usize {
        let num_samples = self.num_samples_per_table();
        bitscan_reverse(num_samples as u32) as usize
    }

    #[inline] fn dt(&self) -> f32 {
        1.0 / (self.num_samples_per_table() as f32)
    }
}
