ix!();

use crate::{
    SurgeVoice,
    LagEntry,
    VoiceRuntimeHandle,
};

impl SurgeVoice {

    pub fn calc_levels(&mut self, cfg: VoiceRuntimeHandle) {

        let cfg = cfg.borrow();

        let o0:  f32 = amp_to_linear(cfg.oscillator_level0);
        let o1:  f32 = amp_to_linear(cfg.oscillator_level1);
        let o2:  f32 = amp_to_linear(cfg.oscillator_level2);
        let on:  f32 = amp_to_linear(cfg.noise_level0);
        let r12: f32 = amp_to_linear(cfg.ring_level0);
        let r23: f32 = amp_to_linear(cfg.ring_level1);

        let pfg: f32 = self.tables.db_to_linear(cfg.noise_level0);

        self.osclevels[LagEntry::Osc1].set_target(o0);
        self.osclevels[LagEntry::Osc2].set_target(o1);
        self.osclevels[LagEntry::Osc3].set_target(o2);
        self.osclevels[LagEntry::Noise].set_target(on);
        self.osclevels[LagEntry::Ring12].set_target(r12);
        self.osclevels[LagEntry::Ring23].set_target(r23);
        self.osclevels[LagEntry::Pfg].set_target(pfg);
    }
}
