ix!();

#[cfg(test)]
pub mod test {

    use crate::*;

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

        use ::rand::prelude::*;
        use ::rand::thread_rng;

        let mut rng = thread_rng();

        let rand11 = rand11();

        let rand11_2: f32 =  (rng.gen::<f32>() * 2.0) - 1.0;

        println!("rand11_2: {:?}",rand11_2);

        println!("rand11: {:?}",rand11);

        assert!((rand11_2.abs() - rand11.abs()).abs() < 1.0);
    }

    #[test] fn test_lerp() {
        let result = lerp(0.25, 5.0, 7.0);
        approx_eq::assert_approx_eq!(result, 5.5);
    }

    #[test] fn test_within_range() {

        let result = within_range(3,5,10);
        assert!(result);

        let result = within_range(3,15,10);
        assert!(!result);
    }

    #[test] fn test_clamp01() {

        let x: f64 = 1e-31;
        assert!(clamp01(x) == 1e-31);

        let x: f64 = -0.3;
        assert!(clamp01(x) == 0.0);

        let x: f64 = 1.3;
        assert!(clamp01(x) == 1.0);
    }

    #[test] fn test_clamp1bp() {
        let x: f64 = 1e-31;
        assert!(clamp1bp(x) == 1e-31);

        let x: f64 = -0.3;
        assert!(clamp1bp(x) == -0.3);

        let x: f64 = 1.3;
        assert!(clamp1bp(x) == 1.0);
    }

    #[test] fn test_flush_denormal() {
        let mut f: f64 = 1e-31;
        flush_denormal(&mut f);
        assert!(f == 0.0);

        let mut f2: f64 = 1e-29;
        flush_denormal(&mut f2);
        assert!(f2 != 0.0);
    }

    #[test] fn test_split_float() {
        let f: f32 = 5.39239;
        let (integral, fractional) = split_float(f);
        let fractional_diff = fractional.abs() - 0.39239; //fp error
        println!("integral:        {}",integral);
        println!("fractional:      {}",fractional);
        println!("fractional_diff: {}",fractional_diff);

        assert_eq!(integral, 5.0);
        assert!( 0.000001 > fractional_diff); 
    }

    #[test] fn test_split_float_negative() {
        let f: f32 = -5.39239;
        let (integral, fractional) = split_float(f);
        let fractional_diff = fractional.abs() - 0.39239; //fp error
        println!("integral:        {}",integral);
        println!("fractional:      {}",fractional);
        println!("fractional_diff: {}",fractional_diff);

        assert_eq!(integral, -5.0);
        assert!( 0.000001 > fractional_diff); 
    }

    //______________________________________________________

    #[cfg(target_arch = "x86_64")] 
    #[test] 
    fn test_tanh_fast_sse() 
    {
        let x1 = tanh_fast32(0.5) as f64;
        let x2 = tanh_fast(0.5);
        println!("x1: {:?}", x1);
        println!("x2: {:?}", x2);

        //TODO: what is our desired tolerance?
        //fails on f64::EPSILON
        approx::assert_relative_eq!(
            x1, x2, epsilon = 0.0001);
    }

    #[test] fn test_tanh_fast() 
    {
        for inc in 0 .. 100 {
            let val = inc as f64 / 100.0;
            let x1 = tanh_faster(val);
            let x2 = tanh_fast(val);
            println!("x1: {:?}", x1);
            println!("x2: {:?}", x2);

            //near 1.0, the error from tanh_faster blows up
            assert!((x1.abs() - x2.abs()).abs() < 0.07 );
        }
    }

    #[test] fn test_trixpan() {

        use crate::trixpan;

        let mut l = 0.1;
        let mut r = 0.2;
        let x = -1.0;
        trixpan(&mut l, &mut r, x);
        println!("l: {:?}", l);
        println!("r: {:?}", r);
        assert!(r == 0.0);
        assert!(l == 0.1 + 0.2);

        let mut l = 0.1;
        let mut r = 0.2;
        let x = 1.0;
        trixpan(&mut l, &mut r, x);
        println!("l: {:?}", l);
        println!("r: {:?}", r);
        assert!(l == 0.0);
        assert!(r == 0.1 + 0.2);
    }

    #[test] fn test_log() {
        {
            let a: f32 = 1.5_f32.log(10.0) / 2.0_f32.log(10.0);
            let b: f32 = 1.5_f32.log2();
            assert_eq!(a,b);
        }
        { 
            let a: f64 = 1.333333333333333333_f64.log(10.0) / 2.0_f64.log(10.0);
            let b: f64 = 1.333333333333333333_f64.log2();
            assert_eq!(a,b);
        }
    }
}

