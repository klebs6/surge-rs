/*
crate::ix!();

#[cfg(test)]
mod coeffmaker_tests {

    use super::*;

    /// Test `make_coeffs()` with a dummy coefficient generator that always returns a fixed set of
    /// coefficients. 
    ///
    /// Assert that the resulting `tcoeff` and `dcoeff` arrays match the expected values.
    ///
    #[test]
    fn test_make_coeffs_fixed() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);
        let mut maker = FilterCoefficientMaker::new(tuner.clone(), tables.clone(), srunit.clone());

        let fixed_coeffs = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let fixed_generator = Box::new(FixedCoeffGenerator { coeffs: fixed_coeffs });

        maker.make_coeffs(432.0, 1.0, fixed_generator);

        let expected_tcoeff = fixed_coeffs;
        let expected_dcoeff = [0.0; N_COEFFMAKER_COEFFS];

        assert_eq!(maker.tcoeff, expected_tcoeff);
        assert_eq!(maker.dcoeff, expected_dcoeff);
    }

    /// Test `make_coeffs()` with a coefficient generator that always returns coefficients outside the
    /// valid range (i.e., less than 0 or greater than 1). 
    ///
    /// Assert that `make_coeffs()` returns an error.
    ///
    #[test]
    fn test_make_coeffs_out_of_range() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);
        let mut maker = FilterCoefficientMaker::new(tuner.clone(), tables.clone(), srunit.clone());

        let out_of_range_generator = Box::new(OutOfRangeCoeffGenerator);

        let result = maker.make_coeffs(432.0, 1.0, out_of_range_generator);

        assert!(result.is_err());
    }

    /// Test `reset()` by setting some values in the `tcoeff` and `dcoeff` arrays, and then calling
    /// `reset()`. 
    ///
    /// Assert that the arrays are all filled with zeros, and that `first_run` is set to true.
    ///
    #[test]
    fn test_reset() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);
        let mut maker = FilterCoefficientMaker::new(tuner.clone(), tables.clone(), srunit.clone());

        maker.tcoeff = [0.1; N_COEFFMAKER_COEFFS];
        maker.dcoeff = [0.2; N_COEFFMAKER_COEFFS];
        maker.first_run = false;

        maker.reset();

        assert_eq!(maker.tcoeff, [0.0; N_COEFFMAKER_COEFFS]);
        assert_eq!(maker.dcoeff, [0.0; N_COEFFMAKER_COEFFS]);
        assert_eq!(maker.first_run, true);
    }

    /// In `test_make_coeffs`, we create a new `FilterCoefficientMaker` and call `make_coeffs` twice
    /// with different parameters, and then we assert that the resulting coefficient arrays have
    /// positive values.
    ///
    #[test]
    fn test_make_coeffs() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);
        let mut maker = FilterCoefficientMaker::new(tuner, tables, srunit);

        maker.make_coeffs(100.0, 1.0, Box::new(Gen1 {}));
        assert!(maker.coeff[0] > 0.0);
        assert!(maker.coeff[1] > 0.0);
        assert!(maker.coeff[2] > 0.0);

        maker.make_coeffs(1000.0, 0.5, Box::new(Gen2 {}));
        assert!(maker.coeff[0] > 0.0);
        assert!(maker.coeff[1] > 0.0);
        assert!(maker.coeff[2] > 0.0);
    }

    /// In `test_reset`, we create a new `FilterCoefficientMaker` and set its coefficient arrays to
    /// 1.0, and then we call `reset` and assert that all arrays have been reset to 0.0 and `first_run`
    /// has been set to `true`.
    ///
    #[test]
    fn test_reset2() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);
        let mut maker = FilterCoefficientMaker::new(tuner, tables, srunit);

        maker.coeff.fill(1.0);
        maker.dcoeff.fill(1.0);
        maker.tcoeff.fill(1.0);
        maker.first_run = false;

        maker.reset();
        assert!(maker.coeff.iter().all(|&c| c == 0.0));
        assert!(maker.dcoeff.iter().all(|&c| c == 0.0));
        assert!(maker.tcoeff.iter().all(|&c| c == 0.0));
        assert_eq!(maker.first_run, true);
    }

    /// In `test_from_direct`, we create a new `FilterCoefficientMaker` and call `from_direct` with an
    /// array of values, and then we assert that the resulting coefficient arrays have been set
    /// correctly, and that `dcoeff` has been set to 0.0 for all values.
    ///
    #[test]
    fn test_from_direct() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);
        let mut maker = FilterCoefficientMaker::new(tuner, tables, srunit);

        let coeffs = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        maker.from_direct(coeffs);

        for i in 0..N_COEFFMAKER_COEFFS {
            assert_eq!(maker.tcoeff[i], coeffs[i]);
            assert_eq!(maker.coeff[i], coeffs[i]);
            assert_eq!(maker.dcoeff[i], 0.0);
        }
    }

    /// The `make_coeffs()` test case checks if the `make_coeffs()` method generates the correct filter
    /// coefficients given a specific frequency and resonance value. It uses two different generators
    /// `Gen1` and `Gen2`, and compares the output coefficients with the expected values.
    ///
    #[test]
    fn make_coeffs() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);

        let mut maker  = FilterCoefficientMaker::new(
            tuner.clone(),
            tables.clone(),
            srunit.clone()
        );

        maker.make_coeffs(1000.0, 0.5, Box::new(Gen1 {}));
        assert_eq!(maker.coeff, [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);

        maker.make_coeffs(1000.0, 0.5, Box::new(Gen2 {}));
        assert_eq!(maker.coeff, [0.9641483, -1.8999989, 0.940621, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }


    /// The `from_direct()` test case checks if the `from_direct()` method correctly updates the
    /// `tcoeff`, `coeff`, and `dcoeff` arrays based on the input coefficients. It uses two different
    /// sets of coefficients, and compares the resulting arrays with the expected values.
    ///
    #[test]
    fn from_direct() {
        let srunit = SampleRateHandle::default();
        let tuner  = TunerHandle::new(&srunit);
        let tables = TablesHandle::new(&srunit);

        let mut maker  = FilterCoefficientMaker::new(
            tuner.clone(),
            tables.clone(),
            srunit.clone()
        );

        let coeffs = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        maker.from_direct(coeffs);
        assert_eq!(maker.tcoeff, coeffs);
        assert_eq!(maker.coeff, coeffs);
        assert_eq!(maker.dcoeff, [0.0; 8]);

        let coeffs = [0.5, 0.0, 0.0, 0.0, 0.2, 0.0, 0.0, 0.1];
        maker.from_direct(coeffs);
        assert_eq!(maker.tcoeff, [0.7, 0.0, 0.0, 0.0, 0.26, 0.0, 0.0, 0.14]);
        assert_eq!(maker.coeff, [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(maker.dcoeff, [
            -0.019986667, 0.0, 0.0, 0.0, -0.00028333337, 0.0, 0.0, -0.0013333333
        ]);
    }
}
*/
