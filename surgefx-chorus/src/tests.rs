crate::ix!();

#[test] pub fn chorus_smoke() {

    const N: usize = 32 ;

    let mut l: Vec<f32> = (0..N).map(|_x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();
    let mut r: Vec<f32> = (0..N).map(|_x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();

    let srunit   = SampleRateHandle::default();
    let tuner    = TunerHandle::new(&srunit);
    let tables   = TablesHandle::new(&srunit);
    let timeunit = TimeUnitHandle::new(&srunit);

    let mut x    = Chorus::new::<N>(&tuner, &tables, &srunit, &timeunit);

    for _ in 0..24{

        x.process::<N>(
            l.as_mut_slice().try_into().unwrap(), 
            r.as_mut_slice().try_into().unwrap()
        );

        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
}
