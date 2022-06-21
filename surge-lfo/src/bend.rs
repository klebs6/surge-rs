crate::ix!();

impl Lfo {

    pub fn bend1(&mut self, mut x: f32) -> f32 {

        let deform_f = pvali![self.params[LfoParam::Deform]] as f32;

        let a: f32 = 0.5 * deform_f;
        x += 0.25;
        x += a * (x * 2.0 * PI_32).sin() / ((2.0 * PI_32) as f32);
        x -= 0.25;
        x
    }

    pub fn bend2(&mut self, mut x: f32) -> f32 {

        let deform_f = pvali![self.params[LfoParam::Deform]] as f32;

        let a: f32 = 0.5 * deform_f;
        x += a * (x * 2.0 * PI_32).sin() / ((2.0 * PI_32) as f32);
        x
    }

    pub fn bend3(&mut self, mut x: f32) -> f32 {

        let deform_f = pvali![self.params[LfoParam::Deform]] as f32;

        let a: f32 = 0.5 * deform_f;
        x = x - a * x * x + a;
        x = x - a * x * x + a; 
        x
    }
}
