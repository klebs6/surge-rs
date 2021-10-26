ix!();

use crate::{
    Conditioner,
    ConditionerParam,
};

impl Update for Conditioner {

    fn update(&mut self) {
        self.update_bands();
    }
}

impl Conditioner {

    pub fn update_bands(&mut self) {

        self.band1.coeff_peak_eq(
            self.band1.calc_omega(-2.5), 
            2.0, 
            self.pvalf(ConditionerParam::Bass) as f64
        );

        self.band2.coeff_peak_eq(
            self.band2.calc_omega(4.75), 
            2.0, 
            self.pvalf(ConditionerParam::Treble) as f64
        );
    }
}
