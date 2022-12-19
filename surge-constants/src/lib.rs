
pub const OSC_OVERSAMPLING:       usize = 2;
pub const BLOCK_SIZE:             usize = 32;
pub const BLOCK_SIZE_INV:         f32   = 1.0 / BLOCK_SIZE as f32;
pub const BLOCK_SIZE_OS:          usize = OSC_OVERSAMPLING * BLOCK_SIZE;
pub const BLOCK_SIZE_OS_INV:      f32   = 1.0 / BLOCK_SIZE_OS as f32;
pub const BLOCK_SIZE_OS_QUAD:     usize = BLOCK_SIZE_OS >> 2;
pub const BLOCK_SIZE_QUAD:        usize = BLOCK_SIZE >> 2;
pub const FIR_IPOL_I16_N:         usize = 8;
pub const FIR_IPOL_M:             usize = 256;
pub const FIR_IPOL_M_BITS:        usize = 8;
pub const FIR_IPOL_N:             usize = 12;
pub const FIR_OFFSET:             usize = FIR_IPOL_N >> 1;
pub const FIR_OFFSET_F32:         f32   = (FIR_IPOL_N >> 1) as f32;
pub const FIR_OFFSET_I16:         usize = FIR_IPOL_I16_N >> 1;
pub const HPF_CYCLE_LOSS:         f32   = 0.99;
pub const MAX_FB_COMB:            usize = 2048; /// must be 2^n
pub const MAX_MIPMAP_LEVELS:      usize = 16;
pub const MAX_SUBTABLES:          usize = 512;
pub const MAX_UNISON:             usize = 16;
pub const MAX_VOICES:             usize = 64;
pub const MAX_WAVETABLE_SIZE:     usize = 4096;
pub const METAPARAM_OFFSET:       usize = 20480; /// has to be bigger than total + 16 * 130 for fake VST3 mapping
pub const N_CUSTOMCONTROLLERS:    usize = 8; /// TODO: remove this one
pub const N_FILTER_REGISTERS:     usize = 15;
pub const N_FX_PARAMS:            usize = 12;
pub const N_GLOBAL_PARAMS:        usize = 113;
pub const N_GLOBAL_POSTPARAMS:    usize = 1;
pub const N_INPUTS:               usize = 2;
pub const N_LFOS_PER_SCENE:       usize = 6;
pub const N_LFOS_VOICE:           usize = 6;
pub const N_LFOS:                 usize = N_LFOS_VOICE + N_LFOS_PER_SCENE;
pub const N_METAPARAMETERS:       usize = N_CUSTOMCONTROLLERS;
pub const N_OSCS:                 usize = 3;
pub const N_OSC_PARAMS:           usize = 7;
pub const N_OUTPUTS:              usize = 2;
pub const N_SCENES:               usize = 2;
pub const N_SCENE_PARAMS:         usize = 271;
pub const N_TOTAL_PARAMS:         usize = N_GLOBAL_PARAMS + 2 * N_SCENE_PARAMS + N_GLOBAL_POSTPARAMS;
pub const OB_LENGTH:              usize = BLOCK_SIZE_OS << 1;
pub const OB_LENGTH_QUAD:         usize = OB_LENGTH >> 2;
pub const ONE:                    f64   = 1.0;
pub const ONE_TWELFTH:            f32   = 1.0 / 12.0;
pub const SLOWRATE:               usize = 8;
pub const SLOWRATE_M1:            usize = SLOWRATE - 1;
pub const WINDOW_SIZE_X:          usize = 904;
pub const WINDOW_SIZE_Y:          usize = 542;
pub const ZERO:                   f64   = 0.0;

/// I don't know why your max wtable samples would
/// be less than your max tables * your max sample
/// size. So lets fix that!
///
/// This size is consistent with the check in
/// WaveTable.cpp 
///
/// CheckRequiredWTSize with ts and tc at 1024 and
/// 512
pub const MAX_WAVETABLE_SAMPLES:         usize = 2097152;

/// const int max_wtable_samples =  268000; // delay pops 4 uses the most

pub const NUM_CONTROLINTERPOLATORS:  usize = 32; /// midicontrol-interpolators
pub const RINGOUT_DEFAULT:           usize = 10000000;
pub const N_COEFFMAKER_COEFFS:       usize = 8;

/// or 440.0 * pow( 2.0, - (69.0/12.0 ) )
pub const MIDI_0_FREQ: f64 = 8.17579891564371; 
pub const NOTE_FREQ_C0: f32 = 16.351_599;
pub const CONCERT_A_HZ: f64 = 432.0;///muahahahahah
pub const A4_MIDI_NOTE:  i8 = 69;
pub const A4_FREQ:      f64 = CONCERT_A_HZ;
