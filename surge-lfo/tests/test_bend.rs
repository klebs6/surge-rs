/*
crate::ix!();

/// These tests check that the bend functions produce the expected output for different input
/// values and different deform values. 
///
/// They use a small epsilon value to account for floating point precision errors.
#[cfg(test)]
mod tests {

    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn test_bend1() {
        let mut lfo = Lfo::new();
        lfo.set_param(LfoParam::Deform, 0);
        assert!((lfo.bend1(0.0) - 0.25).abs() < EPSILON);
        assert!((lfo.bend1(1.0) - 1.25).abs() < EPSILON);
        lfo.set_param(LfoParam::Deform, 1);
        assert!((lfo.bend1(0.0) - 0.25).abs() < EPSILON);
        assert!((lfo.bend1(1.0) - 1.25).abs() < EPSILON);
    }

    #[test]
    fn test_bend2() {
        let mut lfo = Lfo::new();
        lfo.set_param(LfoParam::Deform, 0);
        assert!((lfo.bend2(0.0) - 0.0).abs() < EPSILON);
        assert!((lfo.bend2(1.0) - 0.0).abs() < EPSILON);
        lfo.set_param(LfoParam::Deform, 1);
        assert!((lfo.bend2(0.0) - 0.0).abs() < EPSILON);
        assert!((lfo.bend2(1.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_bend3() {
        let mut lfo = Lfo::new();
        lfo.set_param(LfoParam::Deform, 0);
        assert!((lfo.bend3(0.0) - 0.0).abs() < EPSILON);
        assert!((lfo.bend3(1.0) - 1.0).abs() < EPSILON);
        lfo.set_param(LfoParam::Deform, 1);
        assert!((lfo.bend3(0.0) - 0.0).abs() < EPSILON);
        assert!((lfo.bend3(1.0) - 1.0).abs() < EPSILON);
    }
}


*/
