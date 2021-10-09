ix!();

use crate::AllpassVerb;

#[test] pub fn allpass_smoke() {

    const S: usize = 32 ;

    let mut L: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();
    let mut R: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();

    let srunit = SampleRateHandle::new();
    let mut x = AllpassVerb::new(&srunit);

    for iter in 0..24{
        x.process(L.as_mut_ptr(), R.as_mut_ptr());
        println!("L: {:?}",L); 
        println!("R: {:?}",R); 
    }
}
