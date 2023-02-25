/// The oversampling factor for the oscillator,
/// used to increase the quality of the generated
/// waveforms. 
///
/// This constant is used in conjunction with the
/// `BLOCK_SIZE` constant to determine the size of
/// the processing blocks.
///
pub const OSC_OVERSAMPLING:       usize = 2;

/// The number of samples in each processing
/// block. 
///
/// This constant is used to determine the size of
/// the input and output buffers for processing
/// audio.
///
pub const BLOCK_SIZE:             usize = 32;

/// The reciprocal of `BLOCK_SIZE` as a `f32`
/// value. 
///
/// This constant is used for normalization during
/// processing.
///
pub const BLOCK_SIZE_INV:         f32   = 1.0 / BLOCK_SIZE as f32;

/// The product of `OSC_OVERSAMPLING` and
/// `BLOCK_SIZE`. 
///
/// This constant is used to determine the size of
/// the intermediate buffer used for oversampling.
///
pub const BLOCK_SIZE_OS:          usize = OSC_OVERSAMPLING * BLOCK_SIZE;

/// The reciprocal of `BLOCK_SIZE_OS` as a `f32`
/// value. 
///
/// This constant is used for normalization during
/// processing.
///
pub const BLOCK_SIZE_OS_INV:      f32   = 1.0 / BLOCK_SIZE_OS as f32;

/// The quotient of `BLOCK_SIZE_OS` and 4. 
///
/// This constant is used to determine the size of
/// the buffer used for computing the four
/// simultaneous oscillator outputs.
///
pub const BLOCK_SIZE_OS_QUAD:     usize = BLOCK_SIZE_OS >> 2;

/// The quotient of `BLOCK_SIZE` and 4. 
///
/// This constant is used to determine the size of
/// the buffer used for computing the four
/// simultaneous oscillator outputs.
///
pub const BLOCK_SIZE_QUAD:        usize = BLOCK_SIZE >> 2;

/// The number of coefficients used by the FIR
/// interpolator for `i16` values.
///
/// This constant is used to determine the number
/// of taps used for filtering during resampling.
///
pub const FIR_IPOL_I16_N:         usize = 8;

/// The number of points used by the FIR
/// interpolator kernel. 
///
/// This constant is used to determine the size of
/// the FIR interpolator kernel buffer.
///
pub const FIR_IPOL_M:             usize = 256;

/// The number of bits used to represent
/// `FIR_IPOL_M`. 
///
/// This constant is used to compute the length of
/// the intermediate buffer for the FIR resampler.
///
pub const FIR_IPOL_M_BITS:        usize = 8;

/// The number of times the FIR interpolator is
/// applied during resampling. 
///
/// This constant is used to determine the length
/// of the intermediate buffer for resampling.
///
pub const FIR_IPOL_N:             usize = 12;

/// The offset used for FIR resampling. 
///
/// This constant is used to compute the number of
/// samples that need to be discarded from the
/// beginning of the resampled output.
///
pub const FIR_OFFSET:             usize = FIR_IPOL_N >> 1;

/// The offset used for FIR resampling as a `f32`
/// value. 
///
/// This constant is used to normalize the offset
/// for resampling.
///
pub const FIR_OFFSET_F32:         f32   = (FIR_IPOL_N >> 1) as f32;

/// The offset used for FIR resampling for `i16`
/// values. 
///
/// This constant is used to compute the number of
/// samples that need to be discarded from the
/// beginning of the resampled output.
///
pub const FIR_OFFSET_I16:         usize = FIR_IPOL_I16_N >> 1;

/// The cycle loss factor for the high-pass
/// filter. 
///
/// This constant is used to control the shape of
/// the filter.
///
/// defines the number of cycles of high-pass
/// filtering applied to the audio signal during
/// each processing block. 
///
/// This constant is used in the `surge-core`
/// crate's implementation of the high-pass filter
/// effect. 
///
/// The higher the value of `HPF_CYCLE_LOSS`, the
/// more aggressive the high-pass filtering will
/// be, resulting in more low frequencies being
/// removed from the audio signal.
///
pub const HPF_CYCLE_LOSS:         f32   = 0.99;

/// The maximum number of feedback comb filters
/// that can be used.
///
/// It must be a power of 2, as this is used for
/// bit masking in the implementation.
///
pub const MAX_FB_COMB:            usize = 2048; /// must be 2^n

/// The maximum number of mipmap levels for
/// a texture.
///
pub const MAX_MIPMAP_LEVELS:      usize = 16;

/// The maximum number of sub-tables that can be
/// used.
///
pub const MAX_SUBTABLES:          usize = 512;

/// The maximum number of unison voices that can
/// be used.
///
pub const MAX_UNISON:             usize = 16;

/// The maximum number of voices that can be used.
///
pub const MAX_VOICES:             usize = 64;

/// The maximum size of a single wave table in
/// samples.
///
pub const MAX_WAVETABLE_SIZE:     usize = 4096;

/// The offset for storing meta parameters. 
///
/// It must be larger than the total size of all
/// parameter storage plus 16 * 130 to allow for
/// fake VST3 mapping.
///
pub const METAPARAM_OFFSET:       usize = 20480;

