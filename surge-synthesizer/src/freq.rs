ix!();

pub const TAU: f64 = PI * 2.0;

/**
  | Convert the midi note's pitch into the
  | equivalent frequency.
  |
  | This function assumes A4 is 440hz.
  */
pub fn midi_pitch_to_freq(note: u8) -> f64 {
    assert!(note <= 127);

    // Midi notes can be 0-127
    ((note as i8 - A4_MIDI_NOTE) as f64 / 12.0).exp2() * A4_FREQ
}
