ix!();

use crate::{
    SurgeSynthesizer,
};

impl SurgeSynthesizer<,'plugin_layer,'synth_out> {

    pub fn channel_aftertouch(&mut self, channel: u8, value: i32) {

        let fval: f32 = (value as f32) / 127.0;

        self.midi_unit.set_pressure(channel,fval);

        if !self.mpe_unit.enabled() || channel == 0 {

            for scene in self.active_patch.scene.iter_mut() {
                scene.set_channel_aftertouch_target(fval);
            }
        }
    }

    pub fn poly_aftertouch(&mut self, _channel: u8, key: u8, value: u8) {

        let idx       = (key & 127) as usize;
        let fval: f32 = (value as f32) / 127.0;

        for scene in self.active_patch.scene.iter_mut() {
            scene.mpe_unit.set_poly_aftertouch(idx, fval);
        }
    }
}
