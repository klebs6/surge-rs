ix!();

pub fn rand01() -> f32 {
    let rand01: f32 =  ( (unsafe{ libc::rand() }) as i32 / libc::RAND_MAX ) as f32;
    rand01
}

pub fn rand11() -> f32 {
    let rand01: f32 = rand01();
    (rand01 * 2.0) - 1.0
}

pub fn correlated_noise( lastval: f64, correlation: f64) 
    -> f64 
{
    let wf:    f64 = correlation * 0.9;
    let wfabs: f64 = wf.abs();

    let rand11 = rand11() as f64;

    let randt: f64 = rand11 * ( 1.0 - wfabs ) - wf * lastval;

    randt
}

pub fn correlated_noise_mk2(mut lastval: f32, correlation: f32) -> f32 
{
    let wf:     f32 = correlation * 0.9;
    let wfabs:  f32 = wf.abs();
    let m:      f32 = 1.0 / (1.0 - wfabs).sqrt();

    let rand11 = rand11();

    lastval = rand11 * (1.0 - wfabs) - wf * lastval;
    lastval * m
}

pub fn drift_noise(mut lastval: f32) -> f32 
{
    let filter: f32 = 0.00001;
    let m: f32 = 1.0 / filter.sqrt();

    let rand11 = rand11();

    lastval = lastval * (1.0 - filter) + rand11 * filter;
    lastval * m
}

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
