crate::ix!();

/// Generates a random float in the range [0.0, 1.0).
///
pub fn rand01() -> f32 {
    thread_rng().gen_range(0.0..1.0)
}

/// Generates a random float in the range [-1.0, 1.0).
///
pub fn rand11() -> f32 {
    thread_rng().gen_range(-1.0..1.0)
}

/// Generates a correlated noise value.
///
/// # Arguments
///
/// * `lastval` - The last noise value.
/// * `correlation` - The correlation factor.
pub fn correlated_noise( lastval: f64, correlation: f64) 
    -> f64 
{
    let wf:    f64 = correlation * 0.9;
    let wfabs: f64 = wf.abs();

    let rand11 = rand11() as f64;

    let randt: f64 = rand11 * ( 1.0 - wfabs ) - wf * lastval;

    randt
}

/// Generates a correlated noise value (mk2 version).
///
/// # Arguments
///
/// * `lastval` - The last noise value.
/// * `correlation` - The correlation factor.
pub fn correlated_noise_mk2(mut lastval: f32, correlation: f32) -> f32 
{
    let wf:     f32 = correlation * 0.9;
    let wfabs:  f32 = wf.abs();
    let m:      f32 = 1.0 / (1.0 - wfabs).sqrt();

    let rand11 = rand11();

    lastval = rand11 * (1.0 - wfabs) - wf * lastval;
    lastval * m
}

/// Generates a drift noise value.
///
/// # Arguments
///
/// * `lastval` - The last noise value.
pub fn drift_noise(mut lastval: f32) -> f32 
{
    let filter: f32 = 0.00001;
    let m: f32 = 1.0 / filter.sqrt();

    let rand11 = rand11();

    lastval = lastval * (1.0 - filter) + rand11 * filter;
    lastval * m
}

/// Generates a correlated noise value (o2 version).
///
/// # Arguments
///
/// * `lastval` - The last noise value.
/// * `lastval2` - The second last noise value.
/// * `correlation` - The correlation factor.
pub fn correlated_noise_o2(lastval: f32, mut lastval2: f32, correlation: f32) -> f32 
{
    let wf:     f32 = correlation * 0.9;
    let wfabs:  f32 = wf.abs();
    let rand11 = rand11();
    let mut randt:  f32 = rand11 * (1.0 - wfabs) - wf * lastval2;

    lastval2 = randt;
    randt = lastval2 * (1.0 - wfabs) - wf * lastval;
    randt
}

/// Generates a correlated noise value (o2mk2 version).
///
/// # Arguments
///
/// * `lastval` - The last noise value.
/// * `lastval2` - The second last noise value.
/// * `correlation` - The correlation factor.
pub fn correlated_noise_o2mk2(mut lastval: f32, mut lastval2: f32, correlation: f32) -> f32 
{
    let mut wf:    f32 = correlation;
    let mut wfabs: f32 = wf.abs() * 0.8;

    wfabs = 2.0 * wfabs - wfabs * wfabs;

    if wf > 0.0 {
        wf = wfabs;
    } else {
        wf = -wfabs;
    }

    let m = if cfg!(target_os = "macos" ) {
        let m: f32 = 1.0 / (1.0 - wfabs).sqrt();
        m

    } else {
        let mut m: f32 = 1.0 - wfabs;

        unsafe {
            let m1: __m128 = _mm_rsqrt_ss(_mm_load_ss(&m));
            _mm_store_ss(&mut m, m1);
            m
        }
    };

    let rand11 = rand11();
    lastval2 = rand11 * (1.0 - wfabs) - wf * lastval2;
    lastval = lastval2 * (1.0 - wfabs) - wf * lastval;
    lastval * m
}
