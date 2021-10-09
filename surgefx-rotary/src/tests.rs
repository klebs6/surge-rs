ix!();

#[test] fn rotary_smoke() {

    const s: usize = 32;

    let mut l: Vec<f32> = (0..s).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();
    let mut r: Vec<f32> = (0..s).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();

    println!("l: {:?}",l); 
    println!("r: {:?}",r); 

    let srunit    = SampleRateHandle::new();
    let tuner     = TunerHandle::new(&srunit);
    let tables    = TablesHandle::new(&srunit);
    let timeunit  = TimeUnitHandle::new(&srunit);

    let mut x     = crate::RotarySpeaker::new(&tuner, &tables, &srunit, &timeunit);
    x.init();

    x.params[crate::RotarySpeakerParam::HornRate].val = PData::Float(1.0);

    for _iter in 0..24{
        x.process(l.as_mut_ptr(), r.as_mut_ptr());
        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
}

