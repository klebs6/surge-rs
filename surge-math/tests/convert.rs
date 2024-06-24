use surge_math::*;
use surge_imports::*;
use surge_errors::*;

#[disable]
#[traced_test] 
fn amp_linear() -> Result<(), ConvertError> {

    use approx::assert_abs_diff_eq;

    let lin  = 0.5;
    let amp  = linear_to_amp(lin);
    let back = amp_to_linear(amp);

    println!("lin: {}, amp: {}", lin, amp);
    assert_abs_diff_eq!(lin, back, epsilon = 1e-6);

    let amp  = 0.7937005;
    let lin  = amp_to_linear(amp);
    let back = linear_to_amp(lin);

    println!("amp: {}, lin: {}", amp, lin);
    assert_abs_diff_eq!(amp, back, epsilon = 1e-6);

    Ok(())
}

#[disable]
#[traced_test] 
fn amp_db() -> Result<(), ConvertError> {

    for amp in 0..100 {
        let amp = amp as f32 / 100.0;
        let db = amp_to_db(amp);
        println!("amp: {}, db: {}", amp, db);
    }

    // according to this, any dB > 0 is saturated to 1
    // is this actually what we want?
    for db in -192..96 {
        let amp: f32 = db_to_amp(db as f32);
        println!("db: {}, amp: {}", db, amp);
    }
    Ok(())
}

#[disable]
#[traced_test]
fn test_amp_to_linear() -> Result<(), ConvertError> {
    assert_eq!(amp_to_linear(0.0), 0.0);
    assert_eq!(amp_to_linear(1.0), 1.0);
    assert_eq!(amp_to_linear(2.0), 8.0);
    assert_eq!(amp_to_linear(-1.0), 0.0); // Clamped to 0
    Ok(())
}

#[disable]
#[traced_test]
fn test_linear_to_amp() -> Result<(), ConvertError> {
    use approx::assert_abs_diff_eq;

    assert_eq!(linear_to_amp(0.0), 0.0);
    assert_eq!(linear_to_amp(1.0), 1.0);
    assert_abs_diff_eq!(linear_to_amp(0.0000000001), 0.0000000464, epsilon = 1e-6); // Approximation of cube root
    assert_eq!(linear_to_amp(8.0), 2.0); // Cube root of 8
    Ok(())
}

#[disable]
#[traced_test]
fn test_amp_to_db() -> Result<(), ConvertError> {
    use approx::assert_abs_diff_eq;

    assert_eq!(amp_to_db(0.0), -192.0); // Clamped to -192
    assert_eq!(amp_to_db(1.0), 0.0);
    assert_abs_diff_eq!(amp_to_db(2.0), 18.0618, epsilon = 1e-2); // 6 * 3 * log2(2)
    assert_abs_diff_eq!(amp_to_db(0.5), -18.0618, epsilon = 1e-2); // 6 * 3 * log2(0.5)
    Ok(())
}

#[disable]
#[traced_test]
fn test_db_to_amp() -> Result<(), ConvertError> {
    use approx::assert_abs_diff_eq;

    assert_eq!(db_to_amp(-192.0), 0.0); // Clamped to 0.0
    assert_eq!(db_to_amp(0.0), 1.0);
    assert_abs_diff_eq!(db_to_amp(18.0), 2.0, epsilon = 1e-6); // 10^(18 / 30) = 10^0.6
    assert_abs_diff_eq!(db_to_amp(-18.0), 0.5, epsilon = 1e-6); // 10^(-18 / 30) = 10^-0.6
    Ok(())
}

#[traced_test]
fn test_db_to_scamp() -> Result<(), ConvertError> {
    use approx::assert_abs_diff_eq;

    assert_eq!(db_to_scamp(-192.0), 0.0); // Clamped to 0.0
    assert_eq!(db_to_scamp(0.0), 1.0);
    assert_abs_diff_eq!(db_to_scamp(18.0), 0.1995, epsilon = 1e-4); // 10^(-0.9)
    assert_eq!(db_to_scamp(-18.0), 1.0); // 10^(0.9) but clamped to 1.0
    Ok(())
}

#[traced_test]
fn test_linear_to_db() -> Result<(), ConvertError> {
    assert_eq!(linear_to_db(1.0), 0.0);
    assert_eq!(linear_to_db(10.0), 20.0); // 20 * log10(10)
    assert_eq!(linear_to_db(0.1), -20.0); // 20 * log10(0.1)
    Ok(())
}

#[traced_test]
fn test_db_to_linear() -> Result<(), ConvertError> {
    assert_eq!(db_to_linear(0.0), 1.0);
    assert_eq!(db_to_linear(20.0), 10.0); // 10^(20/20)
    assert_eq!(db_to_linear(-20.0), 0.1); // 10^(-20/20)
    Ok(())
}

#[traced_test]
fn test_timecent_to_seconds() -> Result<(), ConvertError> {
    assert_eq!(timecent_to_seconds(0.0), 1.0);
    assert_eq!(timecent_to_seconds(1200.0), 2.0); // 2^1
    assert_eq!(timecent_to_seconds(-1200.0), 0.5); // 2^-1
    Ok(())
}

#[traced_test]
fn test_seconds_to_envtime() -> Result<(), ConvertError> {
    use approx::assert_abs_diff_eq;

    assert_eq!(seconds_to_envtime(0.0), 0.0);
    assert_eq!(seconds_to_envtime(30.0), 1.0); // (30/30)^(1/3)
    assert_abs_diff_eq!(seconds_to_envtime(1.0), 0.3218, epsilon = 1e-4); // (1/30)^(1/3)
    assert_abs_diff_eq!(seconds_to_envtime(0.1), 0.4671, epsilon = 1e-4); // (0.1/30)^(1/3)
    assert_abs_diff_eq!(seconds_to_envtime(15.0), 0.6810, epsilon = 1e-4); // (15/30)^(1/3)
    assert_abs_diff_eq!(seconds_to_envtime(60.0), 1.0, epsilon = 1e-4); // clamped to 1.0
    Ok(())
}

#[traced_test]
fn test_timecent_to_envtime() -> Result<(), ConvertError> {
    assert_eq!(timecent_to_envtime(0.0), 0.0);
    assert_eq!(timecent_to_envtime(1200.0), 1.0);
    assert_eq!(timecent_to_envtime(-1200.0), -1.0);
    Ok(())
}

#[traced_test]
fn test_int7_to_bipolar_float() -> Result<(), ConvertError> {
    use approx::assert_abs_diff_eq;

    assert_eq!(int7_to_bipolar_float(0), -1.0);
    assert_eq!(int7_to_bipolar_float(64), 0.0);
    assert_eq!(int7_to_bipolar_float(127), 1.0);
    assert_eq!(int7_to_bipolar_float(32), -0.5); 
    assert_abs_diff_eq!(int7_to_bipolar_float(96), 0.5079, epsilon = 1e-4);
    Ok(())
}
