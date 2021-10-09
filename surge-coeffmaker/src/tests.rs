ix!();

#[test] fn smoke() {

    let srunit = SampleRateHandle::default();
    let tuner  = TunerHandle::new(&srunit);
    let tables = TablesHandle::new(&srunit);

    let mut maker  = crate::FilterCoefficientMaker::new(
        tuner.clone(),
        tables.clone(),
        srunit.clone()
    );

    //todo!();
    //maker.make_coeffs(432.0, 1.0, gen1);
    //maker.make_coeffs(432.0, 1.0, gen2);

    println!("maker C: {:?}",  maker.coeff);
    println!("maker dC: {:?}", maker.dcoeff);
    println!("maker tC: {:?}", maker.tcoeff);

    maker.reset();
}
