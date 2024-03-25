/*
crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_attack_shape_square() {
        let mut lfo = Lfo::default();
        lfo.phase = 0.0;
        lfo.attack_shape_square();
        assert_eq!(lfo.output, 1.0);

        lfo.phase = 0.75;
        lfo.attack_shape_square();
        assert_eq!(lfo.output, -1.0);
    }

    #[test]
    fn test_attack_shape_ramp() {
        let mut lfo = Lfo::default();
        lfo.phase = 0.0;
        lfo.attack_shape_ramp();
        assert_eq!(lfo.output, -1.0);

        lfo.phase = 0.5;
        lfo.attack_shape_ramp();
        assert_eq!(lfo.output, 0.0);

        lfo.phase = 1.0;
        lfo.attack_shape_ramp();
        assert_eq!(lfo.output, 1.0);
    }

    #[test]
    fn test_attack_shape_envelope() {
        let mut lfo = Lfo::default();
        lfo.env_val = 0.5;
        lfo.attack_shape_envelope();
        assert_eq!(lfo.output, 0.5);

        lfo.env_val = -1.0;
        lfo.attack_shape_envelope();
        assert_eq!(lfo.output, -1.0);
    }

    #[test]
    fn test_attack_shape_sine() {
        let mut lfo = Lfo::default();
        lfo.env_phase = std::f32::consts::PI;
        lfo.attack_shape_sine();
        assert_eq!(lfo.env_val, -1.0);
    }

    #[test]
    fn test_attack_shape_tri() {
        let mut lfo = Lfo::default();
        lfo.env_phase = std::f32::consts::PI;
        lfo.attack_shape_tri();
        assert_eq!(lfo.env_val, 0.0);
    }

    #[test]
    fn test_attack_shape_square2() {
        let mut lfo = Lfo::default();
        lfo.attack_shape_square();
        assert_eq!(lfo.env_val, 1.0);
    }

    #[test]
    fn test_attack_shape_ramp() {
        let mut lfo = Lfo::default();
        lfo.attack_shape_ramp();
        assert_eq!(lfo.env_val, 0.0);
    }

    #[test]
    fn test_attack_shape_noise() {
        let mut lfo = Lfo::default();
        lfo.attack_shape_noise();
        assert!(lfo.env_val >= -1.0 && lfo.env_val <= 1.0);
    }

    #[test]
    fn test_attack_shape_snh() {
        let mut lfo = Lfo::default();
        lfo.attack_shape_snh();
        assert!(lfo.env_val >= -1.0 && lfo.env_val <= 1.0);
    }

    #[test]
    fn test_attack_shape_envelope() {
        let mut lfo = Lfo::default();
        lfo.attack_shape_envelope();
        assert_eq!(lfo.env_val, 0.0);
    }

    #[test]
    fn test_attack_shape_stepseq() {
        let mut lfo = Lfo::default();
        lfo.attack_shape_stepseq(0.0);
        assert_eq!(lfo.env_val, 0.0);
    }

    #[test]
    fn test_attack() {
        let mut lfo = Lfo::default();
        lfo.set_param(LfoParam::Shape, 1.0);
        lfo.set_param(LfoParam::Trigmode, 0.0);
        lfo.set_param(LfoParam::Delay, 0.0);
        lfo.set_param(LfoParam::Attack, 0.5);
        lfo.set_param(LfoParam::Hold, 0.0);
        lfo.set_param(LfoParam::Rate, 1.0);
        lfo.attack();
        assert_eq!(lfo.env_state, LfoEnvState::Attack);
    }
}
*/
