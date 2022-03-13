ix!();

use crate::*;

impl SurgeVoice {

    pub fn legato(&mut self, 
        key: i32, velocity: i32, _detune: usize) 
    {
        if self.state.portaphase > 1.0 {

            self.state.portasrc_key = self.state.get_pitch() as f64;

        } else {

            let phase = self.state.portaphase;
            let interp_a = self.state.portasrc_key;
            let interp_b = self.state.get_pitch();

            self.state.portasrc_key = lerp(phase, interp_a, interp_b as f64);
        }

        self.state.key = key;

        self.mpe_unit.set_lastkey(key);

        self.state.portaphase = 0.0;

        self.state.velocity = velocity;
        self.state.fvel = ((velocity as f32) / 127.0) as f64;
    }

    pub fn update_portamento(&mut self, 
        portamento: f32, temposync: bool) 
    {
        let tsyncratio = self.get_temposyncratio();

        let maybe_temposyncratio = match temposync 
        {
            true  => tsyncratio,
            false => 1.0,
        };

        self.state.portaphase += 
            (self.tables.envelope_rate_linear(portamento) * maybe_temposyncratio) as f64;

        if self.state.portaphase < 1.0 {

            self.state.pkey = 
                (1.0 - self.state.portaphase) * 
                self.state.portasrc_key +
                self.state.portaphase * 
                self.state.get_pitch() as f64;

        } else {
            self.state.pkey = self.state.get_pitch() as f64;
        }
    }
}
