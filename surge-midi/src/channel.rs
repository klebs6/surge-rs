ix!();

#[derive(Debug)]
pub struct MidiKeyState {
    pub keystate:   i32,
    pub lastdetune: usize,
}

#[derive(Debug)]
pub struct MidiChannelState {
    pub key_state:               [MidiKeyState; 128],

    // are these per scene? if so, remove
    pub nrpn:                    [i32; 2],
    pub nrpn_v:                  [i32; 2],
    pub rpn:                     [i32; 2],
    pub rpn_v:                   [i32; 2],

    pub pitchbend:               i32,
    pub nrpn_last:               bool,
    pub hold:                    bool,
    pub pan:                     f32,
    pub pitchbend_in_semitones:  PitchBendValue,
    pub pressure:                f32,
    pub timbre:                  f32,
}

default_default![MidiKeyState];
default_default![MidiChannelState];


