crate::ix!();

#[derive(Debug,Copy,Clone)]
pub struct WaveTableDim {
    pub mipmap_levels: usize,
    pub num_tables:    usize,
    pub table_len:     usize,
}

impl Default for WaveTableDim {
    fn default() -> Self {
        Self {
            mipmap_levels: 1,
            num_tables: 1,
            table_len: 1024,
        }
    }
}

pub fn required_mipmap_levels(table_len: usize) -> usize 
{
    let mut levels = 1;

    while (( 1 << levels ) < table_len ) & (levels < MAX_MIPMAP_LEVELS) 
    {
        levels += 1;
    }

    levels as usize
}
