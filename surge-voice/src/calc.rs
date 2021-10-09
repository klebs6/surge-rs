ix!();

use crate::{
    SurgeVoice,
    VoiceRuntimeHandle,
};

impl<'sr> SurgeVoice<'sr> {

    pub fn calc_ctrldata<const B: bool>(&mut self, 
        cfg: VoiceRuntimeHandle<'sr>,
        qfcs: Option<&mut QuadFilterChainState>, e: i32) 
    {
        self.calc_lfos(cfg.clone());
        self.calc_envelopes();
        self.calc_modsources(cfg.clone());
        self.maybe_calc_mpe(cfg.clone());

        {
            let cfg = cfg.borrow();

            self.update_portamento(
                cfg.portamento, 
                cfg.portamento_temposync
            );
        }
        self.calc_pitch(cfg.clone());
        self.calc_keytrack(cfg.clone());
        self.maybe_calc_poly_aftertouch(cfg.clone());
        self.calc_levels(cfg.clone());
        self.calc_routes(cfg.clone());
        self.calc_pan(cfg.clone(), qfcs, e);
    }
}
