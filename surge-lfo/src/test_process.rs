crate::ix!();

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lfo::{LfoEnvState, LfoParam, LfoShape, TimeUnit};

    #[test]
    fn test_lfo_process_initializes_phase() {
        let mut lfo = Lfo::new(44100.0, TimeUnit::Milliseconds);
        lfo.set_param(LfoParam::StartPhase, 0.5);
        lfo.process();
        assert_eq!(lfo.phase_initialized, true);
        assert_eq!(lfo.phase, 0.5);
    }

    #[test]
    fn test_lfo_process_resets_retriggers() {
        let mut lfo = Lfo::new(44100.0, TimeUnit::Milliseconds);
        lfo.trigger = true;
        lfo.retrig_count = 1;
        lfo.process();
        assert_eq!(lfo.trigger, false);
        assert_eq!(lfo.retrig_count, 0);
    }

    #[test]
    fn test_lfo_process_determines_shape() {
        let mut lfo = Lfo::new(44100.0, TimeUnit::Milliseconds);
        lfo.set_param(LfoParam::Shape, 2.0);
        lfo.process();
        assert_eq!(lfo.get_shape(), LfoShape::Ramp);
    }

    #[test]
    fn test_lfo_process_calculates_tempo_synced_phase() {
        let mut lfo = Lfo::new(44100.0, TimeUnit::Milliseconds);
        lfo.set_param(LfoParam::TempoSync, 4.0);
        lfo.set_param(LfoParam::Rate, 1.0);
        lfo.process();
        assert_eq!(lfo.phase, 0.25);
    }

    #[test]
    fn test_lfo_process_updates_envelope_if_not_stuck() {
        let mut lfo = Lfo::new(44100.0, TimeUnit::Milliseconds);
        lfo.env_state = LfoEnvState::Attack;
        lfo.process();
        assert_ne!(lfo.env_val, 0.0);
    }
}
