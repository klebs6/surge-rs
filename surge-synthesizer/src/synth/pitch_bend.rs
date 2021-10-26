ix!();

use crate::SurgeSynthesizer;

impl SurgeSynthesizer<,'plugin_layer,'synth_out> {

    pub fn pitchbend(&mut self, channel: usize, value: i32) {

        if self.mpe_unit.enabled().0 {

            let bend_normalized: PitchBendValue = PitchBendValue((value as f32) / 8192.0);

            self.midi_unit.set_pitchbend(channel as u8,value);

            self.midi_unit.set_pitchbend_semitones(
                channel as u8, 
                bend_normalized.0 * self.mpe_unit.global_pitchbend_range().0
            );

            if channel == 0 {

                self.midi_unit.set_pitchbend_semitones(
                    channel as u8, 
                    bend_normalized.0 * self.mpe_unit.global_pitchbend_range().0
                );

            } else {

                self.midi_unit.set_pitchbend_semitones(
                    channel as u8, 
                    bend_normalized.0 * self.mpe_unit.pitchbend_range().0
                );
            }
        }

        /*
          | So here's the thing. We want global
          | pitch bend modulation to work for other
          | things in MPE mode.
          |
          | This code has beenhere forever. But
          | that means we need to ignore the
          | channel[0] mpe pitchbend elsewhere,
          | especially since the range was
          | hardwired to 2 (but is now 0). As far
          | as I know the main MPE devices don't
          | have a global pitch bend anyway so this
          | just screws up regular keyboards
          | sending channel 0 pitch bend in MPE
          | mode.
          */
        if !self.mpe_unit.enabled() || channel == 0 {

            self.mpe_unit.set_pitchbend((value as f32) / 8192.0);

            for scene in self.active_patch.scene.iter_mut() {
                match &mut scene.modsources[ ModSource::PitchBend ] {
                    Some(box ModulationSource::ControllerModulationSource(ref mut inner)) => 
                        inner.set_target(self.mpe_unit.pitchbend().0 as f64),
                    _ => unreachable!(),
                };
            }
        }
    }
}
