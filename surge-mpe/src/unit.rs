ix!();

#[derive(Debug)]
pub struct MPEUnit<'sr> {
    pub enabled:                   MpeEnableSwitch,
    pub num_voices:                NumVoices,
    pub pitchbend_range:           PitchBendRange,
    pub global_pitchbend_range:    PitchBendRange,
    pub pitchbend:                 PitchBendValue,
    pub poly_aftertouch:           [f32; 128],
    pub last_key:                  i32,
    pub phantom:                   PhantomData<&'sr u8>,
}

impl Init for MPEUnit<'sr> {
    fn init(&mut self) {
        self.last_key = 60;
        self.pitchbend = PitchBendValue(0.0);
        for cc in 0..128 {
            self.poly_aftertouch[cc] = 0.0;
        }
    }
}

impl Default for MPEUnit<'sr> {
    fn default() -> Self {
        let mut x = Self {
            enabled:                   MpeEnableSwitch(false),
            num_voices:                NumVoices(0),

            //getUserDefaultValue(&storage, "mpePitchBendRange", 48);
            pitchbend_range:          PitchBendRange(48.0),
            global_pitchbend_range:    PitchBendRange(0.0),
            pitchbend:                PitchBendValue(0.0),
            poly_aftertouch:           [0.0; 128],
            last_key:                  0,
            phantom:                   Default::default(),
        };
        x.init();
        x
    }
}

impl MPEUnit<'sr> {
    pub fn get_mpe_main_channel(&self, voice_channel: u8, _key: u8) -> u8 {
        match self.enabled {
            MpeEnableSwitch(true)  => 0,
            MpeEnableSwitch(false) => voice_channel,
        }
    }
}
