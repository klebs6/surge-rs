ix!();

use crate::{RingModulator,RingModulatorParam};

#[test] fn ringmod_smoke() {

    const S: usize = 32;

    let mut l: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();
    let mut r: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, x as f64 / 16.0) as f32).collect();

    println!("l: {:?}",l); 
    println!("r: {:?}",r); 

    let srunit    = SampleRateHandle::new();
    let tuner     = TunerHandle::new(&srunit);
    let tables    = TablesHandle::new(&srunit);
    let timeunit  = TimeUnitHandle::new(&srunit);

    let mut x     = RingModulator::new(&tuner, &tables, &srunit);

    x.init();

    x.params[RingModulatorParam::CarrierFreq].val = PData::Float(100.0);

    for iter in 0..24{
        x.process(l.as_mut_ptr(), r.as_mut_ptr());
        println!("l: {:?}",l); 
        println!("r: {:?}",r); 
    }
}





