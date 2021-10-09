ix!();

use crate::{DualDelay,DualDelayParam};

#[test] fn dual_delay_smoke() {

    const S: usize = 32;

    let mut l: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, x as f64) as f32).collect();
    let mut r: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, x as f64) as f32).collect();

    println!("l: {:?}",l); 
    println!("r: {:?}",r); 

    let srunit    = SampleRateHandle::new();
    let tuner     = TunerHandle::new(&srunit);
    let tables    = TablesHandle::new(&srunit);
    let time_unit = TimeUnitHandle::new(&srunit);

    let mut x     = DualDelay::new(&tuner, &tables, &srunit, &time_unit);

    x.params[DualDelayParam::Mix].val = PData::Float(0.7);

    for _iter in 0..24{
        x.process(l.as_mut_ptr(), r.as_mut_ptr());
        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
}
