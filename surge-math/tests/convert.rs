use surge_math::*;

#[test] fn amp_linear() {
    let lin = 0.5;
    let amp = linear_to_amp(lin);
    let back = amp_to_linear(amp);
    println!("lin: {}, amp: {}", lin, amp);
    assert_eq!(lin,back);

    let amp = 0.7937005;
    let lin = amp_to_linear(amp);
    let back = linear_to_amp(lin);
    println!("amp: {}, lin: {}", amp, lin);
    assert_eq!(amp,back);
}

#[test] fn amp_db() {

    for amp in 0..100 {
        let amp = amp as f32 / 100.0;
        let db = amp_to_db(amp);
        println!("amp: {}, db: {}",amp,db);
    }

    //according to this, any dB > 0 is saturated to 1
    //is this actually what we want?
    for db in -192..96 {
        let amp: f32 = db_to_amp(db as f32);
        println!("db: {}, amp: {}",db,amp);
    }
}
