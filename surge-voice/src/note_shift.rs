ix!();

use crate::{
    SurgeVoice,
};

pub struct VoiceNoteShiftCfg {

    ///the key + octave
    pub note0_pitch:              f32,

    //from the osc for pitch diffs
    pub oscillator_pitch:         f32,
    pub oscillator_absolute:      bool,
    pub oscillator_extend_range:  bool,

}

impl SurgeVoice<'sr> {

    /// Given a note0 and an oscilator this returns the appropriate note.
    /// This is a pretty easy calculation in non-absolute mode. Just add.
    /// But in absolute mode you need to find the virtual note which would
    /// map to that frequency shift.
    pub fn note_shift_from_pitch_param(&mut self, cfg: VoiceNoteShiftCfg) -> f32 {

        let pitch_is_absolute  = cfg.oscillator_absolute;
        let pitch_extend_range = cfg.oscillator_extend_range;

        let _maybe_range_extended = match pitch_extend_range{
                true  => 12.0,
                false => 1.0,
            };

        let pitch_f            = cfg.oscillator_pitch;

        let maybe_pitch_extend_range = match pitch_extend_range {
            true  => 12.0,
            false => 1.0,
        };

        match pitch_is_absolute {
            true => {

                // remember n2p is linear interpolation on storage->table_pitch from
                // position note + 256 % 512
                //
                // OK so now what we are searching for is 
                // the pair which surrounds us plus the pitch drift... so
                let freq_shift: f32 = (10.0 * pitch_f * maybe_pitch_extend_range) as f32;

                let tablenote0:   f32 = cfg.note0_pitch + 256.0;
                let mut table_idx: i32 = tablenote0 as i32;

                if table_idx > 0x1fe {
                    table_idx = 0x1fe;
                }

                // so just iterate up. Deal with negative also of course. 
                // Since we will always be close just do it brute force 
                // for now but later we can do a binary or some such.
                let mut pitch0:  f32 = self.get_table_pitch(table_idx);
                let target_pitch: f32 = pitch0 + freq_shift * 32.0 / 261.626;

                if freq_shift > 0.0 {

                    while table_idx < 0x1fe {

                        let pitch1: f32 = self.get_table_pitch(table_idx + 1);

                        if pitch0 <= target_pitch && 
                            pitch1 > target_pitch 
                        {
                            break;
                        }

                        pitch0 = pitch1;
                        table_idx += 1;
                    }

                } else {

                    while table_idx > 0 {

                        let pitch1: f32 = self.get_table_pitch(table_idx - 1);

                        if pitch0 >= target_pitch && pitch1 < target_pitch {

                            table_idx -= 1;
                            break;
                        }

                        pitch0 = pitch1;
                        table_idx -= 1;
                    }
                }

                // So what's the frac
                // (1-x) * [table_idx] + x * [table_idx+1] = target_pitch
                // Or: x = ( target - table) / ( [ table+1 ] - [table] );
                let frac: f32 = (target_pitch - self.get_table_pitch(table_idx)) /
                    ( self.get_table_pitch(table_idx + 1) - self.get_table_pitch(table_idx) );

                // frac = 1 -> targetpitch = +1; frac = 0 -> target_pitch

                (table_idx as f32) + frac - 256.0

            },
            false => {
                cfg.note0_pitch + ((pitch_f * maybe_pitch_extend_range) as f32)

            },
        }
    }
}
