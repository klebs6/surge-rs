use surge_math::*;
use surge_imports::rand;

#[test] fn test_rand1() {

    for _ in 0..10 {
        let f01 = rand01();
        assert!(f01 <= 1.0);
        assert!(f01 >= 0.0);

        let f11 = rand11();
        assert!(f11 <= 1.0);
        assert!(f11 >= -1.0);
    }
}

#[test] fn test_rand2() {

    use rand::prelude::*;
    use rand::thread_rng;

    let mut rng = thread_rng();

    let rand11 = rand11();

    let rand11_2: f32 =  (rng.gen::<f32>() * 2.0) - 1.0;

    println!("rand11_2: {:?}",rand11_2);

    println!("rand11: {:?}",rand11);

    assert!((rand11_2.abs() - rand11.abs()).abs() < 1.0);
}

