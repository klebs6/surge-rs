use surge_math::*;

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

