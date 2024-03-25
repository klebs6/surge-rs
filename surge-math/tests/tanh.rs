use surge_math::*;

/*
#[cfg(target_arch = "x86_64")] 
#[test] 
fn test_tanh_fast_sse() 
{
    use surge_imports::assert_approx_eq;
    let x1 = tanh_fast32(0.5) as f64;
    let x2 = tanh_fast(0.5);
    println!("x1: {:?}", x1);
    println!("x2: {:?}", x2);

    //TODO: what is our desired tolerance?
    //fails on f64::EPSILON
    assert_approx_eq!(x1, x2, 0.0001);
}
*/

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
