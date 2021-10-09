/// OK so lets say we want lowest tunable frequency to be 23.5hz at 96k 96000/23.5 = 4084 
/// And lets future proof a bit and make it a power of 2 so we can use & properly
pub const FLANGER_DELAY_SIZE:        usize = 8192;
pub const FLANGER_DELAY_SIZE_MASK:   usize = FLANGER_DELAY_SIZE - 1;
pub const FLANGER_LFO_TABLE_MASK:    usize = FLANGER_LFO_TABLE_SIZE - 1;
pub const FLANGER_LFO_TABLE_SIZE:    usize = 8192;
pub const FLANGER_COMBS_PER_CHANNEL: usize = 4;
pub const FLANGER_ONE_OVER_F0:       f64   = 1.0 / 8.175798915;
