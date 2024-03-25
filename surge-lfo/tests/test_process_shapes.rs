/*
crate::ix!();

/// Note: The test cases assume that the `Lfo` instance has been initialized and that the relevant
/// parameters and variables have been set appropriately.
///
#[cfg(test)]
mod tests {

    #[test] fn test_process_shape_sine() {
        let mut lfo = Lfo::new();
        lfo.process_shape_sine();
        assert_eq!(lfo.iout, lfo.bend3(lfo.tables.lookup_waveshape_warp(3, 2.0 - 4.0 * lfo.phase)));
    }

    #[test] fn test_process_shape_tri() {
        let mut lfo = Lfo::new();
        lfo.process_shape_tri();
        let expected = if lfo.phase > 0.5 {
            1.0 - lfo.phase
        } else {
            lfo.phase
        };
        assert_eq!(lfo.iout, lfo.bend3(-1.0 + 4.0 * expected));
    }

    #[test] fn test_process_shape_square() {
        let mut lfo = Lfo::new();
        lfo.process_shape_square();
        let deform = pvalf![lfo.params[LfoParam::Deform]];
        let expected = if lfo.phase > (0.5 + 0.5 * deform) {
            -1.0
        } else {
            1.0
        };
        assert_eq!(lfo.iout, expected);
    }

    #[test] fn test_process_shape_ramp() {
        let mut lfo = Lfo::new();
        lfo.process_shape_ramp();
        assert_eq!(lfo.iout, lfo.bend3(1.0 - 2.0 * lfo.phase));
    }

    #[test] fn test_process_shape_noise() {
        let mut lfo = Lfo::new();
        lfo.process_shape_noise();
        assert_eq!(lfo.iout, cubic_interpolate(lfo.wf_history[3], lfo.wf_history[2], lfo.wf_history[1], lfo.wf_history[0], lfo.phase));
    }

    #[test] fn test_process_shape_snh() {
        let mut lfo = Lfo::new();
        lfo.process_shape_snh();
        // no-op
    }

    #[test] fn test_process_shape_envelope() {
        let mut lfo = Lfo::new();
        lfo.params[LfoParam::Deform] = 0.5; // set deformation parameter to 0.5
        lfo.env_val = 0.75; // set envelope value to 0.75
        lfo.process_shape_envelope();
        assert_eq!(lfo.iout, 0.625);
    }
}
*/
