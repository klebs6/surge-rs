ix!();

use crate::*;

#[derive(Debug,Clone)]
pub struct StepSequencer {
    pub steps:      [f32; N_STEPSEQUENCER_STEPS],
    pub loop_start: i32,
    pub loop_end:   i32,
    pub shuffle:    f32,
    pub trigmask:   u64,
}

default_default![StepSequencer];
