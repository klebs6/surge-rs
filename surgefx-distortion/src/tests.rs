
#[test] fn distortion_smoke() {

    ix!();
    const N: usize = 32;

    let mut l: Vec<f32> = (0..N).map(|x| surge_math::correlated_noise(0.0, x as f64) as f32).collect();
    let mut r: Vec<f32> = (0..N).map(|x| surge_math::correlated_noise(0.0, x as f64) as f32).collect();

    println!("l: {:?}",l); 
    println!("r: {:?}",r); 

    let srunit   = SampleRateHandle::default();
    let tuner    = TunerHandle::new(&srunit);
    let tables   = TablesHandle::new(&srunit);

    let mut x    = crate::Distortion::new(&tuner, &tables, &srunit);

    for _ in 0..24 {

        //this is broken 
        x.process::<N>(
            l.as_mut_slice().try_into().unwrap(), 
            r.as_mut_slice().try_into().unwrap()
        );

        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
}
