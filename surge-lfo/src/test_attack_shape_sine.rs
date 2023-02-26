crate::ix!();

#[cfg(test)]
mod test {

    use super::*;

    use rustsynth::lfo::{Lfo, LfoShape, LfoParam};

    #[test]
    fn test_attack_shape_sine_unipolar() {
        let mut lfo = Lfo::default();
        lfo.params[LfoParam::Unipolar] = 1.0.into();
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.75);
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.5);
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.25);
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.0);
    }

    #[test]
    fn test_attack_shape_sine_bipolar() {
        let mut lfo = Lfo::default();
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.0);
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.5);
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.0);
        lfo.attack_shape(LfoShape::Sine, 0.0);
        assert_eq!(lfo.phase, 0.5);
    }
}

