ix!();

use crate::BiquadFilter;
use crate::ProcessBlockStereo;

#[test] fn smoke() {
    const S: usize = 32;

    let mut l: Vec::<f32> = (0..S).map(|_x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();
    let mut r: Vec::<f32> = (0..S).map(|_x| surge_math::correlated_noise(0.0, 0.0) as f32).collect();

    let srunit          = SampleRateHandle::default();
    let tuner           = TunerHandle::new(&srunit);
    let tables          = TablesHandle::new(&srunit);

    let mut biquad      = BiquadFilter::new(&tuner,&tables,&srunit);

    biquad.coeff_instantize();
    biquad.coeff_process();

    unsafe {
        biquad.process_block_stereo(
            l.as_mut_ptr(),
            r.as_mut_ptr(),
            None
        );
    }

    println!("biquad: {:?}",
        (biquad.a1, biquad.a2, biquad.b0, biquad.b1, biquad.b2, biquad.reg0, biquad.reg1)
    );
}
