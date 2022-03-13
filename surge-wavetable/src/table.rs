ix!();

use crate::*;

pub const INITIAL_WAVETABLE_DATA_SIZES: usize = 35000;
pub const MIN_F32_WAVETABLES:             u32 = 3;
pub const WAVETABLE_SAMPLE_PADDING:     usize = 3;

bitflags! {
    #[derive(Default)]
    pub struct WaveTableFlags: u16 {
        const CLEARED     = 0b0000;
        const IS_SAMPLE   = 0b0001;
        const LOOP_SAMPLE = 0b0010;
    }
}

#[derive(Debug,Clone)]
pub struct WaveTableHeader {
    pub flags:     WaveTableFlags,
    pub dim:       WaveTableDim,
}

#[enum_dispatch]
pub trait WaveTableProperties {
    fn num_tables(&self) -> usize;
    fn num_mipmap_levels(&self) -> usize;
    fn num_samples_per_table(&self) -> usize;
    fn num_samples_per_table_po2(&self) -> usize;
    fn dt(&self) -> f32;
}

#[enum_dispatch(WaveTableProperties)]
#[derive(Debug,Clone)]
pub enum WaveTable {
    IeeeFloat(WaveTableBase<f32>),
    Pcm(WaveTableBase<i16>),
}

impl Default for WaveTable {
    fn default() -> Self {
        WaveTable::Pcm(WaveTableBase::<i16>::new_zero(WaveTableDim {
            mipmap_levels: 1,
            num_tables:    1,
            table_len:  1024,
        }))
    }
}

impl TryFrom<WaveTableFilename> for WaveTable {

    type Error = WaveTableBuildError;

    fn try_from(filename: WaveTableFilename) -> Result<Self,Self::Error> {

        let format = get_wavetable_format(&filename.0).unwrap();

        match format {
            WaveTableFileFormat::Wt => {
                Self::try_from(WaveTableDataFilename(filename.0))
            },
            WaveTableFileFormat::Wav => {
                Self::try_from(WaveTableWavFilename(filename.0))
            },
        }
    }
}
