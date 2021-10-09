pub const REVERB_DELAY_QUADS:    usize = (REVERB_TAPS * REVERB_MAX_DELAY) >> 2;
pub const REVERB_PREDELAY_QUADS: usize = REVERB_MAX_DELAY >> 2;
pub const REVERB_BITS:           usize = 15;
pub const REVERB_MAX_DELAY:      usize = 1 << REVERB_BITS;
pub const REVERB_TAPS:           usize = 1 << REVERB_TAP_BITS;
pub const REVERB_TAP_BITS:       usize = 4;

