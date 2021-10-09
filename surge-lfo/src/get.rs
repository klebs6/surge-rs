ix!();

use crate::{
    Lfo,
    LfoParam,
};

impl Lfo<'sr> {

    #[inline] pub fn get_shape(&self) -> LfoShape {
        LfoShape::try_from(
            pvali![self.params[LfoParam::Shape]]
            as usize).unwrap()
    }

    #[inline] pub fn get_mode(&self) -> LfoMode {
        LfoMode::try_from(
            pvali![self.params[LfoParam::Trigmode]]
            as usize).unwrap()
    }

    #[inline] pub fn get_rate(&self, temposyncratio: f32) -> f32 {

        let mut rate = self.tables.envelope_rate_linear(
            pvalf![self.params[LfoParam::Rate]]
        );

        if self.params[LfoParam::Rate].temposync  {
            rate *=  temposyncratio;
        }

        rate
    }
}
