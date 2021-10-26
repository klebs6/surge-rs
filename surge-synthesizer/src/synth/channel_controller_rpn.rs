ix!();

use crate::SurgeSynthesizer;

impl SurgeSynthesizer<'plugin_layer,'synth_out> {

    #[inline] pub fn channel_controller_handle_rpn_nrpn(&mut self, 
        channel: u8, cc: u8, fval: &mut f32) -> usize 
    {
        let mut cc_encoded = cc;

        match cc {
            6 | 38 => {
                // handle RPN/NRPNs (untested)
                let   _tv: i32;
                let _cnum: i32;

                match self.midi_unit.nrpn_last(channel) {
                    true => {

                        let nrpn_v0 = self.midi_unit.get_nrpn_v(channel,0);
                        let nrpn_v1 = self.midi_unit.get_nrpn_v(channel,1);

                        let nrpn0 = self.midi_unit.get_nrpn(channel,0);
                        let nrpn1 = self.midi_unit.get_nrpn(channel,1);

                        _tv = (nrpn_v1 << 7) + nrpn_v0;

                        _cnum = (nrpn1 << 7) + nrpn0;

                        cc_encoded = (_cnum | (1 << 16)) as u8;
                    },
                    false => {

                        let rpn_v0 = self.midi_unit.get_rpn_v(channel,0);
                        let rpn_v1 = self.midi_unit.get_rpn_v(channel,1);

                        let rpn0 = self.midi_unit.get_rpn(channel,0);
                        let rpn1 = self.midi_unit.get_rpn(channel,1);

                        _tv = (rpn_v1 << 7) + rpn_v0;

                        _cnum = (rpn1 << 7) + rpn0;

                        cc_encoded = (_cnum | (2 << 16)) as u8;
                    },
                }

                *fval = (_tv as f32) / 16384.0;

            }, 
            _ => {},
        }
        cc_encoded.into()
    }
}
