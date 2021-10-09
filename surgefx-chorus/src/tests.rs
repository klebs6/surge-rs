ix!();

use crate::Chorus;

#[test] pub fn chorus_smoke() {
    const S: usize = 32 ;

    let mut L: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();
    let mut R: Vec<f32> = (0..S).map(|x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();

    let srunit   = SampleRateHandle::new();
    let tuner    = TunerHandle::new(&srunit);
    let tables   = TablesHandle::new(&srunit);
    let timeunit = TimeUnitHandle::new(&srunit);

    let mut x    = Chorus::new(&tuner, &tables, &srunit, &timeunit);

    for iter in 0..24{
        x.process(L.as_mut_ptr(), R.as_mut_ptr());
        println!("L: {:?}",L); 
        println!("R: {:?}",R); 
    }
}
