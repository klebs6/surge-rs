ix!();

use crate::{
    SurgeVoice,
};

impl SurgeVoice {

    pub fn maybe_toggle_fm(&mut self, 
        cfg: &FmConfiguration) 
    {
        let some_o0 = self.osc[0].is_some();
        let some_o1 = self.osc[1].is_some();
        let some_o2 = self.osc[2].is_some();

        match cfg {
            FmConfiguration::Off => {

            },
            FmConfiguration::OneToZero => {
                if some_o0 && some_o1 
                {
                    let o1out = match self.osc[1] {
                        Some(ref mut osc) => osc.out_l(),
                        _ => unreachable!(),
                    };

                    if let Some(ref mut osc) = &mut self.osc[0] {
                        osc.assign_fm(o1out);
                    }
                }
            },
            FmConfiguration::TwoToOneToZero => {
                if some_o0 && some_o1 {

                    let o1out = match self.osc[1] {
                        Some(ref mut osc) => osc.out_l(),
                        _ => unreachable!(),
                    };

                    if let Some(ref mut osc) = &mut self.osc[0] {
                        osc.assign_fm(o1out);
                    }
                }
                if some_o1 && some_o2 {

                    let o2out = match self.osc[2] {
                        Some(ref mut osc) => osc.out_l(),
                        _ => unreachable!(),
                    };

                    if let Some(ref mut osc) = &mut self.osc[1] {
                        osc.assign_fm(o2out);
                    }
                }
            },
            FmConfiguration::OneAndTwoToZero => {
                if some_o0 {
                    if let Some(ref mut osc) = &mut self.osc[0] {
                        osc.assign_fm(self.fmbuffer.as_mut_ptr());
                    }
                }
            },
        }
    }
}
