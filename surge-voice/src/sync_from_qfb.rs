ix!();

use crate::{
    SurgeVoice,
};

pub struct SyncQFBRegistersCfg {
    filterunit_type:  [FilterType; 2],
    filterblock_cfg:  FilterBlockConfiguration,
}

impl SurgeVoice<'sr> {

    /// get the updated registers from the QuadFB
    pub fn sync_registers_from_qfb(&mut self, cfg: &SyncQFBRegistersCfg) {
        for u in 0..2 {

            if cfg.filterunit_type[u] != FilterType::Off {

                for reg_idx in 0..N_FILTER_REGISTERS {
                    unsafe {
                        self.fbp.fu[u].reg[reg_idx] = 
                            get1f(
                                &(*self.filterblock_state.fbq).unit_state[u].reg[reg_idx], 
                                self.filterblock_state.fbqi
                            );
                    }
                }

                for coeff_idx in 0..N_COEFFMAKER_COEFFS {
                    unsafe {
                        self.coeffmaker[u].coeff[coeff_idx] = 
                            get1f(
                                &(*self.filterblock_state.fbq).unit_state[u].coeff[coeff_idx], 
                                self.filterblock_state.fbqi);
                    }
                }

                unsafe {
                    self.fbp.fu[u].comb_write_position = (*self.filterblock_state.fbq).unit_state[u].comb_write_position[
                        self.filterblock_state.fbqi as usize
                    ] as u32;
                }

                let fbc = cfg.filterblock_cfg;

                if fbc.is_wide() {
                    for reg_idx in 0..N_FILTER_REGISTERS {
                        unsafe {
                            self.fbp.fu[u + 2].reg[reg_idx] = get1f(
                                &(*self.filterblock_state.fbq).unit_state[u + 2].reg[reg_idx], 
                                self.filterblock_state.fbqi
                            );
                        }
                    }
                }
            }
        }

        unsafe{ 
            self.fbp.feedback_line_l = get1f(&(*self.filterblock_state.fbq).feedback_line_l, 
                self.filterblock_state.fbqi);

            self.fbp.feedback_line_r = get1f(&(*self.filterblock_state.fbq).feedback_line_r, 
                self.filterblock_state.fbqi);

            self.fbp.ws_lpf   = get1f(&(*self.filterblock_state.fbq).ws_lpf,   
                self.filterblock_state.fbqi);
        }
    }
}
