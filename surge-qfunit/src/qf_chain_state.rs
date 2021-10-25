ix!();

use crate::{QuadFilterUnitState};

/**
  typically used with MAX_VOICES >> 2 elements ie,
  each QuadFilterChainState performs filtering for
  four voices
  */
#[derive(Debug)]
pub struct QuadFilterChain<'tables> {
    pub state: Align16<Vec<QuadFilterChainState<'tables>>>,
}

impl QuadFilterChain<'tables> {
    pub fn new(len: usize, tables: &'tables TablesHandle<'tables>) -> Self {
        Self {
            state: Align16(vec![QuadFilterChainState::new(tables); len]),
        }
    }
}

#[derive(Debug,Clone)]
#[repr(align(16))]
pub struct QuadFilterChainState<'tables> {
    pub unit_state:       [QuadFilterUnitState<'tables>; 4],
    pub gain:             __m128 , 
    pub feedback:         __m128 , 
    pub mix1:             __m128 , 
    pub mix2:             __m128 , 
    pub drive:            __m128 ,
    pub d_gain:           __m128 , 
    pub d_feedback:       __m128 , 
    pub d_mix1:           __m128 , 
    pub d_mix2:           __m128 , 
    pub d_drive:           __m128 ,
    pub ws_lpf:            __m128 , 
    pub feedback_line_l:   __m128 , 
    pub feedback_line_r:   __m128 ,
    pub dl:               [__m128; BLOCK_SIZE_OS], 
    pub dr:               [__m128; BLOCK_SIZE_OS], // wavedata
    pub out_l:             __m128 , 
    pub out_r:             __m128 , 
    pub dout_l:            __m128 , 
    pub dout_r:            __m128 ,
    pub out_2l:            __m128 , 
    pub out_2r:            __m128 , 
    pub dout_2l:           __m128 , 
    pub dout_2r:           __m128 , // fb_stereo only
}

impl Init for QuadFilterChainState<'tables> {

    /**
      | Original note on the out-of-line function
      | performing this function:
      |
      | I originally had this as a member but
      | since moved it out of line so as to not
      | run any risk of alignment problems in
      | QuadFilterChainState where only the head
      | of the array is __align_malloced
      |
      */
    fn init(&mut self) {
        self.gain            = unsafe { z128![] };
        self.feedback        = unsafe { z128![] };
        self.mix1            = unsafe { z128![] };
        self.mix2            = unsafe { z128![] };
        self.drive           = unsafe { z128![] };
        self.d_gain          = unsafe { z128![] };
        self.d_feedback      = unsafe { z128![] };
        self.d_mix1          = unsafe { z128![] };
        self.d_mix2          = unsafe { z128![] };
        self.d_drive         = unsafe { z128![] };
        self.ws_lpf          = unsafe { z128![] };
        self.feedback_line_l = unsafe { z128![] };
        self.feedback_line_r = unsafe { z128![] };

        for i in 0..BLOCK_SIZE_OS {
            self.dl[i] = unsafe { z128![] };
            self.dr[i] = unsafe { z128![] };
        }

        self.out_l    = unsafe { z128![] };
        self.out_r    = unsafe { z128![] };
        self.dout_l   = unsafe { z128![] };
        self.dout_r   = unsafe { z128![] };
        self.out_2l   = unsafe { z128![] };
        self.out_2r   = unsafe { z128![] };
        self.dout_2l  = unsafe { z128![] };
        self.dout_2r  = unsafe { z128![] };
    }
}

impl QuadFilterChainState<'tables> {
    pub fn new( tables: &'tables TablesHandle<'tables>) -> Self {
        Self {
            unit_state:       [
                QuadFilterUnitState::new(tables),
                QuadFilterUnitState::new(tables),
                QuadFilterUnitState::new(tables),
                QuadFilterUnitState::new(tables),
            ],
            gain:             unsafe { z128![] }, 
            feedback:         unsafe { z128![] }, 
            mix1:             unsafe { z128![] }, 
            mix2:             unsafe { z128![] }, 
            drive:            unsafe { z128![] },
            d_gain:           unsafe { z128![] }, 
            d_feedback:       unsafe { z128![] }, 
            d_mix1:           unsafe { z128![] }, 
            d_mix2:           unsafe { z128![] }, 
            d_drive:          unsafe { z128![] },
            ws_lpf:           unsafe { z128![] }, 
            feedback_line_l:  unsafe { z128![] }, 
            feedback_line_r:  unsafe { z128![] },
            dl:               [unsafe { z128![] }; BLOCK_SIZE_OS], 
            dr:               [unsafe { z128![] }; BLOCK_SIZE_OS], 
            out_l:             unsafe { z128![] }, 
            out_r:             unsafe { z128![] }, 
            dout_l:            unsafe { z128![] }, 
            dout_r:            unsafe { z128![] },
            out_2l:            unsafe { z128![] }, 
            out_2r:            unsafe { z128![] }, 
            dout_2l:           unsafe { z128![] }, 
            dout_2r:           unsafe { z128![] }, 
        }
    }
}
