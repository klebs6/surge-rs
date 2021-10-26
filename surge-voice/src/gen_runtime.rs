ix!();

use crate::{
    SurgeVoice,
    OscillatorRuntime,
    VoiceNoteShiftCfg,
    VoiceRuntimeHandle,
};

impl SurgeVoice {

    pub fn gen_oscillator_runtime(&mut self, 
        cfg: VoiceRuntimeHandle) 
        -> OscillatorRuntime 
    {
        let cfg = cfg.borrow();

        let is_wide: bool = cfg.filterblock_cfg.is_wide();

        let tblock_l = WetBlock1::<BLOCK_SIZE_OS>::default();
        let tblock_r = WetBlock1::<BLOCK_SIZE_OS>::default();

        let ktrkroot: f32 = 60.0;

        let drift:   f32 = cfg.drift;
        let fmdepth = self.tables.db_to_linear(cfg.fm_depth);

        let noise_colour = {
            let nc = cfg.noise_colour;
            limit_range( nc, -1.0, 1.0)
        };

        let octave0 = cfg.oscillator_octave[0];
        let octave1 = cfg.oscillator_octave[1];
        let octave2 = cfg.oscillator_octave[2];

        let pitch_or_ktrkroot = match cfg.oscillator_keytrack2 {
            true => self.state.pitch as f32,
            false => ktrkroot,
        };

        //oscNum 0
        let pitch0 = self.note_shift_from_pitch_param(
            VoiceNoteShiftCfg {
                note0_pitch: {
                    pitch_or_ktrkroot + 
                        self.octave_size * octave0
                },
                oscillator_pitch:        cfg.oscillator_pitch[0],
                oscillator_absolute:     cfg.oscillator_absolute[0],
                oscillator_extend_range: cfg.oscillator_extend_range[0],
            }
        );

        //oscNum 1
        let pitch1 = self.note_shift_from_pitch_param(
            VoiceNoteShiftCfg {
                note0_pitch: {
                    pitch_or_ktrkroot + 
                        self.octave_size *  octave1
                },
                oscillator_pitch:        cfg.oscillator_pitch[1],
                oscillator_absolute:     cfg.oscillator_absolute[1],
                oscillator_extend_range: cfg.oscillator_extend_range[1],
            }
        );

        //oscNum 2
        let pitch2 = self.note_shift_from_pitch_param(
            VoiceNoteShiftCfg {
                note0_pitch: {
                    pitch_or_ktrkroot + 
                        self.octave_size *  octave2
                },
                oscillator_pitch:        cfg.oscillator_pitch[2],
                oscillator_absolute:     cfg.oscillator_absolute[2],
                oscillator_extend_range: cfg.oscillator_extend_range[2],
            }
        );

        OscillatorRuntime {
            is_wide,
            tblock_l,
            tblock_r,
            drift,
            fmdepth,
            noise_colour,
            octave:  [octave0, octave1, octave2],
            pitch:   [pitch0, pitch1, pitch2],
        }
    }
}
