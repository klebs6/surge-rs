pub const ALLPASS_REVERB_DELAY_LEN_MASK:          usize = ALLPASS_REVERB_MAX_DELAY_LEN - 1;
pub const ALLPASS_REVERB_DELAY_SUBSAMPLE_BITS:    usize = 8;
pub const ALLPASS_REVERB_DELAY_SUBSAMPLE_RANGE:   usize = 1 << ALLPASS_REVERB_DELAY_SUBSAMPLE_BITS;
pub const ALLPASS_REVERB_MAX_ALLPASS_LEN:         usize = 16384;
pub const ALLPASS_REVERB_MAX_DELAY_LEN:           usize = 16384;
pub const ALLPASS_REVERB_NUM_ALLPASSES_PER_BLOCK: usize = 2;
pub const ALLPASS_REVERB_NUM_BLOCKS:              usize = 4;
pub const ALLPASS_REVERB_NUM_INPUT_ALLPASSES:     usize = 4;

///max sample rate is 48K * 4 probably
pub const ALLPASS_REVERB_PREDELAY_BUFFER_SIZE:    usize = 48000 * 4 * 2;
