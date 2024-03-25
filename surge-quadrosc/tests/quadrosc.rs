use surge_quadrosc::*;

#[test] fn api_smoke() {

    let mut r: f64 = 0.5;
    let mut p: f64 = 0.0;

    let mut quadr = QuadrOsc::new();
    println!("created quadr: {:?}",quadr);

    println!("setting rate {}",r); quadr.set_rate(r);
    println!("quadr: {:?}",quadr);

    println!("setting phase {}",p); quadr.set_phase(p);
    println!("quadr: {:?}",quadr);

    for _i in 0..4 {
        quadr.process();
        println!("quadr: {:?}",quadr);
    }

    r = core::f64::consts::PI;
    println!("setting rate {}",r); quadr.set_rate(r);
    println!("quadr: {:?}",quadr);

    for _i in 0..4 {
        quadr.process();
        println!("quadr: {:?}",quadr);
    }

    r = core::f64::consts::PI + 0.1;
    println!("setting rate {}",r); quadr.set_rate(r);
    println!("quadr: {:?}",quadr);

    for _i in 0..4 {
        quadr.process();
        println!("quadr: {:?}",quadr);
    }

    p = 0.5;
    println!("setting phase {}",p); quadr.set_phase(p);
    println!("quadr: {:?}",quadr);

    for _i in 0..4 {
        quadr.process();
        println!("quadr: {:?}",quadr);
    }
}
