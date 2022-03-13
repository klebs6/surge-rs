ix!();

use crate::*;

impl SurgeVoice {

    pub fn maybe_toggle_filter(
        &mut self, 
        cfg: VoiceRuntimeHandle) 
    {
        let cfg = cfg.borrow();

        for u in 0_usize..2_usize 
        {
            let f_ty     = cfg.filterunit_filtertype[u];
            let f_subty  = cfg.filterunit_filtersubtype[u];

            let fu_ty    = self.fbp.fu[u].ty;
            let fu_subty = self.fbp.fu[u].subty;

            if (f_ty != fu_ty) || (f_subty != fu_subty) {

                self.fbp.fu[u].init();

                self.fbp.fu[u].ty    = f_ty;
                self.fbp.fu[u].subty = f_subty;

                let fbc = cfg.filterblock_cfg;

                if fbc.is_wide() {
                    self.fbp.fu[u+2].init();
                    self.fbp.fu[u+2].ty    = f_ty;
                    self.fbp.fu[u+2].subty = f_subty;
                }

                self.coeffmaker[u].reset();
            }
        }
    }
}