/// The number of custom controllers used. 
///
/// This value is currently not used and will be removed.
///
pub const N_CUSTOMCONTROLLERS:    usize = 8;

/// The number of filter registers.
pub const N_FILTER_REGISTERS:     usize = 15;

/// The number of effect parameters.
pub const N_FX_PARAMS:            usize = 12;

/// The number of global parameters.
pub const N_GLOBAL_PARAMS:        usize = 113;

/// The number of global post parameters.
pub const N_GLOBAL_POSTPARAMS:    usize = 1;

/// The number of input channels.
pub const N_INPUTS:               usize = 2;

/// The number of LFOs per scene.
pub const N_LFOS_PER_SCENE:       usize = 6;

/// The number of LFOs per voice.
pub const N_LFOS_VOICE:           usize = 6;

/// The total number of LFOs.
pub const N_LFOS:                 usize = N_LFOS_VOICE + N_LFOS_PER_SCENE;

/// The number of meta-parameters.
pub const N_METAPARAMETERS:       usize = N_CUSTOMCONTROLLERS;

/// The number of oscillators.
pub const N_OSCS:                 usize = 3;

/// The number of oscillator parameters.
pub const N_OSC_PARAMS:           usize = 7;

/// The number of output channels.
///
pub const N_OUTPUTS:              usize = 2;

/// The number of scenes.
///
pub const N_SCENES:               usize = 2;

/// The number of scene parameters.
///
pub const N_SCENE_PARAMS:         usize = 271;

/// The total number of parameters.
///
pub const N_TOTAL_PARAMS:         usize = N_GLOBAL_PARAMS + 2 * N_SCENE_PARAMS + N_GLOBAL_POSTPARAMS;

/// The length of the output buffer.
///
pub const OB_LENGTH:              usize = BLOCK_SIZE_OS << 1;

/// The length of the output buffer divided by 4.
///
pub const OB_LENGTH_QUAD:         usize = OB_LENGTH >> 2;

/// The constant value of 1 as a double.
///
pub const ONE:                    f64   = 1.0;

/// The value of one twelfth, or 1/12.
///
pub const ONE_TWELFTH:            f32   = 1.0 / 12.0;

/// The slow rate value used for some operations.
///
pub const SLOWRATE:               usize = 8;

/// This constant represents the slow rate
/// subtracted by one, which is used to calculate
/// the frequency of a wave.
/// 
pub const SLOWRATE_M1:            usize = SLOWRATE - 1;

/// These constants are of type `usize` and
/// represent the window size of a graphical user
/// interface used in the `surge-ui` crate. 
/// 
/// This crate is responsible for displaying the
/// user interface of the
/// synthesizer. `WINDOW_SIZE_X` represents the
/// width of the window and `WINDOW_SIZE_Y`
/// represents the height of the window.
///
pub const WINDOW_SIZE_X: usize = 904;
pub const WINDOW_SIZE_Y: usize = 542;

/// The value of zero
///
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
///
/// The maximum number of samples in a wavetable. 
///
/// This value is set to 2097152 to fix
/// a consistency issue with the check in
/// WaveTable.cpp, which checks that the max
/// wtable samples is greater than or equal to the
/// product of the max tables and the max sample
/// size. 
///
/// In this case, ts and tc are set to 1024 and
/// 512, respectively.
///
pub const MAX_WAVETABLE_SAMPLES:         usize = 2097152;

/// const int max_wtable_samples =  268000; // delay pops 4 uses the most

/// The number of MIDI control interpolators
/// available.
/// 
/// This constant is used by the
/// midicontrol-interpolators module.
///
/// Control interpolators are used to smooth out
/// parameter changes over time.
///
pub const NUM_CONTROLINTERPOLATORS:  usize = 32;

/// The default ringout value, set to 10000000.
/// 
/// This constant is used in the ringout module.
///
pub const RINGOUT_DEFAULT:           usize = 10000000;

/// The number of coefficients used by the
/// coeffmaker module.
/// 
/// This constant is set to 8.
///
/// The coefficient maker is responsible for
/// generating the coefficients used in the reverb
/// algorithm.
///
pub const N_COEFFMAKER_COEFFS:       usize = 8;

/// The frequency of MIDI note 0, which is set to
/// 8.17579891564371.
/// 
/// This constant is used in the midi module.
///
/// 440.0 * pow( 2.0, - (69.0/12.0 ) )
///
pub const MIDI_0_FREQ: f64 = 8.17579891564371;

/// The frequency of note C0, which is set
/// to 16.351_599.
/// 
/// This constant is used in various modules
/// throughout the surge-rs suite.
///
pub const NOTE_FREQ_C0: f32 = 16.351_599;

/// The frequency of concert pitch A, which
/// is set to 432.0.
/// 
/// This constant is used in various modules
/// throughout the surge-rs suite.
///
pub const CONCERT_A_HZ: f64 = 432.0; ///muahahahahah

/// The MIDI note number for A4, which is
/// set to 69.
///
pub const A4_MIDI_NOTE:  i8 = 69;

/// The frequency of A4 in Hz, which is set to the
/// value of CONCERT_A_HZ.
/// 
pub const A4_FREQ:      f64 = CONCERT_A_HZ;
