ix!();

use crate::Conditioner;

#[test] pub fn conditioner_smoke() {

    const S: usize = 32;

    let mut L: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(x as f64, 0.1) as f32).collect();
    let mut R: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(x as f64, 0.1) as f32).collect();

    println!("L: {:?}",L); 
    println!("R: {:?}",R); 

    let srunit   = SampleRateHandle::new();
    let tuner    = TunerHandle::new(&srunit);
    let tables   = TablesHandle::new(&srunit);

    let mut x    = Conditioner::new(&tuner, &tables, &srunit);

    for iter in 0..24{
        x.process(L.as_mut_ptr(), R.as_mut_ptr());
        println!("L: {:?}",L); 
        println!("R: {:?}",R); 
    }
}

